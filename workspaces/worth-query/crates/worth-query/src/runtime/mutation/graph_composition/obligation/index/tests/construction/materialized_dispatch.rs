use crate::runtime::{
    WorthQueryGraphObligationIndex, WorthQueryGraphObligationMaterializedDispatch,
    WorthQueryGraphObligationOperatingWorldDescriptor,
    WorthQueryGraphObligationOperatingWorldSelector, WorthQueryGraphObligationSupportLane,
    WorthQueryGraphObligationSupportPosture,
};

use super::super::fixtures::{
    blocking_registration, catalog, collection_selector, relation_kind_id_selector,
    schema_registration, symbolic_relation_retirement_descriptor,
};

#[test]
fn materialized_dispatch_consumes_real_selection_without_losing_obligations() {
    let world = WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority();
    let descriptor = symbolic_relation_retirement_descriptor();
    let index = WorthQueryGraphObligationIndex::from_catalog(&catalog(vec![
        schema_registration("relation-kind", relation_kind_id_selector(), world)
            .with_support_posture(WorthQueryGraphObligationSupportPosture::supported(
                WorthQueryGraphObligationSupportLane::GraphComposition,
            )),
        blocking_registration("collection", collection_selector(), world).with_support_posture(
            WorthQueryGraphObligationSupportPosture::diagnostic_only(
                WorthQueryGraphObligationSupportLane::GraphComposition,
            ),
        ),
    ]));
    let selection = index.select_for_touch(
        &descriptor,
        &WorthQueryGraphObligationOperatingWorldDescriptor::any_committed_authority(),
    );

    let dispatch = WorthQueryGraphObligationMaterializedDispatch::from_selection(selection);
    let result_envelope = dispatch.selected_result_envelope();

    assert_eq!(dispatch.inputs().len(), 2);
    assert_eq!(result_envelope.rows().len(), 2);
    assert!(result_envelope
        .rows()
        .iter()
        .any(|row| row.status().as_str() == "executed"));
    assert!(result_envelope
        .rows()
        .iter()
        .any(|row| row.verdict().is_some_and(|verdict| verdict.is_advisory())));
}
