use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactShapeError, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::{HadwigerCanonicalArtifact, HadwigerQueryDeclarationReference};

use super::equivalence_proofs::{TilingCandidateEquivalenceProof, TilingEquivalenceCounters};
use super::equivalence_scopes::TilingEquivalenceScope;
use super::suppression_requests::TilingCandidateSuppressionRequest;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TilingSuppressionPosture {
    BlocksEquivalentExperiment,
    Unsupported,
}

impl TilingSuppressionPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BlocksEquivalentExperiment => "blocks_equivalent_experiment",
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TilingCandidateSuppressionProof {
    core: HadwigerArtifactCore,
    suppression_id: String,
    equivalence: TilingCandidateEquivalenceProof,
    posture: TilingSuppressionPosture,
    query_declaration_reference: HadwigerQueryDeclarationReference,
    counters: TilingEquivalenceCounters,
}

impl TilingCandidateSuppressionProof {
    pub(crate) fn checked(
        request: TilingCandidateSuppressionRequest,
        query_declaration_reference: HadwigerQueryDeclarationReference,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let blocks = request
            .equivalence()
            .equivalence_scope()
            .requires_reactivation_for_replan()
            && request.equivalence().posture()
                != super::equivalence_proofs::TilingCandidateEquivalencePosture::Unsupported
            && request.suppression_proof().blocks_equivalent_experiment();
        let posture = if blocks {
            TilingSuppressionPosture::BlocksEquivalentExperiment
        } else {
            TilingSuppressionPosture::Unsupported
        };
        let counters = TilingEquivalenceCounters::suppression(1);
        let core = artifact_core(
            HadwigerArtifactKind::TilingCandidateSuppressionProof,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::QueryDeclaration(query_declaration_reference.clone()),
            vec![
                request.corpus().reference(),
                request.equivalence().reference(),
                request.suppression_proof().reference(),
            ],
            vec![
                HadwigerArtifactPayloadEntry::text(
                    "schema",
                    "WORTH.hadwiger.tiling_suppression.v1",
                ),
                HadwigerArtifactPayloadEntry::text("suppression_id", request.suppression_id()),
                HadwigerArtifactPayloadEntry::text(
                    "equivalence_scope",
                    request.equivalence().equivalence_scope().as_str(),
                ),
                HadwigerArtifactPayloadEntry::text(
                    "equivalence_token",
                    request.equivalence().equivalence_token(),
                ),
                HadwigerArtifactPayloadEntry::text("posture", posture.as_str()),
                HadwigerArtifactPayloadEntry::text(
                    "query_declaration",
                    query_declaration_reference.stable_token(),
                ),
                HadwigerArtifactPayloadEntry::text("counters", counters.stable_token()),
            ],
        )?;
        Ok(Self {
            core,
            suppression_id: request.suppression_id().to_string(),
            equivalence: request.equivalence().clone(),
            posture,
            query_declaration_reference,
            counters,
        })
    }

    pub fn suppression_id(&self) -> &str {
        &self.suppression_id
    }

    pub fn equivalence(&self) -> &TilingCandidateEquivalenceProof {
        &self.equivalence
    }

    pub fn equivalence_scope(&self) -> TilingEquivalenceScope {
        self.equivalence.equivalence_scope()
    }

    pub fn posture(&self) -> TilingSuppressionPosture {
        self.posture
    }

    pub fn query_declaration_reference(&self) -> &HadwigerQueryDeclarationReference {
        &self.query_declaration_reference
    }

    pub fn counters(&self) -> &TilingEquivalenceCounters {
        &self.counters
    }

    pub fn blocks_equivalent_experiment(&self) -> bool {
        self.posture == TilingSuppressionPosture::BlocksEquivalentExperiment
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(TilingCandidateSuppressionProof, core);
