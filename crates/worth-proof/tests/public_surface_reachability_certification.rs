//! Every publicly exported type must be obtainable.
//!
//! `DisjointPair` shipped through `raw.rs` with `left()`, `right()`, `proof()`,
//! and `into_parts()` all public, and a single `pub(crate)` constructor marked
//! `#[allow(dead_code)]`. No consumer could build one; neither could the crate.
//! It was found by a human reading source, and it is mechanically detectable.
//!
//! The rule: for each publicly exported type, at least one **public function
//! anywhere in the crate returns it**. That is deliberately weaker than "has a
//! public constructor" — most stronger forms here are correctly produced by a
//! transition rather than by `Self::new`, and that is the design working. What
//! it catches is a type nothing at all produces.
//!
//! A type exported with no way in tells a reader the machinery is inert, and
//! sends them off to hand-roll their own. That is exactly what happened in
//! milestone 9.16 Phase 8.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

/// Types whose only producer is outside this crate's public functions, with the
/// reason each is legitimate. An empty reason is not accepted by review.
const REACHABILITY_EXEMPT: &[(&str, &str)] = &[
    // Phase and marker types are inhabited only as type parameters; they are
    // never values, so nothing returns them by design.
    ("Unresolved", "phase marker, never a value"),
    ("Resolved", "phase marker, never a value"),
    ("Lowered", "phase marker, never a value"),
    ("Admitted", "phase marker, never a value"),
    ("CurrentValidity", "freshness marker, never a value"),
    ("NoProofs", "empty proof set, inhabited as a type parameter"),
    ("CanonicalOrder", "structural fact marker, never a value"),
    ("Uniqueness", "structural fact marker, never a value"),
    ("Disjointness", "structural fact marker, never a value"),
    ("Normalization", "structural fact marker, never a value"),
    (
        "StructuralProofAuthority",
        "proof authority marker, never a value",
    ),
    // Produced only through its `pub type BoundaryBridged*Basis` aliases, which
    // `Recipe::bridge_trust_boundary` and friends return. This scan does not
    // follow alias indirection; verified by hand at
    // `assumption/readmission.rs:35-37`.
    (
        "BoundaryBridged",
        "returned via the BoundaryBridged*Basis type aliases",
    ),
];

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn rust_sources(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            rust_sources(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            out.push(path);
        }
    }
}

/// Type names re-exported from the crate's public doors.
fn publicly_exported_types(sources: &[PathBuf]) -> BTreeSet<String> {
    let mut exported = BTreeSet::new();
    for path in sources {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default();
        if !matches!(name, "lib.rs" | "facade.rs" | "prelude.rs" | "raw.rs") {
            continue;
        }
        let Ok(source) = fs::read_to_string(path) else {
            continue;
        };
        // `pub use` statements routinely span several lines, so read each one
        // through to its terminating `;` rather than a line at a time. A
        // line-wise scan silently found a tenth of the surface.
        for statement in source.split("pub use").skip(1) {
            let statement = statement.split(';').next().unwrap_or_default();
            for token in statement.split(|c: char| !(c.is_alphanumeric() || c == '_')) {
                // Type names are UpperCamel; functions and modules are not.
                if token.len() > 2
                    && token.starts_with(|c: char| c.is_ascii_uppercase())
                    && token.chars().any(|c| c.is_ascii_lowercase())
                {
                    exported.insert(token.to_owned());
                }
            }
        }
    }
    exported
}

/// Is this name a struct that a consumer cannot build by literal?
///
/// Traits, enums, and structs whose fields are all `pub` (or which have no
/// fields) are constructible or inhabited by other means, so "nothing returns
/// it" says nothing about them. The defect this test exists for is narrower: a
/// struct sealed by private fields whose only constructor is not public.
fn is_sealed_struct(type_name: &str, sources: &[PathBuf]) -> bool {
    for path in sources {
        let Ok(source) = fs::read_to_string(path) else {
            continue;
        };
        for (index, line) in source.lines().enumerate() {
            let trimmed = line.trim();
            let is_decl = trimmed.starts_with(&format!("pub struct {type_name}"))
                && trimmed
                    .strip_prefix(&format!("pub struct {type_name}"))
                    .is_some_and(|rest| rest.is_empty() || rest.starts_with(['<', '(', ' ', '{']));
            if !is_decl {
                continue;
            }
            // Unit struct: constructible by name.
            if trimmed.ends_with(';') && !trimmed.contains('(') {
                return false;
            }
            // Tuple struct: sealed when any field lacks `pub`.
            if let Some(open) = trimmed.find('(') {
                let fields = &trimmed[open + 1..];
                let fields = fields.split(')').next().unwrap_or_default();
                return !fields.trim().is_empty() && !fields.trim_start().starts_with("pub");
            }
            // Braced struct: sealed when any field lacks `pub`.
            let body: Vec<&str> = source
                .lines()
                .skip(index + 1)
                .take_while(|l| !l.trim().starts_with('}'))
                .collect();
            let field_lines: Vec<&&str> = body
                .iter()
                .filter(|l| l.contains(':') && !l.trim().starts_with("//"))
                .collect();
            return field_lines
                .iter()
                .any(|l| !l.trim_start().starts_with("pub"));
        }
    }
    false // not a struct declared here (trait, enum, alias, or foreign)
}

