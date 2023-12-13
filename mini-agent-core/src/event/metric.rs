use std::borrow::Cow;
use std::collections::BTreeMap;

use crate::time::{now, Timestamp};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Metric {
    pub timestamp: Timestamp,
    pub name: String,
    pub tags: BTreeMap<Cow<'static, str>, Cow<'static, str>>,
    pub value: f64,
}

impl Metric {
    pub fn now<N: Into<String>>(name: N, value: f64) -> Self {
        Self {
            timestamp: now(),
            name: name.into(),
            tags: Default::default(),
            value,
        }
    }

    pub fn with_optional_tag<K: Into<Cow<'static, str>>, V: Into<Cow<'static, str>>>(
        self,
        name: K,
        value: Option<V>,
    ) -> Self {
        if let Some(inner) = value {
            self.with_tag(name, inner)
        } else {
            self
        }
    }

    pub fn with_tag<K: Into<Cow<'static, str>>, V: Into<Cow<'static, str>>>(
        mut self,
        name: K,
        value: V,
    ) -> Self {
        self.tags.insert(name.into(), value.into());
        self
    }

    pub fn with_tags(mut self, tags: Vec<(&'static str, String)>) -> Self {
        for (name, value) in tags {
            self.tags.insert(name.into(), value.into());
        }
        self
    }
}
