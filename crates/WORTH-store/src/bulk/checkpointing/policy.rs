#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BulkCheckpointPolicy {
    Skip,
    Publish,
}

impl BulkCheckpointPolicy {
    pub fn should_publish(self) -> bool {
        matches!(self, Self::Publish)
    }
}
