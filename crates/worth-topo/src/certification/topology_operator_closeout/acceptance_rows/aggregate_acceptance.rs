use std::collections::{BTreeMap, BTreeSet};

use crate::derived_topology::materialized_graph::MaterializationFallbackClass;
use crate::topology_operators::{
    TopologyDerivedRegion, TopologyEditChangedScope, TopologyEditDerivedFallbackPolicy,
    TopologyEditFamily, TopologyEditRejectionClass,
};

use super::super::report::{
    MilestoneThreeChangedScopeCoverageRow, MilestoneThreeDerivedRegionCoverageRow,
    MilestoneThreeDeterminismRuleRow, MilestoneThreeEditBreadthCounterRow,
    MilestoneThreeEditFalloutBreadthRow, MilestoneThreeEditFalloutClass,
    MilestoneThreeFailureLocalityRow, MilestoneThreeHostileOutcomeClass,
    MilestoneThreeHostileScenario, MilestoneThreeHostileScenarioReport,
};
use super::determinism_rules::build_determinism_rule_rows;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct MilestoneThreeAggregateAcceptanceRows {
    pub changed_scope_coverage_rows: Vec<MilestoneThreeChangedScopeCoverageRow>,
    pub derived_region_coverage_rows: Vec<MilestoneThreeDerivedRegionCoverageRow>,
    pub determinism_rule_rows: Vec<MilestoneThreeDeterminismRuleRow>,
    pub edit_breadth_counter_rows: Vec<MilestoneThreeEditBreadthCounterRow>,
    pub edit_fallout_breadth_rows: Vec<MilestoneThreeEditFalloutBreadthRow>,
    pub failure_locality_rows: Vec<MilestoneThreeFailureLocalityRow>,
}

pub(super) fn build_aggregate_acceptance_rows(
    reports: &[MilestoneThreeHostileScenarioReport],
) -> MilestoneThreeAggregateAcceptanceRows {
    MilestoneThreeAggregateAcceptanceRows {
        changed_scope_coverage_rows: build_changed_scope_coverage_rows(reports),
        derived_region_coverage_rows: build_derived_region_coverage_rows(reports),
        determinism_rule_rows: build_determinism_rule_rows(reports),
        edit_breadth_counter_rows: build_edit_breadth_counter_rows(reports),
        edit_fallout_breadth_rows: build_edit_fallout_breadth_rows(reports),
        failure_locality_rows: build_failure_locality_rows(reports),
    }
}

fn build_changed_scope_coverage_rows(
    reports: &[MilestoneThreeHostileScenarioReport],
) -> Vec<MilestoneThreeChangedScopeCoverageRow> {
    let mut rows = BTreeMap::<TopologyEditChangedScope, Vec<MilestoneThreeHostileScenario>>::new();
    for report in reports {
        for scope in changed_scopes_from_report(report) {
            rows.entry(scope).or_default().push(report.scenario);
        }
    }
    rows.into_iter()
        .map(|(changed_scope, mut scenarios)| {
            scenarios.sort();
            scenarios.dedup();
            MilestoneThreeChangedScopeCoverageRow {
                changed_scope,
                scenario_count: scenarios.len(),
                row_digest: format!(
                    "changed_scope={changed_scope:?};scenario_count={}",
                    scenarios.len()
                ),
                scenarios,
            }
        })
        .collect()
}

fn build_derived_region_coverage_rows(
    reports: &[MilestoneThreeHostileScenarioReport],
) -> Vec<MilestoneThreeDerivedRegionCoverageRow> {
    let mut rows = BTreeMap::<TopologyDerivedRegion, Vec<MilestoneThreeHostileScenario>>::new();
    for report in reports {
        for region in derived_regions_from_report(report) {
            rows.entry(region).or_default().push(report.scenario);
        }
    }
    rows.into_iter()
        .map(|(derived_region, mut scenarios)| {
            scenarios.sort();
            scenarios.dedup();
            MilestoneThreeDerivedRegionCoverageRow {
                derived_region,
                scenario_count: scenarios.len(),
                row_digest: format!(
                    "derived_region={derived_region:?};scenario_count={}",
                    scenarios.len()
                ),
                scenarios,
            }
        })
        .collect()
}

