use crate::{
    CoverageGapDenial, PhysicalInterleavingSchedule,
    PhysicalIsolationCompactionMutationCoverageRow, PhysicalIsolationCompactionMutationKind,
    PhysicalScenarioActorRole, PhysicalSimulationPlan,
};
use forge_store_physical_isolation::{
    CompactionMutationLaneOrigin, CompactionMutationLaneReceipt, CompactionMutationLaneReceiptKind,
    CompactionReadInterlockDenial,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalIsolationCompactionMutationObservationSet {
    plan_identity: [u8; 32],
    schedule_identity: [u8; 32],
    origin: CompactionMutationLaneOrigin,
    rows: Vec<PhysicalIsolationCompactionMutationCoverageRow>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalIsolationCompactionMutationReplayBinding {
    plan_identity: [u8; 32],
    schedule_identity: [u8; 32],
    origin: CompactionMutationLaneOrigin,
    compaction_actor_step_index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalIsolationCompactionMutationLaneExecution {
    origin: CompactionMutationLaneOrigin,
    row: PhysicalIsolationCompactionMutationCoverageRow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalIsolationCompactionMutationScheduledLaneOutput {
    plan_identity: [u8; 32],
    schedule_identity: [u8; 32],
    compaction_actor_step_index: usize,
    execution: PhysicalIsolationCompactionMutationLaneExecution,
}

impl PhysicalIsolationCompactionMutationObservationSet {
    pub fn from_scheduled_lanes(
        binding: PhysicalIsolationCompactionMutationReplayBinding,
        lanes: impl IntoIterator<Item = PhysicalIsolationCompactionMutationScheduledLaneOutput>,
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

    pub fn rows(&self) -> &[PhysicalIsolationCompactionMutationCoverageRow] {
        &self.rows
    }

    pub const fn origin(&self) -> &CompactionMutationLaneOrigin {
        &self.origin
    }
}

impl PhysicalIsolationCompactionMutationReplayBinding {
    pub fn from_plan_and_schedule(
        plan: &PhysicalSimulationPlan,
        schedule: &PhysicalInterleavingSchedule,
    ) -> Result<Self, CoverageGapDenial> {
        let compaction_actor_step_index = require_schedule_matches_compaction_plan(plan, schedule)?;
        let origin = plan
            .physical_isolation_compaction_mutation_origin()
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

impl PhysicalIsolationCompactionMutationLaneExecution {
    pub fn from_operation_receipt(receipt: CompactionMutationLaneReceipt) -> Self {
        let origin = receipt.origin().clone();
        Self {
            origin,
            row: PhysicalIsolationCompactionMutationCoverageRow::observed(
                physical_isolation_kind_from_receipt_kind(receipt.kind()),
                receipt.denial(),
            )
            .expect("receipt kind and denial are admitted by physical-isolation"),
        }
    }

    pub const fn kind(&self) -> PhysicalIsolationCompactionMutationKind {
        self.row.kind()
    }

    pub const fn denial(&self) -> CompactionReadInterlockDenial {
        self.row.denial()
    }

    pub const fn origin(&self) -> &CompactionMutationLaneOrigin {
        &self.origin
    }
}

impl PhysicalIsolationCompactionMutationScheduledLaneOutput {
    pub fn from_schedule_step_receipt(
        binding: &PhysicalIsolationCompactionMutationReplayBinding,
        schedule: &PhysicalInterleavingSchedule,
        actor_step_index: usize,
        receipt: CompactionMutationLaneReceipt,
    ) -> Result<Self, CoverageGapDenial> {
        require_schedule_step_matches_binding(binding, schedule, actor_step_index)?;
        Ok(Self {
            plan_identity: binding.plan_identity,
            schedule_identity: binding.schedule_identity,
            compaction_actor_step_index: actor_step_index,
            execution: PhysicalIsolationCompactionMutationLaneExecution::from_operation_receipt(
                receipt,
            ),
        })
    }

    pub const fn kind(&self) -> PhysicalIsolationCompactionMutationKind {
        self.execution.kind()
    }

    pub const fn denial(&self) -> CompactionReadInterlockDenial {
        self.execution.denial()
    }

    pub const fn origin(&self) -> &CompactionMutationLaneOrigin {
        self.execution.origin()
    }
}

fn physical_isolation_kind_from_receipt_kind(
    kind: CompactionMutationLaneReceiptKind,
) -> PhysicalIsolationCompactionMutationKind {
    match kind {
        CompactionMutationLaneReceiptKind::InPlaceOverwriteDenied => {
            PhysicalIsolationCompactionMutationKind::InPlaceOverwriteDenied
        }
        CompactionMutationLaneReceiptKind::EarlyReclaimDenied => {
            PhysicalIsolationCompactionMutationKind::EarlyReclaimDenied
        }
        CompactionMutationLaneReceiptKind::StaleEpochReuseDenied => {
            PhysicalIsolationCompactionMutationKind::StaleEpochReuseDenied
        }
        CompactionMutationLaneReceiptKind::BackendResidueCandidateSelectionDenied => {
            PhysicalIsolationCompactionMutationKind::BackendResidueCandidateSelectionDenied
        }
        CompactionMutationLaneReceiptKind::LatchHierarchyInversionDenied => {
            PhysicalIsolationCompactionMutationKind::LatchHierarchyInversionDenied
        }
        CompactionMutationLaneReceiptKind::MixedRootReadDenied => {
            PhysicalIsolationCompactionMutationKind::MixedRootReadDenied
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
            is_physical_isolation_mutation_actor(step.actor_role())
                && step.yieldpoint() == declared_yieldpoint
        })
        .ok_or(CoverageGapDenial::MissingMutationResult)
}

fn require_schedule_step_matches_binding(
    binding: &PhysicalIsolationCompactionMutationReplayBinding,
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
        .filter(|step| is_physical_isolation_mutation_actor(step.actor_role()))
        .map(|_| ())
        .ok_or(CoverageGapDenial::MissingMutationResult)
}

fn is_physical_isolation_mutation_actor(role: PhysicalScenarioActorRole) -> bool {
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
    binding: &PhysicalIsolationCompactionMutationReplayBinding,
    lanes: impl IntoIterator<Item = PhysicalIsolationCompactionMutationScheduledLaneOutput>,
) -> Result<Vec<PhysicalIsolationCompactionMutationCoverageRow>, CoverageGapDenial> {
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
    if rows.len() != PhysicalIsolationCompactionMutationKind::REQUIRED_FOR_S5_INTERLEAVING.len() {
        return Err(CoverageGapDenial::MissingMutationResult);
    }
    for required in PhysicalIsolationCompactionMutationKind::REQUIRED_FOR_S5_INTERLEAVING {
        if rows.iter().filter(|row| row.kind() == required).count() != 1 {
            return Err(CoverageGapDenial::MissingMutationResult);
        }
    }
    Ok(rows)
}
