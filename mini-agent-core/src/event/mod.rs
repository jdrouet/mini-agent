mod log;
mod metric;

pub use log::EventLog;
pub use metric::EventMetric;

#[derive(Clone, Debug)]
pub enum Event {
    Log(EventLog),
    Metric(EventMetric),
}

impl From<EventLog> for Event {
    fn from(value: EventLog) -> Self {
        Self::Log(value)
    }
}

impl From<EventMetric> for Event {
    fn from(value: EventMetric) -> Self {
        Self::Metric(value)
    }
}
