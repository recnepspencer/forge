use std::path::Path;

use crate::{
    CompactionVisibilityAction, DurabilityRecoveryAction, ImportPublicationAction,
    LeaseReclaimAction, LsmExecutionAction, LsmMaintenanceAction, LsmMembershipAction,
    ModeledOutcome, QuarantineReadmissionState, ReplicationAdmissionAction, SharedFrontierAction,
    SourcePrecedenceAction,
};

use super::{CanonicalProtocolAction, ProtocolCheckInvocation};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CheckedOperatorBinding {
    operator: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckedOperatorBindingDenial {
    ModelRead(String),
    OperatorNotDeclared { operator: &'static str },
}

pub fn require_checked_operator_bindings(
    invocation: &ProtocolCheckInvocation,
    actions: &[CanonicalProtocolAction],
) -> Result<Vec<CheckedOperatorBinding>, CheckedOperatorBindingDenial> {
    let model = read_model(invocation.model_path())?;
    actions
        .iter()
        .map(|action| {
            let binding = checked_operator_for_action(action);
            if operator_is_declared(&model, binding.operator) {
                Ok(binding)
            } else {
                Err(CheckedOperatorBindingDenial::OperatorNotDeclared {
                    operator: binding.operator,
                })
            }
        })
        .collect()
}

fn read_model(path: &Path) -> Result<String, CheckedOperatorBindingDenial> {
    std::fs::read_to_string(path)
        .map_err(|error| CheckedOperatorBindingDenial::ModelRead(error.to_string()))
}

fn operator_is_declared(model: &str, operator: &str) -> bool {
    model.lines().any(|line| {
        let line = line.trim_start();
        line.starts_with(&format!("{operator} ==")) || line.starts_with(&format!("{operator}("))
    })
}

fn checked_operator_for_action(action: &CanonicalProtocolAction) -> CheckedOperatorBinding {
    let operator = match action {
        CanonicalProtocolAction::DurabilityRecovery(action) => durability_operator(*action),
        CanonicalProtocolAction::RecoverySourcePrecedence(action) => source_operator(*action),
        CanonicalProtocolAction::CompactionVisibility(action) => compaction_operator(*action),
        CanonicalProtocolAction::LeaseReclaim(action) => lease_operator(*action),
        CanonicalProtocolAction::QuarantineReadmission(state) => quarantine_operator(*state),
        CanonicalProtocolAction::ImportPublication(action) => import_operator(*action),
        CanonicalProtocolAction::ReplicationAdmission(action) => replication_operator(*action),
        CanonicalProtocolAction::SharedFrontier(action) => shared_operator(*action),
    };
    CheckedOperatorBinding { operator }
}

fn durability_operator(action: DurabilityRecoveryAction) -> &'static str {
    match action {
        DurabilityRecoveryAction::WalAppendProposed => "WalPropose",
        DurabilityRecoveryAction::WalAppendCompletedInMemory => "WalWrite",
        DurabilityRecoveryAction::WalFenceRequested => "WalFenceRequest",
        DurabilityRecoveryAction::WalFenceCompleted => "WalFenceComplete",
        DurabilityRecoveryAction::WalAcknowledgmentLegal => "WalAcknowledge",
        DurabilityRecoveryAction::PageFlushRequested => "PageRequest",
        DurabilityRecoveryAction::PageFlushCompleted => "PageComplete",
        DurabilityRecoveryAction::PageFlushDurabilityUncertain => "PageUncertain",
        DurabilityRecoveryAction::CheckpointBegun => "CheckpointBegin",
        DurabilityRecoveryAction::CheckpointDurable => "CheckpointDurable",
        DurabilityRecoveryAction::DirectorySyncCompleted => "DirectorySyncComplete",
        DurabilityRecoveryAction::DirectorySyncFailed => "DirectorySyncFail",
        DurabilityRecoveryAction::CheckpointPublished => "CheckpointPublish",
        DurabilityRecoveryAction::CheckpointSelected => "CheckpointSelect",
        DurabilityRecoveryAction::RecoveryReplayRequired => "ReplayRequire",
        DurabilityRecoveryAction::RecoveryReplayApplied => "ReplayApply",
        DurabilityRecoveryAction::RecoveryReplayRejectedGenerationMismatch => {
            "ReplayRejectGeneration"
        }
        DurabilityRecoveryAction::RecoveryReplaySkippedIdempotent => "ReplaySkip",
        DurabilityRecoveryAction::RecoveredRootPublicationPending => "RootPending",
        DurabilityRecoveryAction::RecoveredRootPublicationCompleted => "RootComplete",
        DurabilityRecoveryAction::Crash => "Crash",
        DurabilityRecoveryAction::Reopen => "Reopen",
    }
}

fn source_operator(action: SourcePrecedenceAction) -> &'static str {
    match action {
        SourcePrecedenceAction::CandidateDiscovered { .. } => "Discover",
        SourcePrecedenceAction::CandidateAdmitted { .. } => "Admit",
        SourcePrecedenceAction::CandidateAdvisoryOnly { .. } => "Advise",
        SourcePrecedenceAction::CandidateRejected { .. } => "Reject",
        SourcePrecedenceAction::ContradictionPreserved => "PreserveContradiction",
        SourcePrecedenceAction::SourceSelected => "Select",
        SourcePrecedenceAction::SourceQuarantined => "Quarantine",
        SourcePrecedenceAction::SourceDenied => "Deny",
    }
}

