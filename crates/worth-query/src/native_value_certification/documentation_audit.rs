use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::identity::hash_parts;

struct DocumentationContract {
    path: &'static str,
    required: &'static [&'static str],
    forbidden: &'static [&'static str],
}

const CONTRACTS: &[DocumentationContract] = &[
    DocumentationContract {
        path: "crates/worth-query/docs/capabilities/native-aspect-values.md",
        required: &[
            "WorthQueryAuthoredAspectValue",
            "WorthQueryPredicateOperand",
            "ConsumedNativeValueView",
            "ConsumedNativeRefinementDenial",
            "AspectValue",
            "StructAspectValue",
            "native_value()",
            "as_struct()",
        ],
        forbidden: &[
            "SchemaFieldKind",
            "ScalarPredicateValue",
            "derived_scalar_field",
        ],
    },
    DocumentationContract {
        path: "crates/worth-query/docs/capabilities/projection-consumption.md",
        required: &["derived_field(...)", "native_value()", "as_struct()"],
        forbidden: &["derived_scalar_field"],
    },
    DocumentationContract {
        path: "crates/worth-query/docs/AI_README.md",
        required: &["./capabilities/native-aspect-values.md"],
        forbidden: &[
            "SchemaFieldKind",
            "ScalarPredicateValue",
            "derived_scalar_field",
        ],
    },
];

const FORBIDDEN_PRODUCT_DOC_TOKENS: &[&str] = &[
    "serde_json::Value",
    "WorthQueryNativeRow",
    "SchemaFieldKind",
    "ScalarPredicateValue",
    "derived_scalar_field",
];

pub(super) struct NativeValueDocumentationAudit {
    pub source_digest: String,
    pub disagreements: Vec<String>,
}

pub(super) fn audit_native_value_documentation(
    repository_root: &Path,
) -> io::Result<NativeValueDocumentationAudit> {
    let mut digest_parts = Vec::new();
    let mut disagreements = Vec::new();
    for contract in CONTRACTS {
        let source = fs::read_to_string(repository_root.join(contract.path))?;
        digest_parts.push(format!("{}:{source}", contract.path));
        for probe in contract.required {
            if !source.contains(probe) {
                disagreements.push(format!("{}:missing:{probe}", contract.path));
            }
        }
        for probe in contract.forbidden {
            if source.contains(probe) {
                disagreements.push(format!("{}:forbidden:{probe}", contract.path));
            }
        }
    }
    let mut product_docs = Vec::new();
    collect_markdown_sources(
        &repository_root.join("crates/worth-query/docs"),
        &mut product_docs,
    )?;
    product_docs.sort();
    for path in product_docs {
        let source = fs::read_to_string(&path)?;
        let relative = path
            .strip_prefix(repository_root)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
        digest_parts.push(format!("{relative}:{source}"));
        for token in FORBIDDEN_PRODUCT_DOC_TOKENS {
            for (offset, _) in source.match_indices(token) {
                let line_number = source[..offset]
                    .bytes()
                    .filter(|byte| *byte == b'\n')
                    .count()
                    + 1;
                disagreements.push(format!("{relative}:{line_number}:forbidden:{token}"));
            }
        }
    }
    Ok(NativeValueDocumentationAudit {
        source_digest: hash_parts(&digest_parts),
        disagreements,
    })
}

fn collect_markdown_sources(directory: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    let mut entries = fs::read_dir(directory)?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|entry| entry.path());
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            collect_markdown_sources(&path, files)?;
        } else if path.extension().and_then(|extension| extension.to_str()) == Some("md") {
            files.push(path);
        }
    }
    Ok(())
}
