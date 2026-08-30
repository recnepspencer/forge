pub(crate) fn perform(
    state: &mut crate::native::UiNativeHostState,
    request: worth_ui_host_contract::UiHostFocusPlacementRequest,
) -> worth_ui_host_contract::UiHostFocusPlacementAcknowledgement {
    let binding = request.binding().diagnostic_value();
    let disposition = match state.retained_draw_lists.get(&binding) {
        Some(retained)
            if retained.frame() == request.presentation().frame()
                && state.presentation_epochs.get(&binding)
                    == Some(&request.presentation().epoch())
                && retained.owns_node_receipt(request.target().node_receipt()) =>
        {
            worth_ui_host_contract::UiHostFocusPlacementDisposition::Applied
        }
        Some(_) => worth_ui_host_contract::UiHostFocusPlacementDisposition::RejectedBeforeEffect(
            worth_ui_host_contract::UiHostFocusPlacementRejection::StalePresentation,
        ),
        None => worth_ui_host_contract::UiHostFocusPlacementDisposition::RejectedBeforeEffect(
            worth_ui_host_contract::UiHostFocusPlacementRejection::UnknownTarget,
        ),
    };
    let acknowledgement =
        worth_ui_host_contract::UiHostFocusPlacementAcknowledgement::settled(request, disposition);
    state
        .semantic_focus
        .insert(request.host_session(), acknowledgement);
    acknowledgement
}

#[cfg(test)]
mod tests {
    use super::*;
    use worth_ui_host_contract::{
        UiHostFocusPlacementDisposition, UiHostFocusPlacementRequest,
        UiHostFocusPlacementRequestIdentity, UiHostFocusPlacementRequestInput,
        UiHostObservationPresentationBasis, UiHostPresentationEpoch, UiHostProtocolContract,
        UiHostProtocolNegotiation, UiMountedFrameIdentity,
    };

    #[test]
    fn native_focus_applies_only_to_the_exact_presented_frame_epoch_and_node_receipt() {
        let world = crate::native::presentation::DrawListWorld::new();
        let frame = UiMountedFrameIdentity::mint_unbound().unwrap();
        let epoch = UiHostPresentationEpoch::issued_by_host(41);
        let host_session = 73;
        let (retained, target) = world.retained_focus_target(frame);
        let protocol = match UiHostProtocolContract::current().negotiate() {
            UiHostProtocolNegotiation::Compatible(protocol) => protocol,
            UiHostProtocolNegotiation::Incompatible(_) => unreachable!(),
        };
        let presentation = UiHostObservationPresentationBasis::new(
            world.requirement.host_surface(),
            frame,
            world.binding,
            epoch,
        );
        let request = UiHostFocusPlacementRequest::new(UiHostFocusPlacementRequestInput {
            identity: UiHostFocusPlacementRequestIdentity::new(1).unwrap(),
            protocol,
            host_session,
            host_surface: world.requirement.host_surface(),
            binding: world.binding,
            presentation,
            target,
        })
        .unwrap();
        let mut state = crate::native::UiNativeHostState::new();
        state
            .retained_draw_lists
            .insert(world.binding.diagnostic_value(), retained);
        state
            .presentation_epochs
            .insert(world.binding.diagnostic_value(), epoch);

        let acknowledgement = perform(&mut state, request);

        assert_eq!(
            acknowledgement.disposition(),
            UiHostFocusPlacementDisposition::Applied
        );
        assert_eq!(acknowledgement.request(), request);
        assert_eq!(
            state.semantic_focus.get(&host_session),
            Some(&acknowledgement)
        );

        let stale_presentation = UiHostObservationPresentationBasis::new(
            world.requirement.host_surface(),
            frame,
            world.binding,
            UiHostPresentationEpoch::issued_by_host(42),
        );
        let stale = UiHostFocusPlacementRequest::new(UiHostFocusPlacementRequestInput {
            identity: UiHostFocusPlacementRequestIdentity::new(2).unwrap(),
            protocol,
            host_session,
            host_surface: world.requirement.host_surface(),
            binding: world.binding,
            presentation: stale_presentation,
            target,
        })
        .unwrap();
        assert_eq!(
            perform(&mut state, stale).disposition(),
            UiHostFocusPlacementDisposition::RejectedBeforeEffect(
                worth_ui_host_contract::UiHostFocusPlacementRejection::StalePresentation
            )
        );
    }
}
