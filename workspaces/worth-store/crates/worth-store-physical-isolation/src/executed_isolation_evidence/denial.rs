#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutedIsolationEvidenceDenial {
    MissingLatchCounters,
    MissingReclaimCounters,
    MissingProtectedByteFootprint,
}
