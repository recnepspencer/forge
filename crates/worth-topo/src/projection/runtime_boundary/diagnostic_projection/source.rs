use std::collections::BTreeMap;

use schema::facade::platform::aspects::Aspect;
use schema::facade::platform::authority::{
    milestone_two_invalidation_declarations, DerivedInvalidationTarget,
};
use schema::facade::topology_authoring::DerivedTopologyReadBasis;

use crate::compiled_product_family::triggered_invalidation_targets_from_touched_aspects;
use crate::derived_topology::compiled_product_consumer_cutover::{
    build_derived_equivalence_contract, DerivedEquivalenceContractReport,
};
use crate::derived_topology::materialized_graph::MaterializedTopologyView;
use crate::derived_topology::traversal_views::InterpretedTopologyView;
use crate::validation::{
    validate_interpreted_topology, DerivedTopologyValidationReport,
    RegisteredTopologyValidationReport, TopologyValidationError,
};

use super::report_types::{
    DerivedFallbackReport, DerivedInvalidationReport, DerivedInvalidationTargetRow,
    DerivedReadDiagnostics, DerivedRebuildReport, DerivedValidationExecutionReport,
};

const DERIVED_READ_VALIDATION_SOURCE: &str = "worth.topo.derived_read.validation";
const TOPOLOGY_DIAGNOSTIC_CONTRACT_NAME: &str = "topology-derived-read-diagnostic-projection";

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) struct TopologyDerivedDiagnosticProjectionSource {
    truth_basis_identity_digest: String,
    diagnostic_contract_name: String,
}

impl TopologyDerivedDiagnosticProjectionSource {
    #[cfg(test)]
    pub(crate) fn truth_basis_identity_digest(&self) -> &str {
        &self.truth_basis_identity_digest
    }

    #[cfg(test)]
    pub(crate) fn diagnostic_contract_name(&self) -> &str {
        &self.diagnostic_contract_name
    }
}

pub(crate) fn topology_derived_diagnostic_projection_source(
    read_basis: &DerivedTopologyReadBasis,
    _equivalence_contract_report: &DerivedEquivalenceContractReport,
) -> TopologyDerivedDiagnosticProjectionSource {
    TopologyDerivedDiagnosticProjectionSource {
        truth_basis_identity_digest: read_basis
            .authority
            .truth_basis_identity
            .mutation_digest_hex
            .clone(),
        diagnostic_contract_name: TOPOLOGY_DIAGNOSTIC_CONTRACT_NAME.to_string(),
    }
}

pub(crate) fn build_derived_read_diagnostics(
    read_basis: &DerivedTopologyReadBasis,
    materialized: &MaterializedTopologyView,
    interpreted: &InterpretedTopologyView,
    validation: &DerivedTopologyValidationReport,
) -> DerivedReadDiagnostics {
    let equivalence_contract_report =
        build_derived_equivalence_contract(read_basis, materialized, interpreted, validation);
    DerivedReadDiagnostics {
        diagnostic_projection_source: topology_derived_diagnostic_projection_source(
            read_basis,
            &equivalence_contract_report,
        ),
        compiled_product_reuse_route_packet_identity: None,
        topology_reuse_posture: None,
        spatial_reuse_posture: None,
        spatial_reuse_decision_identity_digest: None,
        spatial_rebuild_denial_identity_digest: None,
        batch_admission_route_packet_identity: None,
        batch_admission_denial_witness_identity: None,
        batch_admission_denial_witness_kind: None,
        conflict_independence_route_packet_identity: None,
        conflict_independence_denial_witness_identity: None,
        conflict_independence_denial_witness_kind: None,
        invalidation_report: build_derived_invalidation_report(read_basis),
        rebuild_report: build_derived_rebuild_report(materialized, interpreted, validation),
        fallback_report: build_derived_fallback_report(read_basis, materialized),
        validation_report: validation.clone(),
        validation_execution_report: derived_validation_execution_report(validation.rows.len()),
        equivalence_contract_report,
    }
}

pub(crate) fn derive_topology_validation_report(
    materialized: &MaterializedTopologyView,
    interpreted: &InterpretedTopologyView,
) -> Result<DerivedTopologyValidationReport, TopologyValidationError> {
    let report = validate_interpreted_topology(materialized, interpreted)?;
    RegisteredTopologyValidationReport::from_report(report.clone())
        .map_err(|error| TopologyValidationError::new("validation_registry", error))?;
    Ok(report)
}

pub(crate) fn derived_validation_execution_report(
    registered_rule_count: usize,
) -> DerivedValidationExecutionReport {
    DerivedValidationExecutionReport {
        source: DERIVED_READ_VALIDATION_SOURCE.to_string(),
        execution_count: 1,
        registered_rule_count,
    }
}

pub(crate) fn build_derived_invalidation_report(
    read_basis: &DerivedTopologyReadBasis,
) -> DerivedInvalidationReport {
    build_derived_invalidation_report_from_aspects(read_basis.touched_aspects().iter().copied())
}

pub(crate) fn build_derived_invalidation_report_from_aspects(
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
        triggered_invalidation_targets_from_touched_aspects(touched_aspects.iter().copied());
    let rows = grouped
        .into_iter()
        .map(|(target, declaration_ids)| DerivedInvalidationTargetRow {
            bridge_scope: target.bridge_scope().to_string(),
            triggered: triggered_targets.contains(&target),
            target,
            declaration_ids,
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

pub(crate) fn build_derived_rebuild_report(
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

pub(crate) fn build_derived_fallback_report(
    read_basis: &DerivedTopologyReadBasis,
    materialized: &MaterializedTopologyView,
) -> DerivedFallbackReport {
    build_derived_fallback_report_from_counts(
        read_basis.precision_fallbacks.len(),
        read_basis.precision_budget_fallbacks.len(),
        materialized,
    )
}

pub(crate) fn build_derived_fallback_report_from_counts(
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
