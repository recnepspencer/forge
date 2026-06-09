use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactShapeError, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::{HadwigerCanonicalArtifact, HadwigerQueryDeclarationReference};

use super::equivalence_proofs::TilingEquivalenceCounters;
use super::reactivation_requests::TilingReactivationRequest;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TilingReactivationPosture {
    PermitsReplanning,
    Rejected,
}

impl TilingReactivationPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PermitsReplanning => "permits_replanning",
            Self::Rejected => "rejected",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TilingReactivationChecked {
    core: HadwigerArtifactCore,
    reactivation_id: String,
    posture: TilingReactivationPosture,
    query_declaration_reference: HadwigerQueryDeclarationReference,
    counters: TilingEquivalenceCounters,
}

impl TilingReactivationChecked {
    pub(crate) fn checked(
        request: TilingReactivationRequest,
        query_declaration_reference: HadwigerQueryDeclarationReference,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let counters = TilingEquivalenceCounters::reactivation(1);
        let posture = TilingReactivationPosture::PermitsReplanning;
        let core = artifact_core(
            HadwigerArtifactKind::TilingReactivationChecked,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::QueryDeclaration(query_declaration_reference.clone()),
            vec![
                request.suppression().reference(),
                request.reactivation_condition().reference(),
                request
                    .reactivation_condition()
                    .qualifying_evidence()
                    .clone(),
            ],
            vec![
                HadwigerArtifactPayloadEntry::text(
                    "schema",
                    "forge.hadwiger.tiling_reactivation.v1",
                ),
                HadwigerArtifactPayloadEntry::text("reactivation_id", request.reactivation_id()),
                HadwigerArtifactPayloadEntry::text("posture", posture.as_str()),
                HadwigerArtifactPayloadEntry::text(
                    "qualifying_evidence",
                    request
                        .reactivation_condition()
                        .qualifying_evidence()
                        .stable_token(),
                ),
                HadwigerArtifactPayloadEntry::text(
                    "query_declaration",
                    query_declaration_reference.stable_token(),
                ),
                HadwigerArtifactPayloadEntry::text("counters", counters.stable_token()),
            ],
        )?;
        Ok(Self {
            core,
            reactivation_id: request.reactivation_id().to_string(),
            posture,
            query_declaration_reference,
            counters,
        })
    }

    pub fn reactivation_id(&self) -> &str {
        &self.reactivation_id
    }

    pub fn posture(&self) -> TilingReactivationPosture {
        self.posture
    }

    pub fn query_declaration_reference(&self) -> &HadwigerQueryDeclarationReference {
        &self.query_declaration_reference
    }

    pub fn counters(&self) -> &TilingEquivalenceCounters {
        &self.counters
    }

    pub fn permits_replanning(&self) -> bool {
        self.posture == TilingReactivationPosture::PermitsReplanning
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(TilingReactivationChecked, core);
