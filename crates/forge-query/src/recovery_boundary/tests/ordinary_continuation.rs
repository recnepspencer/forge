use crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts;
use crate::ordinary_outcome::{
    ForgeQueryOrdinaryCheckedTopology, ForgeQueryOrdinaryContinuationCheckedTopologyKind,
    ForgeQueryOrdinaryNextStep, ForgeQueryOrdinaryOutcome, ForgeQueryOrdinaryPosture,
    ForgeQueryOrdinaryPostureKind,
};
use crate::recovery_boundary::{
    forge_query_recovery_brief_from_ordinary_outcome, ForgeQueryRecoveryAction,
    ForgeQueryRecoveryStopFamily, ForgeQueryRecoveryStopKind,
};

#[test]
fn ordinary_continuation_wrong_world_maps_to_world_repair() {
    let brief = forge_query_recovery_brief_from_ordinary_outcome(
        &ForgeQueryOrdinaryOutcome::<()>::WrongWorld(ForgeQueryOrdinaryPosture::new(
            "wrong world",
            ForgeQueryOrdinaryPostureKind::WrongWorld,
            ForgeQueryOrdinaryNextStep::CorrectWorld,
            ForgeQueryOrdinaryCheckedTopology::continuation(
                ForgeQueryOrdinaryContinuationCheckedTopologyKind::WrongWorld,
                ForgeQueryBindingLinkedArtifacts::new().with_envelope_digest("env-1"),
            ),
        )),
    )
    .expect("wrong-world continuation should yield a recovery brief");

    assert_eq!(
        brief.stop_family(),
        ForgeQueryRecoveryStopFamily::Continuation
    );
    assert_eq!(brief.stop_kind(), ForgeQueryRecoveryStopKind::WrongWorld);
    assert_eq!(
        brief.recommended_action(),
        ForgeQueryRecoveryAction::CorrectWorld
    );
}

#[test]
fn ordinary_continuation_stale_completion_keeps_typed_stop_kind() {
    let brief = forge_query_recovery_brief_from_ordinary_outcome(
        &ForgeQueryOrdinaryOutcome::<()>::Stale(ForgeQueryOrdinaryPosture::new(
            "completion is stale",
            ForgeQueryOrdinaryPostureKind::Stale,
            ForgeQueryOrdinaryNextStep::RefreshBasis,
            ForgeQueryOrdinaryCheckedTopology::continuation(
                ForgeQueryOrdinaryContinuationCheckedTopologyKind::StaleCompletion,
                ForgeQueryBindingLinkedArtifacts::new()
                    .with_envelope_digest("env-stale-completion"),
            ),
        )),
    )
    .expect("stale completion should yield a recovery brief");

    assert_eq!(
        brief.stop_family(),
        ForgeQueryRecoveryStopFamily::Continuation
    );
    assert_eq!(
        brief.stop_kind(),
        ForgeQueryRecoveryStopKind::StaleCompletion
    );
    assert_eq!(
        brief.recommended_action(),
        ForgeQueryRecoveryAction::RefreshBasis
    );
}

#[test]
fn ordinary_continuation_preview_crossed_residue_keeps_typed_stop_kind() {
    let brief = forge_query_recovery_brief_from_ordinary_outcome(
        &ForgeQueryOrdinaryOutcome::<()>::RebindRequired(ForgeQueryOrdinaryPosture::new(
            "preview residue crossed",
            ForgeQueryOrdinaryPostureKind::RebindRequired,
            ForgeQueryOrdinaryNextStep::UseExplicitHandoff,
            ForgeQueryOrdinaryCheckedTopology::continuation(
                ForgeQueryOrdinaryContinuationCheckedTopologyKind::PreviewCrossedResidue,
                ForgeQueryBindingLinkedArtifacts::new().with_envelope_digest("env-preview-residue"),
            ),
        )),
    )
    .expect("preview-crossed residue should yield a recovery brief");

    assert_eq!(
        brief.stop_family(),
        ForgeQueryRecoveryStopFamily::Continuation
    );
    assert_eq!(
        brief.stop_kind(),
        ForgeQueryRecoveryStopKind::PreviewCrossedResidue
    );
    assert_eq!(
        brief.recommended_action(),
        ForgeQueryRecoveryAction::UseExplicitHandoff
    );
}
