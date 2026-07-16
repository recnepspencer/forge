use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::identity::hash_parts;

const FORBIDDEN_CONSUMER_TOKENS: &[&str] = &[
    "SchemaFieldKind",
    "ScalarPredicateValue",
    "DerivedScalarField",
    "WorthQueryNativeRow",
    "derived_scalar_field",
];

pub(super) struct NativeValueConsumerSourceAudit {
    pub source_digest: String,
    pub findings: Vec<String>,
}

pub(super) fn audit_native_value_consumers(
    repository_root: &Path,
) -> io::Result<NativeValueConsumerSourceAudit> {
    let roots = [
        repository_root.join("crates/hadwiger-research/src"),
        repository_root.join("workspaces/worth-ui/crates"),
    ];
    let mut files = Vec::new();
    for root in roots {
        collect_rust_sources(&root, &mut files)?;
    }
    files.sort();

    let mut digest_parts = Vec::new();
    let mut findings = Vec::new();
    for path in files {
        let source = fs::read_to_string(&path)?;
        let relative = path
            .strip_prefix(repository_root)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
        digest_parts.push(format!("{relative}:{source}"));
        for token in FORBIDDEN_CONSUMER_TOKENS {
            for (line, _) in source.match_indices(token) {
                let line_number = source[..line].bytes().filter(|byte| *byte == b'\n').count() + 1;
                findings.push(format!("{relative}:{line_number}:{token}"));
            }
        }
    }
    Ok(NativeValueConsumerSourceAudit {
        source_digest: hash_parts(&digest_parts),
        findings,
    })
}

fn collect_rust_sources(directory: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    let mut entries = fs::read_dir(directory)?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|entry| entry.path());
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            collect_rust_sources(&path, files)?;
        } else if path.extension().and_then(|extension| extension.to_str()) == Some("rs") {
            files.push(path);
        }
    }
    Ok(())
}
