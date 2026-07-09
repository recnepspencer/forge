use crate::{
    CoverageGapDenial, PhysicalInterleavingSchedule, PhysicalScenarioActorRole,
    PhysicalSimulationPlan, S5CompactionMutationCoverageRow, S5CompactionMutationKind,
};
use worth_store_physical_isolation::{
    CompactionMutationLaneOrigin, CompactionMutationLaneReceipt, CompactionMutationLaneReceiptKind,
    CompactionReadInterlockDenial,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S5CompactionMutationObservationSet {
    plan_identity: [u8; 32],
    schedule_identity: [u8; 32],
    origin: CompactionMutationLaneOrigin,
    rows: Vec<S5CompactionMutationCoverageRow>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S5CompactionMutationReplayBinding {
    plan_identity: [u8; 32],
    schedule_identity: [u8; 32],
    origin: CompactionMutationLaneOrigin,
    compaction_actor_step_index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S5CompactionMutationLaneExecution {
    origin: CompactionMutationLaneOrigin,
    row: S5CompactionMutationCoverageRow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S5CompactionMutationScheduledLaneOutput {
    plan_identity: [u8; 32],
    schedule_identity: [u8; 32],
    compaction_actor_step_index: usize,
    execution: S5CompactionMutationLaneExecution,
}

impl S5CompactionMutationObservationSet {
    pub fn from_scheduled_lanes(
        binding: S5CompactionMutationReplayBinding,
        lanes: impl IntoIterator<Item = S5CompactionMutationScheduledLaneOutput>,
    ) -> Result<Self, CoverageGapDenial> {
        let rows = require_complete_origin_bound_rows(&binding, lanes)?;
        Ok(Self {
            plan_identity: binding.plan_identity,
            schedule_identity: binding.schedule_identity,
            origin: binding.origin,
            rows,
        })
    }

    pub const fn plan_identity(&self) -> &[u8; 32] {
        &self.plan_identity
    }

    pub const fn schedule_identity(&self) -> &[u8; 32] {
        &self.schedule_identity
    }

    pub fn rows(&self) -> &[S5CompactionMutationCoverageRow] {
        &self.rows
    }

    pub const fn origin(&self) -> &CompactionMutationLaneOrigin {
        &self.origin
    }
}

impl S5CompactionMutationReplayBinding {
    pub fn from_plan_and_schedule(
        plan: &PhysicalSimulationPlan,
        schedule: &PhysicalInterleavingSchedule,
    ) -> Result<Self, CoverageGapDenial> {
        let compaction_actor_step_index = require_schedule_matches_compaction_plan(plan, schedule)?;
        let origin = plan
            .s5_compaction_mutation_origin()
            .cloned()
            .ok_or(CoverageGapDenial::MissingMutationResult)?;
        Ok(Self {
            plan_identity: *plan.identity().digest_bytes(),
            schedule_identity: *schedule.identity().digest_bytes(),
            origin,
            compaction_actor_step_index,
        })
    }

    pub const fn origin(&self) -> &CompactionMutationLaneOrigin {
        &self.origin
    }

    pub const fn compaction_actor_step_index(&self) -> usize {
        self.compaction_actor_step_index
    }
}

impl S5CompactionMutationLaneExecution {
    pub fn from_operation_receipt(receipt: CompactionMutationLaneReceipt) -> Self {
        let origin = receipt.origin().clone();
        Self {
            origin,
            row: S5CompactionMutationCoverageRow::observed(
                s5_kind_from_receipt_kind(receipt.kind()),
                receipt.denial(),
            )
            .expect("receipt kind and denial are admitted by physical-isolation"),
        }
    }

    pub const fn kind(&self) -> S5CompactionMutationKind {
        self.row.kind()
    }

    pub const fn denial(&self) -> CompactionReadInterlockDenial {
        self.row.denial()
    }

    pub const fn origin(&self) -> &CompactionMutationLaneOrigin {
        &self.origin
    }
}

impl S5CompactionMutationScheduledLaneOutput {
    pub fn from_schedule_step_receipt(
        binding: &S5CompactionMutationReplayBinding,
        schedule: &PhysicalInterleavingSchedule,
        actor_step_index: usize,
        receipt: CompactionMutationLaneReceipt,
    ) -> Result<Self, CoverageGapDenial> {
        require_schedule_step_matches_binding(binding, schedule, actor_step_index)?;
        Ok(Self {
            plan_identity: binding.plan_identity,
            schedule_identity: binding.schedule_identity,
            compaction_actor_step_index: actor_step_index,
            execution: S5CompactionMutationLaneExecution::from_operation_receipt(receipt),
        })
    }

    pub const fn kind(&self) -> S5CompactionMutationKind {
        self.execution.kind()
    }

    pub const fn denial(&self) -> CompactionReadInterlockDenial {
        self.execution.denial()
    }

    pub const fn origin(&self) -> &CompactionMutationLaneOrigin {
        self.execution.origin()
    }
}

fn s5_kind_from_receipt_kind(kind: CompactionMutationLaneReceiptKind) -> S5CompactionMutationKind {
    match kind {
        CompactionMutationLaneReceiptKind::InPlaceOverwriteDenied => {
            S5CompactionMutationKind::InPlaceOverwriteDenied
        }
        CompactionMutationLaneReceiptKind::EarlyReclaimDenied => {
            S5CompactionMutationKind::EarlyReclaimDenied
        }
        CompactionMutationLaneReceiptKind::StaleEpochReuseDenied => {
            S5CompactionMutationKind::StaleEpochReuseDenied
        }
        CompactionMutationLaneReceiptKind::BackendResidueCandidateSelectionDenied => {
            S5CompactionMutationKind::BackendResidueCandidateSelectionDenied
        }
        CompactionMutationLaneReceiptKind::LatchHierarchyInversionDenied => {
            S5CompactionMutationKind::LatchHierarchyInversionDenied
        }
        CompactionMutationLaneReceiptKind::MixedRootReadDenied => {
            S5CompactionMutationKind::MixedRootReadDenied
        }
    }
}

fn require_schedule_matches_compaction_plan(
    plan: &PhysicalSimulationPlan,
    schedule: &PhysicalInterleavingSchedule,
) -> Result<usize, CoverageGapDenial> {
    if !schedule.replay_identity_matches_plan(plan) {
        return Err(CoverageGapDenial::MissingMutationResult);
    }
    let declared_yieldpoint = plan.yieldpoint_binding().declared_yieldpoint().name();
    schedule
        .actor_steps()
        .iter()
        .position(|step| {
            is_s5_mutation_actor(step.actor_role()) && step.yieldpoint() == declared_yieldpoint
        })
        .ok_or(CoverageGapDenial::MissingMutationResult)
}

fn require_schedule_step_matches_binding(
    binding: &S5CompactionMutationReplayBinding,
    schedule: &PhysicalInterleavingSchedule,
    actor_step_index: usize,
) -> Result<(), CoverageGapDenial> {
    if schedule.identity().digest_bytes() != &binding.schedule_identity
        || actor_step_index != binding.compaction_actor_step_index
    {
        return Err(CoverageGapDenial::MissingMutationResult);
    }
    schedule
        .actor_steps()
        .get(actor_step_index)
        .filter(|step| is_s5_mutation_actor(step.actor_role()))
        .map(|_| ())
        .ok_or(CoverageGapDenial::MissingMutationResult)
}

fn is_s5_mutation_actor(role: PhysicalScenarioActorRole) -> bool {
    matches!(
        role,
        PhysicalScenarioActorRole::CompactionDriver
            | PhysicalScenarioActorRole::MaintenanceReclaimer
            | PhysicalScenarioActorRole::CheckpointDriver
            | PhysicalScenarioActorRole::ForegroundWriter
            | PhysicalScenarioActorRole::RecoveryDriver
    )
}

fn require_complete_origin_bound_rows(
    binding: &S5CompactionMutationReplayBinding,
    lanes: impl IntoIterator<Item = S5CompactionMutationScheduledLaneOutput>,
) -> Result<Vec<S5CompactionMutationCoverageRow>, CoverageGapDenial> {
    let lanes = lanes.into_iter().collect::<Vec<_>>();
    if lanes.iter().any(|lane| {
        lane.plan_identity != binding.plan_identity
            || lane.schedule_identity != binding.schedule_identity
            || lane.compaction_actor_step_index != binding.compaction_actor_step_index
            || lane.origin() != &binding.origin
    }) {
        return Err(CoverageGapDenial::MissingMutationResult);
    }
    let rows = lanes
        .into_iter()
        .map(|lane| lane.execution.row)
        .collect::<Vec<_>>();
    if rows.len() != S5CompactionMutationKind::REQUIRED_FOR_S5_INTERLEAVING.len() {
        return Err(CoverageGapDenial::MissingMutationResult);
    }
    for required in S5CompactionMutationKind::REQUIRED_FOR_S5_INTERLEAVING {
        if rows.iter().filter(|row| row.kind() == required).count() != 1 {
            return Err(CoverageGapDenial::MissingMutationResult);
        }
    }
    Ok(rows)
}
