use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use syn::{ImplItem, Item, Type, Visibility};

use super::{
    audit_public_authority_surface_symbols, worth_query_public_authority_surface_rows,
    WorthQueryPublicAuthoritySurfaceClass, WorthQueryPublicAuthoritySurfaceFindingKind,
};

#[test]
fn manifest_symbols_are_unique_and_structurally_complete() {
    let rows = worth_query_public_authority_surface_rows();
    let symbols = rows.iter().map(|row| row.symbol()).collect::<BTreeSet<_>>();

    assert_eq!(symbols.len(), rows.len(), "manifest symbols must be unique");
    assert!(
        rows.len() >= 40,
        "phase-one manifest must inventory the full risky surface"
    );
    for row in rows {
        assert!(!row.operational_consumer().is_empty());
        assert!(!row.replacement().is_empty());
        assert!(!row.owner().as_str().is_empty());
        assert!(!row.current_class().as_str().is_empty());
        assert!(!row.target_class().as_str().is_empty());
    }
}

#[test]
fn facade_root_has_no_topology_mirror_or_tooling_leak() {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let facade = read(crate_root, "src/facade.rs");

    for forbidden in [
        "pub use exports_application::*",
        "pub use exports_foundation::*",
        "pub use exports_policy::*",
        "pub use exports_runtime::*",
        "pub use exports_runtime_phase_nine::*",
        "pub use identity_authority::*",
    ] {
        assert!(
            !facade.contains(forbidden),
            "ordinary facade root must not mirror {forbidden}"
        );
    }
    for namespace in [
        "pub mod foundation",
        "pub mod policy",
        "pub mod runtime",
        "pub mod certification",
        "pub mod consumer_kit",
    ] {
        assert!(
            facade.contains(namespace),
            "stable facade namespace {namespace} is missing"
        );
    }
}

#[test]
fn exact_observed_inventory_is_classified_without_findings() {
    let observed = worth_query_public_authority_surface_rows()
        .iter()
        .map(|row| row.symbol())
        .collect::<Vec<_>>();
    let audit = audit_public_authority_surface_symbols(&observed);

    assert!(
        audit.is_complete(),
        "unexpected findings: {:?}",
        audit.findings()
    );
    assert_eq!(audit.observed_surface_count(), observed.len());
    assert_eq!(audit.classified_surface_count(), observed.len());
}

#[test]
fn authority_bearing_impls_have_no_unclassified_public_constructor() {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut observed = Vec::new();
    observed.extend(public_constructor_symbols(
        &read(crate_root, "src/identity/digest.rs"),
        &["CanonicalQueryDigest", "SchemaBasisDigest", "BasisDigest"],
    ));
    observed.extend(public_constructor_symbols(
        &read(crate_root, "src/identity_evolution/request.rs"),
        &["IdentityEvolutionQueryContext"],
    ));
    observed.extend(public_constructor_symbols(
        &read(crate_root, "src/basis/schema_authority.rs"),
        &["QueryExternalSchemaBasisToken"],
    ));
    observed.extend(public_constructor_symbols(
        &read(crate_root, "src/historical/request.rs"),
        &[
            "HistoricalEvaluationRequest",
            "HistoricalCapabilityDescriptor",
            "HistoricalMaterializationDescriptor",
        ],
    ));
    observed.extend(public_constructor_symbols(
        &read(crate_root, "src/query_context/basis.rs"),
        &["QueryBasisContextRequest"],
    ));
    observed.push("resolve_runtime_current_snapshot_basis".to_string());
    observed.push("admit_runtime_current_snapshot_basis".to_string());
    observed.extend(public_constructor_symbols(
        &read(crate_root, "src/intent_admission/eligibility/request.rs"),
        &["WorthQueryRawIntentAdmissionRequest"],
    ));
    observed.extend(public_constructor_symbols(
        &read(crate_root, "src/intent_admission/eligibility/artifact.rs"),
        &["WorthQueryIntentAdmissionEligibility"],
    ));
    observed.extend(public_constructor_symbols(
        &read(crate_root, "src/subscription/input.rs"),
        &["LiveQueryAdmissionArtifact"],
    ));
    observed.extend(public_constructor_symbols(
        &read(crate_root, "src/runtime/inspection/causal/builder.rs"),
        &["CausalInspection"],
    ));

    let manifest_symbols = worth_query_public_authority_surface_rows()
        .iter()
        .map(|row| row.symbol())
        .collect::<BTreeSet<_>>();
    let unclassified = observed
        .iter()
        .filter(|symbol| !manifest_symbols.contains(symbol.as_str()))
        .collect::<Vec<_>>();
    assert!(
        unclassified.is_empty(),
        "unclassified public authority constructors: {unclassified:?}"
    );
}

