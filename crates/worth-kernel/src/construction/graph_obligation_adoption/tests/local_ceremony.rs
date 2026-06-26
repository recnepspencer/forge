use super::super::{
    primitive_construction_graph_obligation_audit_sources,
    primitive_construction_graph_obligation_local_ceremony_audit,
};

#[test]
fn local_ceremony_audit_rejects_seeded_kernel_shadow_authority() {
    let seeded = primitive_construction_graph_obligation_audit_sources().source_file(
        "seeded.local-legality",
        "seeded.rs",
        "fn bypass() { local_legality_graph(); }",
    );
    let audit =
        forge_query::facade::consumer_kit::ForgeQueryGraphObligationLocalCeremonyAudit::evaluate(
            &seeded,
        );

    assert!(
        !primitive_construction_graph_obligation_local_ceremony_audit()
            .findings()
            .iter()
            .any(|finding| finding.pattern() == "local_legality_graph")
    );
    assert!(audit
        .findings()
        .iter()
        .any(|finding| finding.pattern() == "local_legality_graph"));
}

#[test]
fn motion_admission_support_contains_no_unreachable_sequencing_folklore() {
    let audit_sources = primitive_construction_graph_obligation_audit_sources();

    assert!(audit_sources
        .sources()
        .iter()
        .filter(|source| source.label().contains("compound-lowering"))
        .all(|source| !source.source().contains("unreachable!")));
}
