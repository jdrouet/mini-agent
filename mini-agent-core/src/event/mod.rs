mod log;
mod metric;

pub use log::Log;
pub use metric::Metric;

#[derive(Clone, Debug)]
pub enum Event {
    Log(Log),
    Metric(Metric),
}

impl From<Log> for Event {
    fn from(value: Log) -> Self {
        Self::Log(value)
    }
}

impl From<Metric> for Event {
    fn from(value: Metric) -> Self {
        Self::Metric(value)
    }
}
