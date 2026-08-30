const PAGE_LOCATION: &str =
    include_str!("../src/physical_runtime/record_serving/access/locate/inline/page_location.rs");
const INLINE_RECORD: &str = include_str!(
    "../src/physical_runtime/record_serving/access/locate/inline/record_projection.rs"
);
const EXTENT_MANIFEST: &str = include_str!(
    "../src/physical_runtime/record_serving/access/locate/extent/manifest_admission.rs"
);
const EXTENT_SESSION: &str =
    include_str!("../src/physical_runtime/record_serving/access/extent_read_session.rs");
const PUBLISHED_TAIL: &str =
    include_str!("../src/physical_runtime/record_serving/planning/published_tail_page.rs");
const PAGE_ALLOCATION: &str = include_str!(
    "../src/physical_runtime/record_serving/planning/inline_segment_plan/page_allocation.rs"
);
const DURABLE_DATA: &str =
    include_str!("../src/physical_runtime/record_serving/publication/durable_data_plan.rs");

#[test]
fn clean_page_and_extent_consumers_use_integrity_admitted_projections() {
    for source in [
        PAGE_LOCATION,
        INLINE_RECORD,
        EXTENT_MANIFEST,
        EXTENT_SESSION,
        PUBLISHED_TAIL,
        PAGE_ALLOCATION,
        DURABLE_DATA,
    ] {
        assert_absent(source);
    }
    assert!(INLINE_RECORD.contains("work_semantics::integrity_admission"));
    assert!(EXTENT_MANIFEST.contains("work_semantics::integrity_admission"));
    assert!(EXTENT_SESSION.contains("work_semantics::integrity_admission"));
    assert!(PUBLISHED_TAIL.contains("integrity_admission::admit_inline_page"));
    assert!(DURABLE_DATA.contains("for_integrity_admitted_materialized_source"));
}

fn assert_absent(source: &str) {
    for forbidden in [
        "inspect_inline_page",
        "inspect_inline_page_records",
        "decode_inline_record",
        "decode_extent_chunk",
        "decode_data_frame_page_lsn",
        "DurableExtentManifest::decode",
    ] {
        assert!(
            !source.contains(forbidden),
            "raw clean route remains: {forbidden}"
        );
    }
}
