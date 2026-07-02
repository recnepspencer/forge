use worth_primitives::{truth_digest_parts, TruthDigestScope};

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum BatchAdmissionPlannerRouteWitnessKind {
    BatchAdmissionDenial,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BatchAdmissionPlannerRouteWitness {
    kind: BatchAdmissionPlannerRouteWitnessKind,
    identity_digest: String,
}

impl BatchAdmissionPlannerRouteWitness {
    pub fn new(selected_batch_plan_digest: &str, batch_execution_receipt_digest: &str) -> Self {
        let identity_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                format!(
                    "kind:{}",
                    BatchAdmissionPlannerRouteWitnessKind::BatchAdmissionDenial.as_str()
                ),
                format!("selected-batch-plan:{selected_batch_plan_digest}"),
                format!("batch-execution-receipt:{batch_execution_receipt_digest}"),
                "worth-schema:batch-admission-route-witness:v1".to_string(),
            ],
        );
        Self {
            kind: BatchAdmissionPlannerRouteWitnessKind::BatchAdmissionDenial,
            identity_digest,
        }
    }

    pub const fn kind(&self) -> BatchAdmissionPlannerRouteWitnessKind {
        self.kind
    }

    pub fn identity_digest(&self) -> &str {
        &self.identity_digest
    }
}

impl BatchAdmissionPlannerRouteWitnessKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BatchAdmissionDenial => "batch-admission-denial",
        }
    }
}
