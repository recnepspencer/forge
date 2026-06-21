mod admitted_artifact;
mod birth_proof_support;
pub(crate) mod family_birth_input;
mod topology_ready_birth;

use self::topology_ready_birth::prepare_primitive_construction_topology_ready_birth;
use super::digest::digest_owned_parts;
use super::request::{PrimitiveConstructionPhaseError, PrimitiveConstructionRequest};
use worth_geom::facade::{
    PrimitiveConditioningWitness, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
};
use worth_primitives::PrimitiveConstructionBirthSynopsisContract;
use worth_primitives::PrimitiveConstructionFamilyKey;

pub(crate) use self::admitted_artifact::PreparedPrimitiveConstructionAdmittedArtifact;
#[cfg(test)]
pub(crate) use self::birth_proof_support::PrimitiveConstructionBirthPlacementFacts;

pub(crate) struct PrimitiveConstructionAdmittedRealizationPosture {
    selected_strategy: PrimitiveRealizationStrategy,
    attempted_strategies: Vec<PrimitiveRealizationStrategy>,
    conditioning_witness: PrimitiveConditioningWitness,
    stability_class: PrimitiveStabilityClass,
    realization_digest: String,
    realization_geometry_digest: String,
}

impl PrimitiveConstructionAdmittedRealizationPosture {
    pub(crate) fn selected_strategy(&self) -> PrimitiveRealizationStrategy {
        self.selected_strategy
    }

    pub(crate) fn attempted_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        &self.attempted_strategies
    }

    pub(crate) fn conditioning_witness(&self) -> &PrimitiveConditioningWitness {
        &self.conditioning_witness
    }

    pub(crate) fn stability_class(&self) -> PrimitiveStabilityClass {
        self.stability_class
    }

    pub(crate) fn realization_digest(&self) -> &str {
        &self.realization_digest
    }

    pub(crate) fn realization_geometry_digest(&self) -> &str {
        &self.realization_geometry_digest
    }
}

pub(crate) struct PrimitiveConstructionAdmittedBirthTopologyTruth {
    family: PrimitiveConstructionFamilyKey,
    birth_contract: PrimitiveConstructionBirthSynopsisContract,
    scaffold_digest: String,
    birth_digest: String,
    topology_birth_class: String,
    supported_vertex_count: usize,
    supported_edge_count: usize,
    supported_loop_count: usize,
    supported_wire_count: usize,
    supported_face_count: usize,
    supported_shell_count: usize,
    supported_body_count: usize,
    consequence_digest: String,
    birth_mapping_digest: String,
}

impl PrimitiveConstructionAdmittedBirthTopologyTruth {
    pub(crate) fn family(&self) -> PrimitiveConstructionFamilyKey {
        self.family
    }

    pub(crate) fn birth_contract(&self) -> PrimitiveConstructionBirthSynopsisContract {
        self.birth_contract
    }

    pub(crate) fn scaffold_digest(&self) -> &str {
        &self.scaffold_digest
    }

    pub(crate) fn birth_digest(&self) -> &str {
        &self.birth_digest
    }

    pub(crate) fn topology_birth_class(&self) -> &str {
        &self.topology_birth_class
    }

    pub(crate) fn supported_vertex_count(&self) -> usize {
        self.supported_vertex_count
    }

    pub(crate) fn supported_edge_count(&self) -> usize {
        self.supported_edge_count
    }

    pub(crate) fn supported_loop_count(&self) -> usize {
        self.supported_loop_count
    }

    pub(crate) fn supported_wire_count(&self) -> usize {
        self.supported_wire_count
    }

    pub(crate) fn supported_face_count(&self) -> usize {
        self.supported_face_count
    }

    pub(crate) fn supported_shell_count(&self) -> usize {
        self.supported_shell_count
    }

    pub(crate) fn supported_body_count(&self) -> usize {
        self.supported_body_count
    }

    pub(crate) fn consequence_digest(&self) -> &str {
        &self.consequence_digest
    }

    pub(crate) fn birth_mapping_digest(&self) -> &str {
        &self.birth_mapping_digest
    }
}

struct PreparedPrimitiveConstructionAdmittedRuntimeInputs {
    birth_topology_truth: PrimitiveConstructionAdmittedBirthTopologyTruth,
    realization_posture: PrimitiveConstructionAdmittedRealizationPosture,
    topology_query_admitted_handoff:
        topology::facade::TopologyPrimitiveConstructionQueryAdmittedHandoff,
}

pub(crate) fn prepare_primitive_construction_admitted_artifact(
    request: &PrimitiveConstructionRequest,
) -> Result<PreparedPrimitiveConstructionAdmittedArtifact, PrimitiveConstructionPhaseError> {
    let prepared = prepare_primitive_construction_admitted_runtime_inputs(request)?;
    Ok(admitted_artifact_from_prepared_runtime_inputs(prepared))
}

