use super::{
    PhysicalIsolationEntryCheckedOutcome, PhysicalIsolationEntryDenial,
    PhysicalIsolationEntryRebindRequired,
};

pub fn reject_copied_recovery_fields_as_physical_isolation_entry(
) -> Result<(), PhysicalIsolationEntryDenial> {
    Err(PhysicalIsolationEntryDenial::CopiedRecoveryFields)
}

pub fn reject_live_runtime_state_as_physical_isolation_entry(
) -> Result<(), PhysicalIsolationEntryDenial> {
    Err(PhysicalIsolationEntryDenial::LiveRuntimeState)
}

pub fn reject_terminal_projection_as_physical_isolation_entry(
) -> Result<(), PhysicalIsolationEntryDenial> {
    Err(PhysicalIsolationEntryDenial::TerminalProjection)
}

pub fn reject_semantic_snapshot_as_physical_isolation_entry(
) -> Result<(), PhysicalIsolationEntryDenial> {
    Err(PhysicalIsolationEntryDenial::SemanticSnapshot)
}

pub fn reject_json_authority_as_physical_isolation_entry(
) -> Result<(), PhysicalIsolationEntryDenial> {
    Err(PhysicalIsolationEntryDenial::JsonAuthority)
}

pub fn reject_foundational_or_proof_projection_as_physical_isolation_entry(
) -> Result<(), PhysicalIsolationEntryDenial> {
    Err(PhysicalIsolationEntryDenial::FoundationalOrProofProjection)
}

pub fn reject_stale_recovery_readiness_as_physical_isolation_entry(
) -> PhysicalIsolationEntryCheckedOutcome {
    PhysicalIsolationEntryCheckedOutcome::Stale(
        PhysicalIsolationEntryDenial::StaleRecoveryReadiness,
    )
}

pub fn require_rebound_s4_recovery_readiness_for_physical_isolation_entry(
) -> PhysicalIsolationEntryCheckedOutcome {
    PhysicalIsolationEntryCheckedOutcome::RebindRequired(
        PhysicalIsolationEntryRebindRequired::S4RecoveryReadinessMustBeRebound,
    )
}
