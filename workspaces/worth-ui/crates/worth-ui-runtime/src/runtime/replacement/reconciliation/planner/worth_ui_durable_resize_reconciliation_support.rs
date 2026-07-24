use crate::capability::MosaicResizePermission;
use crate::runtime::{
    WorthUiDurableResizeInputDisposition, WorthUiDurableResizeInputPosture,
    WorthUiDurableStateFamily, WorthUiDurableStateFamilyId,
    WorthUiDurableStateReconciliationReceipt, WorthUiIdentityMatchNodeKind,
    WorthUiNodeLifecycleTransition, WorthUiNodeReplacementClassification,
};

pub(super) fn classification_targets_splitter_surface(
    classification: &WorthUiNodeReplacementClassification,
) -> bool {
    matches!(
        classification
            .active_kind()
            .or(classification.candidate_kind()),
        Some(WorthUiIdentityMatchNodeKind::Surface)
    )
}

pub(super) fn splitter_resize_input_for_carry(
    classification: &WorthUiNodeReplacementClassification,
    family: &WorthUiDurableStateFamily,
) -> Option<WorthUiDurableResizeInputDisposition> {
    if !is_splitter_surface_family(classification, family)
        || !classification.active_has_restorable_splitter_state()
        || !classification.candidate_has_restorable_splitter_state()
        || !splitter_resize_shapes_match(classification)
    {
        return None;
    }
    let resize_permission = classification
        .candidate_resize_permission()
        .filter(|permission| **permission == MosaicResizePermission::UserResizable)?
        .clone();
    Some(WorthUiDurableResizeInputDisposition::new(
        classification.identity_basis().to_owned(),
        classification.authored_provenance_digest(),
        family.id().clone(),
        classification.transition(),
        resize_permission,
        WorthUiDurableResizeInputPosture::AdmittedPlanningTimeOnly,
        true,
    ))
}

pub(super) fn splitter_resize_input_for_replacement(
    classification: &WorthUiNodeReplacementClassification,
    family: &WorthUiDurableStateFamily,
    receipt: &WorthUiDurableStateReconciliationReceipt,
) -> Option<WorthUiDurableResizeInputDisposition> {
    if !is_splitter_surface_family(classification, family) {
        return None;
    }

    let posture = match classification.transition() {
        WorthUiNodeLifecycleTransition::LaneChange
            if classification.candidate_resize_permission()
                == Some(&MosaicResizePermission::UserResizable) =>
        {
            WorthUiDurableResizeInputPosture::RemappedForChangedResizeLane
        }
        WorthUiNodeLifecycleTransition::Preserve
        | WorthUiNodeLifecycleTransition::Move
        | WorthUiNodeLifecycleTransition::Rebind
        | WorthUiNodeLifecycleTransition::Replace
        | WorthUiNodeLifecycleTransition::LaneChange => {
            WorthUiDurableResizeInputPosture::DeniedIncompatibleCarryForwardShape
        }
        WorthUiNodeLifecycleTransition::Create | WorthUiNodeLifecycleTransition::Drop => {
            return None;
        }
    };

    let resize_permission = classification
        .candidate_resize_permission()
        .or_else(|| classification.active_resize_permission())
        .cloned()
        .unwrap_or_else(MosaicResizePermission::missing_for_diagnostics);
    Some(WorthUiDurableResizeInputDisposition::new(
        receipt.identity_basis().to_owned(),
        classification.authored_provenance_digest(),
        receipt.family_id().clone(),
        classification.transition(),
        resize_permission,
        posture,
        true,
    ))
}

fn is_splitter_surface_family(
    classification: &WorthUiNodeReplacementClassification,
    family: &WorthUiDurableStateFamily,
) -> bool {
    family.id() == &WorthUiDurableStateFamilyId::SplitterPosition
        && classification_targets_splitter_surface(classification)
        && (classification.active_has_restorable_splitter_state()
            || classification.candidate_has_restorable_splitter_state())
}

fn splitter_resize_shapes_match(classification: &WorthUiNodeReplacementClassification) -> bool {
    classification.active_resize_contract_id() == classification.candidate_resize_contract_id()
        && classification.active_resize_permission() == classification.candidate_resize_permission()
        && classification.active_resize_shape_digest()
            == classification.candidate_resize_shape_digest()
}
