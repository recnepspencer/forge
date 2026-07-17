use sha2::{Digest, Sha256};
use worth_store_operations::OperationalControlRecord;

use super::{
    check_operational_recovery_refinement, map_operational_control_record,
    OperationalRecoveryAction, OperationalRecoveryActionKind as Action,
    OperationalRecoveryControlledDefect, OperationalRecoveryInvariant,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OperationalRecoveryModelFamily {
    BackupMaterialization,
    BackupReachability,
    Authorization,
    OwnerExecution,
    RecoveryStaging,
    Publication,
    Promotion,
    ReplicaBootstrap,
    PromotionPublication,
    OldPrimaryRejoin,
}

impl OperationalRecoveryModelFamily {
    pub const fn all() -> [Self; 10] {
        [
            Self::BackupMaterialization,
            Self::BackupReachability,
            Self::Authorization,
            Self::OwnerExecution,
            Self::RecoveryStaging,
            Self::Publication,
            Self::Promotion,
            Self::ReplicaBootstrap,
            Self::PromotionPublication,
            Self::OldPrimaryRejoin,
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperationalRecoveryMutationSensitivityDenial {
    CleanProductionHistoryRejected(OperationalRecoveryInvariant),
    DefectiveHistoryAccepted(OperationalRecoveryModelFamily),
    WrongInvariant {
        family: OperationalRecoveryModelFamily,
        observed: OperationalRecoveryInvariant,
    },
    DefectDidNotRemoveExpectedInvariant(OperationalRecoveryModelFamily),
    NoProductionRelevantMutation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationalRecoveryMutationSensitivityReceipt {
    family: OperationalRecoveryModelFamily,
    defect: OperationalRecoveryControlledDefect,
    localized_invariant: OperationalRecoveryInvariant,
    localized_transition: String,
    removed_production_artifact_identity: [u8; 32],
    affected_production_artifact_identity: [u8; 32],
    receipt_identity: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationalRecoveryMutationSensitivitySuite {
    receipts: Vec<OperationalRecoveryMutationSensitivityReceipt>,
    source_refinement_identity: [u8; 32],
    suite_identity: [u8; 32],
}

pub fn check_operational_recovery_mutation_sensitivity(
    records: &[OperationalControlRecord],
) -> Result<
    (
        super::OperationalRecoveryRefinementReceipt,
        OperationalRecoveryMutationSensitivitySuite,
    ),
    OperationalRecoveryMutationSensitivityDenial,
> {
    let clean = check_operational_recovery_refinement(records, None).map_err(|failure| {
        OperationalRecoveryMutationSensitivityDenial::CleanProductionHistoryRejected(
            failure.invariant(),
        )
    })?;
    let actions = records
        .iter()
        .map(map_operational_control_record)
        .collect::<Vec<_>>();
    let mut receipts = Vec::new();
    for case in MutationCase::all() {
        let Some(edge) = locate_production_edge(case, &actions) else {
            continue;
        };
        receipts.push(check_case(
            case,
            edge,
            records,
            clean.refinement_identity(),
        )?);
    }
    if receipts.is_empty() {
        return Err(OperationalRecoveryMutationSensitivityDenial::NoProductionRelevantMutation);
    }
    let source_refinement_identity = clean.refinement_identity();
    let mut digest = Sha256::new();
    digest.update(b"worth-store-operational-recovery-mutation-suite-v3");
    digest.update(source_refinement_identity);
    for receipt in &receipts {
        digest.update(receipt.receipt_identity);
    }
    Ok((
        clean,
        OperationalRecoveryMutationSensitivitySuite {
            receipts,
            source_refinement_identity,
            suite_identity: digest.finalize().into(),
        },
    ))
}

impl OperationalRecoveryMutationSensitivityReceipt {
    pub const fn family(&self) -> OperationalRecoveryModelFamily {
        self.family
    }
    pub const fn defect(&self) -> OperationalRecoveryControlledDefect {
        self.defect
    }
    pub const fn localized_invariant(&self) -> OperationalRecoveryInvariant {
        self.localized_invariant
    }
    pub fn localized_transition(&self) -> &str {
        &self.localized_transition
    }
    pub const fn removed_production_artifact_identity(&self) -> [u8; 32] {
        self.removed_production_artifact_identity
    }
    pub const fn affected_production_artifact_identity(&self) -> [u8; 32] {
        self.affected_production_artifact_identity
    }
    pub const fn receipt_identity(&self) -> [u8; 32] {
        self.receipt_identity
    }
}

impl OperationalRecoveryMutationSensitivitySuite {
    pub fn receipts(&self) -> &[OperationalRecoveryMutationSensitivityReceipt] {
        &self.receipts
    }
    pub const fn source_refinement_identity(&self) -> [u8; 32] {
        self.source_refinement_identity
    }
    pub const fn suite_identity(&self) -> [u8; 32] {
        self.suite_identity
    }
}

#[derive(Clone, Copy)]
pub(super) struct MutationCase {
    family: OperationalRecoveryModelFamily,
    defect: OperationalRecoveryControlledDefect,
    expected: OperationalRecoveryInvariant,
    pub(super) prerequisite: Action,
    pub(super) affected: &'static [Action],
}

impl MutationCase {
    pub(super) const fn all() -> [Self; 10] {
        [
            Self::new(
                OperationalRecoveryModelFamily::BackupMaterialization,
                OperationalRecoveryControlledDefect::VerificationWithoutMaterialization,
                OperationalRecoveryInvariant::MaterializationBeforeVerification,
                Action::MaterializationRecorded,
                &[Action::IndependentVerificationRecorded],
            ),
            Self::new(
                OperationalRecoveryModelFamily::BackupReachability,
                OperationalRecoveryControlledDefect::MaterializationWithoutSourceLease,
                OperationalRecoveryInvariant::SourceLeaseBeforeMaterialization,
                Action::SourceLeasePersisted,
                &[Action::MaterializationOpened],
            ),
            Self::new(
                OperationalRecoveryModelFamily::Authorization,
                OperationalRecoveryControlledDefect::ExecutionWithoutAuthorization,
                OperationalRecoveryInvariant::AuthorizationBeforeExecution,
                Action::AuthorizationConsumed,
                &[
                    Action::OwnerExecutionOpened,
                    Action::ReplicaBootstrapTransferRecorded,
                    Action::ReplicaPromotionFenceRecorded,
                ],
            ),
            Self::new(
                OperationalRecoveryModelFamily::OwnerExecution,
                OperationalRecoveryControlledDefect::OwnerReceiptWithoutEffectStart,
                OperationalRecoveryInvariant::OwnerEffectBeforeReceipt,
                Action::OwnerEffectStarted,
                &[Action::OwnerReceiptPersisted],
            ),
            Self::new(
                OperationalRecoveryModelFamily::RecoveryStaging,
                OperationalRecoveryControlledDefect::StagingWithoutOwnerReceipts,
                OperationalRecoveryInvariant::CompleteOwnerReceiptsBeforeStaging,
                Action::WorkflowOwnerReceiptPersisted,
                &[Action::StagingCompleted],
            ),
            Self::new(
                OperationalRecoveryModelFamily::Publication,
                OperationalRecoveryControlledDefect::PublicationWithoutPreparation,
                OperationalRecoveryInvariant::PreparationBeforePublication,
                Action::PublicationPrepared,
                &[Action::PublicationPending],
            ),
            Self::new(
                OperationalRecoveryModelFamily::Promotion,
                OperationalRecoveryControlledDefect::PromotionWithoutExternalFence,
                OperationalRecoveryInvariant::ExternalFenceBeforePromotion,
                Action::ReplicaPromotionFenceRecorded,
                &[Action::ReplicaPromotionRecorded],
            ),
            Self::new(
                OperationalRecoveryModelFamily::ReplicaBootstrap,
                OperationalRecoveryControlledDefect::BootstrapCompletionWithoutTransfer,
                OperationalRecoveryInvariant::BootstrapTransferBeforeCompletion,
                Action::ReplicaBootstrapTransferRecorded,
                &[Action::ReplicaBootstrapCompleted],
            ),
            Self::new(
                OperationalRecoveryModelFamily::PromotionPublication,
                OperationalRecoveryControlledDefect::PromotionPublicationWithoutPromotion,
                OperationalRecoveryInvariant::PromotionBeforePublication,
                Action::ReplicaPromotionRecorded,
                &[Action::ReplicaPromotionPublished],
            ),
            Self::new(
                OperationalRecoveryModelFamily::OldPrimaryRejoin,
                OperationalRecoveryControlledDefect::RejoinCompletionWithoutPlan,
                OperationalRecoveryInvariant::RejoinPlanBeforeCompletion,
                Action::OldPrimaryRejoinPlanned,
                &[Action::OldPrimaryRejoinCompleted],
            ),
        ]
    }

    const fn new(
        family: OperationalRecoveryModelFamily,
        defect: OperationalRecoveryControlledDefect,
        expected: OperationalRecoveryInvariant,
        prerequisite: Action,
        affected: &'static [Action],
    ) -> Self {
        Self {
            family,
            defect,
            expected,
            prerequisite,
            affected,
        }
    }
}

#[derive(Clone, Copy)]
pub(super) struct ProductionEdge {
    pub(super) removed_index: usize,
    pub(super) affected_index: usize,
}

pub(super) fn locate_production_edge(
    case: MutationCase,
    actions: &[OperationalRecoveryAction],
) -> Option<ProductionEdge> {
    for (affected_index, affected) in actions.iter().enumerate() {
        if !case.affected.contains(&affected.kind()) {
            continue;
        }
        let removed_index = actions[..affected_index]
            .iter()
            .enumerate()
            .rev()
            .find(|(_, candidate)| {
                same_operation(candidate, affected) && candidate.kind() == case.prerequisite
            })
            .map(|(index, _)| index);
        if let Some(removed_index) = removed_index {
            return Some(ProductionEdge {
                removed_index,
                affected_index,
            });
        }
    }
    None
}

fn same_operation(left: &OperationalRecoveryAction, right: &OperationalRecoveryAction) -> bool {
    left.authority_identity() == right.authority_identity()
        && left.operation_identity() == right.operation_identity()
}

fn check_case(
    case: MutationCase,
    edge: ProductionEdge,
    records: &[OperationalControlRecord],
    source_refinement_identity: [u8; 32],
) -> Result<
    OperationalRecoveryMutationSensitivityReceipt,
    OperationalRecoveryMutationSensitivityDenial,
> {
    let removed = records[edge.removed_index].stable_fingerprint();
    let affected = records[edge.affected_index].stable_fingerprint();
    let mut defective = records.to_vec();
    defective.remove(edge.removed_index);
    let counterexample = check_operational_recovery_refinement(&defective, None)
        .err()
        .ok_or(
            OperationalRecoveryMutationSensitivityDenial::DefectiveHistoryAccepted(case.family),
        )?;
    if counterexample.invariant() != case.expected {
        return Err(
            OperationalRecoveryMutationSensitivityDenial::WrongInvariant {
                family: case.family,
                observed: counterexample.invariant(),
            },
        );
    }
    check_operational_recovery_refinement(&defective, Some(case.defect)).map_err(|_| {
        OperationalRecoveryMutationSensitivityDenial::DefectDidNotRemoveExpectedInvariant(
            case.family,
        )
    })?;
    let mut digest = Sha256::new();
    digest.update(b"worth-store-operational-recovery-mutation-receipt-v3");
    digest.update(model_family_label(case.family).as_bytes());
    digest.update(source_refinement_identity);
    digest.update(removed);
    digest.update(affected);
    digest.update(counterexample.transition_identity().as_bytes());
    Ok(OperationalRecoveryMutationSensitivityReceipt {
        family: case.family,
        defect: case.defect,
        localized_invariant: case.expected,
        localized_transition: counterexample.transition_identity().to_owned(),
        removed_production_artifact_identity: removed,
        affected_production_artifact_identity: affected,
        receipt_identity: digest.finalize().into(),
    })
}

const fn model_family_label(family: OperationalRecoveryModelFamily) -> &'static str {
    match family {
        OperationalRecoveryModelFamily::BackupMaterialization => "backup-materialization",
        OperationalRecoveryModelFamily::BackupReachability => "backup-reachability",
        OperationalRecoveryModelFamily::Authorization => "authorization",
        OperationalRecoveryModelFamily::OwnerExecution => "owner-execution",
        OperationalRecoveryModelFamily::RecoveryStaging => "recovery-staging",
        OperationalRecoveryModelFamily::Publication => "publication",
        OperationalRecoveryModelFamily::Promotion => "promotion",
        OperationalRecoveryModelFamily::ReplicaBootstrap => "replica-bootstrap",
        OperationalRecoveryModelFamily::PromotionPublication => "promotion-publication",
        OperationalRecoveryModelFamily::OldPrimaryRejoin => "old-primary-rejoin",
    }
}
