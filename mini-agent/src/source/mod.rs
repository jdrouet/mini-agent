use mini_agent_core::event::Event;
use tokio::sync::mpsc::Sender;

pub enum Source {
    HttpServer(mini_agent_source_http_server::HttpServer),
    RandomLogs(mini_agent_source_random_logs::RandomLogs),
    Sysinfo(mini_agent_source_sysinfo::Sysinfo),
    Timer(mini_agent_source_timer::Timer),
}

impl mini_agent_core::prelude::Component for Source {
    async fn run(self) {
        match self {
            Self::HttpServer(inner) => inner.run().await,
            Self::RandomLogs(inner) => inner.run().await,
            Self::Sysinfo(inner) => inner.run().await,
            Self::Timer(inner) => inner.run().await,
        }
    }
}

impl mini_agent_source_prelude::prelude::Source for Source {}

impl From<mini_agent_source_http_server::HttpServer> for Source {
    fn from(value: mini_agent_source_http_server::HttpServer) -> Self {
        Self::HttpServer(value)
    }
}

impl From<mini_agent_source_timer::Timer> for Source {
    fn from(value: mini_agent_source_timer::Timer) -> Self {
        Self::Timer(value)
    }
}

impl From<mini_agent_source_sysinfo::Sysinfo> for Source {
    fn from(value: mini_agent_source_sysinfo::Sysinfo) -> Self {
        Self::Sysinfo(value)
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub(crate) enum SourceConfig {
    HttpServer(mini_agent_source_http_server::HttpServerConfig),
    RandomLogs(mini_agent_source_random_logs::RandomLogsConfig),
    Sysinfo(mini_agent_source_sysinfo::SysinfoConfig),
    Timer(mini_agent_source_timer::TimerConfig),
}

impl mini_agent_source_prelude::prelude::SourceConfig for SourceConfig {
    type Output = Source;

    fn build(self, sender: Sender<Event>) -> Source {
        match self {
            Self::HttpServer(inner) => Source::HttpServer(inner.build(sender)),
            Self::RandomLogs(inner) => Source::RandomLogs(inner.build(sender)),
            Self::Sysinfo(inner) => Source::Sysinfo(inner.build(sender)),
            Self::Timer(inner) => Source::Timer(inner.build(sender)),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct OuterSourceConfig {
    pub target: String,
    #[serde(flatten)]
    pub inner: SourceConfig,
}
