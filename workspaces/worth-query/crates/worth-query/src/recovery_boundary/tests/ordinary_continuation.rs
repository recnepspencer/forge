use crate::binding_pipeline::WorthQueryBindingLinkedArtifacts;
use crate::ordinary_outcome::{
    WorthQueryOrdinaryCheckedTopology, WorthQueryOrdinaryContinuationCheckedTopologyKind,
    WorthQueryOrdinaryNextStep, WorthQueryOrdinaryOutcome, WorthQueryOrdinaryPosture,
    WorthQueryOrdinaryPostureKind,
};
use crate::recovery_boundary::{
    worth_query_recovery_brief_from_ordinary_outcome, WorthQueryRecoveryAction,
    WorthQueryRecoveryStopFamily, WorthQueryRecoveryStopKind,
};

#[test]
fn ordinary_continuation_wrong_world_maps_to_world_repair() {
    let brief = worth_query_recovery_brief_from_ordinary_outcome(
        &WorthQueryOrdinaryOutcome::<()>::WrongWorld(WorthQueryOrdinaryPosture::new(
            "wrong world",
            WorthQueryOrdinaryPostureKind::WrongWorld,
            WorthQueryOrdinaryNextStep::CorrectWorld,
            WorthQueryOrdinaryCheckedTopology::continuation(
                WorthQueryOrdinaryContinuationCheckedTopologyKind::WrongWorld,
                WorthQueryBindingLinkedArtifacts::new().with_envelope_digest("env-1"),
            ),
        )),
    )
    .expect("wrong-world continuation should yield a recovery brief");

    assert_eq!(
        brief.stop_family(),
        WorthQueryRecoveryStopFamily::Continuation
    );
    assert_eq!(brief.stop_kind(), WorthQueryRecoveryStopKind::WrongWorld);
    assert_eq!(
        brief.recommended_action(),
        WorthQueryRecoveryAction::CorrectWorld
    );
}

#[test]
fn ordinary_continuation_stale_completion_keeps_typed_stop_kind() {
    let brief = worth_query_recovery_brief_from_ordinary_outcome(
        &WorthQueryOrdinaryOutcome::<()>::Stale(WorthQueryOrdinaryPosture::new(
            "completion is stale",
            WorthQueryOrdinaryPostureKind::Stale,
            WorthQueryOrdinaryNextStep::RefreshBasis,
            WorthQueryOrdinaryCheckedTopology::continuation(
                WorthQueryOrdinaryContinuationCheckedTopologyKind::StaleCompletion,
                WorthQueryBindingLinkedArtifacts::new()
                    .with_envelope_digest("env-stale-completion"),
            ),
        )),
    )
    .expect("stale completion should yield a recovery brief");

    assert_eq!(
        brief.stop_family(),
        WorthQueryRecoveryStopFamily::Continuation
    );
    assert_eq!(
        brief.stop_kind(),
        WorthQueryRecoveryStopKind::StaleCompletion
    );
    assert_eq!(
        brief.recommended_action(),
        WorthQueryRecoveryAction::RefreshBasis
    );
}

#[test]
fn ordinary_continuation_preview_crossed_residue_keeps_typed_stop_kind() {
    let brief = worth_query_recovery_brief_from_ordinary_outcome(
        &WorthQueryOrdinaryOutcome::<()>::RebindRequired(WorthQueryOrdinaryPosture::new(
            "preview residue crossed",
            WorthQueryOrdinaryPostureKind::RebindRequired,
            WorthQueryOrdinaryNextStep::UseExplicitHandoff,
            WorthQueryOrdinaryCheckedTopology::continuation(
                WorthQueryOrdinaryContinuationCheckedTopologyKind::PreviewCrossedResidue,
                WorthQueryBindingLinkedArtifacts::new().with_envelope_digest("env-preview-residue"),
            ),
        )),
    )
    .expect("preview-crossed residue should yield a recovery brief");

    assert_eq!(
        brief.stop_family(),
        WorthQueryRecoveryStopFamily::Continuation
    );
    assert_eq!(
        brief.stop_kind(),
        WorthQueryRecoveryStopKind::PreviewCrossedResidue
    );
    assert_eq!(
        brief.recommended_action(),
        WorthQueryRecoveryAction::UseExplicitHandoff
    );
}
