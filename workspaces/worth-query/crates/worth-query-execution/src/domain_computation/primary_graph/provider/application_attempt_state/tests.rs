use super::{
    WorthQueryApplicationAttemptPhase as AttemptPhase, WorthQueryPrimaryGraphOverlay as Overlay,
};

fn overlay(identity: &str) -> Overlay {
    Overlay {
        identity: identity.to_owned(),
        facts: Vec::new(),
    }
}

#[test]
fn second_overlay_cannot_replace_the_registered_attempts_first_overlay() {
    let mut phase = AttemptPhase::Registered;
    phase.stage_overlay(overlay("overlay:first")).unwrap();

    assert_eq!(
        phase.stage_overlay(overlay("overlay:substitute")),
        Err("provider session cannot stage this application overlay")
    );
    let AttemptPhase::OverlayStaged(retained) = phase else {
        panic!("failed restaging must preserve the exact staged phase")
    };
    assert_eq!(retained.identity, "overlay:first");
}

#[test]
fn foreign_overlay_evidence_is_noninterfering_before_exact_discard() {
    let mut phase = AttemptPhase::Registered;
    phase.stage_overlay(overlay("overlay:owner")).unwrap();

    assert!(!phase.discard_overlay("overlay:foreign"));
    assert!(matches!(
        &phase,
        AttemptPhase::OverlayStaged(retained) if retained.identity == "overlay:owner"
    ));

    assert!(phase.discard_overlay("overlay:owner"));
    assert!(matches!(phase, AttemptPhase::Registered));
}
