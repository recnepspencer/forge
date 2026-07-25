use std::rc::Rc;

use crate::evidence::{UiAllocationNeighborhoodIdentity, UiMeasurementBasisGeneration};
use crate::graph::UiGraphGeneration;

#[derive(Clone, Debug)]
pub(crate) struct UiAdmittedAllocationPlanReference {
    planning_identity_digest: u64,
    measurement_basis_generation: UiMeasurementBasisGeneration,
    neighborhood_identity: UiAllocationNeighborhoodIdentity,
    candidate: Rc<crate::runtime::UiAllocationCandidate>,
}

impl PartialEq for UiAdmittedAllocationPlanReference {
    fn eq(&self, other: &Self) -> bool {
        self.generation_key() == other.generation_key() && self.candidate == other.candidate
    }
}

impl Eq for UiAdmittedAllocationPlanReference {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiReplanGenerationKey {
    pub(in crate::graph::allocation_neighborhood) neighborhood_identity:
        UiAllocationNeighborhoodIdentity,
    graph_generation: UiGraphGeneration,
    measurement_basis_generation: UiMeasurementBasisGeneration,
    pub(in crate::graph::allocation_neighborhood) planning_identity_digest: u64,
}

impl UiReplanGenerationKey {
    pub(crate) fn identity_digest(&self) -> u64 {
        self.neighborhood_identity.identity_digest()
            ^ self.graph_generation.as_u64().rotate_left(11)
            ^ self.measurement_basis_generation.raw().rotate_left(23)
            ^ self.planning_identity_digest.rotate_left(37)
    }

    pub(in crate::graph::allocation_neighborhood) fn measurement_generation(
        &self,
    ) -> UiMeasurementBasisGeneration {
        self.measurement_basis_generation
    }
}

impl UiAdmittedAllocationPlanReference {
    pub(crate) fn from_candidate(candidate: crate::runtime::UiAllocationCandidate) -> Self {
        Self {
            planning_identity_digest: candidate.planning_identity_digest(),
            measurement_basis_generation: candidate.measurement_basis().generation(),
            neighborhood_identity: candidate.allocation_neighborhood().identity().clone(),
            candidate: Rc::new(candidate),
        }
    }

    pub(crate) fn candidate(&self) -> &crate::runtime::UiAllocationCandidate {
        &self.candidate
    }

    pub(crate) fn planning_identity_digest(&self) -> u64 {
        self.planning_identity_digest
    }

    pub(crate) fn generation_key(&self) -> UiReplanGenerationKey {
        UiReplanGenerationKey {
            neighborhood_identity: self.neighborhood_identity.clone(),
            graph_generation: self.neighborhood_identity.graph_generation(),
            measurement_basis_generation: self.measurement_basis_generation,
            planning_identity_digest: self.planning_identity_digest,
        }
    }
}
