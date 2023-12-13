#![allow(async_fn_in_trait)]

use mini_agent_core::event::Event;
use mini_agent_core::prelude::Component;
use tokio::sync::mpsc;

pub trait Executor {
    async fn execute(&mut self, input: Event);
}

pub struct SinkSingle<E> {
    pub receiver: mpsc::Receiver<Event>,
    pub executor: E,
}

impl<E: Executor> Component for SinkSingle<E> {
    async fn run(mut self) {
        while let Some(event) = self.receiver.recv().await {
            self.executor.execute(event).await;
        }
    }
}

impl<E: Executor> crate::prelude::Sink for SinkSingle<E> {}
