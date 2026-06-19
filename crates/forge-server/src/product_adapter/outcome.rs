use super::{
    ForgeServerLoweredProductOperationPlan, ForgeServerProductOperationDenial,
    ForgeServerProductOperationEnvelope, ForgeServerProductSchedulerAdmission,
    ForgeServerScheduledProductOperation,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerProductOperationSuccess {
    result_key: String,
    result_digest: String,
}

impl ForgeServerProductOperationSuccess {
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
pub struct ForgeServerProductOperationFailure {
    reason_key: String,
    detail: String,
}

impl ForgeServerProductOperationFailure {
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
pub enum ForgeServerProductOperationOutcome {
    Success(ForgeServerProductOperationSuccess),
    Denied(ForgeServerProductOperationDenial),
    Failed(ForgeServerProductOperationFailure),
}

impl ForgeServerProductOperationOutcome {
    pub fn failed(reason_key: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::Failed(ForgeServerProductOperationFailure::new(reason_key, detail))
    }
}

#[derive(Clone, Debug)]
pub struct ForgeServerCompletedProductOperation {
    outcome: ForgeServerProductOperationOutcome,
    envelope: ForgeServerProductOperationEnvelope,
    proof: Option<ForgeServerCompletedProductOperationProof>,
    adapter_execution_attempted: bool,
    replay_receipt: Option<crate::ForgeServerProductOperationReplayReceipt>,
}

impl ForgeServerCompletedProductOperation {
    pub(crate) fn new(
        outcome: ForgeServerProductOperationOutcome,
        envelope: ForgeServerProductOperationEnvelope,
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
        scheduled_operation: &ForgeServerScheduledProductOperation,
    ) -> Self {
        self.proof = Some(ForgeServerCompletedProductOperationProof::new(
            scheduled_operation.plan().clone(),
            scheduled_operation.scheduler_admission().clone(),
        ));
        self.adapter_execution_attempted = true;
        self
    }

    pub(crate) fn with_replay_receipt(
        mut self,
        replay_receipt: crate::ForgeServerProductOperationReplayReceipt,
    ) -> Self {
        self.replay_receipt = Some(replay_receipt);
        self
    }

    pub(crate) fn to_replayed(
        &self,
        replay_receipt: crate::ForgeServerProductOperationReplayReceipt,
    ) -> Self {
        let mut cloned = self.clone();
        cloned.adapter_execution_attempted = false;
        cloned.replay_receipt = Some(replay_receipt);
        cloned
    }

    pub fn outcome(&self) -> &ForgeServerProductOperationOutcome {
        &self.outcome
    }

    pub fn envelope(&self) -> &ForgeServerProductOperationEnvelope {
        &self.envelope
    }

    pub fn plan(&self) -> Option<&ForgeServerLoweredProductOperationPlan> {
        self.proof
            .as_ref()
            .map(ForgeServerCompletedProductOperationProof::plan)
    }

    pub fn scheduler_admission(&self) -> Option<&ForgeServerProductSchedulerAdmission> {
        self.proof
            .as_ref()
            .map(ForgeServerCompletedProductOperationProof::scheduler_admission)
    }

    pub fn support_posture(&self) -> Option<&crate::ForgeServerOperationSupportPosture> {
        self.plan()
            .map(ForgeServerLoweredProductOperationPlan::support_posture)
    }

    pub fn precondition_posture(&self) -> Option<&crate::ForgeServerOperationPreconditionPosture> {
        self.plan()
            .map(ForgeServerLoweredProductOperationPlan::precondition_posture)
    }

    pub fn replay_receipt(&self) -> Option<&crate::ForgeServerProductOperationReplayReceipt> {
        self.replay_receipt.as_ref()
    }

    pub fn replay_diagnostics(&self) -> ForgeServerProductOperationReplayDiagnostics {
        ForgeServerProductOperationReplayDiagnostics::new(
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
pub enum ForgeServerProductOperationReplayClass {
    BestEffort,
    Authoritative,
    Replayed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerProductOperationReplayDiagnostics {
    class: ForgeServerProductOperationReplayClass,
    replay_receipt: Option<crate::ForgeServerProductOperationReplayReceipt>,
    adapter_execution_attempted: bool,
    envelope_digest: String,
}

impl ForgeServerProductOperationReplayDiagnostics {
    fn new(
        replay_receipt: Option<crate::ForgeServerProductOperationReplayReceipt>,
        adapter_execution_attempted: bool,
        envelope_digest: String,
    ) -> Self {
        let class = match replay_receipt.as_ref() {
            None => ForgeServerProductOperationReplayClass::BestEffort,
            Some(receipt) if receipt.is_replayed() => {
                ForgeServerProductOperationReplayClass::Replayed
            }
            Some(_) => ForgeServerProductOperationReplayClass::Authoritative,
        };
        Self {
            class,
            replay_receipt,
            adapter_execution_attempted,
            envelope_digest,
        }
    }

    pub fn class(&self) -> &ForgeServerProductOperationReplayClass {
        &self.class
    }

    pub fn replay_receipt(&self) -> Option<&crate::ForgeServerProductOperationReplayReceipt> {
        self.replay_receipt.as_ref()
    }

    pub fn is_authoritative(&self) -> bool {
        self.class == ForgeServerProductOperationReplayClass::Authoritative
    }

    pub fn is_replayed(&self) -> bool {
        self.class == ForgeServerProductOperationReplayClass::Replayed
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
struct ForgeServerCompletedProductOperationProof {
    plan: ForgeServerLoweredProductOperationPlan,
    scheduler_admission: ForgeServerProductSchedulerAdmission,
}

impl ForgeServerCompletedProductOperationProof {
    fn new(
        plan: ForgeServerLoweredProductOperationPlan,
        scheduler_admission: ForgeServerProductSchedulerAdmission,
    ) -> Self {
        Self {
            plan,
            scheduler_admission,
        }
    }

    fn plan(&self) -> &ForgeServerLoweredProductOperationPlan {
        &self.plan
    }

    fn scheduler_admission(&self) -> &ForgeServerProductSchedulerAdmission {
        &self.scheduler_admission
    }
}
