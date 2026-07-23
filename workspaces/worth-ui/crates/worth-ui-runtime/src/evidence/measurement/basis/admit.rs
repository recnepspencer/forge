use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::declaration::{
    declared_measurement_basis_requirements, UiDeclarationIdentity,
    UiDeclaredMeasurementPolicyPosture,
};
use crate::graph::{UiGraphNodeIdentity, UiGraphWorldProfile};

use super::assembly::SelectedEvidence;
use super::denial::UiMeasurementBasisDenial;
use super::evidence_index::UiMeasurementEvidenceIndex;
use super::identity::{basis_generation, basis_identity_digest};
use crate::evidence::measurement::{
    derive_measurement_dependency_map, derive_measurement_neighborhood_class_hint,
    MeasurementEvidenceInput, UiMeasurementDependencyLineage, UiMeasurementDependencyMap,
    UiMeasurementGenerationCompatibility, UiMeasurementNeighborhoodClassHint,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMeasurementBasisGeneration {
    value: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMeasurementBasisPosture {
    QueryOnly,
    HostOnly,
    QueryAndHost,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UiMeasurementBasis {
    identity_digest: u64,
    generation: UiMeasurementBasisGeneration,
    declaration_identity: UiDeclarationIdentity,
    graph_node_identity: UiGraphNodeIdentity,
    world_profile: UiGraphWorldProfile,
    declaration_support_authority_generation: UiEvidenceAuthorityGeneration,
    declared_measurement_policy: UiDeclaredMeasurementPolicyPosture,
    basis_posture: UiMeasurementBasisPosture,
    evidence_inputs: Box<[MeasurementEvidenceInput]>,
    evidence_index: UiMeasurementEvidenceIndex,
    generation_compatibility: UiMeasurementGenerationCompatibility,
    dependency_lineage: UiMeasurementDependencyLineage,
    dependency_map: UiMeasurementDependencyMap,
    neighborhood_class_hint: UiMeasurementNeighborhoodClassHint,
    denial_posture: Option<UiMeasurementBasisDenial>,
}

pub fn admit_measurement_basis(
    declaration_identity: UiDeclarationIdentity,
    graph_node_identity: UiGraphNodeIdentity,
    world_profile: UiGraphWorldProfile,
    declaration_support_authority_generation: UiEvidenceAuthorityGeneration,
    declared_measurement_policy: &UiDeclaredMeasurementPolicyPosture,
    evidence_inputs: &[MeasurementEvidenceInput],
) -> UiMeasurementBasis {
    let requirements = declared_measurement_basis_requirements(declared_measurement_policy);
    let selected = SelectedEvidence::from_inputs(&requirements, evidence_inputs);
    let generation_compatibility =
        selected.generation_compatibility(&world_profile, declaration_support_authority_generation);
    let denial_posture = selected.denial_posture(&requirements, &generation_compatibility);
    let evidence_inputs = selected.admitted_inputs();
    let evidence_index = UiMeasurementEvidenceIndex::build(&evidence_inputs, graph_node_identity);
    let dependency_lineage = selected.dependency_lineage();
    let dependency_map = derive_measurement_dependency_map(&dependency_lineage);
    let neighborhood_class_hint =
        derive_measurement_neighborhood_class_hint(&requirements, &dependency_map);
    let basis_posture = selected.basis_posture();
    let generation = basis_generation(
        declaration_support_authority_generation,
        selected
            .query_receipt
            .map(|receipt| receipt.declaration_support_authority_generation()),
        selected.host_capability_report,
        [
            selected.host_results.text_intrinsic_size,
            selected.host_results.font_metrics,
            selected.host_results.native_control_intrinsic_size,
            selected.host_results.viewport_extent,
            selected.host_results.portal_anchor_rect,
            selected.host_results.scroll_container_viewport,
        ],
    );
    let identity_digest = basis_identity_digest(super::UiMeasurementBasisIdentityInput {
        requirements: &requirements,
        declaration_identity: &declaration_identity,
        graph_node_identity,
        world_profile: &world_profile,
        declaration_support_authority_generation,
        declared_measurement_policy,
        evidence_inputs: &evidence_inputs,
        dependency_lineage: &dependency_lineage,
        dependency_map: &dependency_map,
        neighborhood_class_hint,
        generation_compatibility: &generation_compatibility,
        denial_posture: denial_posture.as_ref(),
    });

    UiMeasurementBasis {
        identity_digest,
        generation,
        declaration_identity,
        graph_node_identity,
        world_profile,
        declaration_support_authority_generation,
        declared_measurement_policy: declared_measurement_policy.clone(),
        basis_posture,
        evidence_inputs,
        evidence_index,
        generation_compatibility,
        dependency_lineage,
        dependency_map,
        neighborhood_class_hint,
        denial_posture,
    }
}

impl UiMeasurementBasisGeneration {
    pub const fn new(value: u64) -> Self {
        Self { value }
    }

    pub const fn raw(self) -> u64 {
        self.value
    }
}

impl UiMeasurementBasis {
    pub(crate) fn operationally_matches(&self, other: &Self) -> bool {
        self.declaration_identity == other.declaration_identity
            && self.graph_node_identity == other.graph_node_identity
            && self.world_profile == other.world_profile
            && self.declared_measurement_policy == other.declared_measurement_policy
            && self.basis_posture == other.basis_posture
            && self.neighborhood_class_hint == other.neighborhood_class_hint
            && self.evidence_inputs.len() == other.evidence_inputs.len()
            && self
                .evidence_inputs
                .iter()
                .zip(other.evidence_inputs.iter())
                .all(|(left, right)| left.operationally_matches(right))
    }

    pub fn identity_digest(&self) -> u64 {
        self.identity_digest
    }

    pub fn generation(&self) -> UiMeasurementBasisGeneration {
        self.generation
    }

    pub fn declaration_identity(&self) -> &UiDeclarationIdentity {
        &self.declaration_identity
    }

    pub fn graph_node_identity(&self) -> UiGraphNodeIdentity {
        self.graph_node_identity
    }

    pub fn world_profile(&self) -> &UiGraphWorldProfile {
        &self.world_profile
    }

    pub fn declaration_support_authority_generation(&self) -> UiEvidenceAuthorityGeneration {
        self.declaration_support_authority_generation
    }

    pub fn declared_measurement_policy(&self) -> &UiDeclaredMeasurementPolicyPosture {
        &self.declared_measurement_policy
    }

    pub fn basis_posture(&self) -> UiMeasurementBasisPosture {
        self.basis_posture
    }

    pub fn evidence_inputs(&self) -> &[MeasurementEvidenceInput] {
        &self.evidence_inputs
    }

    pub(crate) fn query_allocation_mappings_for_source(
        &self,
        source_key: &super::UiQueryAllocationSourceKey,
    ) -> &[super::UiQueryAllocationTargetMapping] {
        self.evidence_index.query_mappings(source_key)
    }

    pub(crate) fn host_measurement_result(
        &self,
        request_identity: worth_ui_host_contract::UiMeasurementRequestIdentity,
    ) -> Option<&crate::evidence::UiMeasurementResult> {
        self.evidence_index
            .host_position(request_identity)
            .and_then(|position| self.evidence_inputs.get(position))
            .and_then(MeasurementEvidenceInput::as_host_measurement_result)
    }

    pub(crate) fn host_allocation_target(
        &self,
        request_identity: worth_ui_host_contract::UiMeasurementRequestIdentity,
    ) -> Option<UiGraphNodeIdentity> {
        self.host_measurement_result(request_identity)
            .map(|_| self.graph_node_identity)
    }

    pub(crate) fn durable_resize_support(
        &self,
        input_identity_digest: u64,
    ) -> Option<&crate::evidence::UiMeasurementSiblingResizeSupport> {
        self.evidence_index
            .durable_position(input_identity_digest)
            .and_then(|position| self.evidence_inputs.get(position))
            .and_then(MeasurementEvidenceInput::as_sibling_resize_support)
    }

    pub(crate) fn query_allocation_mappings(
        &self,
    ) -> impl Iterator<
        Item = (
            &super::UiQueryAllocationSourceKey,
            &super::UiQueryAllocationTargetMapping,
        ),
    > {
        self.evidence_index.query_rows()
    }

    pub(crate) fn host_allocation_requests(
        &self,
    ) -> impl Iterator<Item = worth_ui_host_contract::UiMeasurementRequestIdentity> + '_ {
        self.evidence_index.host_requests()
    }

    pub(crate) fn durable_resize_inputs(&self) -> impl Iterator<Item = u64> + '_ {
        self.evidence_index.durable_inputs()
    }

    pub fn generation_compatibility(&self) -> &UiMeasurementGenerationCompatibility {
        &self.generation_compatibility
    }

    pub fn dependency_lineage(&self) -> &UiMeasurementDependencyLineage {
        &self.dependency_lineage
    }

    pub fn dependency_map(&self) -> &UiMeasurementDependencyMap {
        &self.dependency_map
    }

    pub fn neighborhood_class_hint(&self) -> UiMeasurementNeighborhoodClassHint {
        self.neighborhood_class_hint
    }

    pub fn denial_posture(&self) -> Option<&UiMeasurementBasisDenial> {
        self.denial_posture.as_ref()
    }

    pub fn is_admitted(&self) -> bool {
        self.denial_posture.is_none()
    }
}
