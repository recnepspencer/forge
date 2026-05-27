mod artifact;
mod declaration;
mod input;
mod orchestration;

pub use artifact::{
    ForgeQueryGroupedDeclarationArtifact, ForgeQueryGroupedDeclarationMember,
    ForgeQueryGroupedOrdering, ForgeQueryGroupedSemantics,
};
pub use declaration::{
    ForgeQueryGroupedDeclarationChecked, ForgeQueryGroupedDeclarationStop,
    ForgeQueryGroupedDeclarationStopKind,
};
pub use input::ForgeQueryGroupedDeclarationInput;
pub use orchestration::{
    ForgeQueryGroupedEnvelopeMember, ForgeQueryGroupedMemberOrchestrationStop,
    ForgeQueryGroupedOrchestration, ForgeQueryGroupedOrchestrationAlignmentStop,
    ForgeQueryGroupedOrchestrationChecked, ForgeQueryGroupedOrchestrationProof,
    ForgeQueryGroupedOrchestrationStop, ForgeQueryGroupedOrchestrationTranscript,
};

pub(crate) use declaration::forge_query_grouped_declaration_checked_on_handle;
pub(crate) use orchestration::{
    forge_query_grouped_orchestration_checked_on_handle,
    forge_query_grouped_orchestration_proof_on_handle,
    ordinary_outcome_from_grouped_orchestration_checked,
};

#[cfg(test)]
mod tests;
