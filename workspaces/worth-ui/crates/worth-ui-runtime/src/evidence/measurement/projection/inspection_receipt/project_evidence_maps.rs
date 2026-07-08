use worth_ui_inspection::{
    UiInspectionMeasurementBasisSource, UiInspectionMeasurementDependencyLineageKind,
    UiInspectionMeasurementEvidenceCategory, UiInspectionMeasurementEvidenceSlot,
    UiInspectionMeasurementNeighborhoodClassHint, UiInspectionMeasurementOwnershipPosture,
};

use crate::declaration::{UiDeclaredMeasurementBasisSource, UiDeclaredMeasurementOwnershipPosture};
use crate::evidence::measurement::{
    UiMeasurementDependencyLineageKind, UiMeasurementEvidenceCategory,
    UiMeasurementNeighborhoodClassHint,
};
use crate::evidence::measurement::basis::UiMeasurementEvidenceSlot;

pub(crate) fn project_lineage_kind(
    kind: UiMeasurementDependencyLineageKind,
) -> UiInspectionMeasurementDependencyLineageKind {
    match kind {
        UiMeasurementDependencyLineageKind::QueryScrollContentExtent => {
            UiInspectionMeasurementDependencyLineageKind::QueryScrollContentExtent
        }
        UiMeasurementDependencyLineageKind::HostTextIntrinsicSize => {
            UiInspectionMeasurementDependencyLineageKind::HostTextIntrinsicSize
        }
        UiMeasurementDependencyLineageKind::HostFontMetrics => {
            UiInspectionMeasurementDependencyLineageKind::HostFontMetrics
        }
        UiMeasurementDependencyLineageKind::HostNativeControlIntrinsicSize => {
            UiInspectionMeasurementDependencyLineageKind::HostNativeControlIntrinsicSize
        }
        UiMeasurementDependencyLineageKind::HostViewportExtent => {
            UiInspectionMeasurementDependencyLineageKind::HostViewportExtent
        }
        UiMeasurementDependencyLineageKind::HostPortalAnchorRect => {
            UiInspectionMeasurementDependencyLineageKind::HostPortalAnchorRect
        }
        UiMeasurementDependencyLineageKind::HostScrollContainerViewport => {
            UiInspectionMeasurementDependencyLineageKind::HostScrollContainerViewport
        }
    }
}

pub(crate) fn project_evidence_category(
    category: UiMeasurementEvidenceCategory,
) -> UiInspectionMeasurementEvidenceCategory {
    match category {
        UiMeasurementEvidenceCategory::TextIntrinsicSize => {
            UiInspectionMeasurementEvidenceCategory::TextIntrinsicSize
        }
        UiMeasurementEvidenceCategory::TextBaselineMetrics => {
            UiInspectionMeasurementEvidenceCategory::TextBaselineMetrics
        }
        UiMeasurementEvidenceCategory::FontMetrics => {
            UiInspectionMeasurementEvidenceCategory::FontMetrics
        }
        UiMeasurementEvidenceCategory::NativeControlIntrinsicSize => {
            UiInspectionMeasurementEvidenceCategory::NativeControlIntrinsicSize
        }
        UiMeasurementEvidenceCategory::ViewportExtent => {
            UiInspectionMeasurementEvidenceCategory::ViewportExtent
        }
        UiMeasurementEvidenceCategory::DpiScaleFactor => {
            UiInspectionMeasurementEvidenceCategory::DpiScaleFactor
        }
        UiMeasurementEvidenceCategory::PortalAnchorRect => {
            UiInspectionMeasurementEvidenceCategory::PortalAnchorRect
        }
        UiMeasurementEvidenceCategory::ScrollContainerViewport => {
            UiInspectionMeasurementEvidenceCategory::ScrollContainerViewport
        }
    }
}

pub(crate) fn project_slot(slot: UiMeasurementEvidenceSlot) -> UiInspectionMeasurementEvidenceSlot {
    match slot {
        UiMeasurementEvidenceSlot::QueryProjectionFactReceipt => {
            UiInspectionMeasurementEvidenceSlot::QueryProjectionFactReceipt
        }
        UiMeasurementEvidenceSlot::HostCapabilityReport => {
            UiInspectionMeasurementEvidenceSlot::HostCapabilityReport
        }
        UiMeasurementEvidenceSlot::HostTextIntrinsicSize => {
            UiInspectionMeasurementEvidenceSlot::HostTextIntrinsicSize
        }
        UiMeasurementEvidenceSlot::HostFontMetrics => {
            UiInspectionMeasurementEvidenceSlot::HostFontMetrics
        }
        UiMeasurementEvidenceSlot::HostNativeControlIntrinsicSize => {
            UiInspectionMeasurementEvidenceSlot::HostNativeControlIntrinsicSize
        }
        UiMeasurementEvidenceSlot::ViewportExtent => {
            UiInspectionMeasurementEvidenceSlot::ViewportExtent
        }
        UiMeasurementEvidenceSlot::PortalAnchorRect => {
            UiInspectionMeasurementEvidenceSlot::PortalAnchorRect
        }
        UiMeasurementEvidenceSlot::ScrollContainerViewport => {
            UiInspectionMeasurementEvidenceSlot::ScrollContainerViewport
        }
    }
}

pub(crate) fn project_basis_source(
    basis_source: UiDeclaredMeasurementBasisSource,
) -> UiInspectionMeasurementBasisSource {
    match basis_source {
        UiDeclaredMeasurementBasisSource::ScrollViewport => {
            UiInspectionMeasurementBasisSource::ScrollViewport
        }
        UiDeclaredMeasurementBasisSource::PortalAnchor => {
            UiInspectionMeasurementBasisSource::PortalAnchor
        }
    }
}

pub(crate) fn project_ownership_posture(
    ownership_posture: UiDeclaredMeasurementOwnershipPosture,
) -> UiInspectionMeasurementOwnershipPosture {
    match ownership_posture {
        UiDeclaredMeasurementOwnershipPosture::ScrollContainerBasis => {
            UiInspectionMeasurementOwnershipPosture::ScrollContainerBasis
        }
        UiDeclaredMeasurementOwnershipPosture::PortalAnchorBasisRequired => {
            UiInspectionMeasurementOwnershipPosture::PortalAnchorBasisRequired
        }
    }
}

pub(crate) fn project_neighborhood_class_hint(
    hint: UiMeasurementNeighborhoodClassHint,
) -> UiInspectionMeasurementNeighborhoodClassHint {
    match hint {
        UiMeasurementNeighborhoodClassHint::LocalIntrinsicContentDependency => {
            UiInspectionMeasurementNeighborhoodClassHint::LocalIntrinsicContentDependency
        }
        UiMeasurementNeighborhoodClassHint::ContainerAvailableSpaceDependency => {
            UiInspectionMeasurementNeighborhoodClassHint::ContainerAvailableSpaceDependency
        }
        UiMeasurementNeighborhoodClassHint::ViewportDependency => {
            UiInspectionMeasurementNeighborhoodClassHint::ViewportDependency
        }
        UiMeasurementNeighborhoodClassHint::ScrollContainerDependency => {
            UiInspectionMeasurementNeighborhoodClassHint::ScrollContainerDependency
        }
        UiMeasurementNeighborhoodClassHint::PortalAnchorDependency => {
            UiInspectionMeasurementNeighborhoodClassHint::PortalAnchorDependency
        }
    }
}
