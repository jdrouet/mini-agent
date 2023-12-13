use std::time::SystemTime;

use crate::time::{now, Timestamp};

#[derive(Clone, Default, Debug, serde::Deserialize, serde::Serialize)]
pub struct Log {
    #[serde(flatten)]
    pub inner: serde_json::value::Map<String, serde_json::value::Value>,
}

impl Log {
    pub fn now() -> Self {
        Self::default().with_timestamp(now())
    }

    pub fn with_timestamp(self, timestamp: Timestamp) -> Self {
        let value = timestamp
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        self.with("ts", value)
    }

    #[inline]
    pub fn with<N: Into<String>, V: Into<serde_json::value::Value>>(
        mut self,
        name: N,
        value: V,
    ) -> Self {
        self.inner.insert(name.into(), value.into());
        self
    }

    pub fn with_message<M: Into<serde_json::value::Value>>(self, message: M) -> Self {
        self.with("message", message)
    }
}
