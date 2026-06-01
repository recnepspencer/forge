use std::collections::BTreeMap;

pub(crate) mod query_diagnostics;
pub(crate) mod read_proof;

const QUERY_SURFACE_FAILURE_ROW_KEY: &str = "query_surface_error";

use schema::facade::platform::aspects::Aspect;
use schema::facade::platform::authority::{
    milestone_two_invalidation_declarations, DerivedInvalidationTarget,
};
use schema::facade::topology_authoring::DerivedTopologyReadBasis;

use crate::certification::support::parity::{
    build_derived_equivalence_contract, DerivedEquivalenceContractReport,
};
use crate::derived_topology::materialized_graph::{
    MaterializationFallbackClass, MaterializedTopologyView,
};
use crate::derived_topology::traversal_views::InterpretedTopologyView;
use crate::validation::DerivedTopologyValidationReport;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DerivedInvalidationTargetRow {
    pub target: DerivedInvalidationTarget,
    pub bridge_scope: String,
    pub declaration_ids: Vec<String>,
    pub triggered: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DerivedInvalidationReport {
    pub touched_aspect_count: usize,
    pub topology_touched: bool,
    pub naming_touched: bool,
    pub triggered_target_count: usize,
    pub rows: Vec<DerivedInvalidationTargetRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DerivedRebuildReport {
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
pub struct DerivedFallbackReport {
    pub whole_view_materialization: bool,
    pub materialization_fallback_class: Option<MaterializationFallbackClass>,
    pub precision_fallback_count: usize,
    pub precision_budget_fallback_count: usize,
    pub explicit_fallback_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DerivedReadDiagnostics {
    pub invalidation_report: DerivedInvalidationReport,
    pub rebuild_report: DerivedRebuildReport,
    pub fallback_report: DerivedFallbackReport,
    pub equivalence_contract_report: DerivedEquivalenceContractReport,
}

pub fn build_derived_read_diagnostics(
    read_basis: &DerivedTopologyReadBasis,
    materialized: &MaterializedTopologyView,
    interpreted: &InterpretedTopologyView,
    validation: &DerivedTopologyValidationReport,
) -> DerivedReadDiagnostics {
    DerivedReadDiagnostics {
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
) -> DerivedInvalidationReport {
    build_derived_invalidation_report_from_aspects(read_basis.touched_aspects().iter().copied())
}

pub fn build_derived_invalidation_report_from_aspects(
    touched_aspects: impl IntoIterator<Item = Aspect>,
) -> DerivedInvalidationReport {
    let touched_aspects = touched_aspects.into_iter().collect::<Vec<_>>();
    let topology_touched = touched_aspects
        .iter()
        .any(|aspect| matches!(aspect, Aspect::Topology(_)));
    let naming_touched = touched_aspects
        .iter()
        .any(|aspect| matches!(aspect, Aspect::Naming(_)));

    let mut grouped = BTreeMap::<DerivedInvalidationTarget, Vec<String>>::new();
    for declaration in milestone_two_invalidation_declarations() {
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
            DerivedInvalidationTargetRow {
                target,
                bridge_scope: target.bridge_scope().to_string(),
                declaration_ids,
                triggered,
            }
        })
        .collect::<Vec<_>>();

    DerivedInvalidationReport {
        touched_aspect_count: touched_aspects.len(),
        topology_touched,
        naming_touched,
        triggered_target_count: rows.iter().filter(|row| row.triggered).count(),
        rows,
    }
}

pub(crate) fn triggered_invalidation_targets(
    read_basis: &DerivedTopologyReadBasis,
) -> Vec<DerivedInvalidationTarget> {
    triggered_invalidation_targets_from_aspects(read_basis.touched_aspects().iter().copied())
}

pub(crate) fn triggered_invalidation_targets_from_aspects(
    touched_aspects: impl IntoIterator<Item = Aspect>,
) -> Vec<DerivedInvalidationTarget> {
    let mut targets = Vec::new();
    for aspect in touched_aspects {
        match aspect {
            Aspect::Topology(topology) => match topology {
                schema::facade::platform::aspects::TopologyAspect::Structure => {
                    push_unique_target(&mut targets, DerivedInvalidationTarget::TopologyStructure);
                }
                schema::facade::platform::aspects::TopologyAspect::Ownership => {
                    push_unique_target(&mut targets, DerivedInvalidationTarget::TopologyOwnership);
                }
                schema::facade::platform::aspects::TopologyAspect::Boundary => {
                    push_unique_target(&mut targets, DerivedInvalidationTarget::TopologyBoundary);
                }
                schema::facade::platform::aspects::TopologyAspect::Radial => {
                    push_unique_target(&mut targets, DerivedInvalidationTarget::TopologyRadial);
                }
            },
            Aspect::Naming(schema::facade::platform::aspects::NamingAspect::PersistentName) => {
                push_unique_target(
                    &mut targets,
                    DerivedInvalidationTarget::NamingPersistentName,
                );
            }
            _ => {}
        }
    }
    targets
}

fn push_unique_target(
    targets: &mut Vec<DerivedInvalidationTarget>,
    target: DerivedInvalidationTarget,
) {
    if !targets.contains(&target) {
        targets.push(target);
    }
}

pub fn build_derived_rebuild_report(
    materialized: &MaterializedTopologyView,
    interpreted: &InterpretedTopologyView,
    validation: &DerivedTopologyValidationReport,
) -> DerivedRebuildReport {
    DerivedRebuildReport {
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
) -> DerivedFallbackReport {
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
) -> DerivedFallbackReport {
    let explicit_fallback_count = precision_fallback_count
        + precision_budget_fallback_count
        + usize::from(materialized.report().fallback_class.is_some());

    DerivedFallbackReport {
        whole_view_materialization: materialized.report().whole_view_materialization,
        materialization_fallback_class: materialized.report().fallback_class,
        precision_fallback_count,
        precision_budget_fallback_count,
        explicit_fallback_count,
    }
}

#[cfg(test)]
mod tests {
    use schema::facade::topology_authoring::{
        seed_milestone_one_primitive, MilestoneOnePrimitiveCase,
    };

    use crate::facade::{
        build_derived_read_diagnostics, interpret_topology_view, validate_interpreted_topology,
        TopologyMaterializer,
    };
    use crate::validation::reference_integrity::milestone_one_runtime_builder;

    #[test]
    fn derived_diagnostics_reports_are_explicit_and_deterministic() {
        let mut runtime = milestone_one_runtime_builder()
            .expect(" milestone one runtime builder")
            .build();
        let verified = seed_milestone_one_primitive(
            &mut runtime,
            "phase-seven-diagnostics",
            &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
        )
        .expect("verified primitive");
        let read_view = runtime
            .read_truth()
            .read_snapshot(&verified.persisted_truth().snapshot)
            .expect("snapshot read");
        let materialized =
            TopologyMaterializer::materialize_from_truth(&read_view).expect("materialized");
        let interpreted = interpret_topology_view(&materialized);
        let validation =
            validate_interpreted_topology(&materialized, &interpreted).expect("validation");

        let diagnostics = build_derived_read_diagnostics(
            &verified.read_basis(),
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
                == schema::facade::platform::authority::DerivedInvalidationTarget::TopologyStructure
                && row.triggered));
        assert!(diagnostics
            .invalidation_report
            .rows
            .iter()
            .any(|row| row.target
                == schema::facade::platform::authority::DerivedInvalidationTarget::TopologyBoundary
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
