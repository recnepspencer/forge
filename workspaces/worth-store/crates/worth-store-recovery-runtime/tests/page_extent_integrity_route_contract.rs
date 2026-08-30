const INLINE: &str =
    include_str!("../src/orchestration/planning/page_observation/materialized/inline.rs");
const EXTENT: &str =
    include_str!("../src/orchestration/planning/page_observation/materialized/extent.rs");

#[test]
fn recovery_page_and_extent_materialization_requires_integrity_ingress() {
    for source in [INLINE, EXTENT] {
        for forbidden in [
            "inspect_inline_page",
            "decode_data_frame_page_lsn",
            "decode_extent_chunk",
            "DurableExtentManifest::decode",
        ] {
            assert!(
                !source.contains(forbidden),
                "raw recovery route remains: {forbidden}"
            );
        }
        assert!(source.contains("crate::integrity_ingress::admit_"));
    }
}
