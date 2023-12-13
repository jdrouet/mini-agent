use mini_agent_core::event::Event;
use mini_agent_core::prelude::Component;
use mini_agent_transform_prelude::prelude::{Transform, TransformConfig};
use tokio::sync::mpsc;

pub const BUFFER_SIZE: usize = 100;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterMethodConfig {
    IsLog,
    IsMetric,
}

#[derive(Debug, serde::Deserialize)]
pub struct FilterConfig {
    pub method: FilterMethodConfig,
}

impl TransformConfig for FilterConfig {
    type Output = Filter;

    fn build(self, output: mpsc::Sender<Event>) -> (Self::Output, mpsc::Sender<Event>) {
        let (sender, receiver) = mpsc::channel(BUFFER_SIZE);
        (
            Filter {
                input: receiver,
                output,
                method: self.method.into(),
            },
            sender,
        )
    }
}

impl From<FilterMethodConfig> for FilterMethod {
    fn from(value: FilterMethodConfig) -> Self {
        match value {
            FilterMethodConfig::IsLog => Self::IsLog,
            FilterMethodConfig::IsMetric => Self::IsMetric,
        }
    }
}

pub enum FilterMethod {
    IsLog,
    IsMetric,
}

impl FilterMethod {
    fn matches(&self, event: &Event) -> bool {
        match self {
            Self::IsLog if event.is_log() => true,
            Self::IsMetric if event.is_metric() => true,
            _ => false,
        }
    }
}

pub struct Filter {
    input: mpsc::Receiver<Event>,
    output: mpsc::Sender<Event>,
    method: FilterMethod,
}

impl Component for Filter {
    async fn run(mut self) {
        while let Some(event) = self.input.recv().await {
            if self.method.matches(&event) {
                if let Err(err) = self.output.send(event).await {
                    eprint!("couldn't transform event {err:?}");
                }
            }
        }
    }
}

impl Transform for Filter {}