/// Does any public function in the crate return this type?
///
/// Signatures are read whole, from `fn` to the opening brace. This codebase
/// formats wide, so a line-wise scan sees `pub fn new(` with no return type at
/// all and concludes nothing is produced — which it did.
fn has_public_producer(type_name: &str, sources: &[PathBuf]) -> bool {
    for path in sources {
        let Ok(source) = fs::read_to_string(path) else {
            continue;
        };
        // Cut the file at its test module, if any. `#[cfg(test)]` also sits on
        // individual items, and cutting there would blind the scan to
        // everything below.
        let test_module_marker = concat!("#[cfg(test)]", "\n", "mod ");
        let production = match source.find(test_module_marker) {
            Some(index) => &source[..index],
            None => &source[..],
        };
        let declares_impl_for = production.contains(" for ");
        for (offset, _) in production.match_indices("fn ") {
            let line_start = production[..offset].rfind('\n').map_or(0, |i| i + 1);
            let prefix = production[line_start..offset].trim();
            // `pub fn` and trait-impl methods (no visibility) both produce
            // reachable values. `pub(crate) fn` deliberately does not — that is
            // exactly what made `DisjointPair` unreachable while looking built.
            let public = prefix == "pub" || prefix == "pub const";
            let trait_impl = (prefix.is_empty() || prefix == "const") && declares_impl_for;
            if !public && !trait_impl {
                continue;
            }
            let Some(body) = production[offset..].find('{') else {
                continue;
            };
            let signature = &production[offset..offset + body];
            let Some(returns) = signature.split("->").nth(1) else {
                continue;
            };
            let names_type = returns
                .split(|c: char| !(c.is_alphanumeric() || c == '_'))
                .any(|token| token == type_name);
            if names_type {
                return true;
            }
            // `-> Self` inside `impl Type`. Impl headers wrap in this codebase —
            // `impl<A, B, C>` on one line and the type on the next — so read the
            // header as a region rather than a line.
            if returns.contains("Self") {
                let lines: Vec<&str> = production.lines().collect();
                let names_type_in_impl_header = lines.iter().enumerate().any(|(index, line)| {
                    if !line.trim_start().starts_with("impl") {
                        return false;
                    }
                    lines[index..]
                        .iter()
                        .take(3)
                        .flat_map(|header| {
                            header.split(|c: char| !(c.is_alphanumeric() || c == '_'))
                        })
                        .any(|token| token == type_name)
                });
                if names_type_in_impl_header {
                    return true;
                }
            }
        }
    }
    false
}

#[test]
fn every_publicly_exported_type_is_obtainable() {
    let root = crate_root();
    let mut sources = Vec::new();
    rust_sources(&root.join("src"), &mut sources);
    assert!(!sources.is_empty(), "no Rust sources found under src/");

    let exported = publicly_exported_types(&sources);
    assert!(
        exported.len() > 20,
        "export scan found only {} types; the scan is broken, not the crate",
        exported.len()
    );

    let exempt: BTreeSet<&str> = REACHABILITY_EXEMPT.iter().map(|(name, _)| *name).collect();
    let unreachable: Vec<&String> = exported
        .iter()
        .filter(|name| !exempt.contains(name.as_str()))
        .filter(|name| is_sealed_struct(name, &sources))
        .filter(|name| !has_public_producer(name, &sources))
        .collect();

    assert!(
        unreachable.is_empty(),
        "publicly exported but produced by no public function:\n  {}\n\n\
         Give each a checked constructor, or add it to REACHABILITY_EXEMPT with \
         the reason it is legitimately never returned.",
        unreachable
            .iter()
            .map(|name| name.as_str())
            .collect::<Vec<_>>()
            .join("\n  ")
    );
}

#[test]
fn every_exemption_carries_a_reason() {
    for (name, reason) in REACHABILITY_EXEMPT {
        assert!(
            !reason.trim().is_empty(),
            "exemption for `{name}` has no reason; an unexplained exemption is \
             how an unconstructible type hides"
        );
    }
}
