use tokio::sync::mpsc;

use super::prelude::{SinkConfig, BUFFER_SIZE};
use crate::event::{Event, Metric};

const BATCH_SIZE: usize = 50;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DatadogRegion {
    Eu,
    Us1,
}

impl DatadogRegion {
    pub fn base_url(&self) -> &'static str {
        match self {
            Self::Eu => "https://api.datadoghq.eu",
            Self::Us1 => "https://api.datadoghq.com",
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct DatadogConfig {
    region: DatadogRegion,
    token: String,
}

impl SinkConfig for DatadogConfig {
    fn build(self) -> (super::Sink, mpsc::Sender<crate::event::Event>) {
        let (sender, receiver) = mpsc::channel(BUFFER_SIZE);

        use reqwest::header;
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Accept",
            header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            "Content-Type",
            header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            "DD-API-KEY",
            header::HeaderValue::from_str(self.token.as_str()).unwrap(),
        );

        (
            super::Sink::Datadog(Datadog {
                receiver,
                metrics_url: format!("{}/api/v2/series", self.region.base_url()),
                client: reqwest::ClientBuilder::new()
                    .default_headers(headers)
                    .build()
                    .unwrap(),
            }),
            sender,
        )
    }
}

#[derive(serde::Serialize)]
pub struct DatadogMetricSeriePoint {
    timestamp: u64,
    value: f64,
}

#[derive(serde::Serialize)]
pub struct DatadogMetricSerie {
    metric: String,
    points: Vec<DatadogMetricSeriePoint>,
    tags: Vec<String>,
    #[serde(rename = "type")]
    kind: u8,
}

#[derive(serde::Serialize)]
pub struct DatadogMetricPayload {
    series: Vec<DatadogMetricSerie>,
}

impl From<Metric> for DatadogMetricSerie {
    fn from(value: Metric) -> Self {
        let tags = value
            .tags
            .into_iter()
            .map(|(key, value)| format!("{key}:{value}"))
            .collect();
        Self {
            metric: value.name,
            points: vec![DatadogMetricSeriePoint {
                timestamp: value
                    .timestamp
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                value: value.value,
            }],
            tags,
            kind: 3,
        }
    }
}

pub struct Datadog {
    receiver: mpsc::Receiver<Event>,
    client: reqwest::Client,
    metrics_url: String,
}

impl Datadog {
    async fn push_metrics(&self, metrics: Vec<Metric>) {
        let payload = DatadogMetricPayload {
            series: metrics.into_iter().map(DatadogMetricSerie::from).collect(),
        };
        match self
            .client
            .post(&self.metrics_url)
            .json(&payload)
            .send()
            .await
        {
            Ok(res) => match res.error_for_status() {
                Ok(_) => (),
                Err(err) => eprintln!("unable to send metrics: {err:?}"),
            },
            Err(err) => eprintln!("unable to send metrics: {err:?}"),
        }
    }
}

impl crate::prelude::Component for Datadog {
    async fn run(mut self) {
        let mut buffer = Vec::with_capacity(BATCH_SIZE);
        loop {
            let _ = self.receiver.recv_many(&mut buffer, BATCH_SIZE).await;

            let mut metrics = Vec::with_capacity(BATCH_SIZE);

            while let Some(event) = buffer.pop() {
                match event {
                    Event::Metric(inner) => metrics.push(inner),
                }
            }

            self.push_metrics(metrics).await;
        }
    }
}