fn build_edit_breadth_counter_rows(
    reports: &[MilestoneThreeHostileScenarioReport],
) -> Vec<MilestoneThreeEditBreadthCounterRow> {
    reports
        .iter()
        .map(|report| MilestoneThreeEditBreadthCounterRow {
            scenario: report.scenario,
            contract_count: report.topology_edit_digest.contract_count,
            family_count: report.topology_edit_digest.family_count,
            changed_scope_count: report.topology_edit_digest.changed_scope_count,
            naming_scope_count: report.topology_edit_digest.naming_scope_count,
            derived_region_count: report.topology_edit_digest.derived_region_count,
            replay_step_count: report.edit_replay_parity_report.step_rows.len(),
            replay_checked: report.edit_replay_parity_report.replay_checked,
            row_digest: format!(
                "scenario={};contracts={};families={};changed_scopes={};naming_scopes={};derived_regions={};replay_steps={}",
                report.scenario.as_str(),
                report.topology_edit_digest.contract_count,
                report.topology_edit_digest.family_count,
                report.topology_edit_digest.changed_scope_count,
                report.topology_edit_digest.naming_scope_count,
                report.topology_edit_digest.derived_region_count,
                report.edit_replay_parity_report.step_rows.len()
            ),
        })
        .collect()
}

fn build_failure_locality_rows(
    reports: &[MilestoneThreeHostileScenarioReport],
) -> Vec<MilestoneThreeFailureLocalityRow> {
    reports
        .iter()
        .filter_map(|report| {
            let rejection_class = report.rejection_class?;
            let rejected_scope = report.rejected_edit_scope_report.as_ref()?;
            let families = rejected_scope
                .rows
                .iter()
                .map(|row| row.family)
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            let changed_scopes = rejected_scope
                .rows
                .iter()
                .flat_map(|row| row.changed_scopes.iter().copied())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            let naming_scopes = rejected_scope
                .rows
                .iter()
                .flat_map(|row| row.naming_scopes.iter().copied())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            let derived_regions = rejected_scope
                .rows
                .iter()
                .flat_map(|row| row.derived_regions.iter().copied())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            Some(MilestoneThreeFailureLocalityRow {
                scenario: report.scenario,
                rejection_class,
                scope_row_count: rejected_scope.rows.len(),
                row_digest: format!(
                    "scenario={};rejection_class={rejection_class:?};scope_rows={};changed_scopes={};derived_regions={}",
                    report.scenario.as_str(),
                    rejected_scope.rows.len(),
                    changed_scopes.len(),
                    derived_regions.len()
                ),
                families,
                changed_scopes,
                naming_scopes,
                derived_regions,
            })
        })
        .collect()
}

pub(super) fn build_edit_fallout_breadth_rows(
    reports: &[MilestoneThreeHostileScenarioReport],
) -> Vec<MilestoneThreeEditFalloutBreadthRow> {
    reports
        .iter()
        .map(|report| {
            let declared_derived_region_count = derived_regions_from_report(report).len();
            let derived_validation_row_count = report
                .derived_validation_report
                .as_ref()
                .map_or(0, |validation| validation.rows.len());
            let fallout_class = edit_fallout_class(report);
            let fallback_count = usize::from(matches!(
                fallout_class,
                MilestoneThreeEditFalloutClass::WholeViewFallback
                    | MilestoneThreeEditFalloutClass::WholeHistoryFallback
            ));
            let fallback_policy = fallback_policy_from_report(report);
            let fallback_policy_exceeded =
                fallback_policy_exceeded(fallback_policy, fallback_count);
            let fallback_rejection_class = fallback_policy_exceeded
                .then_some(TopologyEditRejectionClass::DerivedFallbackExceeded);
            MilestoneThreeEditFalloutBreadthRow {
                scenario: report.scenario,
                fallout_class,
                fallback_policy,
                fallback_policy_exceeded,
                fallback_rejection_class,
                declared_derived_region_count,
                derived_validation_row_count,
                fallback_count,
                locality_claim_mismatch: false,
                row_digest: format!(
                    "scenario={};fallout_class={fallout_class:?};fallback_policy={};policy_exceeded={fallback_policy_exceeded};rejection_class={:?};declared_regions={declared_derived_region_count};validation_rows={derived_validation_row_count};fallback_count={fallback_count};locality_claim_mismatch=false",
                    report.scenario.as_str(),
                    fallback_policy.as_str(),
                    fallback_rejection_class,
                ),
            }
        })
        .collect()
}

fn fallback_policy_from_report(
    report: &MilestoneThreeHostileScenarioReport,
) -> TopologyEditDerivedFallbackPolicy {
    if report.topology_edit_digest.fallback_rejection_policy_count > 0 {
        TopologyEditDerivedFallbackPolicy::RejectAnyFallback
    } else {
        TopologyEditDerivedFallbackPolicy::AllowExplicitFallback
    }
}

