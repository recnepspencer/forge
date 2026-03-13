use crate::authority::commit::preparation::reduction::keys::PostCommitReductionKey;
use crate::diagnostics::data::RelationalDiagnosticsEntry;
use crate::snapshots::data::SnapshotId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PostCommitConsumerKind {
    PublicationDiagnostic,
    PublishedHandlePrunePlan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PostCommitConsumerPacket {
    pub(crate) packet_index: usize,
    pub(crate) kind: PostCommitConsumerKind,
    pub(crate) reduction_key: PostCommitReductionKey,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PostCommitConsumerObservation {
    PublicationDiagnosticEntries(Vec<RelationalDiagnosticsEntry>),
    PublishedHandlePrunePlan(Vec<SnapshotId>),
}
