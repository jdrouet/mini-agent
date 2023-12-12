pub(crate) enum Sink {
    Console(mini_agent_sink_console::Console),
    Datadog(mini_agent_sink_datadog::Datadog),
}

impl mini_agent_core::prelude::Component for Sink {
    async fn run(self) {
        match self {
            Self::Console(inner) => inner.run().await,
            Self::Datadog(inner) => inner.run().await,
        }
    }
}

impl mini_agent_sink_prelude::prelude::Sink for Sink {}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub(crate) enum SinkConfig {
    Console(mini_agent_sink_console::ConsoleConfig),
    Datadog(mini_agent_sink_datadog::DatadogConfig),
}

impl mini_agent_sink_prelude::prelude::SinkConfig for SinkConfig {
    type Output = Sink;

    fn build(
        self,
    ) -> (
        Sink,
        tokio::sync::mpsc::Sender<mini_agent_core::event::Event>,
    ) {
        match self {
            Self::Console(inner) => {
                let (sink, sender) = inner.build();
                (Sink::Console(sink), sender)
            }
            Self::Datadog(inner) => {
                let (sink, sender) = inner.build();
                (Sink::Datadog(sink), sender)
            }
        }
    }
}
