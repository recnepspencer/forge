use worth_store_layout_indexes::{
    BaselineLsmExecutionAdmissionDenialKind as OwnerDenial,
    LsmExecutionDisposition as OwnerDisposition, LsmExecutionOperation as OwnerOperation,
    LsmExecutionOwnerCaseId, LsmExecutionOwnerCaseObservation,
};

use super::action::{
    CompactionVisibilityAction, LsmExecutionAction, LsmExecutionDenial, ModeledOutcome,
};
use crate::protocol_bindings::{
    CompactionVisibilityMappedOwnerCase, CompactionVisibilityOwnerCase,
};

pub fn map_lsm_execution_observation(
    observation: LsmExecutionOwnerCaseObservation,
) -> CompactionVisibilityMappedOwnerCase {
    map_lsm_execution_case(observation.id())
}

pub(crate) fn map_lsm_execution_case(
    owner_case: LsmExecutionOwnerCaseId,
) -> CompactionVisibilityMappedOwnerCase {
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
        OwnerDisposition::Denied(denial) => ModeledOutcome::Denied(map_denial(denial)),
    };
    CompactionVisibilityMappedOwnerCase::new(
        CompactionVisibilityOwnerCase::LsmExecution(owner_case),
        CompactionVisibilityAction::LsmExecution { operation, outcome },
    )
}

const fn map_denial(denial: OwnerDenial) -> LsmExecutionDenial {
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