fn compaction_operator(action: CompactionVisibilityAction) -> &'static str {
    match action {
        CompactionVisibilityAction::LsmMembership {
            outcome: ModeledOutcome::Denied(_),
            ..
        }
        | CompactionVisibilityAction::LsmExecution {
            outcome: ModeledOutcome::Denied(_),
            ..
        }
        | CompactionVisibilityAction::LsmMaintenance {
            outcome: ModeledOutcome::Denied(_),
            ..
        } => "DenyLsmOwnerCase",
        CompactionVisibilityAction::LsmMembership { operation, .. } => match operation {
            LsmMembershipAction::Open | LsmMembershipAction::SelectCompaction => "Plan",
            LsmMembershipAction::PersistRecord => "Durable",
            LsmMembershipAction::ReplaceMembership => "AttemptPublish",
            LsmMembershipAction::LookupPublishedReplacement => "Publish",
        },
        CompactionVisibilityAction::LsmExecution { operation, .. } => match operation {
            LsmExecutionAction::PrepareCompaction => "Plan",
            LsmExecutionAction::BindPhysicalCompaction => "Write",
            LsmExecutionAction::PrepareMembershipActivation => "Durable",
            LsmExecutionAction::PublishCompaction => "Publish",
            LsmExecutionAction::ExecuteReplay => "Retry",
        },
        CompactionVisibilityAction::LsmMaintenance { operation, .. } => match operation {
            LsmMaintenanceAction::AdmitRunPublication => "AttemptPublish",
            LsmMaintenanceAction::AdmitReplay => "Retry",
            LsmMaintenanceAction::AdmitCompaction => "Plan",
        },
        CompactionVisibilityAction::LowerRewrite => "LowerRewrite",
        CompactionVisibilityAction::PublishRewrite => "PublishRewrite",
        CompactionVisibilityAction::AdmitRecoveryVisibility => "AdmitRecoveryVisibility",
        CompactionVisibilityAction::DeferReclaim => "DeferReclaim",
        CompactionVisibilityAction::DrainReclaimAfterReadRelease => "DrainReclaimAfterReadRelease",
        CompactionVisibilityAction::DenyInPlaceOverwrite => "DenyInPlaceOverwrite",
        CompactionVisibilityAction::DenyEarlyReclaim => "DenyEarlyReclaim",
        CompactionVisibilityAction::DenyStaleEpochReuse => "DenyStaleEpochReuse",
        CompactionVisibilityAction::DenyBackendResidueCandidateSelection => {
            "DenyBackendResidueCandidateSelection"
        }
        CompactionVisibilityAction::DenyLatchHierarchyInversion => "DenyLatchHierarchyInversion",
        CompactionVisibilityAction::DenyMixedRootRead => "DenyMixedRootRead",
    }
}

