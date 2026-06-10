use super::{
    planar_structural_identity_authority_entries, planar_structural_identity_digest,
    PlanarStructuralIdentityBasis, PlanarStructuralIdentityCounters,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarStructuralIdentityReceipt {
    basis: PlanarStructuralIdentityBasis,
    declaration_digest: String,
    envelope_digest: String,
    structural_identity_digest: String,
    canonical_transform_basis_digest: String,
    counters: PlanarStructuralIdentityCounters,
}

impl PlanarStructuralIdentityReceipt {
    pub(crate) fn new(
        basis: PlanarStructuralIdentityBasis,
        declaration_digest: String,
        envelope_digest: String,
        structural_identity_digest: String,
        canonical_transform_basis_digest: String,
        counters: PlanarStructuralIdentityCounters,
    ) -> Self {
        Self {
            basis,
            declaration_digest,
            envelope_digest,
            structural_identity_digest,
            canonical_transform_basis_digest,
            counters,
        }
    }

    pub(crate) fn structural_digest_for(basis: &PlanarStructuralIdentityBasis) -> String {
        planar_structural_identity_digest(&structural_digest_parts(basis))
    }

    pub(crate) fn transform_digest_for(basis: &PlanarStructuralIdentityBasis) -> String {
        planar_structural_identity_digest(&[
            format!(
                "local_frame:{}",
                basis.canonical_transform_basis().local_frame_identity()
            ),
            format!(
                "movement_rotation:{}",
                basis
                    .canonical_transform_basis()
                    .movement_rotation_posture_identity()
            ),
            format!(
                "transform_chain:{}",
                basis.canonical_transform_basis().transform_chain_digest()
            ),
            format!(
                "orientation:{}",
                basis
                    .canonical_transform_basis()
                    .orientation_policy()
                    .as_str()
            ),
        ])
    }

    pub fn basis(&self) -> &PlanarStructuralIdentityBasis {
        &self.basis
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn envelope_digest(&self) -> &str {
        &self.envelope_digest
    }

    pub fn structural_identity_digest(&self) -> &str {
        &self.structural_identity_digest
    }

    pub fn canonical_transform_basis_digest(&self) -> &str {
        &self.canonical_transform_basis_digest
    }

    pub fn binding_identity(&self) -> &str {
        self.basis.binding_identity()
    }

    pub fn counters(&self) -> PlanarStructuralIdentityCounters {
        self.counters
    }
}

fn structural_digest_parts(basis: &PlanarStructuralIdentityBasis) -> Vec<String> {
    planar_structural_identity_authority_entries(basis)
        .into_iter()
        .map(|entry| entry.digest_part())
        .collect()
}
