use sha2::{Digest, Sha256};
use worth_store_operations::certification_scenario::RejectedPoisonedBackupScenario;
use worth_store_operations::{
    ControlStoreSelectionIndeterminate, CurrentReplicaPromotion,
    DivergentControlGenerationSelectionReceipt,
};
use worth_store_replication::{
    ReplicaPromotionDenial, ReplicaPromotionRejectionReceipt, SplitBrainReconciliationReceipt,
};

use super::S10OperationalScenarioKind;

mod recovery_receipts;
mod repair;
pub use recovery_receipts::{
    PublishedReadmissionRecoveryReceipt, RevokedAuthorizationRecoveryReceipt,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S10HostileProgramEvidence {
    kind: S10OperationalScenarioKind,
    covered_requirements: u32,
    crash_coverage_identity: [u8; 32],
    evidence_identity: [u8; 32],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum S10HostileProgramRequirement {
    PoisonedBackupMultiFault,
    CrashRecoveredCutIdentity,
    CorruptControlSelectionDenial,
    FreshProcessDestroyedPrimaryVerification,
    AuthorizationRace,
    CrashResumableStaging,
    OutOfFootprintMutationRejection,
    PublishedReadmissionRecovery,
    ForensicCustodyExclusion,
    DivergentCandidateSelection,
    DeterministicNetworkPartition,
    IndependentSurvivorAcquisition,
    RevokedAuthorizationRecovery,
    OldPrimaryLeaseExpiryExclusion,
    DivergentControlGenerationSelection,
    ForensicOldPrimaryRejoin,
    RepairBreadth,
    BoundedRestartingOfflineScan,
    RepairSourceAuthorityDenials,
    CanonicalOwnerDagPermutation,
    CrashEveryOwnerEffect,
    RevocationCancellationRecovery,
    FootprintAndReceiptMutants,
    RetainedAuthorityRollback,
}

impl S10HostileProgramRequirement {
    const fn mask(self) -> u32 {
        1_u32 << self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S10HostileProgramDenial {
    PoisonedBackupNotRejected,
    PoisonedBackupLeaseReleased,
    PoisonedBackupFaultProgramIncomplete,
    CorruptControlStateWasNotRejected,
    HighestObservedCandidateNotDivergent,
    PromotionDecisionBasisMismatch,
    HighestObservedCandidateNotHigher,
    HighestObservedCandidateNotWorseAcknowledgedTruth,
    RepairBreadthBelowHundreds,
    StagingResumeProgramIncomplete,
    RepairSourceProgramIncomplete,
    OwnerDagProgramIncomplete,
    RepairRecoveryProgramIncomplete,
    RepairMutantProgramIncomplete,
    RetainedRollbackProgramIncomplete,
}

impl S10HostileProgramEvidence {
    #[allow(clippy::too_many_arguments)]
    pub fn burning_primary(
        rejected: &RejectedPoisonedBackupScenario,
        crash_cuts: &[worth_store_physical_certification::OperationalRecoveryCrashCutEvidence],
        corrupt_control: &ControlStoreSelectionIndeterminate,
        destroyed_primary: worth_store_physical_certification::FreshProcessDestroyedPrimaryEvidence,
        authorization_race: worth_store_operations::certification_scenario::ScenarioAuthorizationRaceReceipt,
        footprint_rejection: worth_store_operations::certification_scenario::ScenarioFootprintMutationRejectionReceipt,
        staging_resume: worth_store_operations::certification_scenario::ScenarioStagingResumeReceipt,
        published_readmission: PublishedReadmissionRecoveryReceipt,
        structural_preflight: super::S10StructuralPreflightEvidence,
    ) -> Result<Self, S10HostileProgramDenial> {
        if rejected.rejection_identity() == [0; 32] || rejected.omitted_artifact().is_empty() {
            return Err(S10HostileProgramDenial::PoisonedBackupNotRejected);
        }
        if rejected.torn_wal_artifact().is_empty()
            || rejected.substituted_index_artifact().is_empty()
            || rejected.independently_localized_defects() < 3
        {
            return Err(S10HostileProgramDenial::PoisonedBackupFaultProgramIncomplete);
        }
        if rejected.retained_source_leases() == 0 {
            return Err(S10HostileProgramDenial::PoisonedBackupLeaseReleased);
        }
        if !matches!(
            corrupt_control,
            ControlStoreSelectionIndeterminate::SelectedPrefixDigestMismatch { .. }
                | ControlStoreSelectionIndeterminate::InvalidHistory(_)
        ) {
            return Err(S10HostileProgramDenial::CorruptControlStateWasNotRejected);
        }
        let source_lease = worth_store_physical_certification::OperationalRecoveryControlTransitionKind::BackupSourceLease;
        if ![
            worth_store_physical_certification::OperationalRecoveryYieldpoint::BeforeDurableControlTransition(source_lease),
            worth_store_physical_certification::OperationalRecoveryYieldpoint::AfterDurableControlTransition(source_lease),
        ]
        .into_iter()
        .all(|point| {
            crash_cuts
                .iter()
                .any(|cut| cut.yieldpoint() == point && cut.evidence_identity() != [0; 32])
        })
        {
            return Err(S10HostileProgramDenial::PoisonedBackupFaultProgramIncomplete);
        }
        if staging_resume.recovered_boundaries() != 5
            || !required_control_crash_cuts(
                crash_cuts,
                &[
                    worth_store_physical_certification::OperationalRecoveryControlTransitionKind::AuthorizationConsumption,
                    worth_store_physical_certification::OperationalRecoveryControlTransitionKind::RecoveryOwnerReceipt,
                    worth_store_physical_certification::OperationalRecoveryControlTransitionKind::RecoveryStagingCompletion,
                    worth_store_physical_certification::OperationalRecoveryControlTransitionKind::RecoveryPublicationPreparation,
                ],
            )
        {
            return Err(S10HostileProgramDenial::StagingResumeProgramIncomplete);
        }
        let crash_coverage_identity = crash_coverage_identity(crash_cuts);
        let mut source = Sha256::new();
        source.update(rejected.rejection_identity());
        source.update(crash_coverage_identity);
        update_control_selection_denial(&mut source, corrupt_control);
        source.update(destroyed_primary.evidence_identity());
        source.update(authorization_race.evidence_identity());
        source.update(footprint_rejection.evidence_identity());
        source.update(staging_resume.evidence_identity());
        source.update(published_readmission.evidence_identity());
        source.update(structural_preflight.reverse_flow_compile_identity());
        Ok(Self::bind(
            S10OperationalScenarioKind::BurningPrimary,
            source.finalize().into(),
            rejected
                .retained_source_leases()
                .saturating_add(rejected.independently_localized_defects()),
            S10HostileProgramRequirement::PoisonedBackupMultiFault.mask()
                | S10HostileProgramRequirement::CrashRecoveredCutIdentity.mask()
                | S10HostileProgramRequirement::CorruptControlSelectionDenial.mask()
                | S10HostileProgramRequirement::FreshProcessDestroyedPrimaryVerification.mask()
                | S10HostileProgramRequirement::AuthorizationRace.mask()
                | S10HostileProgramRequirement::CrashResumableStaging.mask()
                | S10HostileProgramRequirement::OutOfFootprintMutationRejection.mask()
                | S10HostileProgramRequirement::PublishedReadmissionRecovery.mask()
                | S10HostileProgramRequirement::ForensicCustodyExclusion.mask(),
            crash_coverage_identity,
        ))
    }

    pub fn split_brain(
        rejected: &ReplicaPromotionRejectionReceipt,
        admitted: &CurrentReplicaPromotion,
        reconciliation: SplitBrainReconciliationReceipt,
        divergent_control: DivergentControlGenerationSelectionReceipt,
        revoked_authorization: RevokedAuthorizationRecoveryReceipt,
    ) -> Result<Self, S10HostileProgramDenial> {
        if rejected.denial() != ReplicaPromotionDenial::DivergentHistory {
            return Err(S10HostileProgramDenial::HighestObservedCandidateNotDivergent);
        }
        let rejected_frontier = rejected.candidate_frontier();
        let admitted_frontier = admitted.promotion_receipt().promoted_frontier();
        if rejected.current_frontier() != admitted_frontier {
            return Err(S10HostileProgramDenial::PromotionDecisionBasisMismatch);
        }
        if rejected_frontier.observed_lsn() <= admitted_frontier.observed_lsn() {
            return Err(S10HostileProgramDenial::HighestObservedCandidateNotHigher);
        }
        if rejected_frontier.client_acknowledged_lsn()
            >= admitted_frontier.client_acknowledged_lsn()
        {
            return Err(S10HostileProgramDenial::HighestObservedCandidateNotWorseAcknowledgedTruth);
        }
        let mut source = Sha256::new();
        source.update(rejected.receipt_identity());
        source.update(reconciliation.receipt_identity());
        source.update(divergent_control.receipt_identity());
        source.update(revoked_authorization.evidence_identity());
        Ok(Self::bind(
            S10OperationalScenarioKind::SplitBrainPromotion,
            source.finalize().into(),
            rejected_frontier
                .observed_lsn()
                .saturating_add(reconciliation.independent_survivors()),
            S10HostileProgramRequirement::DivergentCandidateSelection.mask()
                | S10HostileProgramRequirement::DeterministicNetworkPartition.mask()
                | S10HostileProgramRequirement::IndependentSurvivorAcquisition.mask()
                | S10HostileProgramRequirement::RevokedAuthorizationRecovery.mask()
                | S10HostileProgramRequirement::OldPrimaryLeaseExpiryExclusion.mask()
                | S10HostileProgramRequirement::ForensicOldPrimaryRejoin.mask()
                | S10HostileProgramRequirement::DivergentControlGenerationSelection.mask(),
            [0; 32],
        ))
    }

    fn bind(
        kind: S10OperationalScenarioKind,
        source_identity: [u8; 32],
        cardinality: u64,
        covered_requirements: u32,
        crash_coverage_identity: [u8; 32],
    ) -> Self {
        let mut digest = Sha256::new();
        digest.update(b"worth-store-s10-hostile-program-evidence-v2");
        digest.update(kind.token().as_bytes());
        digest.update(source_identity);
        digest.update(cardinality.to_be_bytes());
        digest.update(covered_requirements.to_be_bytes());
        digest.update(crash_coverage_identity);
        Self {
            kind,
            covered_requirements,
            crash_coverage_identity,
            evidence_identity: digest.finalize().into(),
        }
    }

    pub const fn kind(self) -> S10OperationalScenarioKind {
        self.kind
    }

    pub const fn evidence_identity(self) -> [u8; 32] {
        self.evidence_identity
    }

    pub(crate) fn matches_crash_coverage(
        self,
        crash_cuts: &[worth_store_physical_certification::OperationalRecoveryCrashCutEvidence],
    ) -> bool {
        self.kind != S10OperationalScenarioKind::BurningPrimary
            || self.crash_coverage_identity == crash_coverage_identity(crash_cuts)
    }

    pub fn missing_requirement(self) -> Option<S10HostileProgramRequirement> {
        required_requirements(self.kind)
            .iter()
            .copied()
            .find(|requirement| self.covered_requirements & requirement.mask() == 0)
    }
}

fn update_control_selection_denial(
    digest: &mut Sha256,
    denial: &ControlStoreSelectionIndeterminate,
) {
    match denial {
        ControlStoreSelectionIndeterminate::SelectedPrefixDigestMismatch { selected, observed } => {
            digest.update(b"selected-prefix-digest-mismatch");
            digest.update(selected);
            digest.update(observed);
        }
        ControlStoreSelectionIndeterminate::InvalidHistory(violation) => {
            digest.update(b"invalid-selected-control-history");
            digest.update(violation.operation_id().as_str().as_bytes());
            digest.update(violation.record_index().to_be_bytes());
        }
        _ => unreachable!("caller validated corrupt-control denial kind"),
    }
}

fn crash_coverage_identity(
    crash_cuts: &[worth_store_physical_certification::OperationalRecoveryCrashCutEvidence],
) -> [u8; 32] {
    let mut cuts = crash_cuts.iter().collect::<Vec<_>>();
    cuts.sort_by_key(|cut| cut.yieldpoint());
    let mut digest = Sha256::new();
    digest.update(b"worth-store-s10-hostile-crash-coverage-v1");
    for cut in cuts {
        digest.update(cut.yieldpoint().token().as_bytes());
        digest.update(cut.evidence_identity());
    }
    digest.finalize().into()
}

fn required_control_crash_cuts(
    crash_cuts: &[worth_store_physical_certification::OperationalRecoveryCrashCutEvidence],
    transitions: &[worth_store_physical_certification::OperationalRecoveryControlTransitionKind],
) -> bool {
    transitions.iter().all(|transition| {
        [
            worth_store_physical_certification::OperationalRecoveryYieldpoint::BeforeDurableControlTransition(*transition),
            worth_store_physical_certification::OperationalRecoveryYieldpoint::AfterDurableControlTransition(*transition),
        ]
        .into_iter()
        .all(|point| crash_cuts.iter().any(|cut| cut.yieldpoint() == point))
    })
}

fn required_requirements(
    kind: S10OperationalScenarioKind,
) -> &'static [S10HostileProgramRequirement] {
    use S10HostileProgramRequirement as Requirement;
    match kind {
        S10OperationalScenarioKind::BurningPrimary => &[
            Requirement::PoisonedBackupMultiFault,
            Requirement::CrashRecoveredCutIdentity,
            Requirement::CorruptControlSelectionDenial,
            Requirement::FreshProcessDestroyedPrimaryVerification,
            Requirement::AuthorizationRace,
            Requirement::CrashResumableStaging,
            Requirement::OutOfFootprintMutationRejection,
            Requirement::PublishedReadmissionRecovery,
            Requirement::ForensicCustodyExclusion,
        ],
        S10OperationalScenarioKind::SplitBrainPromotion => &[
            Requirement::DivergentCandidateSelection,
            Requirement::DeterministicNetworkPartition,
            Requirement::IndependentSurvivorAcquisition,
            Requirement::RevokedAuthorizationRecovery,
            Requirement::OldPrimaryLeaseExpiryExclusion,
            Requirement::DivergentControlGenerationSelection,
            Requirement::ForensicOldPrimaryRejoin,
        ],
        S10OperationalScenarioKind::AuthorityRepairRollback => &[
            Requirement::RepairBreadth,
            Requirement::BoundedRestartingOfflineScan,
            Requirement::RepairSourceAuthorityDenials,
            Requirement::CanonicalOwnerDagPermutation,
            Requirement::CrashEveryOwnerEffect,
            Requirement::RevocationCancellationRecovery,
            Requirement::FootprintAndReceiptMutants,
            Requirement::RetainedAuthorityRollback,
        ],
    }
}
