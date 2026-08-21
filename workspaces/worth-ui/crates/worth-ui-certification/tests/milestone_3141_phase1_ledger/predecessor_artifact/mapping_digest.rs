use serde_json::Value;
use sha2::{Digest, Sha256};

pub(super) fn calculate_mapping_digest(rows: &Value) -> String {
    let mut ordered = rows.as_array().cloned().unwrap_or_default();
    ordered.sort_by(|left, right| {
        left["requirement"]
            .as_str()
            .cmp(&right["requirement"].as_str())
    });
    let mut digest = Sha256::new();
    for row in ordered {
        for field in ["requirement", "production_entry", "independent_oracle"] {
            digest.update(row[field].as_str().unwrap_or_default().as_bytes());
            digest.update([0]);
        }
        for source in row["mapping_source_identity"]
            .as_array()
            .into_iter()
            .flatten()
        {
            digest.update(source.as_str().unwrap_or_default().as_bytes());
            digest.update([0]);
        }
        digest.update([0xff]);
    }
    format!("{:x}", digest.finalize())
}
