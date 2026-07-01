use serde::{Deserialize, Serialize};

use super::artifact_kind::PlannerExplanationArtifactKind;
use super::error::{
    PlannerOwnedRoutingSemanticGraphVocabularyError,
    PlannerOwnedRoutingSemanticGraphVocabularyErrorKind,
};
use super::identity_digest::planner_owned_routing_semantic_graph_identity_digest;
use super::selected_route_identity::PlannerSelectedRouteIdentity;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PlannerSelectedProductIdentity {
    selected_route_identity_digest: String,
    selected_product_name: String,
    identity_digest: String,
}

impl PlannerSelectedProductIdentity {
    pub fn selected_route_identity_digest(&self) -> &str {
        &self.selected_route_identity_digest
    }

    pub fn selected_product_name(&self) -> &str {
        &self.selected_product_name
    }

    pub fn identity_digest(&self) -> &str {
        &self.identity_digest
    }

    pub const fn artifact_kind(&self) -> PlannerExplanationArtifactKind {
        PlannerExplanationArtifactKind::AuthoritativePlannerOutput
    }
}

pub fn admit_planner_selected_product_identity(
    selected_route_identity: &PlannerSelectedRouteIdentity,
    selected_product_name: impl Into<String>,
) -> Result<PlannerSelectedProductIdentity, PlannerOwnedRoutingSemanticGraphVocabularyError> {
    let selected_product_name = selected_product_name.into();
    if selected_product_name.trim().is_empty() {
        return Err(PlannerOwnedRoutingSemanticGraphVocabularyError::new(
            PlannerOwnedRoutingSemanticGraphVocabularyErrorKind::EmptySelectedProductName,
            "planner selected product identity requires a non-empty product name",
        ));
    }
    let identity_digest = planner_owned_routing_semantic_graph_identity_digest(
        "worth-schema:planner-selected-product-identity:v1",
        &[
            format!("route:{}", selected_route_identity.identity_digest()),
            format!("product:{selected_product_name}"),
        ],
    );
    Ok(PlannerSelectedProductIdentity {
        selected_route_identity_digest: selected_route_identity.identity_digest().to_string(),
        selected_product_name,
        identity_digest,
    })
}
