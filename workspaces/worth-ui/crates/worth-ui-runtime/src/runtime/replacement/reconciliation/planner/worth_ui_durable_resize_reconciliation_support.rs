use crate::capability::MosaicResizePermission;
use crate::runtime::replacement::reconciliation::plan::WorthUiDurableResizeInputDispositionInput;
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
        WorthUiDurableResizeInputDispositionInput {
            identity_basis: classification.identity_basis().to_owned(),
            authored_provenance_digest: classification.authored_provenance_digest(),
            family_id: family.id().clone(),
            transition: classification.transition(),
            resize_permission,
            resize_contract_id: classification.candidate_resize_contract_id()?.clone(),
            resize_shape_digest: classification.candidate_resize_shape_digest()?,
            posture: WorthUiDurableResizeInputPosture::AdmittedPlanningTimeOnly,
        },
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
        WorthUiNodeLifecycleTransition::Create
            if classification.candidate_has_restorable_splitter_state()
                && classification.candidate_resize_permission()
                    == Some(&MosaicResizePermission::UserResizable) =>
        {
            WorthUiDurableResizeInputPosture::AdmittedPlanningTimeOnly
        }
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
    let resize_contract_id = classification
        .candidate_resize_contract_id()
        .or_else(|| classification.active_resize_contract_id())?
        .clone();
    let resize_shape_digest = classification
        .candidate_resize_shape_digest()
        .or_else(|| classification.active_resize_shape_digest())?;
    Some(WorthUiDurableResizeInputDisposition::new(
        WorthUiDurableResizeInputDispositionInput {
            identity_basis: receipt.identity_basis().to_owned(),
            authored_provenance_digest: classification.authored_provenance_digest(),
            family_id: receipt.family_id().clone(),
            transition: classification.transition(),
            resize_permission,
            resize_contract_id,
            resize_shape_digest,
            posture,
        },
    ))
}

pub(super) fn initial_mounted_resize_input(
    definition: &crate::runtime::replacement::artifact_durable_state_definition::WorthUiArtifactDurableResizeDefinition,
) -> WorthUiDurableResizeInputDisposition {
    WorthUiDurableResizeInputDisposition::new(WorthUiDurableResizeInputDispositionInput {
        identity_basis: definition.identity_basis().to_owned(),
        authored_provenance_digest: Some(definition.authored_provenance_digest()),
        family_id: WorthUiDurableStateFamilyId::SplitterPosition,
        transition: WorthUiNodeLifecycleTransition::Create,
        resize_permission: definition.resize_permission().clone(),
        resize_contract_id: definition.resize_contract_id().clone(),
        resize_shape_digest: definition.resize_shape_digest(),
        posture: WorthUiDurableResizeInputPosture::AdmittedPlanningTimeOnly,
    })
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
