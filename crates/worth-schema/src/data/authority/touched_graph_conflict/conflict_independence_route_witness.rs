use worth_primitives::{truth_digest_parts, TruthDigestScope};

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ConflictIndependencePlannerRouteWitnessKind {
    ConflictRouteDenial,
    IndependenceDenial,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConflictIndependencePlannerRouteWitness {
    kind: ConflictIndependencePlannerRouteWitnessKind,
    identity_digest: String,
}

impl ConflictIndependencePlannerRouteWitness {
    pub fn new(
        kind: ConflictIndependencePlannerRouteWitnessKind,
        selected_batch_plan_digest: &str,
        batch_execution_receipt_digest: &str,
    ) -> Self {
        let identity_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                format!("kind:{}", kind.as_str()),
                format!("selected-batch-plan:{selected_batch_plan_digest}"),
                format!("batch-execution-receipt:{batch_execution_receipt_digest}"),
                "worth-schema:conflict-independence-route-witness:v1".to_string(),
            ],
        );
        Self {
            kind,
            identity_digest,
        }
    }

    pub const fn kind(&self) -> ConflictIndependencePlannerRouteWitnessKind {
        self.kind
    }

    pub fn identity_digest(&self) -> &str {
        &self.identity_digest
    }
}

impl ConflictIndependencePlannerRouteWitnessKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConflictRouteDenial => "conflict-route-denial",
            Self::IndependenceDenial => "independence-denial",
        }
    }
}
