use forge_query::facade::consumer_kit::ForgeQuerySupportSnapshotRow;

fn main() {
    let _ = ForgeQuerySupportSnapshotRow {
        surface: String::new(),
        facade_family: None,
        status: String::new(),
        teaching_posture: String::new(),
        owner_milestone: String::new(),
        extension_rule: String::new(),
        parallel_api_forbidden: false,
        admission_fail_closed: false,
        support_contract_digest: None,
        live_row_digest: String::new(),
        snapshot_row_digest: String::new(),
    };
}
