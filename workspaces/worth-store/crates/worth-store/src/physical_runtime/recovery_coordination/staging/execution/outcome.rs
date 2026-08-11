use crate::physical_runtime::{PhysicalWorkPreEffectDenial, PhysicalWorkSchedulerPosture};

use super::super::{
    PhysicalRecoveryStagingCommandDenial, PhysicalRecoveryStagingCommandDenialKind,
    PhysicalRecoveryStagingCommandOutcome, PhysicalRecoveryStagingCommandStage,
    PhysicalRecoveryStagingMaterialization,
};

pub(super) fn denied(
    stage: PhysicalRecoveryStagingCommandStage,
    denial: PhysicalRecoveryStagingCommandDenialKind,
    materialization: Option<PhysicalRecoveryStagingMaterialization>,
    scheduler: Option<PhysicalWorkSchedulerPosture>,
) -> PhysicalRecoveryStagingCommandOutcome {
    PhysicalRecoveryStagingCommandOutcome::DeniedBeforeEffect(
        PhysicalRecoveryStagingCommandDenial::new(stage, denial, materialization, scheduler),
    )
}

pub(super) fn attach_materialization(
    outcome: PhysicalRecoveryStagingCommandOutcome,
    materialization: PhysicalRecoveryStagingMaterialization,
) -> PhysicalRecoveryStagingCommandOutcome {
    match outcome {
        PhysicalRecoveryStagingCommandOutcome::DeniedBeforeEffect(denial) => denied(
            denial.stage(),
            denial.denial(),
            Some(materialization),
            denial.scheduler_posture(),
        ),
        other => other,
    }
}

pub(super) fn pre_effect(
    stage: PhysicalRecoveryStagingCommandStage,
    denial: PhysicalWorkPreEffectDenial,
) -> PhysicalRecoveryStagingCommandOutcome {
    denied(
        stage,
        PhysicalRecoveryStagingCommandDenialKind::PreEffect(denial),
        None,
        None,
    )
}
