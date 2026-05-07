use std::collections::BTreeMap;

use worth_schema::facade::{
    worth_milestone_two_invalidation_declarations, DerivedTopologyReadBasis, WorthAspect,
    WorthDerivedInvalidationTarget,
};

use crate::interpretation::InterpretedTopologyView;
use crate::materialization::{MaterializationFallbackClass, MaterializedTopologyView};
use crate::parity::{build_derived_equivalence_contract, WorthDerivedEquivalenceContractReport};
use crate::validators::DerivedTopologyValidationReport;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WorthDerivedInvalidationTargetRow {
    pub target: WorthDerivedInvalidationTarget,
    pub bridge_scope: String,
    pub declaration_ids: Vec<String>,
    pub triggered: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WorthDerivedInvalidationReport {
    pub touched_aspect_count: usize,
    pub topology_touched: bool,
    pub naming_touched: bool,
    pub triggered_target_count: usize,
    pub rows: Vec<WorthDerivedInvalidationTargetRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WorthDerivedRebuildReport {
    pub whole_view_rebuild: bool,
    pub topology_entity_count: usize,
    pub topology_relation_count: usize,
    pub interpreted_wire_count: usize,
    pub interpreted_shell_count: usize,
    pub boundary_interpretation_count: usize,
    pub radial_interpretation_count: usize,
    pub validation_row_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WorthDerivedFallbackReport {
    pub whole_view_materialization: bool,
    pub materialization_fallback_class: Option<MaterializationFallbackClass>,
    pub precision_fallback_count: usize,
    pub precision_budget_fallback_count: usize,
    pub explicit_fallback_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WorthDerivedReadDiagnostics {
    pub invalidation_report: WorthDerivedInvalidationReport,
    pub rebuild_report: WorthDerivedRebuildReport,
    pub fallback_report: WorthDerivedFallbackReport,
    pub equivalence_contract_report: WorthDerivedEquivalenceContractReport,
}

pub fn build_derived_read_diagnostics(
    read_basis: &DerivedTopologyReadBasis,
    materialized: &MaterializedTopologyView,
    interpreted: &InterpretedTopologyView,
    validation: &DerivedTopologyValidationReport,
) -> WorthDerivedReadDiagnostics {
    WorthDerivedReadDiagnostics {
        invalidation_report: build_derived_invalidation_report(read_basis),
        rebuild_report: build_derived_rebuild_report(materialized, interpreted, validation),
        fallback_report: build_derived_fallback_report(read_basis, materialized),
        equivalence_contract_report: build_derived_equivalence_contract(
            read_basis,
            materialized,
            interpreted,
            validation,
        ),
    }
}

pub fn build_derived_invalidation_report(
    read_basis: &DerivedTopologyReadBasis,
) -> WorthDerivedInvalidationReport {
    build_derived_invalidation_report_from_aspects(read_basis.touched_aspects().iter().copied())
}

pub fn build_derived_invalidation_report_from_aspects(
    touched_aspects: impl IntoIterator<Item = WorthAspect>,
) -> WorthDerivedInvalidationReport {
    let touched_aspects = touched_aspects.into_iter().collect::<Vec<_>>();
    let topology_touched = touched_aspects
        .iter()
        .any(|aspect| matches!(aspect, WorthAspect::Topology(_)));
    let naming_touched = touched_aspects
        .iter()
        .any(|aspect| matches!(aspect, WorthAspect::Naming(_)));

    let mut grouped = BTreeMap::<WorthDerivedInvalidationTarget, Vec<String>>::new();
    for declaration in worth_milestone_two_invalidation_declarations() {
        grouped
            .entry(declaration.target)
            .or_default()
            .push(declaration.declaration_id.to_string());
    }

    let triggered_targets =
        triggered_invalidation_targets_from_aspects(touched_aspects.iter().copied());
    let rows = grouped
        .into_iter()
        .map(|(target, declaration_ids)| {
            let triggered = triggered_targets.contains(&target);
            WorthDerivedInvalidationTargetRow {
                target,
                bridge_scope: target.bridge_scope().to_string(),
                declaration_ids,
                triggered,
            }
        })
        .collect::<Vec<_>>();

    WorthDerivedInvalidationReport {
        touched_aspect_count: touched_aspects.len(),
        topology_touched,
        naming_touched,
        triggered_target_count: rows.iter().filter(|row| row.triggered).count(),
        rows,
    }
}

pub(crate) fn triggered_invalidation_targets(
    read_basis: &DerivedTopologyReadBasis,
) -> Vec<WorthDerivedInvalidationTarget> {
    triggered_invalidation_targets_from_aspects(read_basis.touched_aspects().iter().copied())
}

pub(crate) fn triggered_invalidation_targets_from_aspects(
    touched_aspects: impl IntoIterator<Item = WorthAspect>,
) -> Vec<WorthDerivedInvalidationTarget> {
    let mut targets = Vec::new();
    for aspect in touched_aspects {
        match aspect {
            WorthAspect::Topology(topology) => match topology {
                worth_schema::facade::WorthTopologyAspect::Structure => {
                    push_unique_target(
                        &mut targets,
                        WorthDerivedInvalidationTarget::TopologyStructure,
                    );
                }
                worth_schema::facade::WorthTopologyAspect::Ownership => {
                    push_unique_target(
                        &mut targets,
                        WorthDerivedInvalidationTarget::TopologyOwnership,
                    );
                }
                worth_schema::facade::WorthTopologyAspect::Boundary => {
                    push_unique_target(
                        &mut targets,
                        WorthDerivedInvalidationTarget::TopologyBoundary,
                    );
                }
                worth_schema::facade::WorthTopologyAspect::Radial => {
                    push_unique_target(
                        &mut targets,
                        WorthDerivedInvalidationTarget::TopologyRadial,
                    );
                }
            },
            WorthAspect::Naming(worth_schema::facade::WorthNamingAspect::PersistentName) => {
                push_unique_target(
                    &mut targets,
                    WorthDerivedInvalidationTarget::NamingPersistentName,
                );
            }
            _ => {}
        }
    }
    targets
}

fn push_unique_target(
    targets: &mut Vec<WorthDerivedInvalidationTarget>,
    target: WorthDerivedInvalidationTarget,
) {
    if !targets.contains(&target) {
        targets.push(target);
    }
}

pub fn build_derived_rebuild_report(
    materialized: &MaterializedTopologyView,
    interpreted: &InterpretedTopologyView,
    validation: &DerivedTopologyValidationReport,
) -> WorthDerivedRebuildReport {
    WorthDerivedRebuildReport {
        whole_view_rebuild: materialized.report().whole_view_materialization,
        topology_entity_count: materialized.report().breadth.topology_entity_count,
        topology_relation_count: materialized.report().breadth.topology_relation_count,
        interpreted_wire_count: interpreted.report().interpreted_wire_count,
        interpreted_shell_count: interpreted.report().interpreted_shell_count,
        boundary_interpretation_count: interpreted.report().boundary_interpretation_count,
        radial_interpretation_count: interpreted.report().radial_interpretation_count,
        validation_row_count: validation.rows.len(),
    }
}

pub fn build_derived_fallback_report(
    read_basis: &DerivedTopologyReadBasis,
    materialized: &MaterializedTopologyView,
) -> WorthDerivedFallbackReport {
    build_derived_fallback_report_from_counts(
        read_basis.precision_fallbacks.len(),
        read_basis.precision_budget_fallbacks.len(),
        materialized,
    )
}

pub fn build_derived_fallback_report_from_counts(
    precision_fallback_count: usize,
    precision_budget_fallback_count: usize,
    materialized: &MaterializedTopologyView,
) -> WorthDerivedFallbackReport {
    let explicit_fallback_count = precision_fallback_count
        + precision_budget_fallback_count
        + usize::from(materialized.report().fallback_class.is_some());

    WorthDerivedFallbackReport {
        whole_view_materialization: materialized.report().whole_view_materialization,
        materialization_fallback_class: materialized.report().fallback_class,
        precision_fallback_count,
        precision_budget_fallback_count,
        explicit_fallback_count,
    }
}

#[cfg(test)]
mod tests {
    use worth_schema::facade::topology_authoring::{
        seed_milestone_one_primitive, WorthMilestoneOnePrimitiveCase,
    };

    use crate::diagnostics::build_derived_read_diagnostics;
    use crate::facade::{
        interpret_topology_view, validate_interpreted_topology,
        worth_milestone_one_runtime_builder, WorthTopologyMaterializer,
    };

    #[test]
    fn derived_diagnostics_reports_are_explicit_and_deterministic() {
        let mut runtime = worth_milestone_one_runtime_builder()
            .expect("worth milestone one runtime builder")
            .build();
        let verified = seed_milestone_one_primitive(
            &mut runtime,
            "phase-seven-diagnostics",
            &WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
        )
        .expect("verified primitive");
        let read_view = runtime
            .read_truth()
            .read_snapshot(&verified.persisted_truth.snapshot)
            .expect("snapshot read");
        let materialized =
            WorthTopologyMaterializer::materialize_from_truth(&read_view).expect("materialized");
        let interpreted = interpret_topology_view(&materialized);
        let validation =
            validate_interpreted_topology(&materialized, &interpreted).expect("validation");

        let diagnostics = build_derived_read_diagnostics(
            &verified.read_basis,
            &materialized,
            &interpreted,
            &validation,
        );

        assert!(diagnostics.invalidation_report.topology_touched);
        assert!(!diagnostics.invalidation_report.rows.is_empty());
        assert!(diagnostics
            .invalidation_report
            .rows
            .iter()
            .any(|row| row.target
                == worth_schema::facade::WorthDerivedInvalidationTarget::TopologyStructure
                && row.triggered));
        assert!(diagnostics
            .invalidation_report
            .rows
            .iter()
            .any(|row| row.target
                == worth_schema::facade::WorthDerivedInvalidationTarget::TopologyBoundary
                && row.triggered));
        assert!(diagnostics.rebuild_report.whole_view_rebuild);
        assert_eq!(
            diagnostics.rebuild_report.validation_row_count,
            validation.rows.len()
        );
        assert!(diagnostics.fallback_report.whole_view_materialization);
        assert_eq!(
            diagnostics.fallback_report.explicit_fallback_count,
            diagnostics.fallback_report.precision_fallback_count
                + diagnostics.fallback_report.precision_budget_fallback_count
                + 1
        );
        assert!(
            diagnostics
                .equivalence_contract_report
                .materialized_topology_digest
                .row_count
                > 0
        );
    }
}
