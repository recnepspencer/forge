use super::{
    UiClearedVisualOverlayReceipt, UiClearingVisualOverlay, UiPendingVisualOverlay,
    UiPublishedVisualOverlay, UiPublishingVisualOverlay, UiVisualOverlayIdentity,
    UiVisualOverlaySelection, UiVisualOverlayTarget, UiVisualOverlayTargetInput,
};

pub(crate) fn map_overlay_retention_denial(
    denial: crate::mounting::UiMountedVisualRetentionDenial,
) -> worth_ui_inspection::UiVisualOverlayDenial {
    match denial {
        crate::mounting::UiMountedVisualRetentionDenial::CapacityExceeded { .. }
        | crate::mounting::UiMountedVisualRetentionDenial::AccountingOverflow { .. } => {
            worth_ui_inspection::UiVisualOverlayDenial::CapacityExceeded
        }
        crate::mounting::UiMountedVisualRetentionDenial::ExpiredFrame
        | crate::mounting::UiMountedVisualRetentionDenial::UnknownFrame => {
            worth_ui_inspection::UiVisualOverlayDenial::Expired
        }
    }
}

pub(crate) fn published_overlay_cost(
    retained_structural_bytes: usize,
) -> worth_ui_inspection::UiVisualInspectionCostReceipt {
    let retained_structural_bytes = u64::try_from(retained_structural_bytes)
        .expect("an admitted mounted structural footprint fits the u64 policy domain");
    worth_ui_inspection::UiVisualInspectionCostReceipt::from_runtime_projection([
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        4,
        1,
        retained_structural_bytes,
    ])
}

pub(crate) const fn cleared_overlay_cost() -> worth_ui_inspection::UiVisualInspectionCostReceipt {
    worth_ui_inspection::UiVisualInspectionCostReceipt::from_runtime_projection([0; 11])
}

pub(crate) fn seal_overlay_target(input: UiVisualOverlayTargetInput) -> UiVisualOverlayTarget {
    UiVisualOverlayTarget {
        selection: UiVisualOverlaySelection {
            session: input.session,
            base_snapshot: input.base_snapshot,
            presentation: input.presentation,
            target_receipt: input.target_receipt,
            target_region: input.target_region,
            host_coordinate_transform: input.host_coordinate_transform,
            trace: input.trace,
        },
        lease: input.lease,
    }
}

pub(crate) fn seal_pending_overlay(
    identity: UiVisualOverlayIdentity,
    selection: UiVisualOverlaySelection,
    registration: super::super::UiPendingVisualOverlayRegistration,
) -> UiPendingVisualOverlay {
    UiPendingVisualOverlay {
        identity,
        selection,
        registration,
    }
}

pub(crate) fn seal_published_overlay(
    publishing: UiPublishingVisualOverlay,
    published_frame: worth_ui_host_contract::UiMountedFrameIdentity,
    cost: worth_ui_inspection::UiVisualInspectionCostReceipt,
) -> UiPublishedVisualOverlay {
    UiPublishedVisualOverlay {
        identity: publishing.identity,
        selection: publishing.selection,
        published_frame,
        cost,
    }
}

pub(crate) fn seal_cleared_overlay(
    clearing: UiClearingVisualOverlay,
    cleared_frame: worth_ui_host_contract::UiMountedFrameIdentity,
) -> UiClearedVisualOverlayReceipt {
    UiClearedVisualOverlayReceipt {
        identity: clearing.identity,
        session: clearing.selection.session,
        base_snapshot: clearing.selection.base_snapshot,
        base_frame: clearing.selection.presentation.frame,
        published_frame: clearing.published_frame,
        cleared_frame,
        cost: cleared_overlay_cost(),
    }
}
