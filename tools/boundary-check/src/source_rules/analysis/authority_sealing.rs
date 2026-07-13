//! Authority sealing law: governed public surfaces must name concrete authority types.

use super::authority_sealing_surface::SurfaceHit;
use super::authority_value_gate::collect_value_gate_violations;
use super::callable_surface::{collect_surface_violations, SurfaceViolation};
use super::crate_modules::{module_path_display, GovernedCrate, ModuleGraph};
use super::public_reachability::{Reachability, ReachableItemKey};
use crate::diagnostics::{Diagnostic, DiagnosticCode};

const SEALING_LAW: &str = "Authority sealing law: governed public surfaces must demand concrete \
platform authority/capability types. Generic bounds over `AuthorityMarker`, `CapabilityMarker`, \
`AuthorityProves`, or `ProofSetAuthorizedBy` are forbidden. Use `AuthorityWitness<ConcreteAuthority>`, \
`CapabilityWitness<ConcreteCapability>`, or `Proof<Fact, ConcreteAuthority>`; keep the concrete \
marker value-gated and mint it only in the owning crate's ceremony.";

const VALUE_GATE_LAW: &str = "Platform authority types are value-gated: private field, no `Default`, \
no public constructor; the only mint is the owning crate's ceremony function. A caller-constructible \
marker can mint the exact `AuthorityWitness` / `CapabilityWitness` / `Proof` the ceremony demands \
via `worth-proof`'s open substrate, so concrete type identity alone is not unforgeable authority.";

const MACRO_FENCE: &str = "Item-position macro invocations in externally reachable modules are \
denied because the AST pass cannot seal macro-expanded public surfaces. Expand the concrete \
public ceremony in source (demanding `AuthorityWitness<ConcreteAuthority>` / \
`CapabilityWitness<ConcreteCapability>` / `Proof<Fact, ConcreteAuthority>`) or keep the macro \
private and unreachable.";

const OPAQUE_ATTR_FENCE: &str = "Opaque attribute or custom derive macros on externally reachable \
items can mint unsealed public ceremony signatures the AST pass cannot inspect. Expand the \
concrete public surface in source, or restrict attributes to the known-safe builtin set.";

const EXTERN_CRATE_FENCE: &str =
    "Public `extern crate` re-exports a foreign crate root as ordinary \
public API; the AST pass cannot seal that foreign surface. Demand concrete local ceremony types \
(`AuthorityWitness<ConcreteAuthority>` / `CapabilityWitness<ConcreteCapability>` / \
`Proof<Fact, ConcreteAuthority>`) instead of `pub extern crate`.";

pub(super) fn parse_failure_diagnostic(governed: &GovernedCrate, error: String) -> Diagnostic {
    Diagnostic::new(
        DiagnosticCode::Bc7001AuthoritySealing,
        format!("{}::{}", governed.package, governed.relative_crate_root),
        format!("fail-closed: could not parse governed crate for authority sealing ({error})"),
    )
}

/// Dependency authority indexing failed (workspace/target path deps, opaque export
/// generation, or recursive sealed-export resolution). Fail closed as BC7001.
pub(super) fn dependency_authority_failure_diagnostic(
    governed: &GovernedCrate,
    error: String,
) -> Diagnostic {
    Diagnostic::new(
        DiagnosticCode::Bc7001AuthoritySealing,
        format!("{}::{}", governed.package, governed.relative_crate_root),
        format!(
            "{SEALING_LAW} Fail-closed: could not build definition-resolved dependency \
authority index for governed crate ({error}). Concrete pattern: \
`AuthorityWitness<ConcreteAuthority>`, `CapabilityWitness<ConcreteCapability>`, \
or `Proof<Fact, ConcreteAuthority>`."
        ),
    )
}

pub(super) fn enforce_authority_sealing(
    governed: &GovernedCrate,
    graph: &ModuleGraph,
    reachability: &Reachability,
) -> Vec<Diagnostic> {
    let mut diagnostics: Vec<Diagnostic> = collect_surface_violations(graph, reachability)
        .into_iter()
        .chain(collect_value_gate_violations(
            &governed.crate_root,
            graph,
            reachability,
        ))
        .map(|violation| sealing_diagnostic(governed, &violation))
        .collect();

    diagnostics.sort_by(Diagnostic::compare_subject_message);
    diagnostics.dedup_by(|a, b| a.has_same_subject_message(b));
    diagnostics
}

fn sealing_diagnostic(governed: &GovernedCrate, violation: &SurfaceViolation) -> Diagnostic {
    let subject = surface_subject(governed, &violation.key, &violation.relative_source);
    let message = match &violation.hit {
        SurfaceHit::ForbiddenBound { trait_spelling } => format!(
            "{SEALING_LAW} Offending trait bound: `{trait_spelling}`. Concrete pattern: \
`AuthorityWitness<ConcreteAuthority>`, `CapabilityWitness<ConcreteCapability>`, \
or `Proof<Fact, ConcreteAuthority>`."
        ),
        SurfaceHit::PublicExternCrate { crate_ident } => {
            format!("{SEALING_LAW} {EXTERN_CRATE_FENCE} Offending extern crate: `{crate_ident}`.")
        }
        SurfaceHit::MintableAuthority {
            marker_type,
            reason,
        } => format!(
            "{SEALING_LAW} {VALUE_GATE_LAW} Offending mintable marker: `{marker_type}` \
(reason: {reason}). Seal with a private field and crate-local ceremony mint; concrete pattern: \
`AuthorityWitness<ConcreteAuthority>`, `CapabilityWitness<ConcreteCapability>`, \
or `Proof<Fact, ConcreteAuthority>`."
        ),
        SurfaceHit::OpaqueMacroExpansion { macro_path } => {
            let is_item_macro = violation.key.item_name.starts_with("macro:")
                || violation.key.item_name.contains("::macro:");
            if is_item_macro {
                format!("{SEALING_LAW} {MACRO_FENCE} Offending item macro: `{macro_path}`.")
            } else {
                format!(
                    "{SEALING_LAW} {OPAQUE_ATTR_FENCE} Offending attribute/derive: `{macro_path}`."
                )
            }
        }
    };
    Diagnostic::new(DiagnosticCode::Bc7001AuthoritySealing, subject, message)
}

fn surface_subject(
    governed: &GovernedCrate,
    key: &ReachableItemKey,
    relative_source: &str,
) -> String {
    let item_path = if key.module_path.is_empty() {
        key.item_name.clone()
    } else {
        format!(
            "{}::{}",
            module_path_display(&key.module_path),
            key.item_name
        )
    };
    format!("{}::{relative_source}::{item_path}", governed.package)
}
