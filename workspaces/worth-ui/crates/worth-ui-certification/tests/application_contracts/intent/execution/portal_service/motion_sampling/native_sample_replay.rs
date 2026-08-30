use worth_ui_host_contract::{UiMountedPaintCommandIdentity, UiMountedPortalOverlayMechanic};
use worth_ui_host_headless::UiHeadlessPresentationSampleObservation;

pub(super) fn assert_first_entrance_sample_clears_published_successor(
    portal: UiMountedPortalOverlayMechanic,
    sample: &UiHeadlessPresentationSampleObservation,
) {
    let portal_identity = UiMountedPaintCommandIdentity::portal_overlay(&portal);
    let portal_change = sample
        .changes()
        .iter()
        .find(|change| change.command() == portal_identity)
        .copied()
        .expect("the first production entrance sample moves the Portal overlay");
    let portal_transform = portal_change
        .transform()
        .expect("the production Portal entrance sample carries its geometry transform");
    assert_eq!(portal_transform.source(), portal.bounds());
    let replay = worth_ui_host_native::certify_portal_sample_replay(
        portal,
        portal_change,
        sample.damage(),
        [1_024, 1_024],
        1.0,
    )
    .expect("the production entrance sample composes through native replay");

    assert_eq!(
        replay.normalized_damage(),
        [portal_transform.source(), portal_transform.sampled()],
        "native replay must clear the exact published successor and sampled entrance geometry"
    );
    assert_eq!(
        replay.published_successor_top_pixel(),
        [0, 0, 0, 0],
        "the first entrance sample must remove pixels from the already-published successor top edge"
    );
}
