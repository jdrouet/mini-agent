use mini_agent_core::event::Event;
use mini_agent_core::prelude::Component;
use tokio::sync::mpsc;

pub trait Transform: Component {}

pub trait TransformConfig {
    type Output: Transform;

    fn build(self, sender: mpsc::Sender<Event>) -> (Self::Output, mpsc::Sender<Event>);
}
