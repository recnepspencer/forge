#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S6LaterMilestoneDestination {
    S7Placement,
    S10Compaction,
    S10BackupExport,
    S10RepairScan,
    S11OperatorReadiness,
}
