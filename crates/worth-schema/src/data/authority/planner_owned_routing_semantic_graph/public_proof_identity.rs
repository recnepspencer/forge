use serde::{Deserialize, Serialize};

use super::artifact_kind::PlannerExplanationArtifactKind;
use super::error::{
    PlannerOwnedRoutingSemanticGraphVocabularyError,
    PlannerOwnedRoutingSemanticGraphVocabularyErrorKind,
};
use super::identity_digest::planner_owned_routing_semantic_graph_identity_digest;
use super::selected_product_identity::PlannerSelectedProductIdentity;
use super::selected_route_identity::PlannerSelectedRouteIdentity;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PlannerPublicProofIdentity {
    selected_route_identity_digest: String,
    selected_product_identity_digest: String,
    public_proof_name: String,
    identity_digest: String,
}

impl PlannerPublicProofIdentity {
    pub fn selected_route_identity_digest(&self) -> &str {
        &self.selected_route_identity_digest
    }

    pub fn selected_product_identity_digest(&self) -> &str {
        &self.selected_product_identity_digest
    }

    pub fn public_proof_name(&self) -> &str {
        &self.public_proof_name
    }

    pub fn identity_digest(&self) -> &str {
        &self.identity_digest
    }

    pub const fn artifact_kind(&self) -> PlannerExplanationArtifactKind {
        PlannerExplanationArtifactKind::DerivedPublicProjection
    }
}

pub fn admit_planner_public_proof_identity(
    selected_route_identity: &PlannerSelectedRouteIdentity,
    selected_product_identity: &PlannerSelectedProductIdentity,
    public_proof_name: impl Into<String>,
) -> Result<PlannerPublicProofIdentity, PlannerOwnedRoutingSemanticGraphVocabularyError> {
    let public_proof_name = public_proof_name.into();
    if public_proof_name.trim().is_empty() {
        return Err(PlannerOwnedRoutingSemanticGraphVocabularyError::new(
            PlannerOwnedRoutingSemanticGraphVocabularyErrorKind::EmptyPublicProofName,
            "planner public-proof identity requires a non-empty proof name",
        ));
    }
    let identity_digest = planner_owned_routing_semantic_graph_identity_digest(
        "worth-schema:planner-public-proof-identity:v1",
        &[
            format!("route:{}", selected_route_identity.identity_digest()),
            format!("product:{}", selected_product_identity.identity_digest()),
            format!("proof:{public_proof_name}"),
        ],
    );
    Ok(PlannerPublicProofIdentity {
        selected_route_identity_digest: selected_route_identity.identity_digest().to_string(),
        selected_product_identity_digest: selected_product_identity.identity_digest().to_string(),
        public_proof_name,
        identity_digest,
    })
}
