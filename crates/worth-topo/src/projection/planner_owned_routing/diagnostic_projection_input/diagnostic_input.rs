use crate::compiled_product_reuse_decision::TopologyDerivedReuseDecisionPosture;
use schema::facade::platform::authority::touched_graph_conflict::{
    BatchAdmissionPlannerRouteWitnessKind, ConflictIndependencePlannerRouteWitnessKind,
};
use schema::facade::topology_authoring::DerivedTopologyReadBasis;

use crate::derived_invalidation_route_input::TopologyInvalidationRouteInput;
use crate::derived_topology::compiled_product_consumer_cutover::{
    build_derived_equivalence_contract, DerivedEquivalenceContractReport,
};
use crate::derived_topology::materialized_graph::MaterializedTopologyView;
use crate::derived_topology::traversal_views::InterpretedTopologyView;
use crate::projection::diagnostic_surfaces::{
    DerivedFallbackReport, DerivedInvalidationReport, DerivedReadDiagnostics, DerivedRebuildReport,
    DerivedValidationExecutionReport,
};
use crate::validation::DerivedTopologyValidationReport;

use super::admission_error::{
    require_string_match, TopologyDerivedReadDiagnosticInputAdmissionError,
};
use super::selected_route_authority::{
    require_selected_route_authority_matches, TopologyDerivedReadDiagnosticSelectedRouteAuthority,
};
use super::source::{
    build_derived_fallback_report, build_derived_invalidation_report, build_derived_rebuild_report,
    derived_validation_execution_report, topology_derived_diagnostic_projection_source,
    TopologyDerivedDiagnosticProjectionSource,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyDerivedReadDiagnosticInput {
    diagnostic_projection_source: TopologyDerivedDiagnosticProjectionSource,
    invalidation_route_input: TopologyInvalidationRouteInput,
    selected_route_identity_digest: String,
    compiled_product_reuse_route_packet_identity: Option<String>,
    topology_reuse_posture: Option<TopologyDerivedReuseDecisionPosture>,
    spatial_reuse_posture: Option<String>,
    spatial_reuse_decision_identity_digest: Option<String>,
    spatial_rebuild_denial_identity_digest: Option<String>,
    batch_admission_route_packet_identity: Option<String>,
    batch_admission_denial_witness_identity: Option<String>,
    batch_admission_denial_witness_kind: Option<BatchAdmissionPlannerRouteWitnessKind>,
    conflict_independence_route_packet_identity: Option<String>,
    conflict_independence_denial_witness_identity: Option<String>,
    conflict_independence_denial_witness_kind: Option<ConflictIndependencePlannerRouteWitnessKind>,
    invalidation_report: DerivedInvalidationReport,
    rebuild_report: DerivedRebuildReport,
    fallback_report: DerivedFallbackReport,
    validation_report: DerivedTopologyValidationReport,
    validation_execution_report: DerivedValidationExecutionReport,
    equivalence_contract_report: DerivedEquivalenceContractReport,
}

pub(crate) fn admit_topology_derived_read_diagnostic_input(
    invalidation_route_input: &TopologyInvalidationRouteInput,
    authority: &TopologyDerivedReadDiagnosticSelectedRouteAuthority,
    read_basis: &DerivedTopologyReadBasis,
    materialized: &MaterializedTopologyView,
    interpreted: &InterpretedTopologyView,
    validation: &DerivedTopologyValidationReport,
) -> Result<TopologyDerivedReadDiagnosticInput, TopologyDerivedReadDiagnosticInputAdmissionError> {
    let equivalence_contract_report =
        build_derived_equivalence_contract(read_basis, materialized, interpreted, validation);
    require_selected_route_authority_matches(&equivalence_contract_report, authority)?;

    let invalidation_report = build_derived_invalidation_report(read_basis);

    let diagnostic_projection_source =
        topology_derived_diagnostic_projection_source(read_basis, &equivalence_contract_report);
    require_string_match(
        "truth basis identity",
        read_basis
            .authority
            .truth_basis_identity
            .mutation_digest_hex
            .as_str(),
        diagnostic_projection_source.truth_basis_identity_digest(),
    )?;

    Ok(TopologyDerivedReadDiagnosticInput {
        diagnostic_projection_source,
        invalidation_route_input: invalidation_route_input.clone(),
        selected_route_identity_digest: authority.selected_route_identity_digest().to_string(),
        compiled_product_reuse_route_packet_identity: authority
            .compiled_product_reuse_route_packet_identity()
            .map(str::to_string),
        topology_reuse_posture: authority.topology_reuse_posture(),
        spatial_reuse_posture: authority.spatial_reuse_posture().map(str::to_string),
        spatial_reuse_decision_identity_digest: authority
            .spatial_reuse_decision_identity()
            .map(str::to_string),
        spatial_rebuild_denial_identity_digest: authority
            .spatial_rebuild_denial_identity()
            .map(str::to_string),
        batch_admission_route_packet_identity: authority
            .batch_admission_route_packet_identity()
            .map(str::to_string),
        batch_admission_denial_witness_identity: authority
            .batch_admission_denial_witness_identity()
            .map(str::to_string),
        batch_admission_denial_witness_kind: authority.batch_admission_denial_witness_kind(),
        conflict_independence_route_packet_identity: authority
            .conflict_independence_route_packet_identity()
            .map(str::to_string),
        conflict_independence_denial_witness_identity: authority
            .conflict_independence_denial_witness_identity()
            .map(str::to_string),
        conflict_independence_denial_witness_kind: authority
            .conflict_independence_denial_witness_kind(),
        invalidation_report,
        rebuild_report: build_derived_rebuild_report(materialized, interpreted, validation),
        fallback_report: build_derived_fallback_report(read_basis, materialized),
        validation_report: validation.clone(),
        validation_execution_report: derived_validation_execution_report(validation.rows.len()),
        equivalence_contract_report,
    })
}

impl TopologyDerivedReadDiagnosticInput {
    pub fn diagnostic_projection_source(&self) -> &str {
        self.diagnostic_projection_source.diagnostic_contract_name()
    }

    pub fn truth_basis_identity_digest(&self) -> &str {
        self.diagnostic_projection_source
            .truth_basis_identity_digest()
    }

    pub fn invalidation_route_input(&self) -> &TopologyInvalidationRouteInput {
        &self.invalidation_route_input
    }

    pub fn selected_route_identity_digest(&self) -> &str {
        &self.selected_route_identity_digest
    }

    pub fn batch_admission_route_packet_identity(&self) -> Option<&str> {
        self.batch_admission_route_packet_identity.as_deref()
    }
    pub fn compiled_product_reuse_route_packet_identity(&self) -> Option<&str> {
        self.compiled_product_reuse_route_packet_identity.as_deref()
    }

    pub fn batch_admission_denial_witness_identity(&self) -> Option<&str> {
        self.batch_admission_denial_witness_identity.as_deref()
    }

    pub fn batch_admission_denial_witness_kind(
        &self,
    ) -> Option<BatchAdmissionPlannerRouteWitnessKind> {
        self.batch_admission_denial_witness_kind
    }

    pub fn conflict_independence_route_packet_identity(&self) -> Option<&str> {
        self.conflict_independence_route_packet_identity.as_deref()
    }

    pub fn conflict_independence_denial_witness_identity(&self) -> Option<&str> {
        self.conflict_independence_denial_witness_identity
            .as_deref()
    }

    pub fn conflict_independence_denial_witness_kind(
        &self,
    ) -> Option<ConflictIndependencePlannerRouteWitnessKind> {
        self.conflict_independence_denial_witness_kind
    }

    pub fn invalidation_report(&self) -> &DerivedInvalidationReport {
        &self.invalidation_report
    }

    pub fn rebuild_report(&self) -> &DerivedRebuildReport {
        &self.rebuild_report
    }

    pub fn fallback_report(&self) -> &DerivedFallbackReport {
        &self.fallback_report
    }

    pub fn validation_report(&self) -> &DerivedTopologyValidationReport {
        &self.validation_report
    }

    pub fn validation_execution_report(&self) -> &DerivedValidationExecutionReport {
        &self.validation_execution_report
    }

    pub fn equivalence_contract_report(&self) -> &DerivedEquivalenceContractReport {
        &self.equivalence_contract_report
    }

    pub fn selected_family_identity(&self) -> Option<&str> {
        self.equivalence_contract_report
            .selected_equivalence_family_identity()
            .map(|identity| identity.as_str())
    }

    pub fn as_read_diagnostics(&self) -> DerivedReadDiagnostics {
        DerivedReadDiagnostics {
            diagnostic_projection_source: self.diagnostic_projection_source.clone(),
            compiled_product_reuse_route_packet_identity: self
                .compiled_product_reuse_route_packet_identity
                .clone(),
            topology_reuse_posture: self.topology_reuse_posture,
            spatial_reuse_posture: self.spatial_reuse_posture.clone(),
            spatial_reuse_decision_identity_digest: self
                .spatial_reuse_decision_identity_digest
                .clone(),
            spatial_rebuild_denial_identity_digest: self
                .spatial_rebuild_denial_identity_digest
                .clone(),
            batch_admission_route_packet_identity: self
                .batch_admission_route_packet_identity
                .clone(),
            batch_admission_denial_witness_identity: self
                .batch_admission_denial_witness_identity
                .clone(),
            batch_admission_denial_witness_kind: self.batch_admission_denial_witness_kind,
            conflict_independence_route_packet_identity: self
                .conflict_independence_route_packet_identity
                .clone(),
            conflict_independence_denial_witness_identity: self
                .conflict_independence_denial_witness_identity
                .clone(),
            conflict_independence_denial_witness_kind: self
                .conflict_independence_denial_witness_kind,
            invalidation_report: self.invalidation_report.clone(),
            rebuild_report: self.rebuild_report.clone(),
            fallback_report: self.fallback_report.clone(),
            validation_report: self.validation_report.clone(),
            validation_execution_report: self.validation_execution_report.clone(),
            equivalence_contract_report: self.equivalence_contract_report.clone(),
        }
    }
}
