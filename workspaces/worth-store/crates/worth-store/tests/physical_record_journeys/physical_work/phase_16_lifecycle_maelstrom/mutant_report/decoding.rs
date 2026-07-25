use serde::Deserialize;

#[derive(Deserialize)]
struct MutationReportHeader {
    schema: String,
}

pub(super) fn require_supported_schema(bytes: &[u8], expected: &str) -> Result<(), String> {
    let header: MutationReportHeader = serde_json::from_slice(bytes)
        .map_err(|error| format!("cannot decode mutation report header: {error}"))?;
    if header.schema != expected {
        return Err(format!(
            "unsupported mutation report schema `{}`",
            header.schema
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::require_supported_schema;

    #[test]
    fn legacy_schema_is_rejected_before_v2_body_decoding() {
        let error = require_supported_schema(
            br#"{"schema":"worth.store.c5_1.mutation-evidence.v1","observations":[]}"#,
            "worth.store.c5_1.mutation-evidence.v2",
        )
        .unwrap_err();

        assert!(
            error.contains("unsupported mutation report schema"),
            "{error}"
        );
        assert!(!error.contains("missing field"), "{error}");
    }
}
