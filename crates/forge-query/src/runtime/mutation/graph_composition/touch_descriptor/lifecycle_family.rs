use crate::runtime::ForgeQueryGraphCompositionProgramStepKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryGraphTouchLifecycleFamily {
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

impl ForgeQueryGraphTouchLifecycleFamily {
    pub fn from_program_step_kind(kind: ForgeQueryGraphCompositionProgramStepKind) -> Self {
        match kind {
            ForgeQueryGraphCompositionProgramStepKind::SymbolicEntityDeclaration
            | ForgeQueryGraphCompositionProgramStepKind::RelationDeclaration
            | ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationDeclaration => {
                Self::Declaration
            }
            ForgeQueryGraphCompositionProgramStepKind::SymbolicEntityFollowupMutation => {
                Self::SameBatchSymbolicEntityFollowup
            }
            ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationFollowupMutation => {
                Self::SameBatchSymbolicRelationFollowup
            }
            ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationRetirement => {
                Self::SameBatchSymbolicRelationRetirement
            }
            ForgeQueryGraphCompositionProgramStepKind::ExistingTargetFollowupMutation => {
                Self::ExistingTargetFollowup
            }
            ForgeQueryGraphCompositionProgramStepKind::ExistingTargetRetarget => {
                Self::ExistingTargetRetarget
            }
            ForgeQueryGraphCompositionProgramStepKind::ExistingTargetSupersession => {
                Self::ExistingTargetSupersession
            }
            ForgeQueryGraphCompositionProgramStepKind::ExistingTargetRetirement => {
                Self::ExistingTargetRetirement
            }
            ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedFollowupMutation => {
                Self::VerifiedExistingTargetFollowup
            }
            ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetarget => {
                Self::VerifiedExistingTargetRetarget
            }
            ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedSupersession => {
                Self::VerifiedExistingTargetSupersession
            }
            ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetirement => {
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

impl std::fmt::Display for ForgeQueryGraphTouchLifecycleFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
