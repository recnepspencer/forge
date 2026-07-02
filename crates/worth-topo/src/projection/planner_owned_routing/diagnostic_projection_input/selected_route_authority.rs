use crate::compiled_product_reuse_decision::TopologyDerivedReuseDecisionPosture;
use schema::facade::platform::authority::touched_graph_conflict::{
    BatchAdmissionPlannerRouteWitnessKind, ConflictIndependencePlannerRouteWitnessKind,
};

use crate::derived_topology::compiled_product_consumer_cutover::DerivedEquivalenceContractReport;

use super::admission_error::{
    require_optional_match, require_string_match, TopologyDerivedReadDiagnosticInputAdmissionError,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyDerivedReadDiagnosticSelectedRouteAuthority {
    selected_route_identity_digest: String,
    selected_equivalence_family_identity: String,
    selected_product_identity_digest: String,
    selected_equivalence_policy_identity_digest: String,
    selected_compatibility_basis_identity_digest: String,
    selected_reuse_basis_identity_digest: String,
    compiled_product_reuse_route_packet_identity: Option<String>,
    topology_reuse_posture: Option<TopologyDerivedReuseDecisionPosture>,
    spatial_reuse_posture: Option<String>,
    spatial_reuse_decision_identity: Option<String>,
    spatial_rebuild_denial_identity: Option<String>,
    batch_admission_route_packet_identity: Option<String>,
    batch_admission_denial_witness_identity: Option<String>,
    batch_admission_denial_witness_kind: Option<BatchAdmissionPlannerRouteWitnessKind>,
    conflict_independence_route_packet_identity: Option<String>,
    conflict_independence_denial_witness_identity: Option<String>,
    conflict_independence_denial_witness_kind: Option<ConflictIndependencePlannerRouteWitnessKind>,
}

impl TopologyDerivedReadDiagnosticSelectedRouteAuthority {
    pub fn from_selected_route_identities(
        selected_route_identity_digest: impl Into<String>,
        selected_equivalence_family_identity: impl Into<String>,
        selected_product_identity_digest: impl Into<String>,
        selected_equivalence_policy_identity_digest: impl Into<String>,
        selected_compatibility_basis_identity_digest: impl Into<String>,
        selected_reuse_basis_identity_digest: impl Into<String>,
        compiled_product_reuse_route_packet_identity: Option<String>,
        topology_reuse_posture: Option<TopologyDerivedReuseDecisionPosture>,
        spatial_reuse_posture: Option<String>,
        spatial_reuse_decision_identity: Option<String>,
        spatial_rebuild_denial_identity: Option<String>,
        batch_admission_route_packet_identity: Option<String>,
        batch_admission_denial_witness_identity: Option<String>,
        batch_admission_denial_witness_kind: Option<BatchAdmissionPlannerRouteWitnessKind>,
        conflict_independence_route_packet_identity: Option<String>,
        conflict_independence_denial_witness_identity: Option<String>,
        conflict_independence_denial_witness_kind: Option<
            ConflictIndependencePlannerRouteWitnessKind,
        >,
    ) -> Self {
        Self {
            selected_route_identity_digest: selected_route_identity_digest.into(),
            selected_equivalence_family_identity: selected_equivalence_family_identity.into(),
            selected_product_identity_digest: selected_product_identity_digest.into(),
            selected_equivalence_policy_identity_digest:
                selected_equivalence_policy_identity_digest.into(),
            selected_compatibility_basis_identity_digest:
                selected_compatibility_basis_identity_digest.into(),
            selected_reuse_basis_identity_digest: selected_reuse_basis_identity_digest.into(),
            compiled_product_reuse_route_packet_identity,
            topology_reuse_posture,
            spatial_reuse_posture,
            spatial_reuse_decision_identity,
            spatial_rebuild_denial_identity,
            batch_admission_route_packet_identity,
            batch_admission_denial_witness_identity,
            batch_admission_denial_witness_kind,
            conflict_independence_route_packet_identity,
            conflict_independence_denial_witness_identity,
            conflict_independence_denial_witness_kind,
        }
    }

    pub(super) fn selected_route_identity_digest(&self) -> &str {
        &self.selected_route_identity_digest
    }

    pub(super) fn batch_admission_route_packet_identity(&self) -> Option<&str> {
        self.batch_admission_route_packet_identity.as_deref()
    }
    pub(super) fn compiled_product_reuse_route_packet_identity(&self) -> Option<&str> {
        self.compiled_product_reuse_route_packet_identity.as_deref()
    }
    pub(super) const fn topology_reuse_posture(
        &self,
    ) -> Option<TopologyDerivedReuseDecisionPosture> {
        self.topology_reuse_posture
    }
    pub(super) fn spatial_reuse_posture(&self) -> Option<&str> {
        self.spatial_reuse_posture.as_deref()
    }
    pub(super) fn spatial_reuse_decision_identity(&self) -> Option<&str> {
        self.spatial_reuse_decision_identity.as_deref()
    }
    pub(super) fn spatial_rebuild_denial_identity(&self) -> Option<&str> {
        self.spatial_rebuild_denial_identity.as_deref()
    }

    pub(super) fn batch_admission_denial_witness_identity(&self) -> Option<&str> {
        self.batch_admission_denial_witness_identity.as_deref()
    }

    pub(super) const fn batch_admission_denial_witness_kind(
        &self,
    ) -> Option<BatchAdmissionPlannerRouteWitnessKind> {
        self.batch_admission_denial_witness_kind
    }

    pub(super) fn conflict_independence_route_packet_identity(&self) -> Option<&str> {
        self.conflict_independence_route_packet_identity.as_deref()
    }

    pub(super) fn conflict_independence_denial_witness_identity(&self) -> Option<&str> {
        self.conflict_independence_denial_witness_identity
            .as_deref()
    }

    pub(super) const fn conflict_independence_denial_witness_kind(
        &self,
    ) -> Option<ConflictIndependencePlannerRouteWitnessKind> {
        self.conflict_independence_denial_witness_kind
    }
}

pub(super) fn require_selected_route_authority_matches(
    equivalence_contract_report: &DerivedEquivalenceContractReport,
    authority: &TopologyDerivedReadDiagnosticSelectedRouteAuthority,
) -> Result<(), TopologyDerivedReadDiagnosticInputAdmissionError> {
    require_string_match(
        "selected product identity",
        equivalence_contract_report
            .compiled_product_identity_digest()
            .unwrap_or_default(),
        authority.selected_product_identity_digest.as_str(),
    )?;
    require_string_match(
        "selected equivalence policy identity",
        equivalence_contract_report
            .equivalence_policy_identity_digest()
            .unwrap_or_default(),
        authority
            .selected_equivalence_policy_identity_digest
            .as_str(),
    )?;
    require_optional_match(
        "selected equivalence family identity",
        equivalence_contract_report
            .selected_equivalence_family_identity()
            .map(|identity| identity.as_str()),
        Some(authority.selected_equivalence_family_identity.as_str()),
    )?;
    require_optional_match(
        "selected compatibility basis identity",
        equivalence_contract_report.selected_compatibility_basis_identity_digest(),
        Some(
            authority
                .selected_compatibility_basis_identity_digest
                .as_str(),
        ),
    )?;
    require_optional_match(
        "selected reuse basis identity",
        equivalence_contract_report.selected_reuse_basis_identity_digest(),
        Some(authority.selected_reuse_basis_identity_digest.as_str()),
    )?;
    Ok(())
}
