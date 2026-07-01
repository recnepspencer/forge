use serde::{Deserialize, Serialize};

use super::artifact_kind::PlannerExplanationArtifactKind;
use super::error::{
    PlannerOwnedRoutingSemanticGraphVocabularyError,
    PlannerOwnedRoutingSemanticGraphVocabularyErrorKind,
};
use super::identity_digest::planner_owned_routing_semantic_graph_identity_digest;
use super::mismatch_locus::PlannerMismatchLocus;
use super::selected_route_identity::PlannerSelectedRouteIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum PlannerWitnessRole {
    DenialOrAdvisory,
    QuerySupportPosture,
}

impl PlannerWitnessRole {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DenialOrAdvisory => "denial-or-advisory",
            Self::QuerySupportPosture => "query-support-posture",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PlannerWitnessIdentity {
    selected_route_identity_digest: String,
    role: PlannerWitnessRole,
    mismatch_locus: PlannerMismatchLocus,
    witness_reason: String,
    identity_digest: String,
}

impl PlannerWitnessIdentity {
    pub fn selected_route_identity_digest(&self) -> &str {
        &self.selected_route_identity_digest
    }

    pub const fn role(&self) -> PlannerWitnessRole {
        self.role
    }

    pub const fn mismatch_locus(&self) -> PlannerMismatchLocus {
        self.mismatch_locus
    }

    pub fn witness_reason(&self) -> &str {
        &self.witness_reason
    }

    pub fn identity_digest(&self) -> &str {
        &self.identity_digest
    }

    pub const fn artifact_kind(&self) -> PlannerExplanationArtifactKind {
        PlannerExplanationArtifactKind::AuthoritativePlannerOutput
    }
}

pub fn admit_planner_witness_identity(
    selected_route_identity: &PlannerSelectedRouteIdentity,
    role: PlannerWitnessRole,
    mismatch_locus: PlannerMismatchLocus,
    witness_reason: impl Into<String>,
) -> Result<PlannerWitnessIdentity, PlannerOwnedRoutingSemanticGraphVocabularyError> {
    let witness_reason = witness_reason.into();
    if witness_reason.trim().is_empty() {
        return Err(PlannerOwnedRoutingSemanticGraphVocabularyError::new(
            PlannerOwnedRoutingSemanticGraphVocabularyErrorKind::EmptyWitnessReason,
            "planner witness identity requires a non-empty witness reason",
        ));
    }
    let identity_digest = planner_owned_routing_semantic_graph_identity_digest(
        "worth-schema:planner-witness-identity:v1",
        &[
            format!("route:{}", selected_route_identity.identity_digest()),
            format!("role:{}", role.as_str()),
            format!("locus:{}", mismatch_locus.as_str()),
            format!("reason:{witness_reason}"),
        ],
    );
    Ok(PlannerWitnessIdentity {
        selected_route_identity_digest: selected_route_identity.identity_digest().to_string(),
        role,
        mismatch_locus,
        witness_reason,
        identity_digest,
    })
}
