use crate::replay::data::RelationalReplayRecord;

#[derive(Debug, Clone)]
pub(crate) struct PublicationArtifacts {
    pub(crate) snapshot: crate::snapshots::data::SnapshotHandle,
    pub(crate) diagnostics_summary: crate::diagnostics::data::RelationalDiagnosticArtifact,
    pub(crate) bundle: crate::publication::data::PublicationBundle<RelationalReplayRecord>,
}
