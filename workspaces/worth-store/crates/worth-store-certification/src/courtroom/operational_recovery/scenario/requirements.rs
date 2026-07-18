use worth_store_formal_models::OperationalRecoveryActionKind;
use worth_store_operations::{OperationalCounterReceipt, OperationalSessionKind};
use worth_store_physical_certification::{
    OperationalRecoveryControlTransitionKind, OperationalRecoveryYieldpoint,
};

use super::{S10OperationalScenarioKind, S10ScenarioCertificationDenial};

pub(super) fn required_yieldpoints(
    kind: S10OperationalScenarioKind,
) -> Vec<OperationalRecoveryYieldpoint> {
    OperationalRecoveryYieldpoint::ALL
        .into_iter()
        .filter(|point| scenario_uses_yieldpoint(kind, *point))
        .collect()
}

fn scenario_uses_yieldpoint(
    scenario: S10OperationalScenarioKind,
    point: OperationalRecoveryYieldpoint,
) -> bool {
    use OperationalRecoveryControlTransitionKind as Control;
    use OperationalRecoveryYieldpoint as Point;
    match point {
        Point::BeforeDurableControlTransition(control)
        | Point::AfterDurableControlTransition(control) => match control {
            Control::RepairExecutionOpen
            | Control::RepairOwnerEffect
            | Control::RepairOwnerReceipt
            | Control::RepairDisposition => scenario != S10OperationalScenarioKind::BurningPrimary,
            Control::WorkflowAbandonment => scenario == S10OperationalScenarioKind::BurningPrimary,
            Control::ReplicaBootstrapTransfer
            | Control::ReplicaBootstrapCompletion
            | Control::ReplicaPromotionFence
            | Control::ReplicaPromotionRecord
            | Control::ReplicaPromotionPublication
            | Control::ReplicaPromotionReadmission => {
                scenario != S10OperationalScenarioKind::AuthorityRepairRollback
            }
            Control::OldPrimaryRejoinPlan | Control::OldPrimaryRejoinCompletion => {
                scenario == S10OperationalScenarioKind::SplitBrainPromotion
            }
            _ => true,
        },
        Point::BeforeOldPrimaryRejoinPlan
        | Point::AfterOldPrimaryRejoinPlan
        | Point::BeforeOldPrimaryRejoinExecution
        | Point::AfterOldPrimaryRejoinExecution
        | Point::BeforeOldPrimaryRejoinCompletion
        | Point::AfterOldPrimaryRejoinCompletion => {
            scenario == S10OperationalScenarioKind::SplitBrainPromotion
        }
        Point::BeforeBootstrapTransfer
        | Point::AfterBootstrapTransfer
        | Point::BeforeBootstrapControlRecord
        | Point::AfterBootstrapControlRecord
        | Point::BeforeBootstrapPostVerification
        | Point::AfterBootstrapPostVerification
        | Point::BeforeBootstrapCompletion
        | Point::AfterBootstrapCompletion
        | Point::BeforePromotionExternalFence
        | Point::AfterPromotionExternalFence
        | Point::BeforePromotionFenceRecord
        | Point::AfterPromotionFenceRecord
        | Point::BeforePromotionRecord
        | Point::AfterPromotionRecord
        | Point::BeforePromotionPostVerification
        | Point::AfterPromotionPostVerification
        | Point::BeforePromotionPublication
        | Point::AfterPromotionPublication
        | Point::BeforePromotionReadmission
        | Point::AfterPromotionReadmission => {
            scenario != S10OperationalScenarioKind::AuthorityRepairRollback
        }
        _ => true,
    }
}

pub(super) fn require_counter_kinds(
    kind: S10OperationalScenarioKind,
    counters: &[OperationalCounterReceipt],
) -> Result<(), S10ScenarioCertificationDenial> {
    let required = match kind {
        S10OperationalScenarioKind::BurningPrimary => vec![
            OperationalSessionKind::Backup,
            OperationalSessionKind::Restore,
            OperationalSessionKind::PointInTimeRecovery,
            OperationalSessionKind::Rollback,
            OperationalSessionKind::ReplicaBootstrap,
            OperationalSessionKind::ReplicaPromotion,
            OperationalSessionKind::OfflineVerification,
            OperationalSessionKind::ForensicAcquisition,
        ],
        S10OperationalScenarioKind::SplitBrainPromotion => vec![
            OperationalSessionKind::Backup,
            OperationalSessionKind::Restore,
            OperationalSessionKind::PointInTimeRecovery,
            OperationalSessionKind::Repair,
            OperationalSessionKind::ReplicaBootstrap,
            OperationalSessionKind::ReplicaPromotion,
            OperationalSessionKind::OfflineVerification,
            OperationalSessionKind::ForensicAcquisition,
        ],
        S10OperationalScenarioKind::AuthorityRepairRollback => vec![
            OperationalSessionKind::Backup,
            OperationalSessionKind::Restore,
            OperationalSessionKind::PointInTimeRecovery,
            OperationalSessionKind::Rollback,
            OperationalSessionKind::Repair,
            OperationalSessionKind::OfflineVerification,
            OperationalSessionKind::ForensicAcquisition,
        ],
    };
    for required_kind in required {
        if !counters
            .iter()
            .any(|receipt| receipt.kind() == required_kind)
        {
            return Err(S10ScenarioCertificationDenial::MissingOperationCounters);
        }
    }
    Ok(())
}

pub(super) fn required_model_transitions(
    kind: S10OperationalScenarioKind,
) -> Vec<OperationalRecoveryActionKind> {
    use OperationalRecoveryActionKind as Action;
    let mut required = vec![
        Action::AuthorizationConsumed,
        Action::StagingCompleted,
        Action::PublicationPrepared,
        Action::PublicationPending,
        Action::PublicationDisposition,
        Action::FenceReleased,
    ];
    match kind {
        S10OperationalScenarioKind::BurningPrimary => required.extend([
            Action::SourceLeasePersisted,
            Action::MaterializationOpened,
            Action::MaterializationRecorded,
            Action::IndependentVerificationRecorded,
            Action::Abandoned,
            Action::WorkflowOwnerReceiptPersisted,
            Action::ReplicaBootstrapTransferRecorded,
            Action::ReplicaBootstrapCompleted,
            Action::ReplicaPromotionFenceRecorded,
            Action::ReplicaPromotionRecorded,
            Action::ReplicaPromotionPublished,
            Action::ReplicaPromotionReadmitted,
        ]),
        S10OperationalScenarioKind::SplitBrainPromotion => required.extend([
            Action::SourceLeasePersisted,
            Action::MaterializationOpened,
            Action::MaterializationRecorded,
            Action::IndependentVerificationRecorded,
            Action::WorkflowOwnerReceiptPersisted,
            Action::ReplicaBootstrapTransferRecorded,
            Action::ReplicaBootstrapCompleted,
            Action::ReplicaPromotionFenceRecorded,
            Action::ReplicaPromotionRecorded,
            Action::ReplicaPromotionPublished,
            Action::ReplicaPromotionReadmitted,
            Action::OldPrimaryRejoinPlanned,
            Action::OldPrimaryRejoinCompleted,
        ]),
        S10OperationalScenarioKind::AuthorityRepairRollback => required.extend([
            Action::SourceLeasePersisted,
            Action::MaterializationOpened,
            Action::MaterializationRecorded,
            Action::IndependentVerificationRecorded,
            Action::OwnerExecutionOpened,
            Action::OwnerEffectStarted,
            Action::OwnerReceiptPersisted,
            Action::WorkflowOwnerReceiptPersisted,
            Action::DispositionRecorded,
        ]),
    }
    required
}
