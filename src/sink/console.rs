use tokio::sync::mpsc;

use crate::event::Event;

use super::prelude::{SinkConfig, BUFFER_SIZE};

#[derive(Debug, serde::Deserialize)]
pub struct ConsoleConfig;

impl SinkConfig for ConsoleConfig {
    fn build(self) -> (super::Sink, mpsc::Sender<crate::event::Event>) {
        let (sender, receiver) = mpsc::channel(BUFFER_SIZE);

        (super::Sink::Console(Console { receiver }), sender)
    }
}

pub struct Console {
    receiver: mpsc::Receiver<Event>,
}

impl crate::prelude::Component for Console {
    async fn run(mut self) {
        while let Some(received) = self.receiver.recv().await {
            println!("event: {received:?}");
        }
    }
}
