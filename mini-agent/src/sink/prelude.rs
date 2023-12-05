use mini_agent_core::event::Event;
use tokio::sync::mpsc;

pub const BUFFER_SIZE: usize = 100;

pub trait SinkConfig {
    fn build(self) -> (super::Sink, mpsc::Sender<Event>);
}