fn lease_operator(action: LeaseReclaimAction) -> &'static str {
    match action {
        LeaseReclaimAction::LeaseAcquired { .. } => "Acquire",
        LeaseReclaimAction::LeaseReleased { .. } => "Release",
        LeaseReclaimAction::LeaseRevoked { .. } => "Revoke",
        LeaseReclaimAction::LeaseExpiredWithoutAuthority { .. } => "Expire",
        LeaseReclaimAction::OwnedCopyStabilized { .. } => "OwnedCopy",
        LeaseReclaimAction::ReclaimAdmitted => "Reclaim",
        LeaseReclaimAction::ReclaimDeniedByLiveLease => "DenyReclaim",
        LeaseReclaimAction::IdentityReuseAdmitted { .. } => "Reuse",
        LeaseReclaimAction::IdentityReuseDenied => "DenyReuse",
    }
}

fn quarantine_operator(state: QuarantineReadmissionState) -> &'static str {
    match state {
        QuarantineReadmissionState::Proposed => "ObserveProposed",
        QuarantineReadmissionState::Sealed => "Seal",
        QuarantineReadmissionState::RecoveryVerificationPending => "BeginVerification",
        QuarantineReadmissionState::Readmitted => "Readmit",
        QuarantineReadmissionState::RetainedForAudit => "RetainAudit",
        QuarantineReadmissionState::Denied => "DenyReadmission",
    }
}

fn import_operator(action: ImportPublicationAction) -> &'static str {
    match action {
        ImportPublicationAction::RawDeclarationObserved => "ObserveRaw",
        ImportPublicationAction::CurrentScopeReadmitted => "Readmit",
        ImportPublicationAction::RecoveredArtifactAdmitted => "AdmitArtifact",
        ImportPublicationAction::LayoutMaterializationAdmitted => "Materialize",
        ImportPublicationAction::PublicationPending => "Ready",
        ImportPublicationAction::PublicationDurable => "Publish",
        ImportPublicationAction::CrashBeforePublication => "CrashBeforePublication",
        ImportPublicationAction::PublicationDenied => "RejectPublication",
    }
}

fn replication_operator(action: ReplicationAdmissionAction) -> &'static str {
    match action {
        ReplicationAdmissionAction::SourceAdmitted => "SourceAdmitted",
        ReplicationAdmissionAction::SourcePeerIdentityDenied
        | ReplicationAdmissionAction::SourceEpochRequiredDenied
        | ReplicationAdmissionAction::SourceLineageIdentityDenied
        | ReplicationAdmissionAction::SourceCurrentAuthorityDenied
        | ReplicationAdmissionAction::SourceReplayIdentityDenied => "SourceDenied",
        ReplicationAdmissionAction::FreshProgressObserved => "FreshProgressObserved",
        ReplicationAdmissionAction::ResumeProgressObserved => "ResumeProgressObserved",
        ReplicationAdmissionAction::DuplicateObserved => "DuplicateObserved",
        ReplicationAdmissionAction::ResumeCurrentAuthorityDenied => "ResumeCurrentAuthorityDenied",
        ReplicationAdmissionAction::SourceEpochDivergenceDetected => {
            "SourceEpochDivergenceDetected"
        }
        ReplicationAdmissionAction::LineageDivergenceDetected => "LineageDivergenceDetected",
        ReplicationAdmissionAction::ReplayOverlapDivergenceDetected => {
            "ReplayOverlapDivergenceDetected"
        }
        ReplicationAdmissionAction::ResumeProgressGapDenied => "ResumeProgressGapDenied",
        ReplicationAdmissionAction::FreshPublicationPending
        | ReplicationAdmissionAction::ResumePublicationPending => "PublicationPending",
        ReplicationAdmissionAction::FreshPublicationDurable
        | ReplicationAdmissionAction::ResumePublicationDurable => "PublicationDurable",
        ReplicationAdmissionAction::PublicationCurrentAuthorityDenied => {
            "PublicationCurrentAuthorityDenied"
        }
        ReplicationAdmissionAction::PublicationPeerProgressChangedDenied => {
            "PublicationPeerProgressChangedDenied"
        }
        ReplicationAdmissionAction::PublicationPeerCapacityDenied
        | ReplicationAdmissionAction::PublicationProgressStoreDenied => "PublicationStorageDenied",
    }
}

