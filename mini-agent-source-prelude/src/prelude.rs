use mini_agent_core::event::Event;
use mini_agent_core::prelude::Component;
use tokio::sync::mpsc;

pub trait Source: Component {}

pub trait SourceConfig {
    type Output: Source;

    fn build(self, sender: mpsc::Sender<Event>) -> Self::Output;
}
