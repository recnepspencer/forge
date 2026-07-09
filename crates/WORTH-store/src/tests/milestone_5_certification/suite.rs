use super::*;

pub(super) fn milestone_5_suite() -> CertificationSuite<String, String> {
    let no_edit = no_edit_bundle();
    let small_in_memory = small_edit_bundle_in_memory();
    let small_sqlite = small_edit_bundle_sqlite();
    let deep = deep_edit_bundle();
    let rewritten_in_memory = rewritten_bundle_in_memory();
    let rewritten_sqlite = rewritten_bundle_sqlite();

    CertificationSuite::new(BRANCH_DELTA_PROPORTIONALITY_AND_REPLAY_PARITY_TEST.suite_name)
        .with_canonical_row(CanonicalRow::new(
            "backend_variation_parity",
            vec![
                LaneResult::new("in_memory", small_in_memory.canonical_json()),
                LaneResult::new("sqlite", small_sqlite.canonical_json()),
            ],
            &[AssertionClass::Equality, AssertionClass::ExactCounter],
        ))
        .with_canonical_row(CanonicalRow::new(
            "delta_growth_tracks_semantic_delta",
            vec![
                LaneResult::new(
                    "no_edit",
                    serde_json::to_string(&no_edit.delta_storage_report).unwrap(),
                ),
                LaneResult::new(
                    "small_edit",
                    serde_json::to_string(&small_in_memory.delta_storage_report).unwrap(),
                ),
                LaneResult::new(
                    "deep_edit",
                    serde_json::to_string(&deep.delta_storage_report).unwrap(),
                ),
            ],
            &[AssertionClass::Inequality, AssertionClass::ExactCounter],
        ))
        .with_canonical_row(CanonicalRow::new(
            "rewritten_stack_control_lane_parity",
            vec![
                LaneResult::new("in_memory", rewritten_in_memory.canonical_json()),
                LaneResult::new("sqlite", rewritten_sqlite.canonical_json()),
            ],
            &[AssertionClass::Equality, AssertionClass::ExactCounter],
        ))
}
