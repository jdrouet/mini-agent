pub type Timestamp = std::time::SystemTime;

pub fn now() -> Timestamp {
    std::time::SystemTime::now()
}
