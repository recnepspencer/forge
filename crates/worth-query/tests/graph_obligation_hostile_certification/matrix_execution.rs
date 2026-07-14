use worth_query::facade::certification::WorthQueryGraphObligationMatrixCertificationCase;
use worth_query::facade::runtime::{
    WorthQueryGraphObligationIndex, WorthQueryGraphObligationMaterializedDispatch,
    WorthQueryGraphObligationRegistrationCatalog,
};

#[test]
fn every_kind_lane_row_selects_and_executes_with_declared_status() {
    for case in WorthQueryGraphObligationMatrixCertificationCase::milestone_9_9_authority_cases() {
        let catalog = WorthQueryGraphObligationRegistrationCatalog::from_registrations(vec![
            case.registration()
        ])
        .expect("single matrix-row registration catalog");
        let selection = WorthQueryGraphObligationIndex::from_catalog(&catalog)
            .select_for_touch(case.touch_descriptor(), case.operating_world());

        assert_eq!(selection.matched_obligation_count(), 1, "{case:?}");
        assert_eq!(selection.counters().registration_full_scan_count(), 0);

        let envelope = WorthQueryGraphObligationMaterializedDispatch::from_selection(selection)
            .selected_result_envelope();
        let result = envelope.rows().first().expect("selected matrix row result");

        assert_eq!(
            result.status(),
            case.expected_execution_status(),
            "{case:?}"
        );
    }
}
