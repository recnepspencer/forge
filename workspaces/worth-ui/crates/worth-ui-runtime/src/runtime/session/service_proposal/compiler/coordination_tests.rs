use super::{
    UiServiceFamilyProposal, UiServiceProducedFactReference, UiServiceProposalCandidate,
    UiServiceProposalCompiler, UiServiceProposalDemand, UiServiceProposalReservationOutcome,
    UiServiceProposalStageReceipt,
};

#[test]
fn focus_reveal_and_declared_selection_share_one_atomic_compiled_proposal() {
    for (identity, disposition, accepted) in [
        (
            71,
            super::UiServiceProposalPublicationDisposition::Accepted,
            true,
        ),
        (
            72,
            super::UiServiceProposalPublicationDisposition::Rejected,
            false,
        ),
    ] {
        prove_focus_selection_settlement(identity, disposition, accepted);
    }
}

fn prove_focus_selection_settlement(
    identity: u64,
    disposition: super::UiServiceProposalPublicationDisposition,
    accepted: bool,
) {
    let reveal = super::super::UiFocusRevealRequirement::recorded_fixture();
    let scope =
        super::super::UiServiceProposalOccupancyScopeIdentity::for_mounted_owner(reveal.target());
    let (action, registration, key) = declared_selection(reveal.target());
    let families = [family::Focus, family::Scroll, family::Selection];
    let proposals = vec![
        UiServiceFamilyProposal::focus(scope),
        crate::runtime::scroll::UiStagedScrollServiceProposal::family_proposal(scope),
        crate::runtime::selection::UiStagedSelectionServiceProposal::family_proposal(action),
    ];
    let demand = UiServiceProposalDemand::recorded_fixture(
        super::super::UiServiceFamilyParticipation::from_families(&families).unwrap(),
        3,
        3,
        0,
    );
    let coherence = super::super::fixture_service_request_coherence(identity);
    let candidate =
        UiServiceProposalCandidate::for_test(identity, demand, coherence.clone(), proposals);
    let support = crate::capability::UiRuntimeServiceSupport::none_installed()
        .with_installed(family::Focus)
        .with_installed(family::Scroll)
        .with_installed(family::Selection);
    let mut compiler = UiServiceProposalCompiler::new();
    let preflighted = compiler.preflight(candidate, &coherence, support).unwrap();
    let UiServiceProposalReservationOutcome::Reserved(reservation) =
        compiler.reserve(preflighted).unwrap()
    else {
        unreachable!()
    };
    let proposal = reservation.identity();
    let scroll =
        crate::runtime::scroll::UiStagedScrollServiceProposal::prepare(proposal, scope, reveal);
    let mut selection_state =
        crate::runtime::selection::UiSelectionRuntimeState::new_session_restore_candidate();
    let selection = crate::runtime::selection::UiStagedDeclaredSelectionTransition::prepare(
        proposal,
        action,
        Some(registration.clone()),
        &selection_state,
    )
    .unwrap();
    let mut staging = compiler.begin_staging(reservation).unwrap();
    compiler
        .advance_staging(
            &mut staging,
            UiServiceProposalStageReceipt::recorded_family_fixture(
                proposal,
                family::Focus,
                scope,
                vec![UiServiceProducedFactReference::recorded_fixture(
                    81,
                    family::Focus,
                    scope,
                )],
                Vec::new(),
            ),
        )
        .unwrap();
    compiler
        .advance_staging(&mut staging, scroll.family_stage_receipt())
        .unwrap();
    compiler
        .advance_staging(&mut staging, selection.family_stage_receipt())
        .unwrap();
    compiler
        .advance_staging(
            &mut staging,
            UiServiceProposalStageReceipt::existing_preparation(proposal),
        )
        .unwrap();
    compiler
        .advance_staging(
            &mut staging,
            UiServiceProposalStageReceipt::focus_resolution(proposal, Some(scope)),
        )
        .unwrap();
    let batch = compiler.finish_staging(staging).unwrap();
    assert_eq!(batch.reveal_refinement(), Some(scope));
    assert_eq!(batch.fact_references().len(), 3);
    assert_eq!(scroll.requirement(), reveal);
    assert_eq!(selection.delta().added(), &[key]);
    assert!(selection_state.selected(action.owner()).is_none());

    let publication =
        super::UiServiceProposalPublicationReceipt::from_staged_batch(&batch, disposition);
    let mut settlement = compiler.begin_settlement(batch, publication).unwrap();
    for family in [family::Focus, family::Scroll] {
        compiler
            .acknowledge_owner(
                &mut settlement,
                super::UiServiceProposalOwnerAcknowledgement::from_family_owner(
                    publication,
                    family,
                    scope,
                ),
            )
            .unwrap();
    }
    compiler
        .acknowledge_owner(
            &mut settlement,
            selection.settlement_acknowledgement(publication),
        )
        .unwrap();
    compiler.finish_settlement(settlement).unwrap();
    if accepted {
        prove_same_owner_drift_rejects_commit(proposal, action, registration, &selection_state);
        let unrelated =
            install_unrelated_selection(&mut selection_state, action.owner().semantic_surface());
        selection.commit(&mut selection_state);
        assert_eq!(selection_state.selected(action.owner()).unwrap().len(), 1);
        assert_eq!(selection_state.selected(unrelated).unwrap().len(), 1);
    } else {
        drop(selection);
        assert!(selection_state.selected(action.owner()).is_none());
    }
    assert!(compiler.census().is_zero());
}

