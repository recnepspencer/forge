use crate::runtime::UiAllocationInvalidationFamily;

#[derive(Debug, PartialEq)]
pub enum UiAllocationInvalidationTarget {
    Graph(crate::graph::UiAdmittedAllocationInvalidationTargetSet),
    ResizePreview {
        sample: crate::runtime::UiResizePreviewSample,
        target: crate::graph::UiAdmittedAllocationInvalidationTargetSet,
    },
    SettledQueryFact {
        target: crate::graph::UiAdmittedAllocationInvalidationTargetSet,
        view_binding_id: crate::capability::ViewBindingId,
        fact: std::sync::Arc<worth_ui_query_binding::WorthUiSettledSnapshotFact>,
    },
    ScrollOwnedContentExtent {
        bindings: Box<[super::UiAdmittedScrollInvalidationBinding]>,
        view_binding_id: crate::capability::ViewBindingId,
        fact: std::sync::Arc<worth_ui_query_binding::WorthUiSettledSnapshotFact>,
    },
    HostMeasurement {
        evidence_generation: worth_ui_inspection::UiEvidenceAuthorityGeneration,
        target: crate::graph::UiAdmittedAllocationInvalidationTargetSet,
    },
    PortalAnchor {
        movement: Box<super::UiAdmittedPortalMovement>,
    },
    ScrollOwnedExtent {
        evidence_generation: worth_ui_inspection::UiEvidenceAuthorityGeneration,
        bindings: Box<[super::UiAdmittedScrollInvalidationBinding]>,
    },
    DurableResize {
        identity_digest: u64,
        extent: crate::runtime::UiResizeLogicalExtent,
        target: crate::graph::UiAdmittedAllocationInvalidationTargetSet,
    },
}

#[derive(Debug, PartialEq)]
pub struct UiNarrowedAllocationInvalidation {
    family: UiAllocationInvalidationFamily,
    target: UiAllocationInvalidationTarget,
}

impl UiNarrowedAllocationInvalidation {
    pub(super) fn new(
        family: UiAllocationInvalidationFamily,
        target: UiAllocationInvalidationTarget,
    ) -> Self {
        Self { family, target }
    }
    pub fn family(&self) -> UiAllocationInvalidationFamily {
        self.family
    }
    pub fn target(&self) -> &UiAllocationInvalidationTarget {
        &self.target
    }
}
