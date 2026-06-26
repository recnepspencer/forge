use crate::graph_read_access_declarations::WorthGraphReadTouchedAuthorityLoweringErrorKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessDeclarationPhaseTwoErrorKind {
    ConflictingTouchedAuthorityReadShape,
    MissingTouchedAuthority,
    MissingReadFamilyTarget,
    MissingRequirementEvidence,
    MissingLoweringTarget,
    MissingSupportPosture,
    MissingMilestoneEightAdoptionTarget,
    TouchedAuthorityLoweringFailed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessDeclarationPhaseTwoError {
    kind: WorthGraphReadAccessDeclarationPhaseTwoErrorKind,
    touched_authority_lowering_error_kind: Option<WorthGraphReadTouchedAuthorityLoweringErrorKind>,
}

impl WorthGraphReadAccessDeclarationPhaseTwoError {
    pub(crate) const fn new(kind: WorthGraphReadAccessDeclarationPhaseTwoErrorKind) -> Self {
        Self {
            kind,
            touched_authority_lowering_error_kind: None,
        }
    }

    pub(crate) const fn touched_authority_lowering_failed(
        touched_authority_lowering_error_kind: WorthGraphReadTouchedAuthorityLoweringErrorKind,
    ) -> Self {
        Self {
            kind: WorthGraphReadAccessDeclarationPhaseTwoErrorKind::TouchedAuthorityLoweringFailed,
            touched_authority_lowering_error_kind: Some(touched_authority_lowering_error_kind),
        }
    }

    pub const fn kind(&self) -> WorthGraphReadAccessDeclarationPhaseTwoErrorKind {
        self.kind
    }

    pub const fn touched_authority_lowering_error_kind(
        &self,
    ) -> Option<WorthGraphReadTouchedAuthorityLoweringErrorKind> {
        self.touched_authority_lowering_error_kind
    }
}
