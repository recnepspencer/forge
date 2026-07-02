use crate::compiled_product_reuse_decision::TopologyDerivedReuseDecisionPosture;
use schema::facade::platform::authority::{
    touched_graph_conflict::{
        BatchAdmissionPlannerRouteWitnessKind, ConflictIndependencePlannerRouteWitnessKind,
    },
    DerivedInvalidationTarget,
};

use crate::derived_topology::compiled_product_consumer_cutover::DerivedEquivalenceContractReport;
use crate::derived_topology::materialized_graph::MaterializationFallbackClass;
use crate::projection::planner_owned_routing::diagnostic_projection_input::TopologyDerivedDiagnosticProjectionSource;
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
pub struct DerivedValidationExecutionReport {
    pub source: String,
    pub execution_count: usize,
    pub registered_rule_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DerivedReadDiagnostics {
    pub(crate) diagnostic_projection_source: TopologyDerivedDiagnosticProjectionSource,
    pub compiled_product_reuse_route_packet_identity: Option<String>,
    pub topology_reuse_posture: Option<TopologyDerivedReuseDecisionPosture>,
    pub spatial_reuse_posture: Option<String>,
    pub spatial_reuse_decision_identity_digest: Option<String>,
    pub spatial_rebuild_denial_identity_digest: Option<String>,
    pub batch_admission_route_packet_identity: Option<String>,
    pub batch_admission_denial_witness_identity: Option<String>,
    pub batch_admission_denial_witness_kind: Option<BatchAdmissionPlannerRouteWitnessKind>,
    pub conflict_independence_route_packet_identity: Option<String>,
    pub conflict_independence_denial_witness_identity: Option<String>,
    pub conflict_independence_denial_witness_kind:
        Option<ConflictIndependencePlannerRouteWitnessKind>,
    pub invalidation_report: DerivedInvalidationReport,
    pub rebuild_report: DerivedRebuildReport,
    pub fallback_report: DerivedFallbackReport,
    pub validation_report: DerivedTopologyValidationReport,
    pub validation_execution_report: DerivedValidationExecutionReport,
    pub equivalence_contract_report: DerivedEquivalenceContractReport,
}

#[cfg(test)]
mod tests {
    use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;

    use crate::derived_topology::materialized_graph::TopologyMaterializer;
    use crate::derived_topology::traversal_views::bootstrap_topology_interpretation;
    use crate::projection::planner_owned_routing::diagnostic_projection_input::build_derived_read_diagnostics;
    use crate::projection::planner_owned_routing::diagnostic_projection_input::derive_topology_validation_report;
    use crate::test_support::schema_topology_authoring_boundary::seed_milestone_one_primitive_through_schema_execution;
    use crate::validation::reference_integrity::milestone_one_runtime_builder;

    #[test]
    fn derived_diagnostics_reports_are_explicit_and_deterministic() {
        let mut runtime = milestone_one_runtime_builder()
            .expect(" milestone one runtime builder")
            .build();
        let verified = seed_milestone_one_primitive_through_schema_execution(
            &mut runtime,
            "phase-six-diagnostics",
            &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
        )
        .expect("verified primitive");
        let read_view = runtime
            .read_truth()
            .read_snapshot(verified.snapshot())
            .expect("snapshot read");
        let materialized =
            TopologyMaterializer::materialize_from_truth(&read_view).expect("materialized");
        let interpreted = bootstrap_topology_interpretation(&materialized);
        let validation =
            derive_topology_validation_report(&materialized, &interpreted).expect("validation");

        let diagnostics = build_derived_read_diagnostics(
            &verified.read_basis(),
            &materialized,
            &interpreted,
            &validation,
        );

        assert_eq!(
            diagnostics
                .diagnostic_projection_source
                .truth_basis_identity_digest(),
            verified
                .read_basis()
                .authority
                .truth_basis_identity
                .mutation_digest_hex
        );
        assert_eq!(
            diagnostics
                .diagnostic_projection_source
                .diagnostic_contract_name(),
            "topology-derived-read-diagnostic-projection"
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
        assert_eq!(diagnostics.validation_report, validation);
        assert_eq!(diagnostics.validation_execution_report.execution_count, 1);
        assert_eq!(
            diagnostics
                .validation_execution_report
                .registered_rule_count,
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
