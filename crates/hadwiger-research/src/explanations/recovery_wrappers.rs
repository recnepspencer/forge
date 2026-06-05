use forge_query::facade::{
    ForgeQueryContributionComposedOrchestrationChecked,
    ForgeQueryDeclarationEntryOrchestrationChecked, ForgeQueryGroupedOrchestrationChecked,
    ForgeQueryGroupedOrchestrationTranscript, ForgeQueryOrdinaryOutcome, ForgeQueryRecoveryBrief,
};

use crate::domain_declarations::HadwigerResearchDeclarationInput;
use crate::query_entry::{HadwigerResearchDomainEntry, HadwigerResearchHandle};

pub fn recover_research_stop_from_outcome<T>(
    handle: &HadwigerResearchHandle,
    outcome: &ForgeQueryOrdinaryOutcome<T>,
) -> Option<ForgeQueryRecoveryBrief> {
    handle.recover_from_outcome(outcome)
}

pub fn recover_research_stop_from_declaration_entry_checked<I>(
    handle: &HadwigerResearchHandle,
    checked: ForgeQueryDeclarationEntryOrchestrationChecked<HadwigerResearchDomainEntry, I>,
) -> Option<ForgeQueryRecoveryBrief>
where
    I: HadwigerResearchDeclarationInput,
{
    handle.recover_from_declaration_entry_checked(checked)
}

pub fn recover_research_stop_from_contribution_composed_checked<I>(
    handle: &HadwigerResearchHandle,
    checked: ForgeQueryContributionComposedOrchestrationChecked<HadwigerResearchDomainEntry, I>,
) -> Option<ForgeQueryRecoveryBrief>
where
    I: HadwigerResearchDeclarationInput,
{
    handle.recover_from_contribution_composed_checked(checked)
}

pub fn recover_research_stop_from_grouped_orchestration_checked<I>(
    handle: &HadwigerResearchHandle,
    checked: ForgeQueryGroupedOrchestrationChecked<HadwigerResearchDomainEntry, I>,
) -> Option<ForgeQueryRecoveryBrief>
where
    I: HadwigerResearchDeclarationInput,
{
    handle.recover_from_grouped_orchestration_checked(checked)
}

pub fn recover_research_stop_from_grouped_orchestration_proof<I>(
    handle: &HadwigerResearchHandle,
    proof: ForgeQueryGroupedOrchestrationTranscript<HadwigerResearchDomainEntry, I>,
) -> Option<ForgeQueryRecoveryBrief>
where
    I: HadwigerResearchDeclarationInput,
{
    handle.recover_from_grouped_orchestration_proof(proof)
}
