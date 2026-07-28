use worth_store_buffer_pool::{
    PhysicalResidencyDenial, PhysicalWritebackClaim, PhysicalWritebackRangePosture,
};
use worth_store_io_scheduler::{
    execute_ready_queue_plan, QueueDurabilityClass, QueueExecutionOutcome, QueueExecutionReadyPlan,
};
use worth_store_physical_backend::{
    ArtifactRangeWriteDurabilityRequirement, ArtifactTreeFailure, BackendQueueExecutionAdaptation,
    BackendQueueSpeculativeScope, CompletedArtifactRangeWrite,
    CompletedScheduledArtifactRangeWrite, IndeterminateArtifactRangeWrite,
    ScheduledArtifactRangeWriteOutcome,
};

use super::artifact_tree::PhysicalRecordArtifactTree;

#[cfg(test)]
#[path = "scheduled_writeback/tests.rs"]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalScheduledWritebackAdmissionDenial {
    ServingRequiresInspection,
    MissingPhysicalDeclaration,
    GroupedPlan,
    ClaimCardinalityMismatch,
    StoreMismatch,
    PoolIncarnationMismatch,
    FrameMismatch,
    RangePostureMismatch,
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
    range_posture: PhysicalWritebackRangePosture,
}

#[derive(Debug)]
pub(in crate::physical_runtime) enum PhysicalScheduledWritebackOutcome {
    RetryableBeforeEffect(ArtifactTreeFailure),
    InspectionRequired(IndeterminateArtifactRangeWrite),
    WrittenButNotApplied {
        physical: CompletedArtifactRangeWrite,
        execution: QueueExecutionOutcome,
    },
    Completed {
        physical: CompletedArtifactRangeWrite,
        execution: QueueExecutionOutcome,
        claim: PhysicalWritebackClaim,
    },
}

pub(in crate::physical_runtime) struct PhysicalScheduledWritebackEffect {
    plan: QueueExecutionReadyPlan,
    claim: PhysicalWritebackClaim,
    completed: Box<CompletedScheduledArtifactRangeWrite>,
}

pub(in crate::physical_runtime) type PhysicalScheduledWritebackEffectResult =
    Result<PhysicalScheduledWritebackEffect, Box<PhysicalScheduledWritebackOutcome>>;

const _: () = assert!(
    std::mem::size_of::<PhysicalScheduledWritebackEffectResult>()
        <= std::mem::size_of::<PhysicalScheduledWritebackEffect>() + std::mem::size_of::<usize>()
);

impl PhysicalScheduledWriteback {
    pub(in crate::physical_runtime) fn admit(
        claim: PhysicalWritebackClaim,
        plan: QueueExecutionReadyPlan,
    ) -> Result<Self, PhysicalScheduledWritebackAdmissionDenial> {
        Self::validate(&claim, &plan)?;
        let durability = durability(&plan)?;
        let range_posture = validated_range_posture(&claim, &plan)?;
        Ok(Self {
            plan,
            claim,
            durability,
            range_posture,
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
            .buffer_pool_writeback_declaration()
            .ok_or(PhysicalScheduledWritebackAdmissionDenial::MissingPhysicalDeclaration)?;
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
        let _ = validated_range_posture(claim, plan)?;
        let security = plan.work().security_scope_identity();
        let grouping = declaration.grouping_scope();
        if grouping.security_scope_identity() != security {
            return Err(PhysicalScheduledWritebackAdmissionDenial::SecurityScopeMismatch);
        }
        let _ = durability(plan)?;
        Ok(())
    }

    pub(in crate::physical_runtime) fn execute_effect(
        self,
        artifacts: &PhysicalRecordArtifactTree<'_>,
        adaptation: BackendQueueExecutionAdaptation,
    ) -> PhysicalScheduledWritebackEffectResult {
        let grouping = self.plan.grouping_basis();
        let scope = BackendQueueSpeculativeScope::admitted(
            grouping.security_scope_identity(),
            grouping.tenant_scope(),
            grouping.key_scope(),
        );
        let coordinate = self.claim.frames()[0].coordinate();
        let bytes = self.claim.frame_bytes(0).expect("one admitted frame");
        let binding = self
            .plan
            .backend_completion_binding()
            .backend_execution_binding();
        let physical = match self.range_posture {
            PhysicalWritebackRangePosture::ExistingRange => artifacts.write_scheduled_exact_at(
                coordinate,
                bytes,
                binding,
                adaptation,
                scope,
                self.durability,
            ),
            PhysicalWritebackRangePosture::CandidateArtifactTail => artifacts
                .append_scheduled_writeback_at_eof(
                    coordinate,
                    bytes,
                    binding,
                    adaptation,
                    scope,
                    self.durability,
                ),
        };
        let completed = match physical {
            ScheduledArtifactRangeWriteOutcome::DeniedBeforeEffect(failure) => {
                return Err(Box::new(
                    PhysicalScheduledWritebackOutcome::RetryableBeforeEffect(failure),
                ));
            }
            ScheduledArtifactRangeWriteOutcome::Indeterminate(failure) => {
                return Err(Box::new(
                    PhysicalScheduledWritebackOutcome::InspectionRequired(failure),
                ));
            }
            ScheduledArtifactRangeWriteOutcome::Completed(completed) => completed,
        };
        Ok(PhysicalScheduledWritebackEffect {
            plan: self.plan,
            claim: self.claim,
            completed,
        })
    }
}

impl PhysicalScheduledWritebackEffect {
    pub(in crate::physical_runtime) fn settle(self) -> PhysicalScheduledWritebackOutcome {
        let Self {
            plan,
            claim,
            completed,
        } = self;
        let physical = completed.physical().clone();
        let execution = execute_ready_queue_plan(plan, completed.queue());
        if !matches!(execution, QueueExecutionOutcome::Executed(_)) {
            return PhysicalScheduledWritebackOutcome::WrittenButNotApplied {
                physical,
                execution,
            };
        }
        PhysicalScheduledWritebackOutcome::Completed {
            physical,
            execution,
            claim,
        }
    }
}

fn validated_range_posture(
    claim: &PhysicalWritebackClaim,
    plan: &QueueExecutionReadyPlan,
) -> Result<PhysicalWritebackRangePosture, PhysicalScheduledWritebackAdmissionDenial> {
    let declaration = plan
        .work()
        .buffer_pool_writeback_declaration()
        .ok_or(PhysicalScheduledWritebackAdmissionDenial::MissingPhysicalDeclaration)?;
    let declared = declaration.range_posture();
    require_matching_range_posture(claim.range_posture(0), declared)
}

fn require_matching_range_posture(
    claimed: Option<PhysicalWritebackRangePosture>,
    declared: PhysicalWritebackRangePosture,
) -> Result<PhysicalWritebackRangePosture, PhysicalScheduledWritebackAdmissionDenial> {
    match claimed {
        Some(claimed) if claimed == declared => Ok(declared),
        _ => Err(PhysicalScheduledWritebackAdmissionDenial::RangePostureMismatch),
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
