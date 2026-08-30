use std::collections::HashSet;

use worth_ui_host_contract::{UiMountedInstanceIdentity, UiMountedPaintCommandIdentity};
use worth_ui_host_headless::UiHeadlessMountedFrameTranscript;

pub(super) fn exact_portal_child_commands(
    transcript: &UiHeadlessMountedFrameTranscript,
    portal_child: UiMountedInstanceIdentity,
) -> HashSet<UiMountedPaintCommandIdentity> {
    let fills = transcript
        .filled_rects()
        .iter()
        .filter(|mechanic| mechanic.mounted_instance() == portal_child)
        .map(|mechanic| mechanic.command_identity())
        .collect::<Vec<_>>();
    let texts = transcript
        .semantic_text()
        .iter()
        .filter(|mechanic| mechanic.mounted_instance() == portal_child)
        .collect::<Vec<_>>();

    assert_eq!(fills.len(), 1, "the Portal child emits its authored fill");
    assert_eq!(
        texts.len(),
        2,
        "the real Portal child must emit semantic text into the sampled command group; observed semantic text: {:?}",
        transcript
            .semantic_text()
            .iter()
            .map(|mechanic| (mechanic.mounted_instance(), mechanic.text()))
            .collect::<Vec<_>>()
    );
    assert_eq!(
        texts
            .iter()
            .map(|mechanic| mechanic.text())
            .collect::<HashSet<_>>(),
        HashSet::from(["Portal motion content", " "]),
        "the child emits its authored value and retained posture rows"
    );

    fills
        .into_iter()
        .chain(
            texts
                .into_iter()
                .map(|mechanic| mechanic.command_identity()),
        )
        .collect()
}
