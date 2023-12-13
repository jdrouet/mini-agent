use mini_agent_core::event::Event;
use mini_agent_sink_prelude::single::{Executor, SinkSingle};
use tokio::sync::mpsc;

pub const BUFFER_SIZE: usize = 100;

#[derive(Debug, serde::Deserialize)]
pub struct ConsoleConfig;

impl mini_agent_sink_prelude::prelude::SinkConfig for ConsoleConfig {
    type Output = Console;

    fn build(self) -> (Self::Output, mpsc::Sender<mini_agent_core::event::Event>) {
        let (sender, receiver) = mpsc::channel(BUFFER_SIZE);

        (
            Console {
                receiver,
                executor: ConsoleExecutor,
            },
            sender,
        )
    }
}

pub type Console = SinkSingle<ConsoleExecutor>;

pub struct ConsoleExecutor;

impl Executor for ConsoleExecutor {
    async fn execute(&mut self, input: Event) {
        println!("event: {input:?}");
    }
}
