#![allow(async_fn_in_trait)]

use std::fmt::Display;

#[derive(Debug)]
pub enum ComponentKind {
    Source,
    Transform,
    Sink,
}

impl Display for ComponentKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Source => write!(f, "source"),
            Self::Transform => write!(f, "transform"),
            Self::Sink => write!(f, "sink"),
        }
    }
}

pub trait Component: Sized {
    fn component_kind(&self) -> ComponentKind;
    async fn run(self);
}
