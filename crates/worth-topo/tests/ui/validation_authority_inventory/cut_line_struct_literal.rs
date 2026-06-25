use topology::validation_authority_inventory::{
    WorthValidationAuthorityCutLine, WorthValidationAuthorityInventoryCounters,
};

fn main() {
    let _cut_line = WorthValidationAuthorityCutLine {
        counters: WorthValidationAuthorityInventoryCounters::default(),
        ready_for_parallel_catalog_lane: true,
        cut_line_digest: "fake".to_string(),
    };
}
