use crate::admission::{
    UiAdmissionHostCapability, UiAdmissionQueryBasis, UiAdmissionSelectionBudget,
    UiAdmissionStaleEvidence,
};
use crate::declaration::UiDeclarationSupportRowSchemaKind;
use crate::obligations::selection::{
    UiObligationSelectionReason, UiObligationSupportSelectionPosture, UiObligationWorldProfileClass,
};
use crate::obligations::touch::{
    UiGraphTouchAspectPosture, UiGraphTouchOriginClass, UiGraphTouchRuntimeLane,
    UiGraphTouchTargetClass,
};
use worth_ui_inspection::{
    UiInspectionAdmissionHostCapability, UiInspectionAdmissionQueryBasis,
    UiInspectionAdmissionStaleEvidence, UiInspectionObligationSelectionReason,
    UiInspectionObligationSupportSelectionPosture, UiInspectionObligationWorldProfileClass,
    UiInspectionSelectionBudget, UiInspectionSupportRowSchemaKind, UiInspectionTouchAspectPosture,
    UiInspectionTouchOriginClass, UiInspectionTouchRuntimeLane, UiInspectionTouchTargetClass,
};

pub(super) fn inspection_selection_reason(
    reason: UiObligationSelectionReason,
) -> UiInspectionObligationSelectionReason {
    match reason {
        UiObligationSelectionReason::TouchTargetClass(class) => {
            UiInspectionObligationSelectionReason::TouchTargetClass(match class {
                UiGraphTouchTargetClass::Node => UiInspectionTouchTargetClass::Node,
                UiGraphTouchTargetClass::SlotOccupancy => {
                    UiInspectionTouchTargetClass::SlotOccupancy
                }
                UiGraphTouchTargetClass::PageMembership => {
                    UiInspectionTouchTargetClass::PageMembership
                }
                UiGraphTouchTargetClass::RegionMembership => {
                    UiInspectionTouchTargetClass::RegionMembership
                }
                UiGraphTouchTargetClass::MosaicMembership => {
                    UiInspectionTouchTargetClass::MosaicMembership
                }
                UiGraphTouchTargetClass::AttachmentLane => {
                    UiInspectionTouchTargetClass::AttachmentLane
                }
            })
        }
        UiObligationSelectionReason::TouchOriginClass(class) => {
            UiInspectionObligationSelectionReason::TouchOriginClass(match class {
                UiGraphTouchOriginClass::DeclarationChange => {
                    UiInspectionTouchOriginClass::DeclarationChange
                }
                UiGraphTouchOriginClass::QueryBindingChange => {
                    UiInspectionTouchOriginClass::QueryBindingChange
                }
                UiGraphTouchOriginClass::QueryFactChange => {
                    UiInspectionTouchOriginClass::QueryFactChange
                }
                UiGraphTouchOriginClass::HostObservation => {
                    UiInspectionTouchOriginClass::HostObservation
                }
                UiGraphTouchOriginClass::ServiceEvent => UiInspectionTouchOriginClass::ServiceEvent,
                UiGraphTouchOriginClass::IntentSubmission => {
                    UiInspectionTouchOriginClass::IntentSubmission
                }
                UiGraphTouchOriginClass::DiagnosticOnly => {
                    UiInspectionTouchOriginClass::DiagnosticOnly
                }
            })
        }
        UiObligationSelectionReason::TouchRuntimeLane(lane) => {
            UiInspectionObligationSelectionReason::TouchRuntimeLane(match lane {
                UiGraphTouchRuntimeLane::Structural => UiInspectionTouchRuntimeLane::Structural,
                UiGraphTouchRuntimeLane::Participation => {
                    UiInspectionTouchRuntimeLane::Participation
                }
                UiGraphTouchRuntimeLane::Measurement => UiInspectionTouchRuntimeLane::Measurement,
                UiGraphTouchRuntimeLane::QueryBinding => UiInspectionTouchRuntimeLane::QueryBinding,
                UiGraphTouchRuntimeLane::IntentOperability => {
                    UiInspectionTouchRuntimeLane::IntentOperability
                }
                UiGraphTouchRuntimeLane::Service => UiInspectionTouchRuntimeLane::Service,
                UiGraphTouchRuntimeLane::HostCapability => {
                    UiInspectionTouchRuntimeLane::HostCapability
                }
                UiGraphTouchRuntimeLane::Diagnostic => UiInspectionTouchRuntimeLane::Diagnostic,
            })
        }
        UiObligationSelectionReason::TouchAspectPosture(posture) => {
            UiInspectionObligationSelectionReason::TouchAspectPosture(match posture {
                UiGraphTouchAspectPosture::Read => UiInspectionTouchAspectPosture::Read,
                UiGraphTouchAspectPosture::Written => UiInspectionTouchAspectPosture::Written,
                UiGraphTouchAspectPosture::Invalidated => {
                    UiInspectionTouchAspectPosture::Invalidated
                }
                UiGraphTouchAspectPosture::Preserved => UiInspectionTouchAspectPosture::Preserved,
            })
        }
        UiObligationSelectionReason::WorldProfile(profile) => {
            UiInspectionObligationSelectionReason::WorldProfile(match profile {
                UiObligationWorldProfileClass::Authoritative => {
                    UiInspectionObligationWorldProfileClass::Authoritative
                }
                UiObligationWorldProfileClass::Preview => {
                    UiInspectionObligationWorldProfileClass::Preview
                }
                UiObligationWorldProfileClass::Branch => {
                    UiInspectionObligationWorldProfileClass::Branch
                }
                UiObligationWorldProfileClass::HotReloadCandidate => {
                    UiInspectionObligationWorldProfileClass::HotReloadCandidate
                }
                UiObligationWorldProfileClass::Diagnostic => {
                    UiInspectionObligationWorldProfileClass::Diagnostic
                }
                UiObligationWorldProfileClass::HostObservation => {
                    UiInspectionObligationWorldProfileClass::HostObservation
                }
                UiObligationWorldProfileClass::TestCertification => {
                    UiInspectionObligationWorldProfileClass::TestCertification
                }
                UiObligationWorldProfileClass::SettledQueryBinding => {
                    UiInspectionObligationWorldProfileClass::SettledQueryBinding
                }
            })
        }
        UiObligationSelectionReason::SupportPosture(posture) => {
            UiInspectionObligationSelectionReason::SupportPosture(match posture {
                UiObligationSupportSelectionPosture::Supported => {
                    UiInspectionObligationSupportSelectionPosture::Supported
                }
                UiObligationSupportSelectionPosture::Unsupported => {
                    UiInspectionObligationSupportSelectionPosture::Unsupported
                }
                UiObligationSupportSelectionPosture::Deferred => {
                    UiInspectionObligationSupportSelectionPosture::Deferred
                }
                UiObligationSupportSelectionPosture::DiagnosticOnly => {
                    UiInspectionObligationSupportSelectionPosture::DiagnosticOnly
                }
                UiObligationSupportSelectionPosture::WrongWorld => {
                    UiInspectionObligationSupportSelectionPosture::WrongWorld
                }
            })
        }
        UiObligationSelectionReason::SupportRow(kind) => {
            UiInspectionObligationSelectionReason::SupportRow(match kind {
                UiDeclarationSupportRowSchemaKind::QueryBinding => {
                    UiInspectionSupportRowSchemaKind::QueryBinding
                }
                UiDeclarationSupportRowSchemaKind::ServiceUsage => {
                    UiInspectionSupportRowSchemaKind::ServiceUsage
                }
                UiDeclarationSupportRowSchemaKind::TouchMeaning => {
                    UiInspectionSupportRowSchemaKind::TouchMeaning
                }
                UiDeclarationSupportRowSchemaKind::MeasurementPolicy => {
                    UiInspectionSupportRowSchemaKind::MeasurementPolicy
                }
                UiDeclarationSupportRowSchemaKind::HostCapability => {
                    UiInspectionSupportRowSchemaKind::HostCapability
                }
            })
        }
        UiObligationSelectionReason::QueryBasis(basis) => {
            UiInspectionObligationSelectionReason::QueryBasis(inspection_query_basis(basis))
        }
        UiObligationSelectionReason::HostCapability(capability) => {
            UiInspectionObligationSelectionReason::HostCapability(inspection_host_capability(
                capability,
            ))
        }
        UiObligationSelectionReason::GraphQueryBindingAttachment => {
            UiInspectionObligationSelectionReason::GraphQueryBindingAttachment
        }
    }
}

