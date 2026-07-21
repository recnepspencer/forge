use worth_proof::{
    Artifact, AssumptionBasis, AuthorityMarker, AuthorityWitness, CurrentValidity,
    FreshnessScopedBasis, NoProofs, PhaseMarker,
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

struct WorthQueryAftermathAuthority {
    _private: (),
}
impl AuthorityMarker for WorthQueryAftermathAuthority {}

pub(crate) type WorthQueryAftermathAuthorityProof = Artifact<
    WorthQueryAdmittedAftermathPhase,
    WorthQueryAftermathAuthorityAnchor,
    NoProofs,
    FreshnessScopedBasis<CurrentValidity, AssumptionBasis<WorthQueryAftermathAuthorityBasis>>,
>;

pub(crate) fn mint_aftermath_authority(
    identity: String,
    predecessor_identity: String,
    basis: WorthQueryAftermathAuthorityBasis,
) -> WorthQueryAftermathAuthorityProof {
    Artifact::with_current_basis(
        WorthQueryAftermathAuthorityAnchor {
            identity,
            predecessor_identity,
        },
        basis,
        AuthorityWitness::from_authority_marker(WorthQueryAftermathAuthority { _private: () }),
    )
}

pub(crate) fn aftermath_authority_basis(
    proof: &WorthQueryAftermathAuthorityProof,
) -> &WorthQueryAftermathAuthorityBasis {
    proof.basis().basis().value()
}
