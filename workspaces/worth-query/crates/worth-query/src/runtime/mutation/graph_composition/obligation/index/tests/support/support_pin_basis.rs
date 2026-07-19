use crate::runtime::{
    WorthQueryGraphObligationIndex, WorthQueryGraphObligationOperatingWorldDescriptor,
    WorthQueryGraphObligationOperatingWorldSelector, WorthQueryGraphObligationSupportLane,
    WorthQueryGraphObligationSupportPosture, WorthQueryGraphObligationSupportStatus,
};

use super::super::fixtures::{
    catalog, relation_kind_id_selector, schema_registration,
    symbolic_relation_retirement_descriptor,
};

#[test]
fn selection_exposes_support_posture_without_private_index_access() {
    let registration = schema_registration(
        "schema",
        relation_kind_id_selector(),
        WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    )
    .with_support_posture(WorthQueryGraphObligationSupportPosture::supported(
        WorthQueryGraphObligationSupportLane::GraphComposition,
    ));
    let index = WorthQueryGraphObligationIndex::from_catalog(&catalog(vec![registration]));
    let descriptor = symbolic_relation_retirement_descriptor();
    let world = WorthQueryGraphObligationOperatingWorldDescriptor::any_committed_authority();

    let selection = index.select_for_touch(&descriptor, &world);
    let postures = selection.matched_support_postures().collect::<Vec<_>>();

    assert_eq!(postures.len(), 1);
    assert_eq!(
        postures[0].lane(),
        WorthQueryGraphObligationSupportLane::GraphComposition
    );
    assert_eq!(postures[0].lane_label(), "graph-composition");
    assert_eq!(
        postures[0].status(),
        WorthQueryGraphObligationSupportStatus::Supported
    );
    assert_eq!(selection.matched_obligation_count(), 1);
}

#[test]
fn support_posture_drift_changes_selection_identity() {
    let supported = selection_digest_for(WorthQueryGraphObligationSupportPosture::supported(
        WorthQueryGraphObligationSupportLane::GraphComposition,
    ));
    let diagnostic =
        selection_digest_for(WorthQueryGraphObligationSupportPosture::diagnostic_only(
            WorthQueryGraphObligationSupportLane::GraphComposition,
        ));

    assert_ne!(supported, diagnostic);
}

fn selection_digest_for(support_posture: WorthQueryGraphObligationSupportPosture) -> String {
    let registration = schema_registration(
        "schema",
        relation_kind_id_selector(),
        WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    )
    .with_support_posture(support_posture);
    let index = WorthQueryGraphObligationIndex::from_catalog(&catalog(vec![registration]));
    let descriptor = symbolic_relation_retirement_descriptor();
    let world = WorthQueryGraphObligationOperatingWorldDescriptor::any_committed_authority();
    index
        .select_for_touch(&descriptor, &world)
        .selection_digest()
        .to_string()
}
