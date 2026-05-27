use crate::application::{ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker};
use crate::continuation_pipeline::{
    ordinary_outcome_from_continuation_checked, ordinary_outcome_from_execution_checked,
    ForgeQueryContinuationExecutionChecked, ForgeQueryContinuationExecutionTranscript,
    ForgeQueryPreparedContinuationChecked, ForgeQueryPreparedContinuationTranscript,
};
use crate::contribution_composed_orchestration::{
    ordinary_outcome_from_contribution_composed_checked,
    ForgeQueryContributionComposedOrchestrationChecked,
    ForgeQueryContributionComposedOrchestrationTranscript,
};
use crate::signal_compatibility_orchestration::{
    ordinary_outcome_from_signal_compatibility_orchestration_checked,
    ForgeQuerySignalCompatibilityOrchestrationChecked,
    ForgeQuerySignalCompatibilityOrchestrationTranscript,
};

use super::brief::ForgeQueryRecoveryBrief;
use super::ordinary::forge_query_recovery_brief_from_ordinary_outcome;

pub fn forge_query_recovery_brief_from_prepared_continuation_checked<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    checked: ForgeQueryPreparedContinuationChecked<D, I>,
) -> Option<ForgeQueryRecoveryBrief> {
    forge_query_recovery_brief_from_ordinary_outcome(&ordinary_outcome_from_continuation_checked(
        checked,
    ))
}

pub fn forge_query_recovery_brief_from_prepared_continuation_proof<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    proof: ForgeQueryPreparedContinuationTranscript<D, I>,
) -> Option<ForgeQueryRecoveryBrief> {
    forge_query_recovery_brief_from_prepared_continuation_checked(proof.into_checked())
}

pub fn forge_query_recovery_brief_from_continuation_execution_checked<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    checked: ForgeQueryContinuationExecutionChecked<D, I>,
) -> Option<ForgeQueryRecoveryBrief> {
    forge_query_recovery_brief_from_ordinary_outcome(&ordinary_outcome_from_execution_checked(
        checked,
    ))
}

pub fn forge_query_recovery_brief_from_continuation_execution_proof<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    proof: ForgeQueryContinuationExecutionTranscript<D, I>,
) -> Option<ForgeQueryRecoveryBrief> {
    forge_query_recovery_brief_from_continuation_execution_checked(proof.into_checked())
}

pub fn forge_query_recovery_brief_from_signal_compatibility_checked<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    checked: ForgeQuerySignalCompatibilityOrchestrationChecked<D, I>,
) -> Option<ForgeQueryRecoveryBrief> {
    forge_query_recovery_brief_from_ordinary_outcome(
        &ordinary_outcome_from_signal_compatibility_orchestration_checked(checked),
    )
}

pub fn forge_query_recovery_brief_from_signal_compatibility_proof<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    proof: ForgeQuerySignalCompatibilityOrchestrationTranscript<D, I>,
) -> Option<ForgeQueryRecoveryBrief> {
    forge_query_recovery_brief_from_signal_compatibility_checked(proof.into_checked())
}

pub fn forge_query_recovery_brief_from_contribution_composed_checked<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    checked: ForgeQueryContributionComposedOrchestrationChecked<D, I>,
) -> Option<ForgeQueryRecoveryBrief> {
    forge_query_recovery_brief_from_ordinary_outcome(
        &ordinary_outcome_from_contribution_composed_checked(checked),
    )
}

pub fn forge_query_recovery_brief_from_contribution_composed_proof<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    proof: ForgeQueryContributionComposedOrchestrationTranscript<D, I>,
) -> Option<ForgeQueryRecoveryBrief> {
    forge_query_recovery_brief_from_contribution_composed_checked(proof.into_checked())
}
