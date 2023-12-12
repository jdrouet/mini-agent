use mini_agent_core::event::{Event, Metric};
use mini_agent_source_prelude::prelude::SourceConfig;
use mini_agent_source_prelude::timer;
use tokio::sync::mpsc;

#[derive(Debug, serde::Deserialize)]
pub struct TimerConfig {
    pub interval: f64,
}

impl SourceConfig for TimerConfig {
    type Output = Timer;

    fn build(self, output: mpsc::Sender<Event>) -> Self::Output {
        Timer {
            interval: tokio::time::interval(tokio::time::Duration::from_secs_f64(self.interval)),
            output,
            executor: TimerExecutor,
        }
    }
}

pub type Timer = timer::Timer<TimerExecutor>;

pub struct TimerExecutor;

impl timer::Executor for TimerExecutor {
    async fn execute(&mut self, output: mpsc::Sender<Event>) {
        if let Err(err) = output
            .send(Event::Metric(Metric::now("instant", 0.0)))
            .await
        {
            eprintln!("unable to send event: {err:?}");
        }
    }
}
