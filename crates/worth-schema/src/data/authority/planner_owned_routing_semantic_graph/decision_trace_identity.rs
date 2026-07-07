use serde::{Deserialize, Serialize};

use super::artifact_kind::PlannerExplanationArtifactKind;
#[cfg(test)]
use super::error::{
    PlannerOwnedRoutingSemanticGraphVocabularyError,
    PlannerOwnedRoutingSemanticGraphVocabularyErrorKind,
};
#[cfg(test)]
use super::identity_digest::planner_owned_routing_semantic_graph_identity_digest;
#[cfg(test)]
use super::selected_route_identity::PlannerSelectedRouteIdentity;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PlannerDecisionTraceIdentity {
    selected_route_identity_digest: String,
    trace_name: String,
    identity_digest: String,
}

impl PlannerDecisionTraceIdentity {
    pub fn selected_route_identity_digest(&self) -> &str {
        &self.selected_route_identity_digest
    }

    pub fn trace_name(&self) -> &str {
        &self.trace_name
    }

    pub fn identity_digest(&self) -> &str {
        &self.identity_digest
    }

    pub const fn artifact_kind(&self) -> PlannerExplanationArtifactKind {
        PlannerExplanationArtifactKind::AuthoritativePlannerOutput
    }
}

#[cfg(test)]
pub(crate) fn admit_planner_decision_trace_identity(
    selected_route_identity: &PlannerSelectedRouteIdentity,
    trace_name: impl Into<String>,
) -> Result<PlannerDecisionTraceIdentity, PlannerOwnedRoutingSemanticGraphVocabularyError> {
    let trace_name = trace_name.into();
    if trace_name.trim().is_empty() {
        return Err(PlannerOwnedRoutingSemanticGraphVocabularyError::new(
            PlannerOwnedRoutingSemanticGraphVocabularyErrorKind::EmptyDecisionTraceName,
            "planner decision-trace identity requires a non-empty trace name",
        ));
    }
    let identity_digest = planner_owned_routing_semantic_graph_identity_digest(
        "worth-schema:planner-decision-trace-identity:v1",
        &[
            format!("route:{}", selected_route_identity.identity_digest()),
            format!("trace:{trace_name}"),
        ],
    );
    Ok(PlannerDecisionTraceIdentity {
        selected_route_identity_digest: selected_route_identity.identity_digest().to_string(),
        trace_name,
        identity_digest,
    })
}
