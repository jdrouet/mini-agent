use mini_agent_core::event::Event;
use tokio::sync::mpsc::Sender;

pub enum Transform {
    Filter(mini_agent_transform_filter::Filter),
}

impl mini_agent_core::prelude::Component for Transform {
    fn component_kind(&self) -> mini_agent_core::prelude::ComponentKind {
        mini_agent_core::prelude::ComponentKind::Transform
    }

    async fn run(self) {
        match self {
            Self::Filter(inner) => inner.run().await,
        }
    }
}

impl mini_agent_transform_prelude::prelude::Transform for Transform {}

impl From<mini_agent_transform_filter::Filter> for Transform {
    fn from(value: mini_agent_transform_filter::Filter) -> Self {
        Self::Filter(value)
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub(crate) enum TransformConfig {
    Filter(mini_agent_transform_filter::FilterConfig),
}

impl mini_agent_transform_prelude::prelude::TransformConfig for TransformConfig {
    type Output = Transform;

    fn build(self, sender: Sender<Event>) -> (Transform, Sender<Event>) {
        match self {
            Self::Filter(inner) => {
                let (component, output) = inner.build(sender);
                (Transform::Filter(component), output)
            }
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct OuterTransformConfig {
    pub target: String,
    #[serde(flatten)]
    pub inner: TransformConfig,
}
