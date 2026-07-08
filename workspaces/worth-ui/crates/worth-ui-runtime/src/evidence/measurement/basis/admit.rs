use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::declaration::{
    declared_measurement_basis_requirements, UiDeclarationIdentity,
    UiDeclaredMeasurementPolicyPosture,
};
use crate::graph::{UiGraphNodeIdentity, UiGraphWorldProfile};

use super::assembly::SelectedEvidence;
use super::denial::UiMeasurementBasisDenial;
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
    let dependency_lineage = selected.dependency_lineage();
    let dependency_map = derive_measurement_dependency_map(&dependency_lineage);
    let neighborhood_class_hint =
        derive_measurement_neighborhood_class_hint(&requirements, &dependency_map);
    let basis_posture = selected.basis_posture();
    let generation = basis_generation(
        declaration_support_authority_generation,
        selected.query_receipt,
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
    let identity_digest = basis_identity_digest(
        &requirements,
        &declaration_identity,
        graph_node_identity,
        &world_profile,
        declaration_support_authority_generation,
        declared_measurement_policy,
        &evidence_inputs,
        &dependency_lineage,
        &dependency_map,
        neighborhood_class_hint,
        &generation_compatibility,
        denial_posture.as_ref(),
    );

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
