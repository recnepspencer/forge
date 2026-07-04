#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum UiEvidenceRetentionPosture {
    CurrentGenerationOnly,
    RetainedForInspection,
    RetainedForReplay,
    RetainedUntilCloseout,
    DiscardedWithTombstone,
}
