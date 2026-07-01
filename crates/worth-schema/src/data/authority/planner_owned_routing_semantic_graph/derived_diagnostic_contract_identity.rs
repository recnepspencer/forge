use serde::{Deserialize, Serialize};

use super::admitted_explanation_input::PlannerAdmittedExplanationInput;
use super::artifact_kind::PlannerExplanationArtifactKind;
use super::error::{
    PlannerOwnedRoutingSemanticGraphVocabularyError,
    PlannerOwnedRoutingSemanticGraphVocabularyErrorKind,
};
use super::identity_digest::planner_owned_routing_semantic_graph_identity_digest;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PlannerDerivedDiagnosticContractIdentity {
    admitted_explanation_input_identity_digest: String,
    diagnostic_contract_name: String,
    identity_digest: String,
}

impl PlannerDerivedDiagnosticContractIdentity {
    pub fn admitted_explanation_input_identity_digest(&self) -> &str {
        &self.admitted_explanation_input_identity_digest
    }

    pub fn diagnostic_contract_name(&self) -> &str {
        &self.diagnostic_contract_name
    }

    pub fn identity_digest(&self) -> &str {
        &self.identity_digest
    }

    pub const fn artifact_kind(&self) -> PlannerExplanationArtifactKind {
        PlannerExplanationArtifactKind::DerivedDiagnosticProjection
    }
}

pub fn admit_planner_derived_diagnostic_contract_identity(
    admitted_explanation_input: &PlannerAdmittedExplanationInput,
    diagnostic_contract_name: impl Into<String>,
) -> Result<PlannerDerivedDiagnosticContractIdentity, PlannerOwnedRoutingSemanticGraphVocabularyError>
{
    let diagnostic_contract_name = diagnostic_contract_name.into();
    if diagnostic_contract_name.trim().is_empty() {
        return Err(PlannerOwnedRoutingSemanticGraphVocabularyError::new(
            PlannerOwnedRoutingSemanticGraphVocabularyErrorKind::EmptyDiagnosticContractName,
            "planner diagnostic contract identity requires a non-empty contract name",
        ));
    }
    let identity_digest = planner_owned_routing_semantic_graph_identity_digest(
        "worth-schema:planner-derived-diagnostic-contract-identity:v1",
        &[
            format!("input:{}", admitted_explanation_input.identity_digest()),
            format!("contract:{diagnostic_contract_name}"),
        ],
    );
    Ok(PlannerDerivedDiagnosticContractIdentity {
        admitted_explanation_input_identity_digest: admitted_explanation_input
            .identity_digest()
            .to_string(),
        diagnostic_contract_name,
        identity_digest,
    })
}
