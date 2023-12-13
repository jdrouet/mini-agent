use mini_agent_core::event::{Event, EventLog, EventMetric};
use mini_agent_sink_prelude::batch::{Executor, SinkBatch};
use tokio::sync::mpsc;

pub const BUFFER_SIZE: usize = 100;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DatadogRegion {
    Eu,
    Us1,
    Us3,
}

impl DatadogRegion {
    pub fn metric_url(&self) -> &'static str {
        match self {
            Self::Eu => "https://api.datadoghq.eu/api/v2/series",
            Self::Us1 => "https://api.datadoghq.com/api/v2/series",
            Self::Us3 => "https://api.us3.datadoghq.com/api/v2/series",
        }
    }

    pub fn logs_url(&self) -> &'static str {
        match self {
            Self::Eu => "https://http-intake.logs.datadoghq.eu/api/v2/logs",
            Self::Us1 => "https://http-intake.logs.datadoghq.com/api/v2/logs",
            Self::Us3 => "https://http-intake.logs.us3.datadoghq.com/api/v2/logs",
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct DatadogConfig {
    region: DatadogRegion,
    token: String,
}

impl mini_agent_sink_prelude::prelude::SinkConfig for DatadogConfig {
    type Output = Datadog;

    fn build(self) -> (Self::Output, mpsc::Sender<mini_agent_core::event::Event>) {
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
            SinkBatch {
                receiver,
                batch_size: 10,
                executor: DatadogExecutor {
                    region: self.region,
                    client: reqwest::ClientBuilder::new()
                        .default_headers(headers)
                        .build()
                        .unwrap(),
                },
            },
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

impl From<EventMetric> for DatadogMetricSerie {
    fn from(value: EventMetric) -> Self {
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
    region: DatadogRegion,
    client: reqwest::Client,
}

impl DatadogExecutor {
    async fn push_logs(&self, logs: Vec<EventLog>) {
        match self
            .client
            .post(self.region.logs_url())
            .json(&logs)
            .send()
            .await
        {
            Ok(res) => match res.error_for_status() {
                Ok(_) => (),
                Err(err) => eprintln!("unable to send logs: {err:?}"),
            },
            Err(err) => eprintln!("unable to send logs: {err:?}"),
        }
    }

    async fn push_metrics(&self, metrics: Vec<EventMetric>) {
        let payload = DatadogMetricPayload {
            series: metrics.into_iter().map(DatadogMetricSerie::from).collect(),
        };
        match self
            .client
            .post(self.region.metric_url())
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
        let mut logs = Vec::with_capacity(inputs.len());

        while let Some(event) = inputs.pop() {
            match event {
                Event::Metric(inner) => metrics.push(inner),
                Event::Log(inner) => logs.push(inner),
            }
        }

        self.push_metrics(metrics).await;
        self.push_logs(logs).await;
    }
}
