#![allow(async_fn_in_trait)]

use mini_agent_core::event::Event;
use tokio::sync::mpsc;

pub const BUFFER_SIZE: usize = 100;

pub trait Executor {
    async fn execute(&mut self, inputs: &[Event]);
}

pub struct SinkBatch<E> {
    pub receiver: mpsc::Receiver<Event>,
    pub executor: E,
    pub batch_size: usize,
}

impl<E: Executor> SinkBatch<E> {
    pub async fn run(mut self) {
        let mut buffer = Vec::with_capacity(self.batch_size);
        loop {
            let _count = self.receiver.recv_many(&mut buffer, self.batch_size).await;
            self.executor.execute(&buffer).await;
        }
    }
}
