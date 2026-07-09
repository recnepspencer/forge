use worth_query::facade::{
    WORTHQueryContributionComposedOrchestrationChecked,
    WORTHQueryDeclarationEntryOrchestrationChecked, WORTHQueryGroupedOrchestrationChecked,
    WORTHQueryGroupedOrchestrationTranscript, WORTHQueryOrdinaryOutcome, WORTHQueryRecoveryBrief,
};

use crate::domain_declarations::HadwigerResearchDeclarationInput;
use crate::query_entry::{HadwigerResearchDomainEntry, HadwigerResearchHandle};

pub fn recover_research_stop_from_outcome<T>(
    handle: &HadwigerResearchHandle,
    outcome: &WORTHQueryOrdinaryOutcome<T>,
) -> Option<WORTHQueryRecoveryBrief> {
    handle.recover_from_outcome(outcome)
}

pub fn recover_research_stop_from_declaration_entry_checked<I>(
    handle: &HadwigerResearchHandle,
    checked: WORTHQueryDeclarationEntryOrchestrationChecked<HadwigerResearchDomainEntry, I>,
) -> Option<WORTHQueryRecoveryBrief>
where
    I: HadwigerResearchDeclarationInput,
{
    handle.recover_from_declaration_entry_checked(checked)
}

pub fn recover_research_stop_from_contribution_composed_checked<I>(
    handle: &HadwigerResearchHandle,
    checked: WORTHQueryContributionComposedOrchestrationChecked<HadwigerResearchDomainEntry, I>,
) -> Option<WORTHQueryRecoveryBrief>
where
    I: HadwigerResearchDeclarationInput,
{
    handle.recover_from_contribution_composed_checked(checked)
}

pub fn recover_research_stop_from_grouped_orchestration_checked<I>(
    handle: &HadwigerResearchHandle,
    checked: WORTHQueryGroupedOrchestrationChecked<HadwigerResearchDomainEntry, I>,
) -> Option<WORTHQueryRecoveryBrief>
where
    I: HadwigerResearchDeclarationInput,
{
    handle.recover_from_grouped_orchestration_checked(checked)
}

pub fn recover_research_stop_from_grouped_orchestration_proof<I>(
    handle: &HadwigerResearchHandle,
    proof: WORTHQueryGroupedOrchestrationTranscript<HadwigerResearchDomainEntry, I>,
) -> Option<WORTHQueryRecoveryBrief>
where
    I: HadwigerResearchDeclarationInput,
{
    handle.recover_from_grouped_orchestration_proof(proof)
}
