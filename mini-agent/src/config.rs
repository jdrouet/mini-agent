use std::collections::HashMap;
use std::path::Path;

use tokio::task::JoinHandle;

use crate::source::OuterSourceConfig;

pub enum Component {
    Source(crate::source::Source),
    Sink(crate::sink::Sink),
}

impl mini_agent_core::prelude::Component for Component {
    async fn run(self) {
        match self {
            Self::Source(inner) => inner.run().await,
            Self::Sink(inner) => inner.run().await,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct Config {
    pub sources: HashMap<String, OuterSourceConfig>,
    pub sinks: HashMap<String, crate::sink::SinkConfig>,
}

impl Config {
    pub fn from_file(path: &Path) -> Self {
        let config = std::fs::read_to_string(path).unwrap();
        toml::from_str(&config).unwrap()
    }

    fn components(self) -> Vec<(String, Component)> {
        let Config { sources, sinks } = self;

        let mut components = Vec::with_capacity(sources.len() + sinks.len());

        let mut targets = HashMap::with_capacity(sinks.len());

        for (key, value) in sinks.into_iter() {
            use crate::sink::prelude::SinkConfig;

            let (runner, sender) = value.build();
            let name = format!("sinks.{key}");
            targets.insert(key, sender);
            components.push((name, Component::Sink(runner)));
        }

        for (key, value) in sources.into_iter() {
            use crate::source::prelude::SourceConfig;

            let sender = targets.remove(&value.target).unwrap();
            let runner = value.inner.build(sender);
            let name = format!("sources.{key}");
            components.push((name, Component::Source(runner)));
        }

        components
    }

    pub fn build(self) -> HashMap<String, JoinHandle<()>> {
        use mini_agent_core::prelude::Component;

        self.components()
            .into_iter()
            .map(|(name, component)| (name, tokio::spawn(async { component.run().await })))
            .collect::<HashMap<_, _>>()
    }
}