fn fallback_policy_exceeded(
    policy: TopologyEditDerivedFallbackPolicy,
    fallback_count: usize,
) -> bool {
    policy == TopologyEditDerivedFallbackPolicy::RejectAnyFallback && fallback_count > 0
}

fn edit_fallout_class(
    report: &MilestoneThreeHostileScenarioReport,
) -> MilestoneThreeEditFalloutClass {
    match report.outcome_class {
        MilestoneThreeHostileOutcomeClass::Rejected => {
            MilestoneThreeEditFalloutClass::RejectedBeforeDerivedWork
        }
        MilestoneThreeHostileOutcomeClass::Accepted
            if report.derived_materialization_fallback_class
                == Some(MaterializationFallbackClass::WholeViewRebuild) =>
        {
            MilestoneThreeEditFalloutClass::WholeViewFallback
        }
        MilestoneThreeHostileOutcomeClass::Accepted => MilestoneThreeEditFalloutClass::Localized,
    }
}

fn changed_scopes_from_report(
    report: &MilestoneThreeHostileScenarioReport,
) -> BTreeSet<TopologyEditChangedScope> {
    let mut scopes = BTreeSet::new();
    if let Some(rejected_scope) = report.rejected_edit_scope_report.as_ref() {
        for row in &rejected_scope.rows {
            scopes.extend(row.changed_scopes.iter().copied());
        }
    }
    for row in &report.naming_edit_continuity_matrix.rows {
        scopes.extend(changed_scopes_for_family(row.family));
    }
    scopes
}

fn derived_regions_from_report(
    report: &MilestoneThreeHostileScenarioReport,
) -> BTreeSet<TopologyDerivedRegion> {
    let mut regions = BTreeSet::new();
    if let Some(rejected_scope) = report.rejected_edit_scope_report.as_ref() {
        for row in &rejected_scope.rows {
            regions.extend(row.derived_regions.iter().copied());
        }
    }
    for row in &report.naming_edit_continuity_matrix.rows {
        regions.extend(derived_regions_for_family(row.family));
    }
    regions
}

fn changed_scopes_for_family(family: TopologyEditFamily) -> &'static [TopologyEditChangedScope] {
    use TopologyEditChangedScope as Scope;
    match family {
        TopologyEditFamily::CreateTopologyEntity | TopologyEditFamily::RetireTopologyEntity => {
            &[Scope::Entity, Scope::Naming]
        }
        TopologyEditFamily::AttachBoundaryMembership
        | TopologyEditFamily::DetachBoundaryMembership
        | TopologyEditFamily::RewireLoopSuccessor
        | TopologyEditFamily::RewireLoopEndpoint => {
            &[Scope::Relation, Scope::Loop, Scope::LocalNeighborhood]
        }
        TopologyEditFamily::AttachShellOrWireMembership
        | TopologyEditFamily::DetachShellOrWireMembership => &[
            Scope::Relation,
            Scope::Wire,
            Scope::Shell,
            Scope::LocalNeighborhood,
        ],
        TopologyEditFamily::SpliceRadialAdjacency | TopologyEditFamily::DetachRadialAdjacency => &[
            Scope::Relation,
            Scope::RadialNeighborhood,
            Scope::LocalNeighborhood,
        ],
    }
}

fn derived_regions_for_family(family: TopologyEditFamily) -> &'static [TopologyDerivedRegion] {
    use TopologyDerivedRegion as Region;
    match family {
        TopologyEditFamily::CreateTopologyEntity | TopologyEditFamily::RetireTopologyEntity => &[
            Region::NamingContinuityRegion,
            Region::EditLocalNeighborhoodRegion,
        ],
        TopologyEditFamily::AttachBoundaryMembership
        | TopologyEditFamily::DetachBoundaryMembership
        | TopologyEditFamily::RewireLoopSuccessor
        | TopologyEditFamily::RewireLoopEndpoint => {
            &[Region::LoopRegion, Region::EditLocalNeighborhoodRegion]
        }
        TopologyEditFamily::AttachShellOrWireMembership
        | TopologyEditFamily::DetachShellOrWireMembership => &[
            Region::WireRegion,
            Region::ShellRegion,
            Region::EditLocalNeighborhoodRegion,
        ],
        TopologyEditFamily::SpliceRadialAdjacency | TopologyEditFamily::DetachRadialAdjacency => &[
            Region::RadialNeighborhoodRegion,
            Region::EditLocalNeighborhoodRegion,
        ],
    }
}
