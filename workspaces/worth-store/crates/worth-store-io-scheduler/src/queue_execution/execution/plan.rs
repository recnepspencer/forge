use worth_foundational::FoundationalPolicyAdmissionReceipt;
use worth_store_physical_backend::{BackendTargetProfile, CapabilityEvidenceClass};

use crate::BackgroundResourceBudget;

use super::progression::{
    execute_queue_execution_proof, ready_queue_execution_proof, QueueExecutedRecipe,
    QueueReadyRecipe,
};
use super::{
    QueueExecutionPlanBinding, QueueExecutionProgression, QueueExecutionReplayIdentity,
    QueueGroupingBasis, QueueWorkDeclaration,
};

#[derive(Debug, Eq, PartialEq)]
pub struct AdmittedQueueExecutionPlan {
    work: QueueWorkDeclaration,
    backend_profile: BackendTargetProfile,
    backend_evidence_class: CapabilityEvidenceClass,
    policy_receipt: FoundationalPolicyAdmissionReceipt,
    grouping_basis: QueueGroupingBasis,
    replay_identity: QueueExecutionReplayIdentity,
    admitted_budget: BackgroundResourceBudget,
    progression: QueueExecutionProgression,
}

#[derive(Debug, Eq, PartialEq)]
pub struct QueueExecutionReadyPlan {
    admitted: AdmittedQueueExecutionPlan,
    progression: QueueExecutionProgression,
    ready_proof: QueueReadyRecipe,
}

#[derive(Debug, Eq, PartialEq)]
pub struct QueueExecutedPlan {
    admitted: AdmittedQueueExecutionPlan,
    progression: QueueExecutionProgression,
    executed_proof: QueueExecutedRecipe,
}

impl AdmittedQueueExecutionPlan {
    pub(crate) fn new(
        work: QueueWorkDeclaration,
        backend_profile: BackendTargetProfile,
        backend_evidence_class: CapabilityEvidenceClass,
        policy_receipt: FoundationalPolicyAdmissionReceipt,
        grouping_basis: QueueGroupingBasis,
        admitted_budget: BackgroundResourceBudget,
    ) -> Self {
        Self {
            replay_identity: QueueExecutionReplayIdentity::new(&work, grouping_basis.clone()),
            work,
            backend_profile,
            backend_evidence_class,
            policy_receipt,
            grouping_basis,
            admitted_budget,
            progression: QueueExecutionProgression::Lowered,
        }
    }

    pub fn into_execution_ready(self) -> QueueExecutionReadyPlan {
        let replay_identity = self.replay_identity.clone();
        QueueExecutionReadyPlan {
            ready_proof: ready_queue_execution_proof(replay_identity),
            admitted: self,
            progression: QueueExecutionProgression::ExecutionReady,
        }
    }

    pub const fn work(&self) -> &QueueWorkDeclaration {
        &self.work
    }

    pub const fn backend_profile(&self) -> BackendTargetProfile {
        self.backend_profile
    }

    pub const fn backend_evidence_class(&self) -> CapabilityEvidenceClass {
        self.backend_evidence_class
    }

    pub fn policy_receipt(&self) -> &FoundationalPolicyAdmissionReceipt {
        &self.policy_receipt
    }

    pub const fn grouping_basis(&self) -> &QueueGroupingBasis {
        &self.grouping_basis
    }

    pub const fn replay_identity(&self) -> &QueueExecutionReplayIdentity {
        &self.replay_identity
    }

    pub const fn admitted_budget(&self) -> BackgroundResourceBudget {
        self.admitted_budget
    }

    pub const fn progression(&self) -> QueueExecutionProgression {
        self.progression
    }
}

impl QueueExecutionReadyPlan {
    pub const fn admitted(&self) -> &AdmittedQueueExecutionPlan {
        &self.admitted
    }

    pub const fn work(&self) -> &QueueWorkDeclaration {
        self.admitted.work()
    }

    pub const fn backend_profile(&self) -> BackendTargetProfile {
        self.admitted.backend_profile()
    }

    pub const fn backend_evidence_class(&self) -> CapabilityEvidenceClass {
        self.admitted.backend_evidence_class()
    }

    pub fn policy_receipt(&self) -> &FoundationalPolicyAdmissionReceipt {
        self.admitted.policy_receipt()
    }

    pub const fn grouping_basis(&self) -> &QueueGroupingBasis {
        self.admitted.grouping_basis()
    }

    pub const fn replay_identity(&self) -> &QueueExecutionReplayIdentity {
        self.admitted.replay_identity()
    }

    pub fn backend_completion_binding(&self) -> QueueExecutionPlanBinding {
        self.admitted
            .replay_identity()
            .clone()
            .backend_completion_binding(self.backend_profile(), self.backend_evidence_class())
    }

    pub const fn admitted_budget(&self) -> BackgroundResourceBudget {
        self.admitted.admitted_budget()
    }

    pub const fn progression(&self) -> QueueExecutionProgression {
        self.progression
    }

    pub(crate) fn execute_proof(self) -> QueueExecutedPlan {
        QueueExecutedPlan {
            admitted: self.admitted,
            progression: QueueExecutionProgression::Executed,
            executed_proof: execute_queue_execution_proof(self.ready_proof),
        }
    }
}

impl QueueExecutedPlan {
    pub const fn admitted(&self) -> &AdmittedQueueExecutionPlan {
        &self.admitted
    }

    pub const fn work(&self) -> &QueueWorkDeclaration {
        self.admitted.work()
    }

    pub const fn backend_profile(&self) -> BackendTargetProfile {
        self.admitted.backend_profile()
    }

    pub const fn backend_evidence_class(&self) -> CapabilityEvidenceClass {
        self.admitted.backend_evidence_class()
    }

    pub fn policy_receipt(&self) -> &FoundationalPolicyAdmissionReceipt {
        self.admitted.policy_receipt()
    }

    pub const fn grouping_basis(&self) -> &QueueGroupingBasis {
        self.admitted.grouping_basis()
    }

    pub const fn replay_identity(&self) -> &QueueExecutionReplayIdentity {
        self.admitted.replay_identity()
    }

    pub const fn admitted_budget(&self) -> BackgroundResourceBudget {
        self.admitted.admitted_budget()
    }

    pub const fn progression(&self) -> QueueExecutionProgression {
        self.progression
    }

    pub fn executed_replay_identity(&self) -> &QueueExecutionReplayIdentity {
        self.executed_proof.payload()
    }
}
