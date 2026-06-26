#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiStatePersistencePosture {
    RuntimeOnly,
    SessionRecorded,
    WorkspaceRecordedForLater,
}
