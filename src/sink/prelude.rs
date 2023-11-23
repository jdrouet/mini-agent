use tokio::sync::mpsc;

use crate::event::Event;

pub const BUFFER_SIZE: usize = 100;

pub trait SinkConfig {
    fn build(self) -> (super::Sink, mpsc::Sender<Event>);
}
