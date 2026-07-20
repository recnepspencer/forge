use super::WorthServerOperationExecutionStrategy;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerOperationPlanReceipt {
    support_composition_digest: String,
    footprint_digest: String,
    strategy: WorthServerOperationExecutionStrategy,
    authorization_proof_digest: String,
    precondition_posture_digest: String,
    expected_scheduler_lane: String,
    plan_identity: String,
    evidence_identity: String,
    canonical_digest: String,
}

pub(crate) struct WorthServerOperationPlanReceiptParts {
    pub(crate) support_composition_digest: String,
    pub(crate) footprint_digest: String,
    pub(crate) strategy: WorthServerOperationExecutionStrategy,
    pub(crate) authorization_proof_digest: String,
    pub(crate) precondition_posture_digest: String,
    pub(crate) expected_scheduler_lane: String,
    pub(crate) plan_identity: String,
    pub(crate) evidence_identity: String,
}

impl WorthServerOperationPlanReceipt {
    pub(crate) fn new(parts: WorthServerOperationPlanReceiptParts) -> Self {
        let WorthServerOperationPlanReceiptParts {
            support_composition_digest,
            footprint_digest,
            strategy,
            authorization_proof_digest,
            precondition_posture_digest,
            expected_scheduler_lane,
            plan_identity,
            evidence_identity,
        } = parts;
        let canonical_digest = format!(
            "worth-server-operation-plan-receipt-v1|support={support_composition_digest}|footprint={footprint_digest}|strategy={}|authorization={authorization_proof_digest}|precondition={precondition_posture_digest}|lane={expected_scheduler_lane}|plan={plan_identity}|evidence={evidence_identity}",
            strategy.as_str(),
        );
        Self {
            support_composition_digest,
            footprint_digest,
            strategy,
            authorization_proof_digest,
            precondition_posture_digest,
            expected_scheduler_lane,
            plan_identity,
            evidence_identity,
            canonical_digest,
        }
    }

    pub fn support_composition_digest(&self) -> &str {
        &self.support_composition_digest
    }

    pub fn footprint_digest(&self) -> &str {
        &self.footprint_digest
    }

    pub fn strategy(&self) -> WorthServerOperationExecutionStrategy {
        self.strategy
    }

    pub fn authorization_proof_digest(&self) -> &str {
        &self.authorization_proof_digest
    }

    pub fn precondition_posture_digest(&self) -> &str {
        &self.precondition_posture_digest
    }

    pub fn expected_scheduler_lane(&self) -> &str {
        &self.expected_scheduler_lane
    }

    pub fn plan_identity(&self) -> &str {
        &self.plan_identity
    }

    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
