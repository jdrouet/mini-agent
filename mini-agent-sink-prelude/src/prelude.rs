use mini_agent_core::event::Event;
use mini_agent_core::prelude::Component;
use tokio::sync::mpsc;

pub trait Sink: Component {}

pub trait SinkConfig {
    type Output: Sink;

    fn build(self) -> (Self::Output, mpsc::Sender<Event>);
}
