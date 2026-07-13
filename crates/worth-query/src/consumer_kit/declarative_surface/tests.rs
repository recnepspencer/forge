use std::collections::BTreeSet;

use super::audit::workspace_declarative_surface_audit;
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
        .map(|row| (row.source_path(), row.function_name()))
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
