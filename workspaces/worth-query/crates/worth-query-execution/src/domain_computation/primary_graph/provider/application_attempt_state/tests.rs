use super::{
    WorthQueryApplicationAttemptPhase as AttemptPhase, WorthQueryPrimaryGraphOverlay as Overlay,
};

fn overlay(identity: &str) -> Overlay {
    Overlay::new(identity.to_owned(), Vec::new())
}

#[test]
fn foreign_overlay_evidence_is_noninterfering_before_exact_discard() {
    let mut phase = AttemptPhase::registered();
    phase.stage_overlay(overlay("overlay:owner")).unwrap();

    assert!(!phase.discard_overlay("overlay:foreign"));
    assert_eq!(phase.overlay().expect("staged").identity(), "overlay:owner");

    assert!(phase.discard_overlay("overlay:owner"));
    assert!(phase.accepts_overlay());
}
