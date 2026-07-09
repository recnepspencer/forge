use super::{
    WorthServerLoweredProductOperationPlan, WorthServerProductOperationDenial,
    WorthServerProductOperationEnvelope, WorthServerProductSchedulerAdmission,
    WorthServerScheduledProductOperation,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerProductOperationSuccess {
    result_key: String,
    result_digest: String,
}

impl WorthServerProductOperationSuccess {
    pub fn new(result_key: impl Into<String>, result_digest: impl Into<String>) -> Self {
        Self {
            result_key: result_key.into(),
            result_digest: result_digest.into(),
        }
    }

    pub fn result_key(&self) -> &str {
        &self.result_key
    }

    pub fn result_digest(&self) -> &str {
        &self.result_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerProductOperationFailure {
    reason_key: String,
    detail: String,
}

impl WorthServerProductOperationFailure {
    pub fn new(reason_key: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            reason_key: reason_key.into(),
            detail: detail.into(),
        }
    }

    pub fn reason_key(&self) -> &str {
        &self.reason_key
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerProductOperationOutcome {
    Success(WorthServerProductOperationSuccess),
    Denied(WorthServerProductOperationDenial),
    Failed(WorthServerProductOperationFailure),
}

impl WorthServerProductOperationOutcome {
    pub fn failed(reason_key: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::Failed(WorthServerProductOperationFailure::new(reason_key, detail))
    }
}

#[derive(Clone, Debug)]
pub struct WorthServerCompletedProductOperation {
    outcome: WorthServerProductOperationOutcome,
    envelope: WorthServerProductOperationEnvelope,
    proof: Option<WorthServerCompletedProductOperationProof>,
    adapter_execution_attempted: bool,
    replay_receipt: Option<crate::WorthServerProductOperationReplayReceipt>,
}

impl WorthServerCompletedProductOperation {
    pub(crate) fn new(
        outcome: WorthServerProductOperationOutcome,
        envelope: WorthServerProductOperationEnvelope,
    ) -> Self {
        Self {
            outcome,
            envelope,
            proof: None,
            adapter_execution_attempted: false,
            replay_receipt: None,
        }
    }

    pub(crate) fn with_scheduled_operation(
        mut self,
        scheduled_operation: &WorthServerScheduledProductOperation,
    ) -> Self {
        self.proof = Some(WorthServerCompletedProductOperationProof::new(
            scheduled_operation.plan().clone(),
            scheduled_operation.scheduler_admission().clone(),
        ));
        self.adapter_execution_attempted = true;
        self
    }

    pub(crate) fn with_replay_receipt(
        mut self,
        replay_receipt: crate::WorthServerProductOperationReplayReceipt,
    ) -> Self {
        self.replay_receipt = Some(replay_receipt);
        self
    }

    pub(crate) fn to_replayed(
        &self,
        replay_receipt: crate::WorthServerProductOperationReplayReceipt,
    ) -> Self {
        let mut cloned = self.clone();
        cloned.adapter_execution_attempted = false;
        cloned.replay_receipt = Some(replay_receipt);
        cloned
    }

    pub fn outcome(&self) -> &WorthServerProductOperationOutcome {
        &self.outcome
    }

    pub fn envelope(&self) -> &WorthServerProductOperationEnvelope {
        &self.envelope
    }

    pub fn plan(&self) -> Option<&WorthServerLoweredProductOperationPlan> {
        self.proof
            .as_ref()
            .map(WorthServerCompletedProductOperationProof::plan)
    }

    pub fn scheduler_admission(&self) -> Option<&WorthServerProductSchedulerAdmission> {
        self.proof
            .as_ref()
            .map(WorthServerCompletedProductOperationProof::scheduler_admission)
    }

    pub fn support_posture(&self) -> Option<&crate::WorthServerOperationSupportPosture> {
        self.plan()
            .map(WorthServerLoweredProductOperationPlan::support_posture)
    }

    pub fn precondition_posture(&self) -> Option<&crate::WorthServerOperationPreconditionPosture> {
        self.plan()
            .map(WorthServerLoweredProductOperationPlan::precondition_posture)
    }

    pub fn replay_receipt(&self) -> Option<&crate::WorthServerProductOperationReplayReceipt> {
        self.replay_receipt.as_ref()
    }

    pub fn replay_diagnostics(&self) -> WorthServerProductOperationReplayDiagnostics {
        WorthServerProductOperationReplayDiagnostics::new(
            self.replay_receipt.clone(),
            self.adapter_execution_attempted(),
            self.envelope.canonical_digest().to_string(),
        )
    }

    pub fn adapter_execution_attempted(&self) -> bool {
        self.adapter_execution_attempted
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerProductOperationReplayClass {
    BestEffort,
    Authoritative,
    Replayed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerProductOperationReplayDiagnostics {
    class: WorthServerProductOperationReplayClass,
    replay_receipt: Option<crate::WorthServerProductOperationReplayReceipt>,
    adapter_execution_attempted: bool,
    envelope_digest: String,
}

impl WorthServerProductOperationReplayDiagnostics {
    fn new(
        replay_receipt: Option<crate::WorthServerProductOperationReplayReceipt>,
        adapter_execution_attempted: bool,
        envelope_digest: String,
    ) -> Self {
        let class = match replay_receipt.as_ref() {
            None => WorthServerProductOperationReplayClass::BestEffort,
            Some(receipt) if receipt.is_replayed() => {
                WorthServerProductOperationReplayClass::Replayed
            }
            Some(_) => WorthServerProductOperationReplayClass::Authoritative,
        };
        Self {
            class,
            replay_receipt,
            adapter_execution_attempted,
            envelope_digest,
        }
    }

    pub fn class(&self) -> &WorthServerProductOperationReplayClass {
        &self.class
    }

    pub fn replay_receipt(&self) -> Option<&crate::WorthServerProductOperationReplayReceipt> {
        self.replay_receipt.as_ref()
    }

    pub fn is_authoritative(&self) -> bool {
        self.class == WorthServerProductOperationReplayClass::Authoritative
    }

    pub fn is_replayed(&self) -> bool {
        self.class == WorthServerProductOperationReplayClass::Replayed
    }

    pub fn adapter_execution_attempted(&self) -> bool {
        self.adapter_execution_attempted
    }

    pub fn adapter_execution_skipped_by_replay(&self) -> bool {
        self.is_replayed() && !self.adapter_execution_attempted
    }

    pub fn envelope_digest(&self) -> &str {
        &self.envelope_digest
    }
}

#[derive(Clone, Debug)]
struct WorthServerCompletedProductOperationProof {
    plan: WorthServerLoweredProductOperationPlan,
    scheduler_admission: WorthServerProductSchedulerAdmission,
}

impl WorthServerCompletedProductOperationProof {
    fn new(
        plan: WorthServerLoweredProductOperationPlan,
        scheduler_admission: WorthServerProductSchedulerAdmission,
    ) -> Self {
        Self {
            plan,
            scheduler_admission,
        }
    }

    fn plan(&self) -> &WorthServerLoweredProductOperationPlan {
        &self.plan
    }

    fn scheduler_admission(&self) -> &WorthServerProductSchedulerAdmission {
        &self.scheduler_admission
    }
}
