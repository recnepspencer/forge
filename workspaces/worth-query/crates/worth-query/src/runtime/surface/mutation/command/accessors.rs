use crate::runtime::{
    WorthQueryContinuityMutationIntent, WorthQueryExistingTruthTargetBinding,
    WorthQueryNamingMutationIntent, WorthQuerySymbolicAspectReference,
    WorthQuerySymbolicTargetReference, WorthQueryWriteCommand,
};

impl WorthQueryWriteCommand {
    pub fn existing_truth_binding(&self) -> Option<&WorthQueryExistingTruthTargetBinding> {
        match self {
            Self::UpdateExistingAspects { binding, .. }
            | Self::VerifyThenUpdateExistingAspects { binding, .. }
            | Self::VerifyThenDeleteExistingAspects { binding, .. }
            | Self::AssertExistingAspects { binding, .. }
            | Self::VerifyExistingAspects { binding, .. }
            | Self::DeleteExistingAspects { binding, .. } => Some(binding),
            _ => None,
        }
    }

    pub fn symbolic_target_reference(&self) -> Option<&WorthQuerySymbolicTargetReference> {
        match self {
            Self::InsertAspects {
                symbolic_target_reference,
                ..
            } => symbolic_target_reference.as_ref(),
            Self::UpdateSymbolicAspects { reference, .. }
            | Self::DeleteSymbolicAspects { reference, .. } => Some(reference),
            _ => None,
        }
    }

    pub fn naming_intent(&self) -> Option<&WorthQueryNamingMutationIntent> {
        match self {
            Self::InsertAspects { naming_intent, .. }
            | Self::UpdateAspects { naming_intent, .. }
            | Self::UpdateExistingAspects { naming_intent, .. }
            | Self::VerifyThenUpdateExistingAspects { naming_intent, .. }
            | Self::VerifyThenDeleteExistingAspects { naming_intent, .. }
            | Self::UpdateSymbolicAspects { naming_intent, .. }
            | Self::DeleteAspects { naming_intent, .. }
            | Self::DeleteExistingAspects { naming_intent, .. }
            | Self::DeleteSymbolicAspects { naming_intent, .. } => naming_intent.as_ref(),
            Self::AssertExistingAspects { .. }
            | Self::VerifyExistingAspects { .. }
            | Self::UpdateAspect { .. }
            | Self::Delete { .. } => None,
        }
    }

    pub fn continuity_intent(&self) -> Option<&WorthQueryContinuityMutationIntent> {
        match self {
            Self::InsertAspects {
                continuity_intent, ..
            }
            | Self::UpdateAspects {
                continuity_intent, ..
            }
            | Self::UpdateExistingAspects {
                continuity_intent, ..
            }
            | Self::VerifyThenUpdateExistingAspects {
                continuity_intent, ..
            }
            | Self::UpdateSymbolicAspects {
                continuity_intent, ..
            } => continuity_intent.as_ref(),
            Self::VerifyThenDeleteExistingAspects { .. }
            | Self::AssertExistingAspects { .. }
            | Self::VerifyExistingAspects { .. }
            | Self::UpdateAspect { .. }
            | Self::DeleteAspects { .. }
            | Self::DeleteExistingAspects { .. }
            | Self::DeleteSymbolicAspects { .. }
            | Self::Delete { .. } => None,
        }
    }

    pub fn symbolic_aspect_references(&self) -> &[WorthQuerySymbolicAspectReference] {
        match self {
            Self::InsertAspects {
                symbolic_aspect_references,
                ..
            }
            | Self::VerifyThenUpdateExistingAspects {
                symbolic_aspect_references,
                ..
            } => symbolic_aspect_references,
            Self::UpdateAspect { .. }
            | Self::UpdateAspects { .. }
            | Self::UpdateExistingAspects { .. }
            | Self::VerifyThenDeleteExistingAspects { .. }
            | Self::AssertExistingAspects { .. }
            | Self::VerifyExistingAspects { .. }
            | Self::UpdateSymbolicAspects { .. }
            | Self::DeleteAspects { .. }
            | Self::DeleteExistingAspects { .. }
            | Self::DeleteSymbolicAspects { .. }
            | Self::Delete { .. } => &[],
        }
    }
}
