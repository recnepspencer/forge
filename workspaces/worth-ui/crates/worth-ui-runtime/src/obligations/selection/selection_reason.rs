use crate::admission::{UiAdmissionHostCapability, UiAdmissionQueryBasis, UiSupportPosture};
use crate::declaration::UiDeclarationSupportRowSchemaKind;
use crate::graph::UiGraphWorldProfile;
use crate::obligations::touch::{
    UiGraphTouchAspectPosture, UiGraphTouchOriginClass, UiGraphTouchRuntimeLane,
    UiGraphTouchTargetClass,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiObligationWorldProfileClass {
    Authoritative,
    Preview,
    Branch,
    HotReloadCandidate,
    Diagnostic,
    HostObservation,
    TestCertification,
    QuerySnapshotBasis,
}

impl UiObligationWorldProfileClass {
    pub(crate) fn from_profile(world_profile: &UiGraphWorldProfile) -> Self {
        match world_profile {
            UiGraphWorldProfile::Authoritative => Self::Authoritative,
            UiGraphWorldProfile::PreviewSessionLabel { .. }
            | UiGraphWorldProfile::PreviewSessionIdentity { .. } => Self::Preview,
            UiGraphWorldProfile::BranchSessionLabel { .. } => Self::Branch,
            UiGraphWorldProfile::HotReloadCandidate { .. } => Self::HotReloadCandidate,
            UiGraphWorldProfile::Diagnostic { .. } => Self::Diagnostic,
            UiGraphWorldProfile::HostObservation { .. } => Self::HostObservation,
            UiGraphWorldProfile::TestCertification { .. } => Self::TestCertification,
            UiGraphWorldProfile::QuerySnapshotBasis { .. } => Self::QuerySnapshotBasis,
            UiGraphWorldProfile::InstalledQueryBasis { .. } => Self::QuerySnapshotBasis,
            UiGraphWorldProfile::SettledQueryBinding { .. } => Self::QuerySnapshotBasis,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiObligationSupportSelectionPosture {
    Supported,
    Unsupported,
    Deferred,
    DiagnosticOnly,
    WrongWorld,
}

impl UiObligationSupportSelectionPosture {
    pub(crate) fn from_support_posture(posture: &UiSupportPosture) -> Self {
        match posture {
            UiSupportPosture::Supported { .. } => Self::Supported,
            UiSupportPosture::Unsupported { .. } => Self::Unsupported,
            UiSupportPosture::Deferred { .. } => Self::Deferred,
            UiSupportPosture::DiagnosticOnly { .. } => Self::DiagnosticOnly,
            UiSupportPosture::WrongWorld { .. } => Self::WrongWorld,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiObligationSelectionReason {
    TouchTargetClass(UiGraphTouchTargetClass),
    TouchOriginClass(UiGraphTouchOriginClass),
    TouchRuntimeLane(UiGraphTouchRuntimeLane),
    TouchAspectPosture(UiGraphTouchAspectPosture),
    WorldProfile(UiObligationWorldProfileClass),
    SupportPosture(UiObligationSupportSelectionPosture),
    SupportRow(UiDeclarationSupportRowSchemaKind),
    QueryBasis(UiAdmissionQueryBasis),
    HostCapability(UiAdmissionHostCapability),
    GraphQueryBindingAttachment,
}
