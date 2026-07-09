#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeRequestKind {
    Authoritative,
    Preview,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgePreviewLifecycleTransitionKind {
    Admit,
    Activate,
    Discard,
    Promote,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgePreviewLifecycleStateKind {
    Declared,
    Admitted,
    Active,
    Discarded,
    Promoted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgePreviewResidueClass {
    AuthoritativeRoutingResidue,
    AuthoritativeCheckpointResidue,
    AuthoritativeReplayResidue,
    AuthoritativeDiagnosticsResidue,
    AuthoritativeWritebackResidue,
    TemporaryRoutingResidue,
    TemporaryStructuralResidue,
    TemporaryDiagnosticsResidue,
    PreviewExecutionRetained,
    PreviewDiagnosticsRetained,
    ReplayRetainedNonAuthoritative,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeSpeculationFailureClass {
    RequestKindMismatch,
    BranchBindingMismatch,
    IllegalLifecycleTransition,
    PromotionAdmissibilityMismatch,
    ReuseEquivalenceMismatch,
    ResidueClassificationMismatch,
}

pub trait BridgePreviewTypestate {
    fn lifecycle_state_kind() -> BridgePreviewLifecycleStateKind;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PreviewDeclared;

impl BridgePreviewTypestate for PreviewDeclared {
    fn lifecycle_state_kind() -> BridgePreviewLifecycleStateKind {
        BridgePreviewLifecycleStateKind::Declared
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PreviewAdmitted;

impl BridgePreviewTypestate for PreviewAdmitted {
    fn lifecycle_state_kind() -> BridgePreviewLifecycleStateKind {
        BridgePreviewLifecycleStateKind::Admitted
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PreviewActive;

impl BridgePreviewTypestate for PreviewActive {
    fn lifecycle_state_kind() -> BridgePreviewLifecycleStateKind {
        BridgePreviewLifecycleStateKind::Active
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PreviewDiscarded;

impl BridgePreviewTypestate for PreviewDiscarded {
    fn lifecycle_state_kind() -> BridgePreviewLifecycleStateKind {
        BridgePreviewLifecycleStateKind::Discarded
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PreviewPromoted;

impl BridgePreviewTypestate for PreviewPromoted {
    fn lifecycle_state_kind() -> BridgePreviewLifecycleStateKind {
        BridgePreviewLifecycleStateKind::Promoted
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BridgePreviewLifecycleStateKind, BridgePreviewLifecycleTransitionKind,
        BridgePreviewResidueClass, BridgePreviewTypestate, BridgeRequestKind,
        BridgeSpeculationFailureClass, PreviewActive, PreviewAdmitted, PreviewDeclared,
        PreviewDiscarded, PreviewPromoted,
    };

    #[test]
    fn speculation_taxonomy_remains_closed_world_for_phase_1() {
        let request_kinds = [BridgeRequestKind::Authoritative, BridgeRequestKind::Preview];
        let transitions = [
            BridgePreviewLifecycleTransitionKind::Admit,
            BridgePreviewLifecycleTransitionKind::Activate,
            BridgePreviewLifecycleTransitionKind::Discard,
            BridgePreviewLifecycleTransitionKind::Promote,
        ];
        let states = [
            BridgePreviewLifecycleStateKind::Declared,
            BridgePreviewLifecycleStateKind::Admitted,
            BridgePreviewLifecycleStateKind::Active,
            BridgePreviewLifecycleStateKind::Discarded,
            BridgePreviewLifecycleStateKind::Promoted,
        ];
        let residue_classes = [
            BridgePreviewResidueClass::AuthoritativeRoutingResidue,
            BridgePreviewResidueClass::AuthoritativeCheckpointResidue,
            BridgePreviewResidueClass::AuthoritativeReplayResidue,
            BridgePreviewResidueClass::AuthoritativeDiagnosticsResidue,
            BridgePreviewResidueClass::AuthoritativeWritebackResidue,
            BridgePreviewResidueClass::TemporaryRoutingResidue,
            BridgePreviewResidueClass::TemporaryStructuralResidue,
            BridgePreviewResidueClass::TemporaryDiagnosticsResidue,
            BridgePreviewResidueClass::PreviewExecutionRetained,
            BridgePreviewResidueClass::PreviewDiagnosticsRetained,
            BridgePreviewResidueClass::ReplayRetainedNonAuthoritative,
        ];
        let failures = [
            BridgeSpeculationFailureClass::RequestKindMismatch,
            BridgeSpeculationFailureClass::BranchBindingMismatch,
            BridgeSpeculationFailureClass::IllegalLifecycleTransition,
            BridgeSpeculationFailureClass::PromotionAdmissibilityMismatch,
            BridgeSpeculationFailureClass::ReuseEquivalenceMismatch,
            BridgeSpeculationFailureClass::ResidueClassificationMismatch,
        ];

        assert_eq!(request_kinds.len(), 2);
        assert_eq!(transitions.len(), 4);
        assert_eq!(states.len(), 5);
        assert_eq!(residue_classes.len(), 11);
        assert_eq!(failures.len(), 6);
        assert_eq!(
            PreviewDeclared::lifecycle_state_kind(),
            BridgePreviewLifecycleStateKind::Declared
        );
        assert_eq!(
            PreviewAdmitted::lifecycle_state_kind(),
            BridgePreviewLifecycleStateKind::Admitted
        );
        assert_eq!(
            PreviewActive::lifecycle_state_kind(),
            BridgePreviewLifecycleStateKind::Active
        );
        assert_eq!(
            PreviewDiscarded::lifecycle_state_kind(),
            BridgePreviewLifecycleStateKind::Discarded
        );
        assert_eq!(
            PreviewPromoted::lifecycle_state_kind(),
            BridgePreviewLifecycleStateKind::Promoted
        );
    }
}
