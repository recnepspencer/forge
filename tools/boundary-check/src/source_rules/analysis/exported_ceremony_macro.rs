//! Fence exported macros whose bodies can mint unsealed public ceremonies.
//!
//! `#[macro_export] macro_rules!` is crate-root public API. The AST pass cannot
//! seal expansions, so exported macros that mention sealed traits, alias them,
//! or template trait-bound positions fail closed. Opaque attributes on reachable
//! surfaces are owned by `callable_surface` (shared local/external walk).

use super::authority_sealing_surface::{SurfaceHit, FORBIDDEN_TRAITS};
use super::crate_modules::{module_path_display, GovernedCrate, ModuleGraph};
use super::public_reachability::{Reachability, ReachableItemKey};
use crate::diagnostics::{Diagnostic, DiagnosticCode};
use syn::{Attribute, Item};

const SEALING_LAW: &str = "Authority sealing law: governed public surfaces must demand concrete \
platform authority/capability types. Generic bounds over `AuthorityMarker`, `CapabilityMarker`, \
`AuthorityProves`, or `ProofSetAuthorizedBy` are forbidden. Use `AuthorityWitness<ConcreteAuthority>`, \
`CapabilityWitness<ConcreteCapability>`, or `Proof<Fact, ConcreteAuthority>`; keep the concrete \
marker value-gated and mint it only in the owning crate's ceremony.";

const MACRO_EXPORT_FENCE: &str = "Exported macros (`#[macro_export]`) are ordinary public API. \
The AST pass cannot seal their expansions; an exported macro whose body mentions a sealed \
authority trait, aliases one, or templates a trait-bound fragment into a public ceremony is \
denied. Expand concrete public ceremonies in source or keep the macro private and unexported.";

pub(super) fn enforce_exported_ceremony_macros(
    governed: &GovernedCrate,
    graph: &ModuleGraph,
    _reachability: &Reachability,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // #[macro_export] is crate-root public API regardless of defining module privacy.
    for (module_path, node) in &graph.modules {
        for item in &node.items {
            let Item::Macro(item_macro) = item else {
                continue;
            };
            if item_macro.ident.is_none() {
                continue;
            }
            if !has_macro_export(&item_macro.attrs) {
                continue;
            }
            let body = item_macro.mac.tokens.to_string();
            let hit = if let Some(trait_spelling) = forbidden_in_token_text(&body) {
                Some(SurfaceHit::ForbiddenBound { trait_spelling })
            } else if macro_export_templates_trait_bound(&body) {
                Some(SurfaceHit::OpaqueMacroExpansion {
                    macro_path: item_macro
                        .ident
                        .as_ref()
                        .map(|id| id.to_string())
                        .unwrap_or_else(|| "macro_export".to_owned()),
                })
            } else {
                None
            };
            if let Some(hit) = hit {
                let name = item_macro
                    .ident
                    .as_ref()
                    .map(|id| id.to_string())
                    .unwrap_or_else(|| "macro".to_owned());
                let key = ReachableItemKey {
                    module_path: module_path.clone(),
                    item_name: format!("macro_export:{name}"),
                };
                diagnostics.push(sealing_diagnostic(
                    governed,
                    &key,
                    &node.relative_source,
                    &hit,
                ));
            }
        }
    }

    diagnostics.sort_by(Diagnostic::compare_subject_message);
    diagnostics.dedup_by(|a, b| a.has_same_subject_message(b));
    diagnostics
}

fn has_macro_export(attrs: &[Attribute]) -> bool {
    attrs
        .iter()
        .any(|attr| attr.path().is_ident("macro_export"))
}

fn forbidden_in_token_text(text: &str) -> Option<String> {
    for trait_name in FORBIDDEN_TRAITS {
        if contains_rust_ident(text, trait_name) {
            return Some((*trait_name).to_owned());
        }
    }
    None
}

/// Fail closed when an exported macro can mint a public ceremony whose trait
/// bound is a macro fragment (`impl $bound`, `: $bound`) rather than a sealed spelling.
fn macro_export_templates_trait_bound(text: &str) -> bool {
    let has_public_ceremony = text.contains("pub fn")
        || text.contains("pub struct")
        || text.contains("pub enum")
        || text.contains("pub trait")
        || text.contains("pub type");
    if !has_public_ceremony {
        return false;
    }
    // Fragment in trait-bound / impl-trait position.
    if text.contains("impl $") {
        return true;
    }
    // Generic bound substitution: `: $name` (token stream spacing varies).
    if contains_colon_dollar_bound(text) {
        return true;
    }
    false
}

fn contains_colon_dollar_bound(text: &str) -> bool {
    let bytes = text.as_bytes();
    let mut i = 0;
    while i + 1 < bytes.len() {
        if bytes[i] == b':' {
            let mut j = i + 1;
            while j < bytes.len() && bytes[j].is_ascii_whitespace() {
                j += 1;
            }
            if j < bytes.len() && bytes[j] == b'$' {
                return true;
            }
        }
        i += 1;
    }
    false
}

fn contains_rust_ident(text: &str, ident: &str) -> bool {
    let bytes = text.as_bytes();
    let needle = ident.as_bytes();
    if needle.is_empty() || bytes.len() < needle.len() {
        return false;
    }
    for start in 0..=(bytes.len() - needle.len()) {
        if &bytes[start..start + needle.len()] != needle {
            continue;
        }
        let before_ok = start == 0 || !is_ident_byte(bytes[start - 1]);
        let after = start + needle.len();
        let after_ok = after >= bytes.len() || !is_ident_byte(bytes[after]);
        if before_ok && after_ok {
            return true;
        }
    }
    false
}

fn is_ident_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

fn sealing_diagnostic(
    governed: &GovernedCrate,
    key: &ReachableItemKey,
    relative_source: &str,
    hit: &SurfaceHit,
) -> Diagnostic {
    let item_path = if key.module_path.is_empty() {
        key.item_name.clone()
    } else {
        format!(
            "{}::{}",
            module_path_display(&key.module_path),
            key.item_name
        )
    };
    let subject = format!("{}::{}::{}", governed.package, relative_source, item_path);
    let message = match hit {
        SurfaceHit::ForbiddenBound { trait_spelling } => format!(
            "{SEALING_LAW} {MACRO_EXPORT_FENCE} Offending trait spelling: `{trait_spelling}`. \
Concrete pattern: `AuthorityWitness<ConcreteAuthority>`, `CapabilityWitness<ConcreteCapability>`, \
or `Proof<Fact, ConcreteAuthority>`."
        ),
        SurfaceHit::OpaqueMacroExpansion { macro_path } => format!(
            "{SEALING_LAW} {MACRO_EXPORT_FENCE} Offending exported macro template: `{macro_path}`."
        ),
        SurfaceHit::PublicExternCrate { crate_ident } => format!(
            "{SEALING_LAW} {MACRO_EXPORT_FENCE} Unexpected extern crate hit: `{crate_ident}`."
        ),
        SurfaceHit::MintableAuthority {
            marker_type,
            reason,
        } => format!(
            "{SEALING_LAW} {MACRO_EXPORT_FENCE} Offending mintable marker in macro template: \
`{marker_type}` (reason: {reason})."
        ),
    };
    Diagnostic::new(DiagnosticCode::Bc7001AuthoritySealing, subject, message)
}
