use crate::application::{
    ForgeQueryDeclarationEntryOrchestrationChecked, ForgeQueryDeclarationEntryOrchestrationProof,
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
};
use crate::continuation_pipeline::{
    ForgeQueryContinuationExecutionChecked, ForgeQueryContinuationExecutionTranscript,
    ForgeQueryPreparedContinuationChecked, ForgeQueryPreparedContinuationTranscript,
};
use crate::contribution_composed_orchestration::{
    ForgeQueryContributionComposedOrchestrationChecked,
    ForgeQueryContributionComposedOrchestrationTranscript,
};
use crate::grouped_authoring::{
    ForgeQueryGroupedOrchestrationChecked, ForgeQueryGroupedOrchestrationTranscript,
};
use crate::ordinary_outcome::ForgeQueryOrdinaryOutcome;
use crate::recovery_boundary::{
    forge_query_recovery_brief_from_continuation_execution_checked,
    forge_query_recovery_brief_from_continuation_execution_proof,
    forge_query_recovery_brief_from_contribution_composed_checked,
    forge_query_recovery_brief_from_contribution_composed_proof,
    forge_query_recovery_brief_from_declaration_entry_checked,
    forge_query_recovery_brief_from_declaration_entry_proof,
    forge_query_recovery_brief_from_declaration_receipt_checked,
    forge_query_recovery_brief_from_declaration_route_plan_checked,
    forge_query_recovery_brief_from_grouped_orchestration_checked,
    forge_query_recovery_brief_from_grouped_orchestration_proof,
    forge_query_recovery_brief_from_ordinary_outcome,
    forge_query_recovery_brief_from_prepared_continuation_checked,
    forge_query_recovery_brief_from_prepared_continuation_proof,
    forge_query_recovery_brief_from_signal_compatibility_checked,
    forge_query_recovery_brief_from_signal_compatibility_proof, ForgeQueryRecoveryBrief,
};
use crate::signal_compatibility_orchestration::{
    ForgeQuerySignalCompatibilityOrchestrationChecked,
    ForgeQuerySignalCompatibilityOrchestrationTranscript,
};

use super::ForgeQueryAdmittedConfiguredDomainHandle;

impl<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>>
    ForgeQueryAdmittedConfiguredDomainHandle<D, C>
{
    pub fn recover_from_outcome<T>(
        &self,
        outcome: &ForgeQueryOrdinaryOutcome<T>,
    ) -> Option<ForgeQueryRecoveryBrief> {
        forge_query_recovery_brief_from_ordinary_outcome(outcome)
    }

    pub fn recover_from_declaration_entry_checked<I: ForgeQueryDeclarationInput<D>>(
        &self,
        checked: ForgeQueryDeclarationEntryOrchestrationChecked<D, I>,
    ) -> Option<ForgeQueryRecoveryBrief> {
        forge_query_recovery_brief_from_declaration_entry_checked(checked)
    }

    pub fn recover_from_declaration_entry_proof<I: ForgeQueryDeclarationInput<D>>(
        &self,
        proof: &ForgeQueryDeclarationEntryOrchestrationProof<D, I>,
    ) -> Option<ForgeQueryRecoveryBrief> {
        forge_query_recovery_brief_from_declaration_entry_proof(proof)
    }

    pub fn recover_from_declaration_route_plan_checked<I: ForgeQueryDeclarationInput<D>>(
        &self,
        checked: crate::application::ForgeQueryDeclarationRoutePlanChecked<D, I>,
    ) -> Option<ForgeQueryRecoveryBrief> {
        forge_query_recovery_brief_from_declaration_route_plan_checked(checked)
    }

    pub fn recover_from_declaration_receipt_checked<I: ForgeQueryDeclarationInput<D>>(
        &self,
        checked: crate::application::ForgeQueryDeclarationReceiptChecked<D, I>,
    ) -> Option<ForgeQueryRecoveryBrief> {
        forge_query_recovery_brief_from_declaration_receipt_checked(checked)
    }

    pub fn recover_from_prepared_continuation_checked<I: ForgeQueryDeclarationInput<D>>(
        &self,
        checked: ForgeQueryPreparedContinuationChecked<D, I>,
    ) -> Option<ForgeQueryRecoveryBrief> {
        forge_query_recovery_brief_from_prepared_continuation_checked(checked)
    }

    pub fn recover_from_prepared_continuation_proof<I: ForgeQueryDeclarationInput<D>>(
        &self,
        proof: ForgeQueryPreparedContinuationTranscript<D, I>,
    ) -> Option<ForgeQueryRecoveryBrief> {
        forge_query_recovery_brief_from_prepared_continuation_proof(proof)
    }

    pub fn recover_from_continuation_execution_checked<I: ForgeQueryDeclarationInput<D>>(
        &self,
        checked: ForgeQueryContinuationExecutionChecked<D, I>,
    ) -> Option<ForgeQueryRecoveryBrief> {
        forge_query_recovery_brief_from_continuation_execution_checked(checked)
    }

    pub fn recover_from_continuation_execution_proof<I: ForgeQueryDeclarationInput<D>>(
        &self,
        proof: ForgeQueryContinuationExecutionTranscript<D, I>,
    ) -> Option<ForgeQueryRecoveryBrief> {
        forge_query_recovery_brief_from_continuation_execution_proof(proof)
    }

    pub fn recover_from_signal_compatibility_checked<I: ForgeQueryDeclarationInput<D>>(
        &self,
        checked: ForgeQuerySignalCompatibilityOrchestrationChecked<D, I>,
    ) -> Option<ForgeQueryRecoveryBrief> {
        forge_query_recovery_brief_from_signal_compatibility_checked(checked)
    }

    pub fn recover_from_signal_compatibility_proof<I: ForgeQueryDeclarationInput<D>>(
        &self,
        proof: ForgeQuerySignalCompatibilityOrchestrationTranscript<D, I>,
    ) -> Option<ForgeQueryRecoveryBrief> {
        forge_query_recovery_brief_from_signal_compatibility_proof(proof)
    }

    pub fn recover_from_contribution_composed_checked<I: ForgeQueryDeclarationInput<D>>(
        &self,
        checked: ForgeQueryContributionComposedOrchestrationChecked<D, I>,
    ) -> Option<ForgeQueryRecoveryBrief> {
        forge_query_recovery_brief_from_contribution_composed_checked(checked)
    }

    pub fn recover_from_contribution_composed_proof<I: ForgeQueryDeclarationInput<D>>(
        &self,
        proof: ForgeQueryContributionComposedOrchestrationTranscript<D, I>,
    ) -> Option<ForgeQueryRecoveryBrief> {
        forge_query_recovery_brief_from_contribution_composed_proof(proof)
    }

    pub fn recover_from_grouped_orchestration_checked<I: ForgeQueryDeclarationInput<D>>(
        &self,
        checked: ForgeQueryGroupedOrchestrationChecked<D, I>,
    ) -> Option<ForgeQueryRecoveryBrief> {
        forge_query_recovery_brief_from_grouped_orchestration_checked(checked)
    }

    pub fn recover_from_grouped_orchestration_proof<I: ForgeQueryDeclarationInput<D>>(
        &self,
        proof: ForgeQueryGroupedOrchestrationTranscript<D, I>,
    ) -> Option<ForgeQueryRecoveryBrief> {
        forge_query_recovery_brief_from_grouped_orchestration_proof(proof)
    }
}