pub(super) fn inspection_query_basis(
    basis: UiAdmissionQueryBasis,
) -> UiInspectionAdmissionQueryBasis {
    match basis {
        UiAdmissionQueryBasis::GraphAligned => UiInspectionAdmissionQueryBasis::GraphAligned,
        UiAdmissionQueryBasis::WrongWorldProjection => {
            UiInspectionAdmissionQueryBasis::WrongWorldProjection
        }
        UiAdmissionQueryBasis::RebindRequired => UiInspectionAdmissionQueryBasis::RebindRequired,
        UiAdmissionQueryBasis::StaleReceipt => UiInspectionAdmissionQueryBasis::StaleReceipt,
        UiAdmissionQueryBasis::AmbiguousSources => {
            UiInspectionAdmissionQueryBasis::AmbiguousSources
        }
    }
}

pub(super) fn inspection_host_capability(
    capability: UiAdmissionHostCapability,
) -> UiInspectionAdmissionHostCapability {
    match capability {
        UiAdmissionHostCapability::Available => UiInspectionAdmissionHostCapability::Available,
        UiAdmissionHostCapability::Missing => UiInspectionAdmissionHostCapability::Missing,
        UiAdmissionHostCapability::Ambiguous => UiInspectionAdmissionHostCapability::Ambiguous,
    }
}

pub(super) fn inspection_stale_evidence(
    evidence: UiAdmissionStaleEvidence,
) -> UiInspectionAdmissionStaleEvidence {
    match evidence {
        UiAdmissionStaleEvidence::DeclarationArtifactMissing => {
            UiInspectionAdmissionStaleEvidence::DeclarationArtifactMissing
        }
        UiAdmissionStaleEvidence::QueryReceiptExpired => {
            UiInspectionAdmissionStaleEvidence::QueryReceiptExpired
        }
    }
}

pub(super) fn inspection_budget(budget: UiAdmissionSelectionBudget) -> UiInspectionSelectionBudget {
    match budget {
        UiAdmissionSelectionBudget::Unbounded => UiInspectionSelectionBudget::Unbounded,
        UiAdmissionSelectionBudget::OrdinaryLaneBudget { lane_limit } => {
            UiInspectionSelectionBudget::OrdinaryLaneBudget { lane_limit }
        }
    }
}
