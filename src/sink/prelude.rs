use tokio::sync::mpsc;

use crate::event::Event;

pub trait SinkConfig {
    fn build(self) -> (super::Sink, mpsc::Sender<Event>);
}
