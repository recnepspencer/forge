use crate::runtime::WorthQueryGraphCompositionProgramStepKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryGraphTouchLifecycleFamily {
    Declaration,
    SameBatchSymbolicEntityFollowup,
    SameBatchSymbolicRelationFollowup,
    SameBatchSymbolicRelationRetirement,
    ExistingTargetFollowup,
    ExistingTargetRetarget,
    ExistingTargetSupersession,
    ExistingTargetRetirement,
    VerifiedExistingTargetFollowup,
    VerifiedExistingTargetRetarget,
    VerifiedExistingTargetSupersession,
    VerifiedExistingTargetRetirement,
}

impl WorthQueryGraphTouchLifecycleFamily {
    pub fn from_program_step_kind(kind: WorthQueryGraphCompositionProgramStepKind) -> Self {
        match kind {
            WorthQueryGraphCompositionProgramStepKind::SymbolicEntityDeclaration
            | WorthQueryGraphCompositionProgramStepKind::RelationDeclaration
            | WorthQueryGraphCompositionProgramStepKind::SymbolicRelationDeclaration => {
                Self::Declaration
            }
            WorthQueryGraphCompositionProgramStepKind::SymbolicEntityFollowupMutation => {
                Self::SameBatchSymbolicEntityFollowup
            }
            WorthQueryGraphCompositionProgramStepKind::SymbolicRelationFollowupMutation => {
                Self::SameBatchSymbolicRelationFollowup
            }
            WorthQueryGraphCompositionProgramStepKind::SymbolicRelationRetirement => {
                Self::SameBatchSymbolicRelationRetirement
            }
            WorthQueryGraphCompositionProgramStepKind::ExistingTargetFollowupMutation => {
                Self::ExistingTargetFollowup
            }
            WorthQueryGraphCompositionProgramStepKind::ExistingTargetRetarget => {
                Self::ExistingTargetRetarget
            }
            WorthQueryGraphCompositionProgramStepKind::ExistingTargetSupersession => {
                Self::ExistingTargetSupersession
            }
            WorthQueryGraphCompositionProgramStepKind::ExistingTargetRetirement => {
                Self::ExistingTargetRetirement
            }
            WorthQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedFollowupMutation => {
                Self::VerifiedExistingTargetFollowup
            }
            WorthQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetarget => {
                Self::VerifiedExistingTargetRetarget
            }
            WorthQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedSupersession => {
                Self::VerifiedExistingTargetSupersession
            }
            WorthQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetirement => {
                Self::VerifiedExistingTargetRetirement
            }
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Declaration => "declaration",
            Self::SameBatchSymbolicEntityFollowup => "same-batch-symbolic-entity-followup",
            Self::SameBatchSymbolicRelationFollowup => "same-batch-symbolic-relation-followup",
            Self::SameBatchSymbolicRelationRetirement => "same-batch-symbolic-relation-retirement",
            Self::ExistingTargetFollowup => "existing-target-followup",
            Self::ExistingTargetRetarget => "existing-target-retarget",
            Self::ExistingTargetSupersession => "existing-target-supersession",
            Self::ExistingTargetRetirement => "existing-target-retirement",
            Self::VerifiedExistingTargetFollowup => "verified-existing-target-followup",
            Self::VerifiedExistingTargetRetarget => "verified-existing-target-retarget",
            Self::VerifiedExistingTargetSupersession => "verified-existing-target-supersession",
            Self::VerifiedExistingTargetRetirement => "verified-existing-target-retirement",
        }
    }
}

impl std::fmt::Display for WorthQueryGraphTouchLifecycleFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