fn shared_operator(action: SharedFrontierAction) -> &'static str {
    match action {
        SharedFrontierAction::DurabilityAdmitted => "DurabilityAdmitted",
        SharedFrontierAction::RecoveryPrecedencePreserved => "RecoveryPrecedencePreserved",
        SharedFrontierAction::LiveLeaseAcquired => "LiveLeaseAcquired",
        SharedFrontierAction::LeaseReleased => "LeaseReleased",
        SharedFrontierAction::CompactionCutover => "CompactionCutover",
        SharedFrontierAction::Crash => "Crash",
        SharedFrontierAction::Reopen => "Reopen",
        SharedFrontierAction::QuarantineSealed => "QuarantineSealed",
        SharedFrontierAction::QuarantineVerificationStarted => "QuarantineVerificationStarted",
        SharedFrontierAction::QuarantineReadmitted => "QuarantineReadmitted",
        SharedFrontierAction::ReclaimDeferred => "ReclaimDeferred",
        SharedFrontierAction::ReclaimReleased => "ReclaimReleased",
        SharedFrontierAction::GenerationReused => "GenerationReused",
        SharedFrontierAction::CheckpointPublicationRequested => "CheckpointPublicationRequested",
        SharedFrontierAction::ImportAdmissionPending => "ImportAdmissionPending",
        SharedFrontierAction::ReplicationAdmissionPending => "ReplicationAdmissionPending",
        SharedFrontierAction::ExternalDurabilityAdmitted => "ExternalDurabilityAdmitted",
        SharedFrontierAction::ExternalPublicationRequested => "ExternalPublicationRequested",
        SharedFrontierAction::ReplicationDivergenceDetected => "ReplicationDivergenceDetected",
    }
}

impl CheckedOperatorBinding {
    pub const fn operator(self) -> &'static str {
        self.operator
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU64;

    use crate::ProtocolFamily;

    use super::*;
    use crate::runner::ProtocolCheckBounds;

    #[test]
    fn missing_checked_operator_is_a_typed_closeout_failure() {
        let root = std::env::temp_dir().join(format!(
            "worth-store-missing-operator-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&root).unwrap();
        let model = root.join("missing.tla");
        std::fs::write(&model, "---- MODULE Missing ----\nWalWrite == TRUE\n====\n").unwrap();
        let invocation = ProtocolCheckInvocation::for_controlled_defect(
            ProtocolFamily::DurabilityRecovery,
            &model,
            root.join("missing.cfg"),
            ProtocolCheckBounds::new(
                NonZeroU64::new(10).unwrap(),
                NonZeroU64::new(4).unwrap(),
            ),
        );
        let denial = require_checked_operator_bindings(
            &invocation,
            &[CanonicalProtocolAction::DurabilityRecovery(
                DurabilityRecoveryAction::WalAppendProposed,
            )],
        )
        .unwrap_err();
        assert_eq!(
            denial,
            CheckedOperatorBindingDenial::OperatorNotDeclared {
                operator: "WalPropose"
            }
        );
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn physical_publication_and_recovery_visibility_bind_distinct_operators() {
        assert_eq!(
            checked_operator_for_action(&CanonicalProtocolAction::CompactionVisibility(
                CompactionVisibilityAction::PublishRewrite,
            ))
            .operator(),
            "PublishRewrite"
        );
        assert_eq!(
            checked_operator_for_action(&CanonicalProtocolAction::CompactionVisibility(
                CompactionVisibilityAction::AdmitRecoveryVisibility,
            ))
            .operator(),
            "AdmitRecoveryVisibility"
        );
    }
}
