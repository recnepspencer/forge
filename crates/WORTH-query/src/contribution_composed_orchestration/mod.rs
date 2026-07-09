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
    WorthQueryContributionComposedContribution, WorthQueryContributionComposedOrchestration,
    WorthQueryContributionComposedSummary,
};
#[allow(unused_imports)]
pub use aspect::{
    WorthQueryContributionComposedDeclarationAspectRecord,
    WorthQueryContributionComposedIntentAspectRecord,
};
pub use composition::{
    WorthQueryContributionComposedClassification, WorthQueryContributionComposedComposition,
    WorthQueryContributionComposedStop,
};
pub use declaration_record::WorthQueryContributionComposedDeclarationRecord;
pub use input::{
    WorthQueryContributionComposedMaterializationPolicy,
    WorthQueryContributionComposedOrchestrationInput, WorthQueryContributionIntent,
};
pub use intent_result::{
    WorthQueryContributionComposedIntentClassification,
    WorthQueryContributionComposedIntentRequestDescriptor,
    WorthQueryContributionComposedIntentResult, WorthQueryContributionComposedIntentStageKind,
    WorthQueryContributionComposedIntentStageResult,
};
pub(crate) use outcome::ordinary_outcome_from_contribution_composed_checked;
pub use outcome::{
    WorthQueryContributionComposedOrchestrationChecked,
    WorthQueryContributionComposedOrchestrationCheckedKind,
    WorthQueryContributionComposedOrchestrationOutcome,
    WorthQueryContributionComposedOrchestrationPosture,
};
pub use transcript::WorthQueryContributionComposedOrchestrationTranscript;

pub(crate) use retained::orchestrate_progressed_declaration_with_contributions_checked_on_handle;
pub(crate) use transcript::orchestrate_declaration_with_contributions_on_handle;

#[cfg(test)]
mod tests;
