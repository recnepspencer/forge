const REDO_PLAN: &str = include_str!("../src/redo_replay/plan.rs");
const REDO_VALIDATION: &str = include_str!("../src/redo_replay/plan/projection_validation.rs");
const REDO_MATERIALIZATION: &str =
    include_str!("../src/redo_replay/plan/projection_materialization.rs");

#[test]
fn redo_projection_consumers_require_integrity_admission() {
    for source in [REDO_PLAN, REDO_VALIDATION, REDO_MATERIALIZATION] {
        for forbidden in [
            "inspect_inline_page_records",
            "decode_inline_record",
            "decode_extent_chunk",
            "decode_data_frame_page_lsn",
            "DurableExtentManifest::decode",
        ] {
            assert!(
                !source.contains(forbidden),
                "raw redo route remains: {forbidden}"
            );
        }
    }
    assert!(REDO_PLAN.contains("projection_admission"));
}
