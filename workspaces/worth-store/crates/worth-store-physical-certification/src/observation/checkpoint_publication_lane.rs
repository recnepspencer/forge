use crate::{
    CheckpointInterlockObservation, ObservationDenial, ObservedPhysicalTrace, ObserverKind,
    PhysicalInterleavingSchedule, PhysicalIsolationCheckpointPublicationShortcutDenialLaneOutput,
    PhysicalScenarioActorRole, PhysicalSimulationPlan, RecoveryOutcomeKind,
    RecoveryOutcomeObservation, ShortcutRejectionObservation,
};
use worth_store_physical_isolation::{
    CheckpointInterlockEvidenceOrigin, CheckpointInterlockFoundationalEvidence,
};

const FRESH_RUNTIME_RECOVERY_YIELDPOINT: &str = "fresh-runtime-replay-open";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalIsolationCheckpointPublicationLaneBinding {
    plan_identity: [u8; 32],
    schedule_identity: [u8; 32],
    checkpoint_actor_step_index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalIsolationCheckpointPublicationScheduledLaneOutput {
    plan_identity: [u8; 32],
    schedule_identity: [u8; 32],
    checkpoint_actor_step_index: usize,
    observation: CheckpointInterlockObservation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointCrashReplayObservation {
    checkpoint_origin: CheckpointInterlockEvidenceOrigin,
    recovery_outcome: RecoveryOutcomeObservation,
    checkpoint_actor_step_index: usize,
    recovery_actor_step_index: usize,
    recovery_plan_identity: [u8; 32],
    recovery_schedule_identity: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalIsolationCheckpointPublicationRecoveryOutcomeLaneOutput {
    checkpoint_plan_identity: [u8; 32],
    checkpoint_schedule_identity: [u8; 32],
    checkpoint_origin: CheckpointInterlockEvidenceOrigin,
    recovery_plan_identity: [u8; 32],
    recovery_schedule_identity: [u8; 32],
    recovery_actor_step_index: usize,
    observation: RecoveryOutcomeObservation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalIsolationCheckpointPublicationCrashLaneOutput {
    plan_identity: [u8; 32],
    schedule_identity: [u8; 32],
    checkpoint_actor_step_index: usize,
    recovery_actor_step_index: usize,
    observation: CheckpointCrashReplayObservation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalIsolationCheckpointPublicationShortcutRejectionOutput {
    plan_identity: [u8; 32],
    schedule_identity: [u8; 32],
    checkpoint_actor_step_index: usize,
    shortcut_actor_step_index: usize,
    observation: ShortcutRejectionObservation,
}

impl PhysicalIsolationCheckpointPublicationLaneBinding {
    pub fn from_plan_and_schedule(
        plan: &PhysicalSimulationPlan,
        schedule: &PhysicalInterleavingSchedule,
    ) -> Result<Self, ObservationDenial> {
        let checkpoint_actor_step_index = require_schedule_matches_checkpoint_plan(plan, schedule)?;
        Ok(Self {
            plan_identity: *plan.identity().digest_bytes(),
            schedule_identity: *schedule.identity().digest_bytes(),
            checkpoint_actor_step_index,
        })
    }

    pub const fn checkpoint_actor_step_index(&self) -> usize {
        self.checkpoint_actor_step_index
    }

    pub(crate) const fn plan_identity_digest(&self) -> &[u8; 32] {
        &self.plan_identity
    }

    pub(crate) const fn schedule_identity_digest(&self) -> &[u8; 32] {
        &self.schedule_identity
    }
}

impl CheckpointCrashReplayObservation {
    pub const fn checkpoint_origin(&self) -> &CheckpointInterlockEvidenceOrigin {
        &self.checkpoint_origin
    }

    pub const fn recovery_outcome(&self) -> RecoveryOutcomeObservation {
        self.recovery_outcome
    }

    pub const fn checkpoint_actor_step_index(&self) -> usize {
        self.checkpoint_actor_step_index
    }

    pub const fn recovery_actor_step_index(&self) -> usize {
        self.recovery_actor_step_index
    }

    pub const fn recovery_plan_identity(&self) -> &[u8; 32] {
        &self.recovery_plan_identity
    }

    pub const fn recovery_schedule_identity(&self) -> &[u8; 32] {
        &self.recovery_schedule_identity
    }
}

impl PhysicalIsolationCheckpointPublicationScheduledLaneOutput {
    pub fn from_schedule_step_evidence(
        binding: &PhysicalIsolationCheckpointPublicationLaneBinding,
        schedule: &PhysicalInterleavingSchedule,
        actor_step_index: usize,
        expected_origin: &CheckpointInterlockEvidenceOrigin,
        evidence: CheckpointInterlockFoundationalEvidence,
    ) -> Result<Self, ObservationDenial> {
        require_schedule_step_matches_binding(binding, schedule, actor_step_index)?;
        require_evidence_origin_matches(expected_origin, evidence.origin())?;
        let observation = CheckpointInterlockObservation::from_store_interlock_evidence(evidence)
            .ok_or(ObservationDenial::MissingCheckpointPublicationLane)?;
        Ok(Self {
            plan_identity: binding.plan_identity,
            schedule_identity: binding.schedule_identity,
            checkpoint_actor_step_index: actor_step_index,
            observation,
        })
    }

    pub fn reject_copied_checkpoint_report_attempt(
        binding: &PhysicalIsolationCheckpointPublicationLaneBinding,
        schedule: &PhysicalInterleavingSchedule,
        actor_step_index: usize,
        expected_origin: &CheckpointInterlockEvidenceOrigin,
        copied_evidence: CheckpointInterlockFoundationalEvidence,
    ) -> Result<Self, ObservationDenial> {
        require_schedule_step_matches_binding(binding, schedule, actor_step_index)?;
        if expected_origin != copied_evidence.origin() {
            return Err(ObservationDenial::CheckpointPublicationEvidenceOriginMismatch);
        }
        Err(ObservationDenial::CopiedCheckpointReportObservationDenied)
    }

    pub const fn plan_identity(&self) -> &[u8; 32] {
        &self.plan_identity
    }

    pub const fn schedule_identity(&self) -> &[u8; 32] {
        &self.schedule_identity
    }

    pub const fn checkpoint_actor_step_index(&self) -> usize {
        self.checkpoint_actor_step_index
    }

    pub const fn observation(&self) -> CheckpointInterlockObservation {
        self.observation
    }
}

impl PhysicalIsolationCheckpointPublicationRecoveryOutcomeLaneOutput {
    pub fn from_fresh_runtime_recovery_trace(
        binding: &PhysicalIsolationCheckpointPublicationLaneBinding,
        checkpoint_schedule: &PhysicalInterleavingSchedule,
        recovery_plan: &PhysicalSimulationPlan,
        recovery_schedule: &PhysicalInterleavingSchedule,
        recovery_actor_step_index: usize,
        recovery_trace: &ObservedPhysicalTrace,
        expected_origin: &CheckpointInterlockEvidenceOrigin,
        evidence: CheckpointInterlockFoundationalEvidence,
    ) -> Result<Self, ObservationDenial> {
        if checkpoint_schedule.identity().digest_bytes() != &binding.schedule_identity {
            return Err(ObservationDenial::CheckpointPublicationLaneScheduleMismatch);
        }
        require_evidence_origin_matches(expected_origin, evidence.origin())?;
        if !recovery_schedule.replay_identity_matches_plan(recovery_plan)
            || recovery_trace.plan_identity().digest_bytes()
                != recovery_plan.identity().digest_bytes()
            || recovery_trace.observer() != ObserverKind::RecoveryOutcomeObserver
        {
            return Err(ObservationDenial::CheckpointPublicationCrashRecoveryTraceMismatch);
        }
        require_role_yieldpoint_step_matches_schedule(
            recovery_schedule,
            recovery_actor_step_index,
            PhysicalScenarioActorRole::RecoveryDriver,
            FRESH_RUNTIME_RECOVERY_YIELDPOINT,
            ObservationDenial::CheckpointPublicationCrashLaneScheduleMismatch,
        )?;
        let observation = *recovery_trace
            .recovery_outcome()
            .ok_or(ObservationDenial::MissingRecoveryOutcomeObservation)?;
        if observation.kind() == RecoveryOutcomeKind::MixedRoot {
            return Err(ObservationDenial::CheckpointPublicationCrashOutcomeMixedRoot);
        }
        Ok(Self {
            checkpoint_plan_identity: binding.plan_identity,
            checkpoint_schedule_identity: binding.schedule_identity,
            checkpoint_origin: expected_origin.clone(),
            recovery_plan_identity: *recovery_plan.identity().digest_bytes(),
            recovery_schedule_identity: *recovery_schedule.identity().digest_bytes(),
            recovery_actor_step_index,
            observation,
        })
    }

    pub const fn observation(&self) -> RecoveryOutcomeObservation {
        self.observation
    }
}

impl PhysicalIsolationCheckpointPublicationCrashLaneOutput {
    pub fn from_schedule_step_recovery(
        binding: &PhysicalIsolationCheckpointPublicationLaneBinding,
        schedule: &PhysicalInterleavingSchedule,
        checkpoint_actor_step_index: usize,
        expected_origin: &CheckpointInterlockEvidenceOrigin,
        evidence: CheckpointInterlockFoundationalEvidence,
        recovery_output: &PhysicalIsolationCheckpointPublicationRecoveryOutcomeLaneOutput,
    ) -> Result<Self, ObservationDenial> {
        require_schedule_step_matches_binding(binding, schedule, checkpoint_actor_step_index)?;
        require_evidence_origin_matches(expected_origin, evidence.origin())?;
        if recovery_output.checkpoint_plan_identity != binding.plan_identity
            || recovery_output.checkpoint_schedule_identity != binding.schedule_identity
            || &recovery_output.checkpoint_origin != expected_origin
        {
            return Err(ObservationDenial::CheckpointPublicationCrashRecoveryTraceMismatch);
        }
        let observation = CheckpointCrashReplayObservation {
            checkpoint_origin: recovery_output.checkpoint_origin.clone(),
            recovery_outcome: recovery_output.observation(),
            checkpoint_actor_step_index,
            recovery_actor_step_index: recovery_output.recovery_actor_step_index,
            recovery_plan_identity: recovery_output.recovery_plan_identity,
            recovery_schedule_identity: recovery_output.recovery_schedule_identity,
        };
        Ok(Self {
            plan_identity: binding.plan_identity,
            schedule_identity: binding.schedule_identity,
            checkpoint_actor_step_index,
            recovery_actor_step_index: recovery_output.recovery_actor_step_index,
            observation,
        })
    }

    pub const fn plan_identity(&self) -> &[u8; 32] {
        &self.plan_identity
    }

    pub const fn schedule_identity(&self) -> &[u8; 32] {
        &self.schedule_identity
    }

    pub const fn checkpoint_actor_step_index(&self) -> usize {
        self.checkpoint_actor_step_index
    }

    pub const fn recovery_actor_step_index(&self) -> usize {
        self.recovery_actor_step_index
    }

    pub const fn recovery_outcome(&self) -> RecoveryOutcomeObservation {
        self.observation.recovery_outcome()
    }

    pub fn observation(&self) -> CheckpointCrashReplayObservation {
        self.observation.clone()
    }
}

impl PhysicalIsolationCheckpointPublicationShortcutRejectionOutput {
    pub fn from_scheduled_same_run_denial(
        binding: &PhysicalIsolationCheckpointPublicationLaneBinding,
        schedule: &PhysicalInterleavingSchedule,
        checkpoint_actor_step_index: usize,
        expected_origin: &CheckpointInterlockEvidenceOrigin,
        evidence: CheckpointInterlockFoundationalEvidence,
        receipt: PhysicalIsolationCheckpointPublicationShortcutDenialLaneOutput,
    ) -> Result<Self, ObservationDenial> {
        require_schedule_step_matches_binding(binding, schedule, checkpoint_actor_step_index)?;
        require_evidence_origin_matches(expected_origin, evidence.origin())?;
        if receipt.plan_identity() != &binding.plan_identity
            || receipt.schedule_identity() != &binding.schedule_identity
            || receipt.checkpoint_origin() != expected_origin
        {
            return Err(ObservationDenial::CheckpointPublicationShortcutLaneScheduleMismatch);
        }
        Ok(Self {
            plan_identity: binding.plan_identity,
            schedule_identity: binding.schedule_identity,
            checkpoint_actor_step_index,
            shortcut_actor_step_index: receipt.shortcut_actor_step_index(),
            observation: receipt.observation(),
        })
    }

    pub const fn plan_identity(&self) -> &[u8; 32] {
        &self.plan_identity
    }

    pub const fn schedule_identity(&self) -> &[u8; 32] {
        &self.schedule_identity
    }

    pub const fn checkpoint_actor_step_index(&self) -> usize {
        self.checkpoint_actor_step_index
    }

    pub const fn shortcut_actor_step_index(&self) -> usize {
        self.shortcut_actor_step_index
    }

    pub const fn observation(&self) -> ShortcutRejectionObservation {
        self.observation
    }
}

fn require_evidence_origin_matches(
    expected_origin: &CheckpointInterlockEvidenceOrigin,
    observed_origin: &CheckpointInterlockEvidenceOrigin,
) -> Result<(), ObservationDenial> {
    if expected_origin == observed_origin {
        Ok(())
    } else {
        Err(ObservationDenial::CheckpointPublicationEvidenceOriginMismatch)
    }
}

fn require_role_yieldpoint_step_matches_schedule(
    schedule: &PhysicalInterleavingSchedule,
    actor_step_index: usize,
    role: PhysicalScenarioActorRole,
    yieldpoint: &str,
    denial: ObservationDenial,
) -> Result<(), ObservationDenial> {
    schedule
        .actor_steps()
        .get(actor_step_index)
        .filter(|step| step.actor_role() == role && step.yieldpoint() == yieldpoint)
        .map(|_| ())
        .ok_or(denial)
}

fn require_schedule_matches_checkpoint_plan(
    plan: &PhysicalSimulationPlan,
    schedule: &PhysicalInterleavingSchedule,
) -> Result<usize, ObservationDenial> {
    if !schedule.replay_identity_matches_plan(plan) {
        return Err(ObservationDenial::CheckpointPublicationLaneScheduleMismatch);
    }
    let declared_yieldpoint = plan.yieldpoint_binding().declared_yieldpoint().name();
    schedule
        .actor_steps()
        .iter()
        .position(|step| {
            step.actor_role() == PhysicalScenarioActorRole::CheckpointDriver
                && step.yieldpoint() == declared_yieldpoint
        })
        .ok_or(ObservationDenial::MissingCheckpointPublicationLane)
}

fn require_schedule_step_matches_binding(
    binding: &PhysicalIsolationCheckpointPublicationLaneBinding,
    schedule: &PhysicalInterleavingSchedule,
    actor_step_index: usize,
) -> Result<(), ObservationDenial> {
    if schedule.identity().digest_bytes() != &binding.schedule_identity
        || actor_step_index != binding.checkpoint_actor_step_index
    {
        return Err(ObservationDenial::CheckpointPublicationLaneScheduleMismatch);
    }
    schedule
        .actor_steps()
        .get(actor_step_index)
        .filter(|step| step.actor_role() == PhysicalScenarioActorRole::CheckpointDriver)
        .map(|_| ())
        .ok_or(ObservationDenial::MissingCheckpointPublicationLane)
}
