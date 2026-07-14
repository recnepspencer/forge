use worth_store_recovery_physics::BoundedRecoveryReceipt;

use super::denial::SimulationHarnessBoundaryDenial;
use super::entry::SimulationHarnessEntry;
use super::inventory::ExistingSimulationHarnessInventory;
use super::proof_progression::admit_entry_request;
use super::request::SimulationHarnessEntryRequest;
use super::requirement_set::SimulationHarnessRoadmapRequirementSet;

pub fn admit_simulation_harness_entry(
    recovery: &BoundedRecoveryReceipt,
    roadmap_requirements: SimulationHarnessRoadmapRequirementSet,
    inventory: ExistingSimulationHarnessInventory,
) -> Result<SimulationHarnessEntry, SimulationHarnessBoundaryDenial> {
    require_roadmap_harness_requirements(&roadmap_requirements)?;
    inventory.validate_for_simulation_harness_entry()?;
    let recovered_state = recovery.execution().recovered_state();

    let request = SimulationHarnessEntryRequest::new(
        recovered_state.recovered_physical_root(),
        recovered_state.source_decision_digest(),
        roadmap_requirements,
        inventory,
    );
    let request = admit_entry_request(request)?;
    Ok(SimulationHarnessEntry::from_admitted_request(
        request,
        recovered_state.page_lsn_frontier(),
        recovery.counters(),
    ))
}

pub const fn reject_simulation_harness_copied_recovery_report(
    _: &str,
) -> SimulationHarnessBoundaryDenial {
    SimulationHarnessBoundaryDenial::CopiedS4ReportCannotAdmitEntry
}

pub const fn reject_simulation_harness_log_output(_: &str) -> SimulationHarnessBoundaryDenial {
    SimulationHarnessBoundaryDenial::LogOutputCannotAdmitEntry
}

pub const fn reject_simulation_harness_old_semantic_harness_label(
    _: &str,
) -> SimulationHarnessBoundaryDenial {
    SimulationHarnessBoundaryDenial::OldSemanticHarnessContextCannotAdmitEntry
}

pub const fn reject_simulation_harness_same_run_self_comparison() -> SimulationHarnessBoundaryDenial
{
    SimulationHarnessBoundaryDenial::SameRunSelfComparisonCannotAdmitEntry
}

pub const fn reject_simulation_harness_terminal_projection(
    _: &str,
) -> SimulationHarnessBoundaryDenial {
    SimulationHarnessBoundaryDenial::TerminalProjectionCannotAdmitEntry
}

pub const fn reject_simulation_harness_physical_isolation_authority_attempt(
) -> SimulationHarnessBoundaryDenial {
    SimulationHarnessBoundaryDenial::PhysicalIsolationAuthorityCannotBeMintedByHarnessEntry
}

pub const fn reject_simulation_harness_foundational_projection_authority(
) -> SimulationHarnessBoundaryDenial {
    SimulationHarnessBoundaryDenial::FoundationalProjectionCannotReplaceStoreAuthority
}

fn require_roadmap_harness_requirements(
    requirements: &SimulationHarnessRoadmapRequirementSet,
) -> Result<(), SimulationHarnessBoundaryDenial> {
    if let Some(missing) = requirements.missing_required() {
        return Err(SimulationHarnessBoundaryDenial::MissingRoadmapHarnessRequirement(missing));
    }
    Ok(())
}
