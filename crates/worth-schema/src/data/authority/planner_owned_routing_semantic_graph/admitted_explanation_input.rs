use serde::{Deserialize, Serialize};

use super::artifact_kind::PlannerExplanationArtifactKind;
use super::error::{
    PlannerOwnedRoutingSemanticGraphVocabularyError,
    PlannerOwnedRoutingSemanticGraphVocabularyErrorKind,
};
use super::identity_digest::planner_owned_routing_semantic_graph_identity_digest;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PlannerAdmittedExplanationInput {
    authority_owner: String,
    admitted_packet_digest: String,
    identity_digest: String,
}

impl PlannerAdmittedExplanationInput {
    pub fn authority_owner(&self) -> &str {
        &self.authority_owner
    }

    pub fn admitted_packet_digest(&self) -> &str {
        &self.admitted_packet_digest
    }

    pub fn identity_digest(&self) -> &str {
        &self.identity_digest
    }

    pub const fn artifact_kind(&self) -> PlannerExplanationArtifactKind {
        PlannerExplanationArtifactKind::PriorProofInput
    }
}

pub(crate) fn admit_planner_admitted_explanation_input(
    authority_owner: impl Into<String>,
    admitted_packet_digest: impl Into<String>,
) -> Result<PlannerAdmittedExplanationInput, PlannerOwnedRoutingSemanticGraphVocabularyError> {
    let authority_owner = authority_owner.into();
    if authority_owner.trim().is_empty() {
        return Err(PlannerOwnedRoutingSemanticGraphVocabularyError::new(
            PlannerOwnedRoutingSemanticGraphVocabularyErrorKind::EmptyAuthorityOwner,
            "planner explanation input requires a non-empty authority owner",
        ));
    }
    let admitted_packet_digest = admitted_packet_digest.into();
    if admitted_packet_digest.trim().is_empty() {
        return Err(PlannerOwnedRoutingSemanticGraphVocabularyError::new(
            PlannerOwnedRoutingSemanticGraphVocabularyErrorKind::EmptyAdmittedPacketDigest,
            "planner explanation input requires a non-empty admitted packet digest",
        ));
    }
    let identity_digest = planner_owned_routing_semantic_graph_identity_digest(
        "worth-schema:planner-admitted-explanation-input:v1",
        &[
            format!("owner:{authority_owner}"),
            format!("packet:{admitted_packet_digest}"),
        ],
    );
    Ok(PlannerAdmittedExplanationInput {
        authority_owner,
        admitted_packet_digest,
        identity_digest,
    })
}
