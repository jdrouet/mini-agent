use tokio::sync::mpsc;

use crate::event::Event;

pub(crate) trait SourceConfig {
    fn build(self, sender: mpsc::Sender<Event>) -> super::Source;
}
