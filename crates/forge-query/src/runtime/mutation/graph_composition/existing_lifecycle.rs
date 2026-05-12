use super::denial::{graph_composition_error, ForgeQueryGraphCompositionDenialKind};
use crate::runtime::{
    ForgeQueryContinuityMutationOutcomeClass, ForgeQueryNamingMutationFamily,
    ForgeQueryRuntimeError, ForgeQueryWriteCommand,
};

pub(super) fn require_retarget_intent(
    command: &ForgeQueryWriteCommand,
    declared_collection: &str,
) -> Result<(), ForgeQueryRuntimeError> {
    if command.continuity_intent().is_some_and(|intent| {
        matches!(
            intent.outcome_class(),
            ForgeQueryContinuityMutationOutcomeClass::ContinuesAsSplitSuccessors
                | ForgeQueryContinuityMutationOutcomeClass::ContinuesViaTruthLoweredCanonicalMergeSuccessor
        )
    }) {
        return Err(graph_composition_error(
            ForgeQueryGraphCompositionDenialKind::ExistingTargetIdentityPreservationUnavailable,
            None,
            Some(declared_collection.to_string()),
            "graph retarget lanes preserve one continuing target identity; split or merge continuity requires supersession_with_lineage instead",
        ));
    }
    let has_naming_rebind = command
        .naming_intent()
        .is_some_and(|intent| intent.family() == ForgeQueryNamingMutationFamily::RebindTarget);
    let has_continuity_rebind = command.continuity_intent().is_some_and(|intent| {
        intent.outcome_class()
            == ForgeQueryContinuityMutationOutcomeClass::ContinuesAsSingleSuccessor
    });
    if has_naming_rebind || has_continuity_rebind {
        return Ok(());
    }
    Err(graph_composition_error(
        ForgeQueryGraphCompositionDenialKind::ExistingTargetRetargetUnsupported,
        None,
        Some(declared_collection.to_string()),
        "graph retarget lanes require naming_rebind_target(...) or continuity_rebind_existing_target(...) on the update component",
    ))
}

pub(super) fn require_supersession_intent(
    command: &ForgeQueryWriteCommand,
    declared_collection: &str,
) -> Result<(), ForgeQueryRuntimeError> {
    let has_supersession_continuity = command.continuity_intent().is_some_and(|intent| {
        matches!(
            intent.outcome_class(),
            ForgeQueryContinuityMutationOutcomeClass::ContinuesAsSplitSuccessors
                | ForgeQueryContinuityMutationOutcomeClass::ContinuesViaTruthLoweredCanonicalMergeSuccessor
        )
    });
    if has_supersession_continuity {
        return Ok(());
    }
    Err(graph_composition_error(
        ForgeQueryGraphCompositionDenialKind::ExistingTargetSupersessionUnsupported,
        None,
        Some(declared_collection.to_string()),
        "graph supersession lanes require continuity_split_successors(...) or continuity_rebind_merge_successor(...) on the update component",
    ))
}
