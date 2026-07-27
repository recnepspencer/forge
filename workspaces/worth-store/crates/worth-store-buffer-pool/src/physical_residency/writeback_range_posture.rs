/// The exact physical range posture admitted for one dirty-frame writeback.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWritebackRangePosture {
    /// The entire frame range must already exist.
    ExistingRange,
    /// A newly admitted candidate frame must begin at the current artifact EOF.
    CandidateArtifactTail,
}
