use worth_query::facade::runtime::{
    WorthQueryGraphObligationIndex, WorthQueryGraphObligationKind,
    WorthQueryGraphObligationRegistrationCatalog,
    WorthQueryGraphObligationSelectorPerturbationCase, WorthQueryGraphObligationSupportLane,
    WorthQueryGraphTouchDescriptor, WorthQueryGraphTouchSelector,
};

use super::support::{committed_world, registration_for_kind};

#[test]
fn selector_axes_reject_neighboring_collection_aspect_operation_family_and_read_shape() {
    for case in
        WorthQueryGraphObligationSelectorPerturbationCase::milestone_9_9_selector_axis_cases()
    {
        let selected = selection_count(case.matching_selector(), case.matching_touch());
        assert_eq!(
            selected,
            1,
            "matching selector must select: {}",
            case.name()
        );

        let missed = selection_count(case.non_matching_selector(), case.non_matching_touch());
        assert_eq!(
            missed,
            0,
            "neighboring selector must not false-fire: {}",
            case.name()
        );
    }
}

fn selection_count(
    selector: WorthQueryGraphTouchSelector,
    touch: &WorthQueryGraphTouchDescriptor,
) -> usize {
    let registration = registration_for_kind(
        WorthQueryGraphObligationKind::BlockingInvariant,
        selector,
        WorthQueryGraphObligationSupportLane::GraphComposition,
    );
    let catalog =
        WorthQueryGraphObligationRegistrationCatalog::from_registrations(vec![registration])
            .unwrap();
    WorthQueryGraphObligationIndex::from_catalog(&catalog)
        .select_for_touch(touch, &committed_world())
        .matched_obligation_count()
}
