use mini_agent_core::event::Event;
use tokio::sync::mpsc;

pub(crate) trait SourceConfig {
    fn build(self, sender: mpsc::Sender<Event>) -> super::Source;
}
