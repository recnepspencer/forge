use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum PlannerExplanationArtifactKind {
    PriorProofInput,
    AuthoritativePlannerOutput,
    DerivedPublicProjection,
    DerivedDiagnosticProjection,
}
