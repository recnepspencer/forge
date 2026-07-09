use crate::{StoreJsonResidueDenial, StoreJsonResidueOccurrence, StoreJsonResidueTokenKind};
use std::{fs, path::Path};

pub(crate) fn certify_store_test_preludes_do_not_export_json() -> Result<(), StoreJsonResidueDenial>
{
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("certification crate has a crates parent")
        .join("worth-store-test-support/src/lib.rs");
    let text = fs::read_to_string(&path)
        .map_err(|error| StoreJsonResidueDenial::SourceScanFailed(error.to_string()))?;
    scan_store_test_prelude_source(
        "workspaces/worth-store/crates/worth-store-test-support/src/lib.rs",
        &text,
    )
}

pub(crate) fn scan_store_test_prelude_source(
    path: &str,
    text: &str,
) -> Result<(), StoreJsonResidueDenial> {
    for (index, line) in text.lines().enumerate() {
        if !is_public_prelude_export(line) {
            continue;
        }
        if let Some(token) = exported_json_token(line) {
            return Err(StoreJsonResidueDenial::OrdinaryPreludeJsonExport(
                StoreJsonResidueOccurrence::new(
                    path,
                    index as u32 + 1,
                    token,
                    line.trim().to_string(),
                ),
            ));
        }
    }
    Ok(())
}

fn is_public_prelude_export(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with("pub use ")
        || trimmed.starts_with("pub fn ")
        || trimmed.starts_with("pub struct ")
        || trimmed.starts_with("pub enum ")
        || trimmed.starts_with("pub type ")
}

fn exported_json_token(line: &str) -> Option<StoreJsonResidueTokenKind> {
    if line.contains("serde_json") {
        Some(StoreJsonResidueTokenKind::SerdeJson)
    } else if line.contains("json!") {
        Some(StoreJsonResidueTokenKind::JsonMacro)
    } else if ["JsonDocument", "json_document", "fixture_json"]
        .iter()
        .any(|needle| line.contains(needle))
    {
        Some(StoreJsonResidueTokenKind::RawJsonHelper)
    } else {
        None
    }
}
