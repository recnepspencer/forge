#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiAdmittedPortalAnchorContract {
    identity: super::UiPortalAnchorIdentity,
    neighborhood_identity: crate::evidence::UiAllocationNeighborhoodIdentity,
    graph_generation: crate::graph::UiGraphGeneration,
    measurement_basis_identity_digest: u64,
    measurement_basis_generation: crate::evidence::UiMeasurementBasisGeneration,
    planning_input_identity_digest: u64,
    source_generation_digest: u64,
    witness: crate::evidence::UiHostMeasurementAuthorityWitness,
}

impl UiAdmittedPortalAnchorContract {
    pub(crate) fn seal(
        identity: super::UiPortalAnchorIdentity,
        basis: &crate::evidence::UiMeasurementBasis,
        neighborhood: &crate::evidence::UiAllocationNeighborhood,
        planning_input_identity_digest: u64,
        source_generation_digest: u64,
        witness: crate::evidence::UiHostMeasurementAuthorityWitness,
    ) -> Self {
        Self {
            identity,
            neighborhood_identity: neighborhood.identity().clone(),
            graph_generation: neighborhood.graph_generation(),
            measurement_basis_identity_digest: basis.identity_digest(),
            measurement_basis_generation: basis.generation(),
            planning_input_identity_digest,
            source_generation_digest,
            witness,
        }
    }

    pub fn identity(&self) -> super::UiPortalAnchorIdentity {
        self.identity
    }

    pub(crate) fn identity_digest(&self) -> u64 {
        self.identity.identity_digest()
            ^ self.neighborhood_identity.identity_digest().rotate_left(7)
            ^ self.graph_generation.as_u64().rotate_left(13)
            ^ self.measurement_basis_identity_digest.rotate_left(19)
            ^ self.measurement_basis_generation.raw().rotate_left(29)
            ^ self.planning_input_identity_digest.rotate_left(41)
            ^ self.source_generation_digest.rotate_left(47)
            ^ self.witness.identity_digest().rotate_left(53)
    }

    pub(crate) fn witness(&self) -> crate::evidence::UiHostMeasurementAuthorityWitness {
        self.witness
    }

    pub(crate) fn neighborhood_identity(
        &self,
    ) -> &crate::evidence::UiAllocationNeighborhoodIdentity {
        &self.neighborhood_identity
    }

    pub(crate) fn graph_generation(&self) -> crate::graph::UiGraphGeneration {
        self.graph_generation
    }

    pub(super) fn matches_basis(&self, basis: &crate::evidence::UiMeasurementBasis) -> bool {
        basis.identity_digest() == self.measurement_basis_identity_digest
            && basis.generation() == self.measurement_basis_generation
    }
}
