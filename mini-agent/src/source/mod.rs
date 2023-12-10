pub(crate) mod sysinfo;
pub(crate) mod timer;

pub(crate) mod prelude;

pub enum Source {
    Sysinfo(sysinfo::Sysinfo),
    Timer(timer::Timer),
}

impl mini_agent_core::prelude::Component for Source {
    async fn run(self) {
        match self {
            Self::Sysinfo(inner) => inner.run().await,
            Self::Timer(inner) => inner.run().await,
        }
    }
}

impl From<timer::Timer> for Source {
    fn from(value: timer::Timer) -> Self {
        Self::Timer(value)
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub(crate) enum SourceConfig {
    Sysinfo(sysinfo::SysinfoConfig),
    Timer(timer::TimerConfig),
}

impl prelude::SourceConfig for SourceConfig {
    fn build(self, sender: tokio::sync::mpsc::Sender<mini_agent_core::event::Event>) -> Source {
        match self {
            Self::Sysinfo(inner) => inner.build(sender),
            Self::Timer(inner) => inner.build(sender),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct OuterSourceConfig {
    pub target: String,
    #[serde(flatten)]
    pub inner: SourceConfig,
}
