use forge_store_recovery_physics::BoundedRecoveryReceipt;

use super::denial::S45HarnessBoundaryDenial;
use super::entry::S45SimulationHarnessEntry;
use super::inventory::S45ExistingHarnessInventory;
use super::proof_progression::admit_entry_request;
use super::request::S45HarnessEntryRequest;
use super::requirement_set::S45RoadmapHarnessRequirementSet;

pub fn admit_s45_simulation_harness_entry(
    recovery: &BoundedRecoveryReceipt,
    roadmap_requirements: S45RoadmapHarnessRequirementSet,
    inventory: S45ExistingHarnessInventory,
) -> Result<S45SimulationHarnessEntry, S45HarnessBoundaryDenial> {
    require_roadmap_harness_requirements(&roadmap_requirements)?;
    inventory.validate_for_s45_entry()?;
    let recovered_state = recovery.execution().recovered_state();

    let request = S45HarnessEntryRequest::new(
        recovered_state.recovered_physical_root(),
        recovered_state.source_decision_digest(),
        roadmap_requirements,
        inventory,
    );
    let request = admit_entry_request(request)?;
    Ok(S45SimulationHarnessEntry::from_admitted_request(
        request,
        recovered_state.page_lsn_frontier(),
        recovery.counters(),
    ))
}

pub const fn reject_s45_copied_recovery_report(_: &str) -> S45HarnessBoundaryDenial {
    S45HarnessBoundaryDenial::CopiedS4ReportCannotAdmitEntry
}

pub const fn reject_s45_log_output(_: &str) -> S45HarnessBoundaryDenial {
    S45HarnessBoundaryDenial::LogOutputCannotAdmitEntry
}

pub const fn reject_s45_old_semantic_harness_label(_: &str) -> S45HarnessBoundaryDenial {
    S45HarnessBoundaryDenial::OldSemanticHarnessContextCannotAdmitEntry
}

pub const fn reject_s45_same_run_self_comparison() -> S45HarnessBoundaryDenial {
    S45HarnessBoundaryDenial::SameRunSelfComparisonCannotAdmitEntry
}

pub const fn reject_s45_terminal_projection(_: &str) -> S45HarnessBoundaryDenial {
    S45HarnessBoundaryDenial::TerminalProjectionCannotAdmitEntry
}

pub const fn reject_s45_s5_isolation_authority_attempt() -> S45HarnessBoundaryDenial {
    S45HarnessBoundaryDenial::S5IsolationAuthorityCannotBeMintedByHarnessEntry
}

pub const fn reject_s45_foundational_projection_authority() -> S45HarnessBoundaryDenial {
    S45HarnessBoundaryDenial::FoundationalProjectionCannotReplaceStoreAuthority
}

fn require_roadmap_harness_requirements(
    requirements: &S45RoadmapHarnessRequirementSet,
) -> Result<(), S45HarnessBoundaryDenial> {
    if let Some(missing) = requirements.missing_required() {
        return Err(S45HarnessBoundaryDenial::MissingRoadmapHarnessRequirement(
            missing,
        ));
    }
    Ok(())
}
