pub(crate) fn place(
    recorder: &super::super::WorthUiHeadlessRecorder,
    request: worth_ui_host_contract::UiHostFocusPlacementRequest,
) -> worth_ui_host_contract::UiHostFocusPlacementAcknowledgement {
    let disposition = {
        let state = recorder.state.borrow();
        match state
            .retained_presentations
            .get(&request.presentation().binding())
        {
            Some(retained)
                if retained.frame == request.presentation().frame()
                    && retained.epoch == Some(request.presentation().epoch())
                    && retained
                        .node_positions
                        .contains_key(&request.target().mounted_instance())
                    && retained_node_receipt(retained, request.target().mounted_instance())
                        == Some(request.target().node_receipt()) =>
            {
                worth_ui_host_contract::UiHostFocusPlacementDisposition::Applied
            }
            Some(_) => {
                worth_ui_host_contract::UiHostFocusPlacementDisposition::RejectedBeforeEffect(
                    worth_ui_host_contract::UiHostFocusPlacementRejection::StalePresentation,
                )
            }
            None => worth_ui_host_contract::UiHostFocusPlacementDisposition::RejectedBeforeEffect(
                worth_ui_host_contract::UiHostFocusPlacementRejection::UnknownTarget,
            ),
        }
    };
    let acknowledgement =
        worth_ui_host_contract::UiHostFocusPlacementAcknowledgement::settled(request, disposition);
    recorder
        .state
        .borrow_mut()
        .semantic_focus
        .insert(request.host_session(), acknowledgement);
    acknowledgement
}

fn retained_node_receipt(
    retained: &super::super::headless_recorder::UiHeadlessRetainedPresentation,
    instance: worth_ui_host_contract::UiMountedInstanceIdentity,
) -> Option<worth_ui_host_contract::UiMountedNodeReceiptIdentity> {
    let receipt = retained.auxiliary.node_receipt_for(instance)?;
    Some(
        retained
            .receipt_affinity
            .map_or(receipt, |affinity| affinity.rebind_node_receipt(receipt)),
    )
}
