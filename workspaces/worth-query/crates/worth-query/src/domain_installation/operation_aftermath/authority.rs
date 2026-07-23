use std::sync::Arc;
use worth_foundational::facade::admit_foundational_authority_identity;
use worth_proof::{
    Artifact, AssumptionBasis, AuthorityMarker, AuthorityWitness, CurrentValidity,
    FreshnessScopedBasis, NoProofs, PhaseMarker,
};

use crate::identity_authority::{
    query_effect_lifecycle_authority, QueryEffectLifecycleAuthorityIdentity,
    QueryEffectLifecycleIdentityKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthQueryAftermathAuthorityBasis {
    pub(crate) runtime_authority: u64,
    pub(crate) installation_generation: u64,
    pub(crate) original_operation_identity: String,
    pub(crate) original_binding_identity: String,
    pub(crate) original_capability_identity: u64,
    pub(crate) original_trace_identity: String,
    pub(crate) candidate_operation_identity: String,
    pub(crate) candidate_binding_identity: String,
    pub(crate) candidate_capability_identity: u64,
    pub(crate) basis_identity: String,
    pub(crate) effect_receipt_identities: Vec<String>,
    pub(crate) original_lineage_report_identity: Option<String>,
}

#[derive(Debug)]
pub(crate) struct WorthQueryAdmittedAftermathPhase;
impl PhaseMarker for WorthQueryAdmittedAftermathPhase {}

#[derive(Debug)]
pub(crate) struct WorthQueryAftermathAuthorityAnchor {
    identity: String,
    predecessor_identity: String,
}

impl WorthQueryAftermathAuthorityAnchor {
    pub(crate) fn identity(&self) -> &str {
        &self.identity
    }
    pub(crate) fn predecessor_identity(&self) -> &str {
        &self.predecessor_identity
    }
}

struct WorthQueryAftermathProofAuthority {
    _private: (),
}
impl AuthorityMarker for WorthQueryAftermathProofAuthority {}

type WorthQueryAftermathProof = Artifact<
    WorthQueryAdmittedAftermathPhase,
    WorthQueryAftermathAuthorityAnchor,
    NoProofs,
    FreshnessScopedBasis<CurrentValidity, AssumptionBasis<WorthQueryAftermathAuthorityBasis>>,
>;

pub(crate) struct WorthQueryAftermathAuthorityProof {
    proof: WorthQueryAftermathProof,
    _owner_identity:
        QueryEffectLifecycleAuthorityIdentity<Arc<str>, QueryEffectLifecycleIdentityKind>,
}

impl WorthQueryAftermathAuthorityProof {
    pub(crate) fn payload(&self) -> &WorthQueryAftermathAuthorityAnchor {
        self.proof.payload()
    }

    fn basis(
        &self,
    ) -> &FreshnessScopedBasis<CurrentValidity, AssumptionBasis<WorthQueryAftermathAuthorityBasis>>
    {
        self.proof.basis()
    }
}

pub(crate) fn mint_aftermath_authority(
    identity: String,
    predecessor_identity: String,
    basis: WorthQueryAftermathAuthorityBasis,
) -> WorthQueryAftermathAuthorityProof {
    let owner_identity = admit_foundational_authority_identity(
        Arc::<str>::from(identity.clone()),
        query_effect_lifecycle_authority(),
    );
    let proof = Artifact::with_current_basis(
        WorthQueryAftermathAuthorityAnchor {
            identity,
            predecessor_identity,
        },
        basis,
        AuthorityWitness::from_authority_marker(WorthQueryAftermathProofAuthority { _private: () }),
    );
    WorthQueryAftermathAuthorityProof {
        proof,
        _owner_identity: owner_identity,
    }
}

pub(crate) fn aftermath_authority_basis(
    proof: &WorthQueryAftermathAuthorityProof,
) -> &WorthQueryAftermathAuthorityBasis {
    proof.basis().basis().value()
}