#[test]
fn seeded_unclassified_and_duplicate_surfaces_fail_locally() {
    let mut observed = worth_query_public_authority_surface_rows()
        .iter()
        .map(|row| row.symbol())
        .collect::<Vec<_>>();
    observed.push("CanonicalQueryDigest::from_domain_parts");
    observed.push("SeededAuthorityEscape::from_raw_label");

    let audit = audit_public_authority_surface_symbols(&observed);
    assert!(!audit.is_complete());
    assert!(audit.findings().iter().any(|finding| {
        finding.kind() == WorthQueryPublicAuthoritySurfaceFindingKind::DuplicateObservedSurface
            && finding.symbol() == "CanonicalQueryDigest::from_domain_parts"
    }));
    assert!(audit.findings().iter().any(|finding| {
        finding.kind() == WorthQueryPublicAuthoritySurfaceFindingKind::UnclassifiedObservedSurface
            && finding.symbol() == "SeededAuthorityEscape::from_raw_label"
    }));
}

#[test]
fn seeded_public_constructor_is_discovered_and_rejected() {
    let seeded_source = r#"
        pub struct CanonicalQueryDigest;
        impl CanonicalQueryDigest {
            pub fn from_domain_parts() -> Self { Self }
            pub fn seeded_from_raw_label() -> Self { Self }
        }
    "#;
    let observed = public_constructor_symbols(seeded_source, &["CanonicalQueryDigest"]);
    let manifest_symbols = worth_query_public_authority_surface_rows()
        .iter()
        .map(|row| row.symbol())
        .collect::<BTreeSet<_>>();

    assert!(observed.iter().any(|symbol| {
        symbol == "CanonicalQueryDigest::seeded_from_raw_label"
            && !manifest_symbols.contains(symbol.as_str())
    }));
}

#[test]
fn phase_one_targets_every_legacy_operational_surface_for_contraction() {
    let rows = worth_query_public_authority_surface_rows();
    assert!(rows.iter().any(|row| {
        row.target_class() == WorthQueryPublicAuthoritySurfaceClass::DeleteBeforeCloseout
    }));
    assert!(rows.iter().any(|row| {
        row.target_class() == WorthQueryPublicAuthoritySurfaceClass::InternalAdapter
    }));
    assert!(rows.iter().any(|row| {
        row.target_class() == WorthQueryPublicAuthoritySurfaceClass::ReadOnlyProjection
    }));
    assert!(rows.iter().any(|row| {
        row.target_class() == WorthQueryPublicAuthoritySurfaceClass::SealedPhaseApi
    }));
}

fn read(crate_root: &Path, relative_path: &str) -> String {
    fs::read_to_string(crate_root.join(relative_path))
        .unwrap_or_else(|error| panic!("failed to read {relative_path}: {error}"))
}

fn public_constructor_symbols(source: &str, impl_names: &[&str]) -> Vec<String> {
    let syntax = syn::parse_file(source).expect("authority source should parse");
    let mut symbols = Vec::new();

    for item in syntax.items {
        let Item::Impl(item_impl) = item else {
            continue;
        };
        let Type::Path(self_type) = item_impl.self_ty.as_ref() else {
            continue;
        };
        let Some(type_name) = self_type
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string())
        else {
            continue;
        };
        if !impl_names.contains(&type_name.as_str()) {
            continue;
        }
        for item in item_impl.items {
            let ImplItem::Fn(function) = item else {
                continue;
            };
            if matches!(function.vis, Visibility::Public(_)) && function.sig.receiver().is_none() {
                symbols.push(format!("{type_name}::{}", function.sig.ident));
            }
        }
    }

    symbols
}
