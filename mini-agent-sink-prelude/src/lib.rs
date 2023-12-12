#![allow(async_fn_in_trait)]

use mini_agent_core::event::Event;
use mini_agent_core::prelude::Component;
use tokio::sync::mpsc;

pub mod prelude;

pub const BUFFER_SIZE: usize = 100;

pub trait Executor {
    async fn execute(&mut self, inputs: Vec<Event>);
}

pub struct SinkBatch<E> {
    pub receiver: mpsc::Receiver<Event>,
    pub executor: E,
    pub batch_size: usize,
}

impl<E: Executor> Component for SinkBatch<E> {
    async fn run(mut self) {
        loop {
            let mut buffer = Vec::with_capacity(self.batch_size);
            let _count = self.receiver.recv_many(&mut buffer, self.batch_size).await;
            self.executor.execute(buffer).await;
        }
    }
}

impl<E: Executor> crate::prelude::Sink for SinkBatch<E> {}
