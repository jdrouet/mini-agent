use lipsum::lipsum_words;
use mini_agent_core::event::{Event, EventLog};
use mini_agent_source_prelude::prelude::SourceConfig;
use mini_agent_source_prelude::timer;
use tokio::sync::mpsc;

#[derive(Debug, serde::Deserialize)]
pub struct RandomLogsConfig {
    pub interval: f64,
}

impl SourceConfig for RandomLogsConfig {
    type Output = RandomLogs;

    fn build(self, output: mpsc::Sender<Event>) -> Self::Output {
        RandomLogs {
            interval: tokio::time::interval(tokio::time::Duration::from_secs_f64(self.interval)),
            output,
            executor: RandomLogsExecutor,
        }
    }
}

pub type RandomLogs = timer::Timer<RandomLogsExecutor>;

pub struct RandomLogsExecutor;

impl timer::Executor for RandomLogsExecutor {
    async fn execute(&mut self, output: mpsc::Sender<Event>) {
        let log = EventLog::now().with_message(lipsum_words(5));
        if let Err(err) = output.send(log.into()).await {
            eprintln!("unable to send event: {err:?}");
        }
    }
}
