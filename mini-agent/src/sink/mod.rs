pub(crate) mod console;
pub(crate) mod datadog;

pub(crate) mod prelude;

pub(crate) enum Sink {
    Console(console::Console),
    Datadog(datadog::Datadog),
}

impl mini_agent_core::prelude::Component for Sink {
    async fn run(self) {
        match self {
            Self::Console(inner) => inner.run().await,
            Self::Datadog(inner) => inner.run().await,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub(crate) enum SinkConfig {
    Console(console::ConsoleConfig),
    Datadog(datadog::DatadogConfig),
}

impl prelude::SinkConfig for SinkConfig {
    fn build(
        self,
    ) -> (
        Sink,
        tokio::sync::mpsc::Sender<mini_agent_core::event::Event>,
    ) {
        match self {
            Self::Console(inner) => inner.build(),
            Self::Datadog(inner) => inner.build(),
        }
    }
}