pub(crate) fn prepare_primitive_construction_executed_admitted_artifact(
    workspace: &mut forge_query::facade::ForgeQueryWorkspace,
    request: &PrimitiveConstructionRequest,
) -> Result<PreparedPrimitiveConstructionAdmittedArtifact, PrimitiveConstructionPhaseError> {
    let prepared = prepare_primitive_construction_admitted_runtime_inputs(request)?;
    let declared_touched_basis =
        topology::facade::TopologyPrimitiveConstructionBirthDeclaredTouchedBasis::from_admitted_handoff(
            &prepared.topology_query_admitted_handoff,
        )
        .map_err(PrimitiveConstructionPhaseError::TopologyBirthCompose)?;
    let compose_execution = topology::facade::run_primitive_construction_birth_declared_touched_basis_compose(
        workspace,
        prepared.topology_query_admitted_handoff.clone(),
        declared_touched_basis,
    )
    .map_err(PrimitiveConstructionPhaseError::TopologyBirthCompose)?;
    Ok(admitted_artifact_from_prepared_runtime_inputs(prepared)
        .with_topology_compose_evidence(compose_execution.evidence().clone()))
}

fn admitted_artifact_from_prepared_runtime_inputs(
    prepared: PreparedPrimitiveConstructionAdmittedRuntimeInputs,
) -> PreparedPrimitiveConstructionAdmittedArtifact {
    PreparedPrimitiveConstructionAdmittedArtifact::from_topology_query_admitted_handoff(
        prepared.topology_query_admitted_handoff,
        prepared
            .birth_topology_truth
            .consequence_digest()
            .to_string(),
        prepared
            .birth_topology_truth
            .birth_mapping_digest()
            .to_string(),
        prepared.realization_posture.conditioning_witness().clone(),
        prepared.realization_posture.selected_strategy(),
        prepared.realization_posture.attempted_strategies().to_vec(),
        prepared.realization_posture.stability_class(),
        prepared
            .realization_posture
            .realization_digest()
            .to_string(),
        prepared
            .realization_posture
            .realization_geometry_digest()
            .to_string(),
    )
}

fn prepare_primitive_construction_admitted_runtime_inputs(
    request: &PrimitiveConstructionRequest,
) -> Result<PreparedPrimitiveConstructionAdmittedRuntimeInputs, PrimitiveConstructionPhaseError> {
    let admitted_birth_input = prepare_primitive_construction_admitted_birth_input(request)?;
    let (birth_topology_truth, realization_posture) =
        admitted_birth_input.into_topology_and_realization();
    let topology_query_admitted_handoff =
        prepare_primitive_construction_topology_ready_birth(&birth_topology_truth)
            .map_err(PrimitiveConstructionPhaseError::TopologyQueryAdmittedHandoff)?;
    Ok(PreparedPrimitiveConstructionAdmittedRuntimeInputs {
        birth_topology_truth,
        realization_posture,
        topology_query_admitted_handoff,
    })
}

fn prepare_primitive_construction_admitted_birth_input(
    request: &PrimitiveConstructionRequest,
) -> Result<
    family_birth_input::PrimitiveConstructionAdmittedBirthInput,
    PrimitiveConstructionPhaseError,
> {
    let intent_digest = digest_owned_parts(&[
        request.request_digest().to_string(),
        request.family().as_str().to_string(),
        "admitted".to_string(),
    ]);
    family_birth_input::build_family_birth_input(request, &intent_digest)
}

pub(crate) fn prepare_primitive_construction_admitted_realization_posture(
    request: &PrimitiveConstructionRequest,
) -> Result<PrimitiveConstructionAdmittedRealizationPosture, PrimitiveConstructionPhaseError> {
    Ok(prepare_primitive_construction_admitted_birth_input(request)?.into_realization_posture())
}

#[cfg(test)]
pub(crate) fn prepare_primitive_construction_birth_placement_facts(
    request: &PrimitiveConstructionRequest,
) -> Result<PrimitiveConstructionBirthPlacementFacts, PrimitiveConstructionPhaseError> {
    Ok(prepare_primitive_construction_admitted_birth_input(request)?.placement_facts())
}

#[cfg(test)]
pub(crate) fn prepare_primitive_construction_topology_query_admitted_handoff_from_request(
    request: &PrimitiveConstructionRequest,
) -> Result<
    topology::facade::TopologyPrimitiveConstructionQueryAdmittedHandoff,
    PrimitiveConstructionPhaseError,
> {
    let birth_topology_truth =
        prepare_primitive_construction_admitted_birth_input(request)?.into_birth_topology_truth();
    topology_ready_birth::prepare_primitive_construction_topology_query_admitted_handoff(
        &birth_topology_truth,
    )
    .map_err(PrimitiveConstructionPhaseError::TopologyQueryAdmittedHandoff)
}
