use crate::runtime::{
    ForgeQueryGraphObligationIndex, ForgeQueryGraphObligationMaterializedDispatch,
    ForgeQueryGraphObligationOperatingWorldDescriptor,
    ForgeQueryGraphObligationOperatingWorldSelector, ForgeQueryGraphObligationSupportLane,
    ForgeQueryGraphObligationSupportPosture,
};

use super::super::fixtures::{
    blocking_registration, catalog, collection_selector, relation_kind_id_selector,
    schema_registration, symbolic_relation_retirement_descriptor,
};

#[test]
fn materialized_dispatch_consumes_real_selection_without_losing_obligations() {
    let world = ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority();
    let descriptor = symbolic_relation_retirement_descriptor();
    let index = ForgeQueryGraphObligationIndex::from_catalog(&catalog(vec![
        schema_registration("relation-kind", relation_kind_id_selector(), world)
            .with_support_posture(ForgeQueryGraphObligationSupportPosture::supported(
                ForgeQueryGraphObligationSupportLane::GraphComposition,
            )),
        blocking_registration("collection", collection_selector(), world).with_support_posture(
            ForgeQueryGraphObligationSupportPosture::diagnostic_only(
                ForgeQueryGraphObligationSupportLane::GraphComposition,
            ),
        ),
    ]));
    let selection = index.select_for_touch(
        &descriptor,
        &ForgeQueryGraphObligationOperatingWorldDescriptor::any_committed_authority(),
    );

    let dispatch = ForgeQueryGraphObligationMaterializedDispatch::from_selection(selection);
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
