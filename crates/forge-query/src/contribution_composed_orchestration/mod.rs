mod artifact;
mod aspect;
mod composition;
mod declaration_record;
mod input;
mod intent_result;
mod lower;
mod mapping;
mod outcome;
mod retained;
mod transcript;

pub use artifact::{
    ForgeQueryContributionComposedContribution, ForgeQueryContributionComposedOrchestration,
    ForgeQueryContributionComposedSummary,
};
#[allow(unused_imports)]
pub use aspect::{
    ForgeQueryContributionComposedDeclarationAspectRecord,
    ForgeQueryContributionComposedIntentAspectRecord,
};
pub use composition::{
    ForgeQueryContributionComposedClassification, ForgeQueryContributionComposedComposition,
    ForgeQueryContributionComposedStop,
};
pub use declaration_record::ForgeQueryContributionComposedDeclarationRecord;
pub use input::{
    ForgeQueryContributionComposedMaterializationPolicy,
    ForgeQueryContributionComposedOrchestrationInput, ForgeQueryContributionIntent,
};
pub use intent_result::{
    ForgeQueryContributionComposedIntentClassification,
    ForgeQueryContributionComposedIntentRequestDescriptor,
    ForgeQueryContributionComposedIntentResult, ForgeQueryContributionComposedIntentStageKind,
    ForgeQueryContributionComposedIntentStageResult,
};
pub(crate) use outcome::ordinary_outcome_from_contribution_composed_checked;
pub use outcome::{
    ForgeQueryContributionComposedOrchestrationChecked,
    ForgeQueryContributionComposedOrchestrationCheckedKind,
    ForgeQueryContributionComposedOrchestrationOutcome,
    ForgeQueryContributionComposedOrchestrationPosture,
};
pub use transcript::ForgeQueryContributionComposedOrchestrationTranscript;

pub(crate) use retained::orchestrate_progressed_declaration_with_contributions_checked_on_handle;
pub(crate) use transcript::orchestrate_declaration_with_contributions_on_handle;

#[cfg(test)]
mod tests;
