use mini_agent_core::event::{Event, Metric};
use mini_agent_sink_prelude::{Executor, SinkBatch};
use tokio::sync::mpsc;

use super::prelude::{SinkConfig, BUFFER_SIZE};

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
    fn build(self) -> (super::Sink, mpsc::Sender<mini_agent_core::event::Event>) {
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
            super::Sink::Datadog(SinkBatch {
                receiver,
                batch_size: 10,
                executor: DatadogExecutor {
                    metrics_url: format!("{}/api/v2/series", self.region.base_url()),
                    client: reqwest::ClientBuilder::new()
                        .default_headers(headers)
                        .build()
                        .unwrap(),
                },
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

pub type Datadog = SinkBatch<DatadogExecutor>;

pub struct DatadogExecutor {
    client: reqwest::Client,
    metrics_url: String,
}

impl DatadogExecutor {
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

impl Executor for DatadogExecutor {
    async fn execute(&mut self, mut inputs: Vec<Event>) {
        let mut metrics = Vec::with_capacity(inputs.len());

        while let Some(event) = inputs.pop() {
            match event {
                Event::Metric(inner) => metrics.push(inner),
            }
        }

        self.push_metrics(metrics).await;
    }
}
