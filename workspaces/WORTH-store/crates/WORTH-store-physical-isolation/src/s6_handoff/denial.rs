use super::UnsupportedQoSClaim;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S6IoQosIsolationReadinessDenial {
    CopiedCloseoutReport,
    LogOrTerminalProjection,
    SyntheticWaitLabel,
    MissingExecutedCounter,
    MissingLatchCounters,
    MissingReclaimCounters,
    MissingProtectedByteFootprint,
    UnsupportedQoSClaimRequested(UnsupportedQoSClaim),
}

pub const fn reject_copied_closeout_report_as_s6_readiness(
) -> Result<(), S6IoQosIsolationReadinessDenial> {
    Err(S6IoQosIsolationReadinessDenial::CopiedCloseoutReport)
}

pub const fn reject_log_or_terminal_projection_as_s6_readiness(
) -> Result<(), S6IoQosIsolationReadinessDenial> {
    Err(S6IoQosIsolationReadinessDenial::LogOrTerminalProjection)
}

pub const fn reject_synthetic_wait_label_as_s6_readiness(
) -> Result<(), S6IoQosIsolationReadinessDenial> {
    Err(S6IoQosIsolationReadinessDenial::SyntheticWaitLabel)
}

pub const fn reject_missing_latch_counters_as_s6_readiness(
) -> Result<(), S6IoQosIsolationReadinessDenial> {
    Err(S6IoQosIsolationReadinessDenial::MissingLatchCounters)
}

pub const fn reject_missing_reclaim_counters_as_s6_readiness(
) -> Result<(), S6IoQosIsolationReadinessDenial> {
    Err(S6IoQosIsolationReadinessDenial::MissingReclaimCounters)
}

pub const fn reject_missing_protected_byte_footprint_as_s6_readiness(
) -> Result<(), S6IoQosIsolationReadinessDenial> {
    Err(S6IoQosIsolationReadinessDenial::MissingProtectedByteFootprint)
}

pub const fn reject_qos_claim_as_s5_readiness(
    claim: UnsupportedQoSClaim,
) -> Result<(), S6IoQosIsolationReadinessDenial> {
    Err(S6IoQosIsolationReadinessDenial::UnsupportedQoSClaimRequested(claim))
}
