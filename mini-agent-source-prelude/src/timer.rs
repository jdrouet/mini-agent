#![allow(async_fn_in_trait)]

use mini_agent_core::event::Event;
use mini_agent_core::prelude::Component;
use tokio::sync::mpsc;

use crate::prelude::Source;

pub trait Executor {
    async fn execute(&mut self, output: mpsc::Sender<Event>);
}

pub struct Timer<E> {
    pub interval: tokio::time::Interval,
    pub output: mpsc::Sender<Event>,
    pub executor: E,
}

impl<E> Timer<E> {}

impl<E: Executor> Component for Timer<E> {
    async fn run(mut self) {
        loop {
            let _ = self.interval.tick().await;
            self.executor.execute(self.output.clone()).await;
        }
    }
}

impl<E: Executor> Source for Timer<E> {}
