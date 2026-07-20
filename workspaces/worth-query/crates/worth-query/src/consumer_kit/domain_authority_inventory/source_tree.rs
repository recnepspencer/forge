use std::fs;
use std::io;
use std::path::Path;

use super::{
    audit_domain_authority_sources, WorthQueryDomainAuthorityInventoryAudit,
    WorthQueryDomainAuthoritySource,
};

pub fn current_domain_authority_inventory_audit(
) -> io::Result<WorthQueryDomainAuthorityInventoryAudit> {
    audit_domain_authority_tree(Path::new(env!("CARGO_MANIFEST_DIR")))
}

pub fn audit_workspace_domain_authority_inventory(
    repository_root: impl AsRef<Path>,
) -> io::Result<WorthQueryDomainAuthorityInventoryAudit> {
    audit_domain_authority_tree(&repository_root.as_ref().join("crates").join("worth-query"))
}

fn audit_domain_authority_tree(
    crate_root: &Path,
) -> io::Result<WorthQueryDomainAuthorityInventoryAudit> {
    let mut sources = Vec::new();
    collect_production_sources(&crate_root.join("src"), crate_root, &mut sources)?;
    Ok(audit_domain_authority_sources(&sources))
}

fn collect_production_sources(
    directory: &Path,
    crate_root: &Path,
    sources: &mut Vec<WorthQueryDomainAuthoritySource>,
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
            .expect("collected Query sources must remain below their crate root")
            .to_string_lossy()
            .replace('\\', "/");
        sources.push(WorthQueryDomainAuthoritySource::new(
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