fn prove_same_owner_drift_rejects_commit(
    proposal: super::UiServiceProposalIdentity,
    action: super::super::UiDeclaredFocusSelectionAction,
    registration: crate::runtime::selection::UiSelectionRegistration,
    predecessor: &crate::runtime::selection::UiSelectionRuntimeState,
) {
    let mut drifted = predecessor.clone();
    let staged = crate::runtime::selection::UiStagedDeclaredSelectionTransition::prepare(
        proposal,
        action,
        Some(registration.clone()),
        &drifted,
    )
    .unwrap();
    drifted
        .synchronize_and_apply(registration, action.request())
        .unwrap();
    let selected_before_rejected_commit = drifted.selected(action.owner()).cloned();

    let rejection = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        staged.commit(&mut drifted);
    }));

    assert!(rejection.is_err());
    assert_eq!(
        drifted.selected(action.owner()).cloned(),
        selected_before_rejected_commit
    );
}

fn install_unrelated_selection(
    state: &mut crate::runtime::selection::UiSelectionRuntimeState,
    surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
) -> crate::runtime::selection::UiSelectionOwnerIdentity {
    let family =
        crate::runtime::UiApplicationItemKeyFamily::new(core::num::NonZeroU64::new(2).unwrap());
    let key = crate::runtime::selection::UiSelectionStableKey::new(
        crate::runtime::UiApplicationItemKey::new(family, core::num::NonZeroU64::new(2).unwrap()),
    );
    let owner = crate::runtime::selection::UiSelectionOwnerIdentity::new(
        surface,
        crate::graph::UiGraphNodeIdentity::new(315_599),
        family,
    );
    let incarnation = crate::runtime::selection::UiSelectionOwnerIncarnation::new(1).unwrap();
    let registration = crate::runtime::selection::UiSelectionRegistration::new(
        owner,
        incarnation,
        crate::runtime::selection::UiSelectionPolicy::Single,
        vec![key],
        crate::runtime::selection::UiSelectionCatalogPosture::Complete,
    )
    .unwrap();
    state
        .synchronize_and_apply(
            registration,
            crate::runtime::selection::UiSelectionRequest::SelectSingle(key),
        )
        .unwrap();
    owner
}

fn declared_selection(
    target: worth_ui_host_contract::UiMountedInstanceIdentity,
) -> (
    super::super::UiDeclaredFocusSelectionAction,
    crate::runtime::selection::UiSelectionRegistration,
    crate::runtime::selection::UiSelectionStableKey,
) {
    let item_family =
        crate::runtime::UiApplicationItemKeyFamily::new(core::num::NonZeroU64::new(1).unwrap());
    let key = crate::runtime::selection::UiSelectionStableKey::new(
        crate::runtime::UiApplicationItemKey::new(
            item_family,
            core::num::NonZeroU64::new(1).unwrap(),
        ),
    );
    let fixture = super::super::UiDeclaredFocusSelectionAction::recorded_fixture(target);
    let owner = fixture.owner();
    let incarnation = fixture.incarnation();
    let action = super::super::UiDeclaredFocusSelectionAction::new(
        target,
        owner,
        incarnation,
        crate::runtime::selection::UiSelectionRequest::SelectSingle(key),
        super::super::UiSelectionInvocationCause::Keyboard,
    );
    assert_eq!(
        action.cause(),
        super::super::UiSelectionInvocationCause::Keyboard
    );
    let pointer = super::super::UiDeclaredFocusSelectionAction::new(
        target,
        owner,
        incarnation,
        crate::runtime::selection::UiSelectionRequest::SelectSingle(key),
        super::super::UiSelectionInvocationCause::Pointer,
    );
    assert_eq!(
        pointer.cause(),
        super::super::UiSelectionInvocationCause::Pointer
    );
    let registration = crate::runtime::selection::UiSelectionRegistration::new(
        owner,
        incarnation,
        crate::runtime::selection::UiSelectionPolicy::Single,
        vec![key],
        crate::runtime::selection::UiSelectionCatalogPosture::Complete,
    )
    .unwrap();
    (action, registration, key)
}

#[allow(non_camel_case_types)]
type family = crate::capability::UiRuntimeServiceFamily;
