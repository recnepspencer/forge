use worth_store_operations::{OperationalControlRecord, OperationalControlRecordKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OperationalRecoveryActionKind {
    WorkflowOpened,
    SourceLeasePersisted,
    MaterializationOpened,
    MaterializationRecorded,
    IndependentVerificationRecorded,
    Abandoned,
    AuthorizationConsumed,
    OwnerExecutionOpened,
    OwnerEffectStarted,
    OwnerReceiptPersisted,
    DispositionRecorded,
    StagingCompleted,
    PublicationPrepared,
    PublicationPending,
    PublicationDisposition,
    FenceReleased,
    ReplicaBootstrapTransferRecorded,
    ReplicaBootstrapCompleted,
    ReplicaPromotionFenceRecorded,
    ReplicaPromotionRecorded,
    ReplicaPromotionPublished,
    ReplicaPromotionReadmitted,
    OldPrimaryRejoinPlanned,
    OldPrimaryRejoinCompleted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationalRecoveryAction {
    operation_identity: String,
    transition_identity: String,
    kind: OperationalRecoveryActionKind,
    evidence_identity: [u8; 32],
}

impl OperationalRecoveryAction {
    pub(super) fn controlled_defect_probe(
        operation_identity: &str,
        transition_identity: &str,
        kind: OperationalRecoveryActionKind,
    ) -> Self {
        let mut digest = sha2::Sha256::new();
        use sha2::Digest;
        digest.update(b"worth-store-controlled-defect-probe-v1");
        digest.update(operation_identity.as_bytes());
        digest.update(transition_identity.as_bytes());
        digest.update([kind as u8]);
        Self {
            operation_identity: operation_identity.to_owned(),
            transition_identity: transition_identity.to_owned(),
            kind,
            evidence_identity: digest.finalize().into(),
        }
    }

    pub fn operation_identity(&self) -> &str {
        &self.operation_identity
    }
    pub fn transition_identity(&self) -> &str {
        &self.transition_identity
    }
    pub const fn kind(&self) -> OperationalRecoveryActionKind {
        self.kind
    }
    pub const fn evidence_identity(&self) -> [u8; 32] {
        self.evidence_identity
    }
}

pub fn map_operational_control_record(
    record: &OperationalControlRecord,
) -> OperationalRecoveryAction {
    let kind = map_operational_kind(record.kind());
    OperationalRecoveryAction {
        operation_identity: record.operation_id().as_str().to_owned(),
        transition_identity: record.transition_id().as_str().to_owned(),
        kind,
        evidence_identity: evidence_identity(record),
    }
}

fn evidence_identity(record: &OperationalControlRecord) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    let mut digest = Sha256::new();
    digest.update(b"worth-store-operational-model-observation-v1");
    digest.update(record.authority_identity().fingerprint());
    digest.update(record.operation_id().as_str().as_bytes());
    digest.update(record.transition_id().as_str().as_bytes());
    digest.update([map_kind_tag(record.kind())]);
    digest.finalize().into()
}

fn map_kind_tag(kind: &OperationalControlRecordKind) -> u8 {
    map_operational_kind(kind) as u8
}

fn map_operational_kind(kind: &OperationalControlRecordKind) -> OperationalRecoveryActionKind {
    match kind {
        OperationalControlRecordKind::WorkflowOpened { .. } => OperationalRecoveryActionKind::WorkflowOpened,
        OperationalControlRecordKind::SourceLeasePersisted { .. } => OperationalRecoveryActionKind::SourceLeasePersisted,
        OperationalControlRecordKind::BackupMaterializationOpened { .. } => OperationalRecoveryActionKind::MaterializationOpened,
        OperationalControlRecordKind::BackupMaterializationRecorded { .. } => OperationalRecoveryActionKind::MaterializationRecorded,
        OperationalControlRecordKind::IndependentBackupVerificationRecordedAndSourceLeaseReleased { .. } => OperationalRecoveryActionKind::IndependentVerificationRecorded,
        OperationalControlRecordKind::BackupAbandoned { .. } => OperationalRecoveryActionKind::Abandoned,
        OperationalControlRecordKind::AuthorizationConsumed { .. } => OperationalRecoveryActionKind::AuthorizationConsumed,
        OperationalControlRecordKind::RepairExecutionOpened { .. } => OperationalRecoveryActionKind::OwnerExecutionOpened,
        OperationalControlRecordKind::RepairOwnerEffectStarted { .. } => OperationalRecoveryActionKind::OwnerEffectStarted,
        OperationalControlRecordKind::RepairOwnerReceiptPersisted { .. }
        | OperationalControlRecordKind::OperationalOwnerReceiptPersisted { .. } => OperationalRecoveryActionKind::OwnerReceiptPersisted,
        OperationalControlRecordKind::RepairDispositionRecorded { .. } => OperationalRecoveryActionKind::DispositionRecorded,
        OperationalControlRecordKind::RecoveryStagingCompleted { .. } => OperationalRecoveryActionKind::StagingCompleted,
        OperationalControlRecordKind::RecoveryPublicationPrepared { .. } => OperationalRecoveryActionKind::PublicationPrepared,
        OperationalControlRecordKind::RecoveryPublicationPending { .. } => OperationalRecoveryActionKind::PublicationPending,
        OperationalControlRecordKind::RecoveryPublicationDisposition { .. } => OperationalRecoveryActionKind::PublicationDisposition,
        OperationalControlRecordKind::RecoveryPublicationFenceReleased { .. } => OperationalRecoveryActionKind::FenceReleased,
        OperationalControlRecordKind::ReplicaBootstrapTransferRecorded { .. } => OperationalRecoveryActionKind::ReplicaBootstrapTransferRecorded,
        OperationalControlRecordKind::ReplicaBootstrapCompleted { .. } => OperationalRecoveryActionKind::ReplicaBootstrapCompleted,
        OperationalControlRecordKind::ReplicaBootstrapAbandoned { .. } => OperationalRecoveryActionKind::Abandoned,
        OperationalControlRecordKind::ReplicaPromotionFenceRecorded { .. } => OperationalRecoveryActionKind::ReplicaPromotionFenceRecorded,
        OperationalControlRecordKind::ReplicaPromotionRecorded { .. } => OperationalRecoveryActionKind::ReplicaPromotionRecorded,
        OperationalControlRecordKind::ReplicaPromotionPublished { .. } => OperationalRecoveryActionKind::ReplicaPromotionPublished,
        OperationalControlRecordKind::ReplicaPromotionReadmitted { .. } => OperationalRecoveryActionKind::ReplicaPromotionReadmitted,
        OperationalControlRecordKind::OldPrimaryRejoinPlanned { .. } => OperationalRecoveryActionKind::OldPrimaryRejoinPlanned,
        OperationalControlRecordKind::OldPrimaryRejoinCompleted { .. } => OperationalRecoveryActionKind::OldPrimaryRejoinCompleted,
    }
}

#[cfg(test)]
mod tests {
    use super::{OperationalRecoveryAction, OperationalRecoveryActionKind as Action};
    use crate::protocols::operational_recovery::{
        OperationalRecoveryControlledDefect, OperationalRecoveryInvariant, OperationalRecoveryModel,
    };

    #[test]
    fn promotion_without_a_durable_external_fence_is_localized() {
        let mut model = OperationalRecoveryModel::default();
        model
            .apply(&action("authorize", Action::AuthorizationConsumed), None)
            .unwrap();

        let denial = model
            .apply(&action("promote", Action::ReplicaPromotionRecorded), None)
            .unwrap_err();

        assert_eq!(
            denial.invariant(),
            OperationalRecoveryInvariant::ExternalFenceBeforePromotion
        );
    }

    #[test]
    fn controlled_defect_demonstrates_sensitivity_to_the_exact_edge() {
        let mut model = OperationalRecoveryModel::default();
        model
            .apply(&action("authorize", Action::AuthorizationConsumed), None)
            .unwrap();

        model
            .apply(
                &action("promote", Action::ReplicaPromotionRecorded),
                Some(OperationalRecoveryControlledDefect::PromotionWithoutExternalFence),
            )
            .expect("the controlled defect removes exactly the fence invariant");
    }

    #[test]
    fn valid_promotion_trace_reaches_each_concrete_transition() {
        let mut model = OperationalRecoveryModel::default();
        for event in [
            action("authorize", Action::AuthorizationConsumed),
            action("fence", Action::ReplicaPromotionFenceRecorded),
            action("promote", Action::ReplicaPromotionRecorded),
            action("publish", Action::ReplicaPromotionPublished),
            action("readmit", Action::ReplicaPromotionReadmitted),
            action("rejoin", Action::OldPrimaryRejoinPlanned),
            action("rejoin-complete", Action::OldPrimaryRejoinCompleted),
        ] {
            model.apply(&event, None).unwrap();
        }

        assert_eq!(model.reached_transitions().len(), 7);
    }

    #[test]
    fn bootstrap_completion_without_transfer_is_localized() {
        let mut model = OperationalRecoveryModel::default();
        model
            .apply(&action("authorize", Action::AuthorizationConsumed), None)
            .unwrap();
        let denial = model
            .apply(&action("complete", Action::ReplicaBootstrapCompleted), None)
            .unwrap_err();
        assert_eq!(
            denial.invariant(),
            OperationalRecoveryInvariant::BootstrapTransferBeforeCompletion
        );
    }

    #[test]
    fn promotion_readmission_without_publication_is_localized() {
        let mut model = OperationalRecoveryModel::default();
        model
            .apply(&action("authorize", Action::AuthorizationConsumed), None)
            .unwrap();
        model
            .apply(
                &action("fence", Action::ReplicaPromotionFenceRecorded),
                None,
            )
            .unwrap();
        model
            .apply(&action("promote", Action::ReplicaPromotionRecorded), None)
            .unwrap();
        let denial = model
            .apply(&action("readmit", Action::ReplicaPromotionReadmitted), None)
            .unwrap_err();
        assert_eq!(
            denial.invariant(),
            OperationalRecoveryInvariant::PromotionPublicationBeforeReadmission
        );
    }

    fn action(transition: &str, kind: Action) -> OperationalRecoveryAction {
        OperationalRecoveryAction {
            operation_identity: "promotion-1".to_owned(),
            transition_identity: transition.to_owned(),
            kind,
            evidence_identity: [7; 32],
        }
    }
}
