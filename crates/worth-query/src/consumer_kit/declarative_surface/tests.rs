use std::collections::BTreeSet;

use super::workspace_audit::workspace_declarative_surface_audit;
use super::{
    audit_declarative_surface_sources, current_declarative_surface_audit,
    worth_query_declarative_surface_rows, WorthQueryDeclarativeSurfaceClass,
    WorthQueryDeclarativeSurfaceFindingKind, WorthQueryDeclarativeSurfaceSource,
};

#[test]
fn current_phase_surface_is_source_backed_and_fully_classified() {
    let audit = current_declarative_surface_audit();

    assert!(
        audit.is_complete(),
        "declarative surface findings: {:?}",
        audit.findings()
    );
    assert_eq!(
        audit.observed_surface_count(),
        audit.classified_surface_count()
    );
}

#[test]
fn registered_internal_mechanism_is_complete_only_when_no_longer_public() {
    let source = WorthQueryDeclarativeSurfaceSource::new(
        "src/planning/mod.rs",
        "pub fn plan_validated_bundle() {}\n",
    );
    let audit = audit_declarative_surface_sources(&[source]);
    let finding = audit
        .findings()
        .iter()
        .find(|finding| {
            finding.kind()
                == WorthQueryDeclarativeSurfaceFindingKind::QuarantinedPhaseSurfaceStillPublic
        })
        .expect("registered internal mechanism must fail while publicly callable");

    assert_eq!(finding.site().path(), "src/planning/mod.rs");
    assert_eq!(finding.site().line(), 1);
    assert_eq!(finding.site().function_name(), "plan_validated_bundle");
    assert_eq!(audit.classified_surface_count(), 0);
}

#[test]
fn ordinary_source_tree_is_discovered_and_fully_classified() {
    let audit = workspace_declarative_surface_audit();

    assert!(
        audit.is_complete(),
        "workspace declarative surface findings: {:?}",
        audit.findings()
    );
    assert_eq!(
        audit.observed_surface_count(),
        audit.classified_surface_count()
    );
}

#[test]
fn registry_keys_are_unique_and_every_row_carries_cutover_intent() {
    let rows = worth_query_declarative_surface_rows();
    let keys = rows
        .iter()
        .map(|row| (row.source_path(), row.owner(), row.function_name()))
        .collect::<BTreeSet<_>>();

    assert_eq!(keys.len(), rows.len());
    assert!(
        rows.len() >= 40,
        "read and generic phase graph must be frozen"
    );
    for row in rows {
        assert!(!row.capability_family().as_str().is_empty());
        assert!(!row.phase_responsibility().as_str().is_empty());
        assert!(!row.current_class().as_str().is_empty());
        assert!(!row.target_class().as_str().is_empty());
        assert!(!row.expected_consumer().is_empty());
        assert!(!row.replacement().is_empty());
    }
    assert!(rows.iter().any(|row| {
        row.current_class() == WorthQueryDeclarativeSurfaceClass::Compatibility
            && row.target_class() == WorthQueryDeclarativeSurfaceClass::InternalMechanism
    }));
}

#[test]
fn seeded_public_phase_transition_fails_at_its_exact_source_location() {
    let source = WorthQueryDeclarativeSurfaceSource::new(
        "seeded/consumer_coordinator.rs",
        "impl SeededCoordinator {\n\n    pub fn plan_seeded_backend_route(&self) {}\n}\n",
    );
    let audit = audit_declarative_surface_sources(&[source]);
    let finding = audit
        .findings()
        .iter()
        .find(|finding| {
            finding.kind()
                == WorthQueryDeclarativeSurfaceFindingKind::UnclassifiedPublicPhaseSurface
        })
        .expect("seeded phase transition must be unclassified");

    assert_eq!(finding.site().path(), "seeded/consumer_coordinator.rs");
    assert_eq!(finding.site().line(), 3);
    assert_eq!(finding.site().function_name(), "plan_seeded_backend_route");
}

#[test]
fn seeded_async_phase_transition_cannot_evade_source_discovery() {
    let source = WorthQueryDeclarativeSurfaceSource::new(
        "seeded/async_consumer_coordinator.rs",
        "pub async fn execute_seeded_backend_route() {}\n",
    );
    let audit = audit_declarative_surface_sources(&[source]);
    let finding = audit
        .findings()
        .iter()
        .find(|finding| {
            finding.kind()
                == WorthQueryDeclarativeSurfaceFindingKind::UnclassifiedPublicPhaseSurface
        })
        .expect("async phase transition must be unclassified");

    assert_eq!(finding.site().line(), 1);
    assert_eq!(
        finding.site().function_name(),
        "execute_seeded_backend_route"
    );
}

