use std::sync::Arc;
use worth_foundational::facade::admit_foundational_authority_identity;
use worth_proof::{
    Artifact, AssumptionBasis, AuthorityMarker, AuthorityWitness, CurrentValidity,
    FreshnessScopedBasis, NoProofs, PhaseMarker,
};

use crate::identity_authority::{
    query_operation_progression_authority, QueryOperationProgressionAuthorityIdentity,
    QueryOperationProgressionIdentityKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthQueryOperationAuthorityBasis {
    pub(crate) runtime_authority: u64,
    pub(crate) installation_runtime_authority: u64,
    pub(crate) installation_generation: u64,
    pub(crate) domain_authority_identity: String,
    pub(crate) operation_identity: String,
    pub(crate) binding_identity: String,
    pub(crate) capability_identity: u64,
    pub(crate) basis_identity: String,
    pub(crate) graph_authority_identities: Vec<String>,
    pub(crate) required_domain_authority_identities: Vec<String>,
}

#[derive(Debug)]
pub(crate) struct WorthQueryBoundOperationPhase;
#[derive(Debug)]
pub(crate) struct WorthQueryExecutedOperationPhase;
#[derive(Debug)]
pub(crate) struct WorthQueryPublishedOperationPhase;
#[derive(Debug)]
pub(crate) struct WorthQueryConsumedOperationPhase;
#[derive(Debug)]
pub(crate) struct WorthQuerySettledOperationPhase;
#[derive(Debug)]
pub(crate) struct WorthQueryWorkflowRunPhase;
#[derive(Debug)]
pub(crate) struct WorthQueryWorkflowStagePhase;
#[derive(Debug)]
pub(crate) struct WorthQueryCompletedWorkflowPhase;
#[derive(Debug)]
pub(crate) struct WorthQueryConditionalReentryPhase;
#[derive(Debug)]
pub(crate) struct WorthQueryDiscardedProvisionalPhase;
#[derive(Debug)]
pub(crate) struct WorthQueryLineageBoundOperationPhase;
#[derive(Debug)]
pub(crate) struct WorthQueryPersistentNamingPhase;
#[derive(Debug)]
pub(crate) struct WorthQueryPromotionOnReferencePhase;
#[derive(Debug)]
pub(crate) struct WorthQueryImpactClassifiedPhase;

impl PhaseMarker for WorthQueryBoundOperationPhase {}
impl PhaseMarker for WorthQueryExecutedOperationPhase {}
impl PhaseMarker for WorthQueryPublishedOperationPhase {}
impl PhaseMarker for WorthQueryConsumedOperationPhase {}
impl PhaseMarker for WorthQuerySettledOperationPhase {}
impl PhaseMarker for WorthQueryWorkflowRunPhase {}
impl PhaseMarker for WorthQueryWorkflowStagePhase {}
impl PhaseMarker for WorthQueryCompletedWorkflowPhase {}
impl PhaseMarker for WorthQueryConditionalReentryPhase {}
impl PhaseMarker for WorthQueryDiscardedProvisionalPhase {}
impl PhaseMarker for WorthQueryLineageBoundOperationPhase {}
impl PhaseMarker for WorthQueryPersistentNamingPhase {}
impl PhaseMarker for WorthQueryPromotionOnReferencePhase {}
impl PhaseMarker for WorthQueryImpactClassifiedPhase {}

#[derive(Debug)]
pub(crate) struct WorthQueryOperationPhaseAnchor {
    identity: String,
    predecessor_identity: Option<String>,
}

impl WorthQueryOperationPhaseAnchor {
    pub(crate) fn identity(&self) -> &str {
        &self.identity
    }

    pub(crate) fn predecessor_identity(&self) -> Option<&str> {
        self.predecessor_identity.as_deref()
    }
}

struct WorthQueryOperationProgressionProofAuthority {
    _private: (),
}

impl AuthorityMarker for WorthQueryOperationProgressionProofAuthority {}

type WorthQueryOperationProgressionProof<P> = Artifact<
    P,
    WorthQueryOperationPhaseAnchor,
    NoProofs,
    FreshnessScopedBasis<CurrentValidity, AssumptionBasis<WorthQueryOperationAuthorityBasis>>,
>;

pub(crate) struct WorthQueryOperationPhaseProof<P: PhaseMarker> {
    proof: WorthQueryOperationProgressionProof<P>,
    _owner_identity:
        QueryOperationProgressionAuthorityIdentity<Arc<str>, QueryOperationProgressionIdentityKind>,
}

impl<P: PhaseMarker> WorthQueryOperationPhaseProof<P> {
    pub(crate) fn payload(&self) -> &WorthQueryOperationPhaseAnchor {
        self.proof.payload()
    }

    fn basis(
        &self,
    ) -> &FreshnessScopedBasis<CurrentValidity, AssumptionBasis<WorthQueryOperationAuthorityBasis>>
    {
        self.proof.basis()
    }
}

impl<P: PhaseMarker> std::fmt::Debug for WorthQueryOperationPhaseProof<P> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryOperationPhaseProof")
            .field("identity", &self.payload().identity())
            .finish_non_exhaustive()
    }
}

pub(crate) fn mint_operation_phase_proof<P: PhaseMarker>(
    identity: impl Into<String>,
    predecessor_identity: Option<&str>,
    basis: WorthQueryOperationAuthorityBasis,
) -> WorthQueryOperationPhaseProof<P> {
    let identity = identity.into();
    let proof = Artifact::with_current_basis(
        WorthQueryOperationPhaseAnchor {
            identity: identity.clone(),
            predecessor_identity: predecessor_identity.map(str::to_owned),
        },
        basis,
        AuthorityWitness::from_authority_marker(WorthQueryOperationProgressionProofAuthority {
            _private: (),
        }),
    );
    let owner_identity = admit_foundational_authority_identity(
        Arc::<str>::from(identity),
        query_operation_progression_authority(),
    );
    WorthQueryOperationPhaseProof {
        proof,
        _owner_identity: owner_identity,
    }
}

pub(crate) fn operation_phase_basis<P: PhaseMarker>(
    proof: &WorthQueryOperationPhaseProof<P>,
) -> &WorthQueryOperationAuthorityBasis {
    proof.basis().basis().value()
}
