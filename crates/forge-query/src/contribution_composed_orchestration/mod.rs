mod artifact;
mod input;
mod lower;
mod mapping;
mod outcome;
mod transcript;

pub use artifact::{
    ForgeQueryContributionComposedContribution, ForgeQueryContributionComposedOrchestration,
    ForgeQueryContributionComposedSummary,
};
pub use input::{
    ForgeQueryContributionComposedMaterializationPolicy,
    ForgeQueryContributionComposedOrchestrationInput, ForgeQueryContributionIntent,
};
pub(crate) use outcome::ordinary_outcome_from_contribution_composed_checked;
pub use outcome::{
    ForgeQueryContributionComposedOrchestrationChecked,
    ForgeQueryContributionComposedOrchestrationOutcome,
    ForgeQueryContributionComposedOrchestrationPosture,
};
pub use transcript::ForgeQueryContributionComposedOrchestrationTranscript;

pub(crate) use transcript::orchestrate_declaration_with_contributions_on_handle;

#[cfg(test)]
mod tests;
