use worth_store_buffer_pool::{
    BufferPoolQueueExecutionKind, PhysicalResidencyDenial, PhysicalWritebackClaim,
};
use worth_store_io_scheduler::{
    execute_ready_queue_plan, QueueDurabilityClass, QueueExecutionOutcome, QueueExecutionReadyPlan,
};
use worth_store_physical_backend::{
    ArtifactRangeWriteDurabilityRequirement, ArtifactTreeFailure, BackendQueueExecutionAdaptation,
    BackendQueueSpeculativeScope, CompletedArtifactRangeWrite, IndeterminateArtifactRangeWrite,
    ScheduledArtifactRangeWriteOutcome,
};

use super::artifact_tree::PhysicalRecordArtifactTree;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalScheduledWritebackAdmissionDenial {
    ServingRequiresInspection,
    MissingPhysicalDeclaration,
    NotWriteback,
    GroupedPlan,
    ClaimCardinalityMismatch,
    StoreMismatch,
    PoolIncarnationMismatch,
    FrameMismatch,
    SecurityScopeMismatch,
    ReadOnlyPlan,
    CanonicalWorkMismatch,
    Residency(PhysicalResidencyDenial),
    Retry(crate::physical_runtime::PhysicalExecutorCommandDenial),
}

#[derive(Debug)]
pub(in crate::physical_runtime) struct PhysicalScheduledWriteback {
    plan: QueueExecutionReadyPlan,
    claim: PhysicalWritebackClaim,
    durability: ArtifactRangeWriteDurabilityRequirement,
}

#[derive(Debug)]
pub(in crate::physical_runtime) enum PhysicalScheduledWritebackOutcome {
    RetryableBeforeEffect(ArtifactTreeFailure),
    InspectionRequired(IndeterminateArtifactRangeWrite),
    WrittenButNotApplied {
        physical: CompletedArtifactRangeWrite,
        execution: QueueExecutionOutcome,
    },
    Applied {
        physical: CompletedArtifactRangeWrite,
        execution: QueueExecutionOutcome,
    },
    ResidencyTerminal {
        physical: CompletedArtifactRangeWrite,
        execution: QueueExecutionOutcome,
        denial: PhysicalResidencyDenial,
    },
}

impl PhysicalScheduledWriteback {
    pub(in crate::physical_runtime) fn admit(
        claim: PhysicalWritebackClaim,
        plan: QueueExecutionReadyPlan,
    ) -> Result<Self, PhysicalScheduledWritebackAdmissionDenial> {
        Self::validate(&claim, &plan)?;
        let durability = durability(&plan)?;
        Ok(Self {
            plan,
            claim,
            durability,
        })
    }

    pub(in crate::physical_runtime) fn validate(
        claim: &PhysicalWritebackClaim,
        plan: &QueueExecutionReadyPlan,
    ) -> Result<(), PhysicalScheduledWritebackAdmissionDenial> {
        if plan
            .backend_completion_binding()
            .grouped_replay_identity()
            .is_some()
        {
            return Err(PhysicalScheduledWritebackAdmissionDenial::GroupedPlan);
        }
        let declaration = plan
            .work()
            .buffer_pool_declaration()
            .ok_or(PhysicalScheduledWritebackAdmissionDenial::MissingPhysicalDeclaration)?;
        if declaration.kind() != BufferPoolQueueExecutionKind::WriteBack {
            return Err(PhysicalScheduledWritebackAdmissionDenial::NotWriteback);
        }
        let [claimed] = claim.frames() else {
            return Err(PhysicalScheduledWritebackAdmissionDenial::ClaimCardinalityMismatch);
        };
        if declaration.store() != claim.store_identity() {
            return Err(PhysicalScheduledWritebackAdmissionDenial::StoreMismatch);
        }
        if declaration.pool() != claim.pool_incarnation() {
            return Err(PhysicalScheduledWritebackAdmissionDenial::PoolIncarnationMismatch);
        }
        if declaration.frame() != claimed.coordinate() {
            return Err(PhysicalScheduledWritebackAdmissionDenial::FrameMismatch);
        }
        let security = plan.work().security_scope_identity();
        let grouping = declaration.grouping_scope();
        if grouping.security_scope_identity() != security {
            return Err(PhysicalScheduledWritebackAdmissionDenial::SecurityScopeMismatch);
        }
        let _ = durability(plan)?;
        Ok(())
    }

    pub(in crate::physical_runtime) fn execute(
        self,
        artifacts: &PhysicalRecordArtifactTree<'_>,
        adaptation: BackendQueueExecutionAdaptation,
    ) -> PhysicalScheduledWritebackOutcome {
        let grouping = self.plan.grouping_basis();
        let scope = BackendQueueSpeculativeScope::admitted(
            grouping.security_scope_identity(),
            grouping.tenant_scope(),
            grouping.key_scope(),
        );
        let coordinate = self.claim.frames()[0].coordinate();
        let physical = artifacts.write_scheduled_exact_at(
            coordinate,
            self.claim.frame_bytes(0).expect("one admitted frame"),
            self.plan
                .backend_completion_binding()
                .backend_execution_binding(),
            adaptation,
            scope,
            self.durability,
        );
        let completed = match physical {
            ScheduledArtifactRangeWriteOutcome::DeniedBeforeEffect(failure) => {
                return PhysicalScheduledWritebackOutcome::RetryableBeforeEffect(failure);
            }
            ScheduledArtifactRangeWriteOutcome::Indeterminate(failure) => {
                return PhysicalScheduledWritebackOutcome::InspectionRequired(failure);
            }
            ScheduledArtifactRangeWriteOutcome::Completed(completed) => completed,
        };
        let physical = completed.physical().clone();
        let execution = execute_ready_queue_plan(self.plan, completed.queue());
        if !matches!(execution, QueueExecutionOutcome::Executed(_)) {
            return PhysicalScheduledWritebackOutcome::WrittenButNotApplied {
                physical,
                execution,
            };
        }
        match self.claim.publish_clean(&physical) {
            Ok(()) => PhysicalScheduledWritebackOutcome::Applied {
                physical,
                execution,
            },
            Err(denial) => PhysicalScheduledWritebackOutcome::ResidencyTerminal {
                physical,
                execution,
                denial,
            },
        }
    }
}

fn durability(
    plan: &QueueExecutionReadyPlan,
) -> Result<ArtifactRangeWriteDurabilityRequirement, PhysicalScheduledWritebackAdmissionDenial> {
    match plan.work().durability_class() {
        QueueDurabilityClass::ReadOnly => {
            Err(PhysicalScheduledWritebackAdmissionDenial::ReadOnlyPlan)
        }
        QueueDurabilityClass::BufferedWrite => {
            Ok(ArtifactRangeWriteDurabilityRequirement::BufferedWrite)
        }
        QueueDurabilityClass::WalCommit | QueueDurabilityClass::PlatformDurable => {
            Ok(ArtifactRangeWriteDurabilityRequirement::FileDataSynchronization)
        }
    }
}
