use std::sync::Arc;
use worth_foundational::facade::admit_foundational_authority_identity;
use worth_foundational::facade::CanonicalEquivalentBasis;
use worth_proof::{
    Artifact, AssumptionBasis, AuthorityMarker, AuthorityWitness, CurrentValidity,
    FreshnessScopedBasis, NoProofs, PhaseMarker,
};

use crate::identity_authority::{
    query_domain_capability_authority, QueryDomainCapabilityAuthorityIdentity,
    QueryDomainCapabilityIdentityKind,
};

use crate::basis_lifecycle::BasisOperationLane;

use super::super::super::WorthQueryBoundDomainOperation;
use super::super::pair::WorthQueryCompatibilityPairBasis;

#[derive(Debug)]
pub(super) struct WorthQueryCompatibilityAnchor {
    relationship: &'static str,
}

impl WorthQueryCompatibilityAnchor {
    pub(super) fn relationship(&self) -> &'static str {
        self.relationship
    }
}

struct WorthQueryOperationalCompatibilityAuthority(());
impl AuthorityMarker for WorthQueryOperationalCompatibilityAuthority {}

type WorthQueryCompatibilityProgressionProof<P> = Artifact<
    P,
    WorthQueryCompatibilityAnchor,
    NoProofs,
    FreshnessScopedBasis<CurrentValidity, AssumptionBasis<WorthQueryCompatibilityPairBasis>>,
>;

pub(super) struct WorthQueryCompatibilityProof<P: PhaseMarker> {
    proof: WorthQueryCompatibilityProgressionProof<P>,
    _owner_identity:
        QueryDomainCapabilityAuthorityIdentity<Arc<str>, QueryDomainCapabilityIdentityKind>,
}

impl<P: PhaseMarker> WorthQueryCompatibilityProof<P> {
    pub(super) fn payload(&self) -> &WorthQueryCompatibilityAnchor {
        self.proof.payload()
    }

    pub(super) fn basis(
        &self,
    ) -> &FreshnessScopedBasis<CurrentValidity, AssumptionBasis<WorthQueryCompatibilityPairBasis>>
    {
        self.proof.basis()
    }
}

pub(in crate::domain_installation::compatibility) struct WorthQueryPortableAndBasisEvidence {
    portable: worth_query_installation::facade::WorthQueryPortableOperationComparisonEquivalent,
    basis: CanonicalEquivalentBasis,
}

impl WorthQueryPortableAndBasisEvidence {
    pub(in crate::domain_installation::compatibility) fn new(
        portable: worth_query_installation::facade::WorthQueryPortableOperationComparisonEquivalent,
        basis: CanonicalEquivalentBasis,
    ) -> Self {
        Self { portable, basis }
    }

    pub(super) fn comparison_count(&self) -> u32 {
        self.portable.work().owner_dimensions_inspected() + self.basis.entry_count()
    }
}

pub(super) fn mint_pair_proof<P: PhaseMarker, D, O, F, L: BasisOperationLane>(
    relationship: &'static str,
    subject: &WorthQueryBoundDomainOperation<D, O, F, L>,
    candidate: &WorthQueryBoundDomainOperation<D, O, F, L>,
) -> WorthQueryCompatibilityProof<P> {
    let owner_identity = admit_foundational_authority_identity(
        Arc::<str>::from(crate::identity::hash_parts(&[
            "worth_query_compatibility_authority_v1".into(),
            format!("relationship:{relationship}"),
            format!("subject:{}", subject.binding_identity()),
            format!("candidate:{}", candidate.binding_identity()),
        ])),
        query_domain_capability_authority(),
    );
    let proof = Artifact::with_current_basis(
        WorthQueryCompatibilityAnchor { relationship },
        WorthQueryCompatibilityPairBasis::from_bounds(subject, candidate),
        AuthorityWitness::from_authority_marker(WorthQueryOperationalCompatibilityAuthority(())),
    );
    WorthQueryCompatibilityProof {
        proof,
        _owner_identity: owner_identity,
    }
}
