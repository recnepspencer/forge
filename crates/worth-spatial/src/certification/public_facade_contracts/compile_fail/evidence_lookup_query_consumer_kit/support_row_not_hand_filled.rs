use forge_query::facade::runtime::ForgeQueryRuntimeFacadeFamily;
use worth_spatial::facade::evidence_lookup_query_consumer_kit::EvidenceLookupQuerySupportPinRow;

fn main() {
    let _ = EvidenceLookupQuerySupportPinRow {
        runtime_family: ForgeQueryRuntimeFacadeFamily::Read,
        query_support_surface: String::new(),
        snapshot_row_digest: String::new(),
        support_pin_report_digest: String::new(),
        row_digest: String::new(),
    };
}
