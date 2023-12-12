use mini_agent_core::event::Event;
use tokio::sync::mpsc;

pub const BUFFER_SIZE: usize = 100;

#[derive(Debug, serde::Deserialize)]
pub struct ConsoleConfig;

impl mini_agent_sink_prelude::prelude::SinkConfig for ConsoleConfig {
    type Output = Console;

    fn build(self) -> (Self::Output, mpsc::Sender<mini_agent_core::event::Event>) {
        let (sender, receiver) = mpsc::channel(BUFFER_SIZE);

        (Console { receiver }, sender)
    }
}

pub struct Console {
    receiver: mpsc::Receiver<Event>,
}

impl mini_agent_core::prelude::Component for Console {
    async fn run(mut self) {
        while let Some(received) = self.receiver.recv().await {
            println!("event: {received:?}");
        }
    }
}

impl mini_agent_sink_prelude::prelude::Sink for Console {}