#[test]
fn multiline_and_public_trait_phase_transitions_cannot_evade_syntax_discovery() {
    let source = WorthQueryDeclarativeSurfaceSource::new(
        "seeded/public_trait.rs",
        "pub trait ConsumerPhase {\n    fn plan_trait_route(&self);\n}\n\npub\nasync fn execute_multiline_route() {}\n",
    );
    let audit = audit_declarative_surface_sources(&[source]);
    let sites = audit
        .findings()
        .iter()
        .filter(|finding| {
            finding.kind()
                == WorthQueryDeclarativeSurfaceFindingKind::UnclassifiedPublicPhaseSurface
        })
        .map(|finding| {
            (
                finding.site().function_name().to_string(),
                finding.site().line(),
            )
        })
        .collect::<BTreeSet<_>>();

    assert_eq!(
        sites,
        BTreeSet::from([
            ("execute_multiline_route".to_string(), 6),
            ("plan_trait_route".to_string(), 2),
        ])
    );
}

#[test]
fn invalid_rust_source_fails_closed_at_the_parse_site() {
    let source =
        WorthQueryDeclarativeSurfaceSource::new("seeded/invalid.rs", "pub fn plan_broken_route( {");
    let audit = audit_declarative_surface_sources(&[source]);
    let finding = audit
        .findings()
        .iter()
        .find(|finding| {
            finding.kind() == WorthQueryDeclarativeSurfaceFindingKind::InvalidRustSource
        })
        .expect("invalid source must not disappear from the inventory");

    assert_eq!(finding.site().path(), "seeded/invalid.rs");
    assert_eq!(finding.site().line(), 1);
}

#[test]
fn same_named_surfaces_in_distinct_sources_are_not_collapsed() {
    let sources = [
        WorthQueryDeclarativeSurfaceSource::new("seeded/alias.rs", "pub fn execute_alias() {}"),
        WorthQueryDeclarativeSurfaceSource::new(
            "seeded/deep_import.rs",
            "pub fn execute_alias() {}",
        ),
    ];
    let audit = audit_declarative_surface_sources(&sources);
    let sites = audit
        .findings()
        .iter()
        .filter(|finding| {
            finding.kind()
                == WorthQueryDeclarativeSurfaceFindingKind::UnclassifiedPublicPhaseSurface
                && finding.site().function_name() == "execute_alias"
        })
        .map(|finding| finding.site().path())
        .collect::<BTreeSet<_>>();

    assert_eq!(
        sites,
        BTreeSet::from(["seeded/alias.rs", "seeded/deep_import.rs"])
    );
}

#[test]
fn facade_reexport_and_deep_definition_remain_distinct_inventory_rows() {
    let rows = worth_query_declarative_surface_rows();
    let declare_rows = rows
        .iter()
        .filter(|row| row.function_name() == "declare")
        .map(|row| row.source_path())
        .collect::<BTreeSet<_>>();

    assert!(declare_rows.contains("src/facade/exports_read.rs"));
    assert!(declare_rows.contains("src/facade/exports_aggregate.rs"));
    assert!(declare_rows.contains("src/ordinary/read/declaration.rs"));
}

#[test]
fn renamed_reexports_are_discovered_by_the_exported_name() {
    let source = WorthQueryDeclarativeSurfaceSource::new(
        "seeded/facade.rs",
        "pub use crate::deep::plan_internal_route as plan_facade_route;\n",
    );
    let audit = audit_declarative_surface_sources(&[source]);
    let finding = audit
        .findings()
        .iter()
        .find(|finding| {
            finding.kind()
                == WorthQueryDeclarativeSurfaceFindingKind::UnclassifiedPublicPhaseSurface
        })
        .expect("renamed phase export must be inventoried");

    assert_eq!(finding.site().function_name(), "plan_facade_route");
    assert_eq!(finding.site().line(), 1);
}

#[test]
fn public_glob_exports_fail_closed_instead_of_hiding_phase_surfaces() {
    let source = WorthQueryDeclarativeSurfaceSource::new(
        "seeded/glob_facade.rs",
        "pub use crate::deep::*;\n",
    );
    let audit = audit_declarative_surface_sources(&[source]);

    assert!(audit.findings().iter().any(|finding| {
        finding.kind() == WorthQueryDeclarativeSurfaceFindingKind::UnclassifiedPublicPhaseSurface
            && finding.site().function_name() == "*"
            && finding.site().line() == 1
    }));
}

#[test]
fn same_named_methods_on_distinct_owners_are_not_reported_as_duplicates() {
    let source = WorthQueryDeclarativeSurfaceSource::new(
        "seeded/owners.rs",
        "impl First { pub fn plan_alias() {} }\nimpl Second { pub fn plan_alias() {} }\n",
    );
    let audit = audit_declarative_surface_sources(&[source]);
    let owners = audit
        .findings()
        .iter()
        .filter(|finding| {
            finding.kind()
                == WorthQueryDeclarativeSurfaceFindingKind::UnclassifiedPublicPhaseSurface
        })
        .filter_map(|finding| finding.site().owner())
        .collect::<BTreeSet<_>>();

    assert_eq!(owners, BTreeSet::from(["First", "Second"]));
    assert!(!audit.findings().iter().any(|finding| {
        finding.kind() == WorthQueryDeclarativeSurfaceFindingKind::DuplicatePublicPhaseSurface
    }));
}
