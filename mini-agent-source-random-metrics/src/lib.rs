use mini_agent_core::event::{Event, EventMetric};
use mini_agent_source_prelude::prelude::SourceConfig;
use mini_agent_source_prelude::timer;
use tokio::sync::mpsc;

#[derive(Debug, serde::Deserialize)]
pub struct RandomMetricsConfig {
    pub interval: f64,
}

impl SourceConfig for RandomMetricsConfig {
    type Output = RandomMetrics;

    fn build(self, output: mpsc::Sender<Event>) -> Self::Output {
        RandomMetrics {
            interval: tokio::time::interval(tokio::time::Duration::from_secs_f64(self.interval)),
            output,
            executor: RandomMetricsExecutor,
        }
    }
}

pub type RandomMetrics = timer::Timer<RandomMetricsExecutor>;

pub struct RandomMetricsExecutor;

impl timer::Executor for RandomMetricsExecutor {
    async fn execute(&mut self, output: mpsc::Sender<Event>) {
        if let Err(err) = output
            .send(Event::Metric(EventMetric::now("instant", 0.0)))
            .await
        {
            eprintln!("unable to send event: {err:?}");
        }
    }
}
