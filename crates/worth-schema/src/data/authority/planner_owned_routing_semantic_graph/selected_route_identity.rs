use serde::{Deserialize, Serialize};

use super::artifact_kind::PlannerExplanationArtifactKind;
use super::error::{
    PlannerOwnedRoutingSemanticGraphVocabularyError,
    PlannerOwnedRoutingSemanticGraphVocabularyErrorKind,
};
use super::identity_digest::planner_owned_routing_semantic_graph_identity_digest;
use super::selected_family_identity::PlannerSelectedFamilyIdentity;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PlannerSelectedRouteIdentity {
    selected_family_identity_digest: String,
    selected_route_name: String,
    identity_digest: String,
}

impl PlannerSelectedRouteIdentity {
    pub fn selected_family_identity_digest(&self) -> &str {
        &self.selected_family_identity_digest
    }

    pub fn selected_route_name(&self) -> &str {
        &self.selected_route_name
    }

    pub fn identity_digest(&self) -> &str {
        &self.identity_digest
    }

    pub const fn artifact_kind(&self) -> PlannerExplanationArtifactKind {
        PlannerExplanationArtifactKind::AuthoritativePlannerOutput
    }
}

pub fn admit_planner_selected_route_identity(
    selected_family_identity: &PlannerSelectedFamilyIdentity,
    selected_route_name: impl Into<String>,
) -> Result<PlannerSelectedRouteIdentity, PlannerOwnedRoutingSemanticGraphVocabularyError> {
    let selected_route_name = selected_route_name.into();
    if selected_route_name.trim().is_empty() {
        return Err(PlannerOwnedRoutingSemanticGraphVocabularyError::new(
            PlannerOwnedRoutingSemanticGraphVocabularyErrorKind::EmptySelectedRouteName,
            "planner selected route identity requires a non-empty route name",
        ));
    }
    let identity_digest = planner_owned_routing_semantic_graph_identity_digest(
        "worth-schema:planner-selected-route-identity:v1",
        &[
            format!("family:{}", selected_family_identity.identity_digest()),
            format!("route:{selected_route_name}"),
        ],
    );
    Ok(PlannerSelectedRouteIdentity {
        selected_family_identity_digest: selected_family_identity.identity_digest().to_string(),
        selected_route_name,
        identity_digest,
    })
}
