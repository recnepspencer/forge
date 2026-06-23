use forge_query::facade::ForgeQueryDerivedPatchPayload;

fn main() {
    let _ = ForgeQueryDerivedPatchPayload::from_retained_row(serde_json::json!({
        "value": "terminal"
    }));
}
