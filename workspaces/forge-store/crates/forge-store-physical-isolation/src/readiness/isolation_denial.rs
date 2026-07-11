use super::scheduler_capability::UnsupportedQoSClaim;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsolationReadinessDenial {
    CopiedCloseoutReport,
    LogOrTerminalProjection,
    SyntheticWaitLabel,
    MissingExecutedCounter,
    MissingLatchCounters,
    MissingReclaimCounters,
    MissingProtectedByteFootprint,
    UnsupportedQoSClaimRequested(UnsupportedQoSClaim),
}

pub const fn reject_copied_closeout_report_as_isolation_readiness(
) -> Result<(), IsolationReadinessDenial> {
    Err(IsolationReadinessDenial::CopiedCloseoutReport)
}

pub const fn reject_log_or_terminal_projection_as_isolation_readiness(
) -> Result<(), IsolationReadinessDenial> {
    Err(IsolationReadinessDenial::LogOrTerminalProjection)
}

pub const fn reject_synthetic_wait_label_as_isolation_readiness(
) -> Result<(), IsolationReadinessDenial> {
    Err(IsolationReadinessDenial::SyntheticWaitLabel)
}

pub const fn reject_missing_latch_counters_as_isolation_readiness(
) -> Result<(), IsolationReadinessDenial> {
    Err(IsolationReadinessDenial::MissingLatchCounters)
}

pub const fn reject_missing_reclaim_counters_as_isolation_readiness(
) -> Result<(), IsolationReadinessDenial> {
    Err(IsolationReadinessDenial::MissingReclaimCounters)
}

pub const fn reject_missing_protected_byte_footprint_as_isolation_readiness(
) -> Result<(), IsolationReadinessDenial> {
    Err(IsolationReadinessDenial::MissingProtectedByteFootprint)
}

pub const fn reject_unsupported_qos_claim_as_isolation_readiness(
    claim: UnsupportedQoSClaim,
) -> Result<(), IsolationReadinessDenial> {
    Err(IsolationReadinessDenial::UnsupportedQoSClaimRequested(
        claim,
    ))
}
