use worth_store_layout_indexes::{
    BaselineLsmExecutionAdmissionDenialKind as OwnerDenial,
    LsmExecutionDisposition as OwnerDisposition, LsmExecutionOperation as OwnerOperation,
    LsmExecutionOwnerCaseId,
};

use crate::protocols::compaction_visibility::{
    CompactionVisibilityAction, LsmExecutionAction, LsmExecutionDenial, ModeledOutcome,
};

pub(super) fn expected_action(owner_case: LsmExecutionOwnerCaseId) -> CompactionVisibilityAction {
    let operation = match owner_case.operation() {
        OwnerOperation::PrepareCompaction => LsmExecutionAction::PrepareCompaction,
        OwnerOperation::BindPhysicalCompaction => LsmExecutionAction::BindPhysicalCompaction,
        OwnerOperation::PrepareMembershipActivation => {
            LsmExecutionAction::PrepareMembershipActivation
        }
        OwnerOperation::PublishCompaction => LsmExecutionAction::PublishCompaction,
        OwnerOperation::ExecuteReplay => LsmExecutionAction::ExecuteReplay,
    };
    let outcome = match owner_case.disposition() {
        OwnerDisposition::Admitted => ModeledOutcome::Admitted,
        OwnerDisposition::Denied(denial) => ModeledOutcome::Denied(expected_denial(denial)),
    };
    CompactionVisibilityAction::LsmExecution { operation, outcome }
}

const fn expected_denial(denial: OwnerDenial) -> LsmExecutionDenial {
    match denial {
        OwnerDenial::StrategyInvariant => LsmExecutionDenial::StrategyInvariant,
        OwnerDenial::CanonicalKeyRequired => LsmExecutionDenial::CanonicalKeyRequired,
        OwnerDenial::MemtableDoesNotFollowSortedRuns => {
            LsmExecutionDenial::MemtableDoesNotFollowSortedRuns
        }
        OwnerDenial::SortedRunsNotCanonical => LsmExecutionDenial::SortedRunsNotCanonical,
        OwnerDenial::ReplayTailNotCanonical => LsmExecutionDenial::ReplayTailNotCanonical,
        OwnerDenial::ReplayBindingMismatch => LsmExecutionDenial::ReplayBindingMismatch,
        OwnerDenial::TombstoneRecordRequired => LsmExecutionDenial::TombstoneRecordRequired,
        OwnerDenial::ValueRecordRequired => LsmExecutionDenial::ValueRecordRequired,
        OwnerDenial::GenerationRecordRequired => LsmExecutionDenial::GenerationRecordRequired,
        OwnerDenial::OutputGenerationOverflow => LsmExecutionDenial::OutputGenerationOverflow,
        OwnerDenial::OutputPublicationMismatch => LsmExecutionDenial::OutputPublicationMismatch,
        OwnerDenial::ManifestPublicationRequired => LsmExecutionDenial::ManifestPublicationRequired,
        OwnerDenial::ManifestDoesNotCoverCompaction => {
            LsmExecutionDenial::ManifestDoesNotCoverCompaction
        }
        OwnerDenial::ManifestMembershipMismatch => LsmExecutionDenial::ManifestMembershipMismatch,
        OwnerDenial::PersistedMembershipAmbiguous => {
            LsmExecutionDenial::PersistedMembershipAmbiguous
        }
        OwnerDenial::PersistedMembershipIncomplete => {
            LsmExecutionDenial::PersistedMembershipIncomplete
        }
        OwnerDenial::PersistedMembershipStale => LsmExecutionDenial::PersistedMembershipStale,
        OwnerDenial::PersistedArtifactInvalid => LsmExecutionDenial::PersistedArtifactInvalid,
        OwnerDenial::PersistedIndexIo => LsmExecutionDenial::PersistedIndexIo,
        OwnerDenial::PhysicalTargetEpochRequired => LsmExecutionDenial::PhysicalTargetEpochRequired,
        OwnerDenial::DurableRecordBindingMismatch => {
            LsmExecutionDenial::DurableRecordBindingMismatch
        }
        OwnerDenial::RecordKeyScopeMismatch => LsmExecutionDenial::RecordKeyScopeMismatch,
        OwnerDenial::PhysicalPublicationBindingMismatch => {
            LsmExecutionDenial::PhysicalPublicationBindingMismatch
        }
        OwnerDenial::SelectedOperationKeyMismatch => {
            LsmExecutionDenial::SelectedOperationKeyMismatch
        }
    }
}
