use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DoctestKind {
    Runnable,
    CompileFail,
    Ignored,
}

pub(super) struct DeclaredDoctest {
    pub(super) stable_case_name: String,
    pub(super) source_line: usize,
    pub(super) kind: DoctestKind,
}

pub(super) fn declared_doctests(
    rust_source_path: &Path,
    source: &str,
) -> Result<Vec<DeclaredDoctest>, String> {
    let mut digest_occurrences = BTreeMap::<String, usize>::new();
    let mut declared = parse_doctests(source, &["///", "//!"], 0, &mut digest_occurrences);
    let included_paths = included_documentation_paths(rust_source_path, source);
    let mut included_line_offset = included_paths
        .first()
        .map(|(_, declaration_line)| declaration_line.saturating_sub(1))
        .unwrap_or_default();
    for (included_path, _) in included_paths {
        let markdown = std::fs::read_to_string(&included_path).map_err(|error| {
            format!(
                "could not read included documentation {}: {error}",
                included_path.display()
            )
        })?;
        declared.extend(parse_doctests(
            &markdown,
            &[""],
            included_line_offset,
            &mut digest_occurrences,
        ));
        included_line_offset += markdown.lines().count();
    }
    Ok(declared)
}

fn parse_doctests(
    source: &str,
    prefixes: &[&'static str],
    source_line_offset: usize,
    digest_occurrences: &mut BTreeMap<String, usize>,
) -> Vec<DeclaredDoctest> {
    let lines: Vec<_> = source.lines().collect();
    let mut declared = Vec::new();
    let mut index = 0;
    while index < lines.len() {
        let Some((kind, prefix)) = opening_fence(lines[index], prefixes) else {
            index += 1;
            continue;
        };
        let opening_line = source_line_offset + index + 1;
        index += 1;
        let mut body = String::new();
        while index < lines.len() && !closing_fence(lines[index], prefix) {
            body.push_str(strip_doc_prefix(lines[index], prefix));
            body.push('\n');
            index += 1;
        }
        let Some(kind) = kind else {
            index += 1;
            continue;
        };
        let digest = format!("{:x}", Sha256::digest(body.as_bytes()));
        let occurrence = digest_occurrences.entry(digest.clone()).or_default();
        *occurrence += 1;
        let posture = match kind {
            DoctestKind::Runnable => "runnable",
            DoctestKind::CompileFail => "compile_fail",
            DoctestKind::Ignored => "ignored",
        };
        declared.push(DeclaredDoctest {
            stable_case_name: format!("doctest_{posture}_{}_{}", &digest[..16], occurrence),
            source_line: opening_line,
            kind,
        });
        index += 1;
    }
    declared
}

fn included_documentation_paths(
    rust_source_path: &Path,
    source: &str,
) -> Vec<(std::path::PathBuf, usize)> {
    let parent = rust_source_path.parent().unwrap_or_else(|| Path::new("."));
    source
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains("doc") && line.contains("include_str!"))
        .filter_map(|(index, line)| {
            Some((
                parent.join(line.split_once("include_str!(\"")?.1.split_once("\"")?.0),
                index + 1,
            ))
        })
        .collect()
}

pub(super) fn declared_doctest_features(source: &str) -> Vec<String> {
    let mut features = source
        .lines()
        .filter_map(|line| {
            line.trim_start()
                .strip_prefix("//!")
                .or_else(|| line.trim_start().strip_prefix("///"))
        })
        .filter_map(|line| line.trim().strip_prefix("store-proof-required-features:"))
        .flat_map(|features| features.split(','))
        .map(str::trim)
        .filter(|feature| !feature.is_empty())
        .map(str::to_owned)
        .collect::<Vec<_>>();
    features.sort();
    features.dedup();
    features
}

fn opening_fence(
    line: &str,
    prefixes: &[&'static str],
) -> Option<(Option<DoctestKind>, &'static str)> {
    for prefix in prefixes {
        let declaration = if prefix.is_empty() {
            line.trim_start()
        } else {
            let Some(declaration) = line.trim_start().strip_prefix(prefix) else {
                continue;
            };
            declaration
        };
        let declaration = declaration.trim_start();
        let attributes = declaration.strip_prefix("```")?;
        let tokens: Vec<_> = attributes
            .split([',', ' ', '\t'])
            .map(str::trim)
            .filter(|token| !token.is_empty())
            .collect();
        if tokens.iter().any(|attribute| *attribute == "compile_fail") {
            return Some((Some(DoctestKind::CompileFail), *prefix));
        }
        if tokens
            .iter()
            .any(|attribute| attribute.starts_with("ignore"))
        {
            return Some((Some(DoctestKind::Ignored), *prefix));
        }
        let runnable = tokens.is_empty()
            || tokens.iter().any(|attribute| {
                matches!(*attribute, "rust" | "no_run" | "should_panic")
                    || attribute.starts_with("edition")
                    || attribute.starts_with('E')
            });
        if runnable {
            return Some((Some(DoctestKind::Runnable), *prefix));
        }
        return Some((None, *prefix));
    }
    None
}

fn closing_fence(line: &str, prefix: &str) -> bool {
    if prefix.is_empty() {
        line.trim() == "```"
    } else {
        line.trim_start()
            .strip_prefix(prefix)
            .is_some_and(|line| line.trim() == "```")
    }
}

fn strip_doc_prefix<'a>(line: &'a str, prefix: &str) -> &'a str {
    line.trim_start()
        .strip_prefix(prefix)
        .map(str::trim_start)
        .unwrap_or(line)
}

#[cfg(test)]
mod tests {
    use super::{declared_doctests, DoctestKind};
    use std::path::Path;

    #[test]
    fn content_identity_is_stable_when_surrounding_lines_move() {
        let first = "//! heading\n//! ```compile_fail\n//! let _ = private();\n//! ```\n";
        let moved =
            "//! extra\n//! heading\n//! ```compile_fail\n//! let _ = private();\n//! ```\n";
        let first = declared_doctests(Path::new("lib.rs"), first).unwrap();
        let moved = declared_doctests(Path::new("lib.rs"), moved).unwrap();
        assert_eq!(first[0].stable_case_name, moved[0].stable_case_name);
        assert_eq!(first[0].kind, DoctestKind::CompileFail);
        assert_ne!(first[0].source_line, moved[0].source_line);
    }

    #[test]
    fn ordinary_rust_fences_are_executable_proof_cases() {
        let declared = declared_doctests(
            Path::new("lib.rs"),
            "//! ```rust,no_run\n//! let value = 7;\n//! ```\n//! ```text\n//! prose\n//! ```\n",
        )
        .unwrap();
        assert_eq!(declared.len(), 1);
        assert_eq!(declared[0].kind, DoctestKind::Runnable);
    }

    #[test]
    fn included_markdown_uses_rustdoc_concatenated_line_coordinates() {
        let root =
            std::env::temp_dir().join(format!("worth-store-doctest-offset-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(root.join("first.md"), "heading\n```rust\nlet _ = 1;\n```\n").unwrap();
        std::fs::write(
            root.join("second.md"),
            "heading\n```compile_fail\nmissing();\n```\n",
        )
        .unwrap();
        let source =
            "#![doc = include_str!(\"first.md\")]\n#![doc = include_str!(\"second.md\")]\n";
        let declared = declared_doctests(&root.join("lib.rs"), source).unwrap();
        assert_eq!(declared[0].source_line, 2);
        assert_eq!(declared[1].source_line, 6);
        std::fs::remove_dir_all(root).unwrap();
    }
}
