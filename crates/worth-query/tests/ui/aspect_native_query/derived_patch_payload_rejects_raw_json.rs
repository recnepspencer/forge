use worth_query::facade::runtime::WorthQueryDerivedPatchPayload;

fn main() {
    let _ = WorthQueryDerivedPatchPayload::from_retained_row(serde_json::json!({
        "value": "terminal"
    }));
}
