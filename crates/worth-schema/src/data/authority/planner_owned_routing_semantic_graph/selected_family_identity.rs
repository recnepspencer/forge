use serde::{Deserialize, Serialize};

use super::admitted_explanation_input::PlannerAdmittedExplanationInput;
use super::artifact_kind::PlannerExplanationArtifactKind;
use super::error::{
    PlannerOwnedRoutingSemanticGraphVocabularyError,
    PlannerOwnedRoutingSemanticGraphVocabularyErrorKind,
};
use super::identity_digest::planner_owned_routing_semantic_graph_identity_digest;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PlannerSelectedFamilyIdentity {
    input_identity_digest: String,
    selected_family_name: String,
    identity_digest: String,
}

impl PlannerSelectedFamilyIdentity {
    pub fn input_identity_digest(&self) -> &str {
        &self.input_identity_digest
    }

    pub fn selected_family_name(&self) -> &str {
        &self.selected_family_name
    }

    pub fn identity_digest(&self) -> &str {
        &self.identity_digest
    }

    pub const fn artifact_kind(&self) -> PlannerExplanationArtifactKind {
        PlannerExplanationArtifactKind::AuthoritativePlannerOutput
    }
}

pub fn admit_planner_selected_family_identity(
    input: &PlannerAdmittedExplanationInput,
    selected_family_name: impl Into<String>,
) -> Result<PlannerSelectedFamilyIdentity, PlannerOwnedRoutingSemanticGraphVocabularyError> {
    let selected_family_name = selected_family_name.into();
    if selected_family_name.trim().is_empty() {
        return Err(PlannerOwnedRoutingSemanticGraphVocabularyError::new(
            PlannerOwnedRoutingSemanticGraphVocabularyErrorKind::EmptySelectedFamilyName,
            "planner selected family identity requires a non-empty family name",
        ));
    }
    let identity_digest = planner_owned_routing_semantic_graph_identity_digest(
        "worth-schema:planner-selected-family-identity:v1",
        &[
            format!("input:{}", input.identity_digest()),
            format!("family:{selected_family_name}"),
        ],
    );
    Ok(PlannerSelectedFamilyIdentity {
        input_identity_digest: input.identity_digest().to_string(),
        selected_family_name,
        identity_digest,
    })
}
