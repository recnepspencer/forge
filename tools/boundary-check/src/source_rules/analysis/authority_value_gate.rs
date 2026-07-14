//! Value-gate concrete ceremony markers on governed public surfaces.
//!
//! BC7001 half-one forbids open `AuthorityMarker` bounds. Half-two requires that
//! every concrete type admitted inside `AuthorityWitness`, `CapabilityWitness`,
//! or `Proof` is not caller-constructible: private field, no `Default`, no
//! public constructor. `worth-proof` stays open; domain markers are sealed.

use super::authority_sealing_surface::SurfaceHit;
use super::authority_value_gate_defs::{
    collect_default_impls, collect_public_self_constructors, collect_public_values,
    collect_trait_factories, index_type_definitions, mintability_reason_for,
};
use super::authority_value_gate_scan::collect_ceremony_admissions;
use super::authority_value_identity::{carrier_aliases, local_type_key};
use super::callable_surface::SurfaceViolation;
use super::crate_modules::ModuleGraph;
use super::public_reachability::{Reachability, ReachableItemKey};
use std::collections::BTreeSet;
use std::path::Path;

/// Collect mintability violations for concrete markers admitted on public surfaces.
pub(super) fn collect_value_gate_violations(
    crate_root: &Path,
    graph: &ModuleGraph,
    reachability: &Reachability,
) -> Vec<SurfaceViolation> {
    let worth_proof_idents =
        match super::path_dependencies::dependency_idents_for_package(crate_root, "worth-proof") {
            Ok(idents) => idents,
            Err(error) => {
                return vec![unresolved_inventory_violation(error)];
            }
        };
    let aliases = carrier_aliases(graph, &worth_proof_idents);
    let admissions =
        collect_ceremony_admissions(graph, reachability, &worth_proof_idents, &aliases);
    if admissions.is_empty() {
        return Vec::new();
    }

    let definitions = index_type_definitions(graph);
    let defaults = collect_default_impls(graph);
    let constructors = collect_public_self_constructors(graph);
    let public_values = collect_public_values(graph, reachability);
    let trait_factories = collect_trait_factories(graph, reachability);

    let mut violations = Vec::new();
    let mut seen: BTreeSet<(ReachableItemKey, String, String)> = BTreeSet::new();

    for admission in admissions {
        let marker_display = marker_type_display(&admission.marker_type);
        let Some(marker_key) =
            local_type_key(graph, &admission.key.module_path, &admission.marker_type)
        else {
            // Local definition missing: cannot prove value-gate. Fail closed.
            let hit = SurfaceHit::MintableAuthority {
                marker_type: marker_display,
                reason: "unresolved_definition".to_owned(),
            };
            push_unique(&mut violations, &mut seen, &admission, hit);
            continue;
        };
        let Some(def) = definitions.get(&marker_key) else {
            continue;
        };

        // Private types cannot be named or constructed by ordinary external callers.
        if !def.is_public {
            continue;
        }

        if let Some(reason) = mintability_reason_for(
            &marker_key,
            def,
            &defaults,
            &constructors,
            &public_values,
            &trait_factories,
            &definitions,
        ) {
            let hit = SurfaceHit::MintableAuthority {
                marker_type: marker_display,
                reason,
            };
            push_unique(&mut violations, &mut seen, &admission, hit);
        }
    }

    violations
}

pub(super) struct CeremonyAdmission {
    pub(super) key: ReachableItemKey,
    pub(super) relative_source: String,
    pub(super) marker_type: syn::Type,
}

fn push_unique(
    violations: &mut Vec<SurfaceViolation>,
    seen: &mut BTreeSet<(ReachableItemKey, String, String)>,
    admission: &CeremonyAdmission,
    hit: SurfaceHit,
) {
    let reason = match &hit {
        SurfaceHit::MintableAuthority { reason, .. } => reason.clone(),
        _ => String::new(),
    };
    let fingerprint = (
        admission.key.clone(),
        marker_type_display(&admission.marker_type),
        reason,
    );
    if !seen.insert(fingerprint) {
        return;
    }
    violations.push(SurfaceViolation {
        key: admission.key.clone(),
        relative_source: admission.relative_source.clone(),
        hit,
    });
}

fn marker_type_display(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Path(path) => path
            .path
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<_>>()
            .join("::"),
        syn::Type::Reference(reference) => marker_type_display(&reference.elem),
        syn::Type::Paren(paren) => marker_type_display(&paren.elem),
        syn::Type::Group(group) => marker_type_display(&group.elem),
        _ => "non-path marker".to_owned(),
    }
}

fn unresolved_inventory_violation(error: String) -> SurfaceViolation {
    SurfaceViolation {
        key: ReachableItemKey {
            module_path: Vec::new(),
            item_name: "value_gate_inventory".to_owned(),
        },
        relative_source: "Cargo.toml".to_owned(),
        hit: SurfaceHit::MintableAuthority {
            marker_type: "worth-proof carrier inventory".to_owned(),
            reason: format!("unresolved_carrier_identity: {error}"),
        },
    }
}
