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
    WorkflowOwnerReceiptPersisted,
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

impl OperationalRecoveryActionKind {
    pub(super) const fn stable_tag(self) -> u8 {
        match self {
            Self::WorkflowOpened => 1,
            Self::SourceLeasePersisted => 2,
            Self::MaterializationOpened => 3,
            Self::MaterializationRecorded => 4,
            Self::IndependentVerificationRecorded => 5,
            Self::Abandoned => 6,
            Self::AuthorizationConsumed => 7,
            Self::OwnerExecutionOpened => 8,
            Self::OwnerEffectStarted => 9,
            Self::OwnerReceiptPersisted => 10,
            Self::WorkflowOwnerReceiptPersisted => 11,
            Self::DispositionRecorded => 12,
            Self::StagingCompleted => 13,
            Self::PublicationPrepared => 14,
            Self::PublicationPending => 15,
            Self::PublicationDisposition => 16,
            Self::FenceReleased => 17,
            Self::ReplicaBootstrapTransferRecorded => 18,
            Self::ReplicaBootstrapCompleted => 19,
            Self::ReplicaPromotionFenceRecorded => 20,
            Self::ReplicaPromotionRecorded => 21,
            Self::ReplicaPromotionPublished => 22,
            Self::ReplicaPromotionReadmitted => 23,
            Self::OldPrimaryRejoinPlanned => 24,
            Self::OldPrimaryRejoinCompleted => 25,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationalRecoveryAction {
    pub(super) authority_identity: [u8; 32],
    pub(super) operation_identity: String,
    pub(super) transition_identity: String,
    pub(super) kind: OperationalRecoveryActionKind,
    pub(super) owner_tag: Option<u8>,
    pub(super) binding: super::binding::OperationalRecoveryActionBinding,
    pub(super) evidence_identity: [u8; 32],
}

impl OperationalRecoveryAction {
    #[cfg(test)]
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
        digest.update([kind.stable_tag()]);
        Self {
            authority_identity: [0; 32],
            operation_identity: operation_identity.to_owned(),
            transition_identity: transition_identity.to_owned(),
            kind,
            owner_tag: None,
            binding: super::binding::OperationalRecoveryActionBinding::None,
            evidence_identity: digest.finalize().into(),
        }
    }

    pub fn operation_identity(&self) -> &str {
        &self.operation_identity
    }

    #[cfg(test)]
    pub(super) fn controlled_owner_receipt_probe(
        operation_identity: &str,
        transition_identity: &str,
        owner_tag: u8,
    ) -> Self {
        let mut action = Self::controlled_defect_probe(
            operation_identity,
            transition_identity,
            OperationalRecoveryActionKind::WorkflowOwnerReceiptPersisted,
        );
        action.owner_tag = Some(owner_tag);
        action
    }
    pub const fn authority_identity(&self) -> [u8; 32] {
        self.authority_identity
    }
    pub fn transition_identity(&self) -> &str {
        &self.transition_identity
    }
    pub const fn kind(&self) -> OperationalRecoveryActionKind {
        self.kind
    }
    pub const fn owner_tag(&self) -> Option<u8> {
        self.owner_tag
    }
    pub const fn evidence_identity(&self) -> [u8; 32] {
        self.evidence_identity
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

    #[test]
    fn owner_receipt_without_open_execution_is_localized() {
        let mut model = OperationalRecoveryModel::default();
        model
            .apply(&action("authorize", Action::AuthorizationConsumed), None)
            .unwrap();
        let denial = model
            .apply(&action("receipt", Action::OwnerReceiptPersisted), None)
            .unwrap_err();
        assert_eq!(
            denial.invariant(),
            OperationalRecoveryInvariant::OwnerExecutionBeforeReceipt
        );
    }

    #[test]
    fn staging_requires_both_distinct_workflow_owner_receipts() {
        let mut model = OperationalRecoveryModel::default();
        model
            .apply(&action("authorize", Action::AuthorizationConsumed), None)
            .unwrap();
        for owner_tag in [1, 2] {
            model
                .apply(
                    &OperationalRecoveryAction::controlled_owner_receipt_probe(
                        "promotion-1",
                        &format!("owner-{owner_tag}"),
                        owner_tag,
                    ),
                    None,
                )
                .unwrap();
        }
        model
            .apply(&action("staged", Action::StagingCompleted), None)
            .expect("complete owner evidence admits aggregate staging completion");
    }

    #[test]
    fn backup_verification_requires_the_complete_materialization_prefix() {
        let mut model = OperationalRecoveryModel::default();
        model
            .apply(&action("open", Action::WorkflowOpened), None)
            .unwrap();
        let denial = model
            .apply(
                &action("verify", Action::IndependentVerificationRecorded),
                None,
            )
            .unwrap_err();
        assert_eq!(
            denial.invariant(),
            OperationalRecoveryInvariant::MaterializationBeforeVerification
        );
    }

    #[test]
    fn completed_backup_is_terminal_in_the_model() {
        let mut model = OperationalRecoveryModel::default();
        for event in [
            action("open", Action::WorkflowOpened),
            action("lease", Action::SourceLeasePersisted),
            action("materialize-open", Action::MaterializationOpened),
            action("materialize", Action::MaterializationRecorded),
            action("verify", Action::IndependentVerificationRecorded),
        ] {
            model.apply(&event, None).unwrap();
        }
        let denial = model
            .apply(&action("late", Action::MaterializationRecorded), None)
            .unwrap_err();
        assert_eq!(
            denial.invariant(),
            OperationalRecoveryInvariant::TerminalOperationHasNoLaterTransition
        );
    }

    fn action(transition: &str, kind: Action) -> OperationalRecoveryAction {
        OperationalRecoveryAction {
            authority_identity: [0; 32],
            operation_identity: "promotion-1".to_owned(),
            transition_identity: transition.to_owned(),
            kind,
            owner_tag: None,
            binding: super::super::binding::OperationalRecoveryActionBinding::None,
            evidence_identity: [7; 32],
        }
    }
}
