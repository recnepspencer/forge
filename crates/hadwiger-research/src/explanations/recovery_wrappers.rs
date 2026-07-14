use worth_query::facade::foundation::{
    WorthQueryContributionComposedOrchestrationChecked,
    WorthQueryDeclarationEntryOrchestrationChecked, WorthQueryGroupedOrchestrationChecked,
    WorthQueryGroupedOrchestrationTranscript, WorthQueryOrdinaryOutcome, WorthQueryRecoveryBrief,
};

use crate::domain_declarations::HadwigerResearchDeclarationInput;
use crate::query_entry::{HadwigerResearchDomainEntry, HadwigerResearchHandle};

pub fn recover_research_stop_from_outcome<T>(
    handle: &HadwigerResearchHandle,
    outcome: &WorthQueryOrdinaryOutcome<T>,
) -> Option<WorthQueryRecoveryBrief> {
    handle.recover_from_outcome(outcome)
}

pub fn recover_research_stop_from_declaration_entry_checked<I>(
    handle: &HadwigerResearchHandle,
    checked: WorthQueryDeclarationEntryOrchestrationChecked<HadwigerResearchDomainEntry, I>,
) -> Option<WorthQueryRecoveryBrief>
where
    I: HadwigerResearchDeclarationInput,
{
    handle.recover_from_declaration_entry_checked(checked)
}

pub fn recover_research_stop_from_contribution_composed_checked<I>(
    handle: &HadwigerResearchHandle,
    checked: WorthQueryContributionComposedOrchestrationChecked<HadwigerResearchDomainEntry, I>,
) -> Option<WorthQueryRecoveryBrief>
where
    I: HadwigerResearchDeclarationInput,
{
    handle.recover_from_contribution_composed_checked(checked)
}

pub fn recover_research_stop_from_grouped_orchestration_checked<I>(
    handle: &HadwigerResearchHandle,
    checked: WorthQueryGroupedOrchestrationChecked<HadwigerResearchDomainEntry, I>,
) -> Option<WorthQueryRecoveryBrief>
where
    I: HadwigerResearchDeclarationInput,
{
    handle.recover_from_grouped_orchestration_checked(checked)
}

pub fn recover_research_stop_from_grouped_orchestration_proof<I>(
    handle: &HadwigerResearchHandle,
    proof: WorthQueryGroupedOrchestrationTranscript<HadwigerResearchDomainEntry, I>,
) -> Option<WorthQueryRecoveryBrief>
where
    I: HadwigerResearchDeclarationInput,
{
    handle.recover_from_grouped_orchestration_proof(proof)
}
