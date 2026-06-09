use super::super::request::{PrimitiveConstructionPhaseError, PrimitiveConstructionRequest};
#[cfg(test)]
use super::birth_proof_support::PrimitiveConstructionBirthPlacementFacts;

mod birth_scaffold;
mod error_mapping;
mod families;
pub(crate) mod geometry;
mod lower_layer_family_translation;
mod request_geometry_dispatch;
mod scalar_admission;
mod topology_counts;

pub(super) struct PrimitiveConstructionAdmittedBirthInput {
    #[cfg(test)]
    birth_topology_truth: super::PrimitiveConstructionAdmittedBirthTopologyTruth,
    realization_posture: super::PrimitiveConstructionAdmittedRealizationPosture,
    #[cfg(test)]
    placement_facts: PrimitiveConstructionBirthPlacementFacts,
}

impl PrimitiveConstructionAdmittedBirthInput {
    #[cfg(test)]
    pub(super) fn into_birth_topology_truth(
        self,
    ) -> super::PrimitiveConstructionAdmittedBirthTopologyTruth {
        self.birth_topology_truth
    }

    pub(super) fn into_realization_posture(
        self,
    ) -> super::PrimitiveConstructionAdmittedRealizationPosture {
        self.realization_posture
    }

    #[cfg(test)]
    pub(super) fn into_topology_and_realization(
        self,
    ) -> (
        super::PrimitiveConstructionAdmittedBirthTopologyTruth,
        super::PrimitiveConstructionAdmittedRealizationPosture,
    ) {
        (self.birth_topology_truth, self.realization_posture)
    }

    #[cfg(test)]
    pub(super) fn placement_facts(&self) -> PrimitiveConstructionBirthPlacementFacts {
        self.placement_facts
    }
}

pub(super) fn build_family_birth_input(
    request: &PrimitiveConstructionRequest,
    intent_digest: &str,
) -> Result<PrimitiveConstructionAdmittedBirthInput, PrimitiveConstructionPhaseError> {
    request_geometry_dispatch::build_request_geometry_birth_input(request, intent_digest)
}
