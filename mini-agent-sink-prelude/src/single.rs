#![allow(async_fn_in_trait)]

use mini_agent_core::event::Event;
use mini_agent_core::prelude::{Component, ComponentKind};
use tokio::sync::mpsc;

pub trait Executor {
    async fn execute(&mut self, input: Event);
}

pub struct SinkSingle<E> {
    pub receiver: mpsc::Receiver<Event>,
    pub executor: E,
}

impl<E: Executor> Component for SinkSingle<E> {
    fn component_kind(&self) -> ComponentKind {
        ComponentKind::Sink
    }

    async fn run(mut self) {
        tracing::info!("starting");
        while let Some(event) = self.receiver.recv().await {
            self.executor.execute(event).await;
        }
        tracing::info!("done");
    }
}

impl<E: Executor> crate::prelude::Sink for SinkSingle<E> {}
