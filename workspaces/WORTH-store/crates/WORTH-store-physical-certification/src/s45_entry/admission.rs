use worth_store_recovery_physics::RecoveryPhysicsCertificationBundle;

use super::denial::S45HarnessBoundaryDenial;
use super::entry::S45SimulationHarnessEntry;
use super::inventory::S45ExistingHarnessInventory;
use super::proof_progression::admit_entry_request;
use super::request::S45HarnessEntryRequest;
use super::requirement_set::S45RoadmapHarnessRequirementSet;

pub fn admit_s45_simulation_harness_entry(
    s4_closeout: &RecoveryPhysicsCertificationBundle,
    roadmap_requirements: S45RoadmapHarnessRequirementSet,
    inventory: S45ExistingHarnessInventory,
) -> Result<S45SimulationHarnessEntry, S45HarnessBoundaryDenial> {
    let report = s4_closeout.closeout_report();
    require_complete_s4_closeout(s4_closeout)?;
    require_roadmap_harness_requirements(&roadmap_requirements)?;
    inventory.validate_for_s45_entry()?;

    let request = S45HarnessEntryRequest::new(
        report.recovered_root(),
        report.source_decision_digest(),
        report.suite_status().completed_lanes(),
        report.suite_status().required_lanes(),
        report.foundational_exact_counter_assertions(),
        roadmap_requirements,
        inventory,
    );
    let request = admit_entry_request(request)?;
    Ok(S45SimulationHarnessEntry::from_admitted_request(
        request,
        report.admitted_page_lsn_frontier(),
        report.counters(),
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

fn require_complete_s4_closeout(
    s4_closeout: &RecoveryPhysicsCertificationBundle,
) -> Result<(), S45HarnessBoundaryDenial> {
    let report = s4_closeout.closeout_report();
    if !report.suite_status().is_complete() {
        return Err(S45HarnessBoundaryDenial::IncompleteS4Closeout);
    }
    if !report
        .synthetic_shortcut_rejections()
        .all_required_shortcuts_denied()
    {
        return Err(S45HarnessBoundaryDenial::S4CloseoutDoesNotRejectSyntheticShortcuts);
    }
    s4_closeout
        .publish_s5_readiness()
        .admit_for_s5_startup()
        .map_err(|_| S45HarnessBoundaryDenial::S4CloseoutMissingS5RecoveryReadiness)?;
    Ok(())
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
