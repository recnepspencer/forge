use std::fs;
use std::io;
use std::path::Path;

use super::{
    audit_native_value_authority_sources, WorthQueryNativeValueAuthorityAudit,
    WorthQueryNativeValueSource,
};

pub fn current_native_value_authority_audit() -> io::Result<WorthQueryNativeValueAuthorityAudit> {
    audit_native_value_authority_tree(Path::new(env!("CARGO_MANIFEST_DIR")))
}

fn audit_native_value_authority_tree(
    crate_root: &Path,
) -> io::Result<WorthQueryNativeValueAuthorityAudit> {
    let mut sources = Vec::new();
    collect_production_sources(&crate_root.join("src"), crate_root, &mut sources)?;
    Ok(audit_native_value_authority_sources(&sources))
}

fn collect_production_sources(
    directory: &Path,
    crate_root: &Path,
    sources: &mut Vec<WorthQueryNativeValueSource>,
) -> io::Result<()> {
    let mut entries = fs::read_dir(directory)?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|entry| entry.path());
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            if path.file_name().and_then(|name| name.to_str()) != Some("tests") {
                collect_production_sources(&path, crate_root, sources)?;
            }
            continue;
        }
        if !is_production_rust_source(&path) {
            continue;
        }
        let relative = path
            .strip_prefix(crate_root)
            .expect("collected Query source must remain below its crate root")
            .to_string_lossy()
            .replace('\\', "/");
        if relative.starts_with("src/consumer_kit/native_value_authority_inventory/") {
            continue;
        }
        sources.push(WorthQueryNativeValueSource::new(
            relative,
            fs::read_to_string(path)?,
        ));
    }
    Ok(())
}

fn is_production_rust_source(path: &Path) -> bool {
    path.extension().and_then(|extension| extension.to_str()) == Some("rs")
        && !path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .is_some_and(|stem| stem == "integration_tests" || stem.ends_with("_tests"))
}
