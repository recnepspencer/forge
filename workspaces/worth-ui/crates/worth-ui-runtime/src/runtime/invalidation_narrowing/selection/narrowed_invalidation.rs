use crate::runtime::UiAllocationInvalidationFamily;

#[derive(Debug, Eq, PartialEq)]
pub enum UiAllocationInvalidationTarget {
    Graph(crate::graph::UiAdmittedAllocationInvalidationTargetSet),
    ResizePreview {
        sample: crate::runtime::UiResizePreviewSample,
        target: crate::graph::UiAdmittedAllocationInvalidationTargetSet,
    },
    QueryProjection {
        basis: worth_ui_query_binding::WorthUiQueryAllocationInvalidationBasis,
        target: crate::graph::UiAdmittedAllocationInvalidationTargetSet,
    },
    ScrollOwnedContentExtent {
        basis: worth_ui_query_binding::WorthUiQueryAllocationInvalidationBasis,
        bindings: Box<[super::UiAdmittedScrollInvalidationBinding]>,
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

#[derive(Debug, Eq, PartialEq)]
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
