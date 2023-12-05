use mini_agent_core::event::{Event, Metric};
use tokio::sync::mpsc;

use super::prelude::SourceConfig;
use crate::prelude::Component;

#[derive(Debug, serde::Deserialize)]
pub struct TimerConfig {
    pub interval: f64,
}

impl SourceConfig for TimerConfig {
    fn build(self, output: mpsc::Sender<Event>) -> super::Source {
        super::Source::Timer(Timer {
            interval: tokio::time::interval(tokio::time::Duration::from_secs_f64(self.interval)),
            output,
        })
    }
}

pub struct Timer {
    interval: tokio::time::Interval,
    output: mpsc::Sender<Event>,
}

impl Component for Timer {
    async fn run(mut self) {
        loop {
            let _ = self.interval.tick().await;
            let event = Metric::now("tick", 0.0);
            if let Err(err) = self.output.send(event.into()).await {
                eprintln!("unable to send event: {err:?}");
            }
        }
    }
}
