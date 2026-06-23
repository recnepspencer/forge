use super::super::intent::PrimitiveConstructionIntent;
use super::super::specs::RegularPyramidSpec;
use super::support::branch_preview_basis::prepare_branch_preview_basis_report;
use super::support::evidence_reports::adoption_sources::{
    assert_reference_consumer_adoption_inventory_consistent,
    expected_defended_residue_symbol_count, reference_consumer_adoption_source_count,
    reference_consumer_adoption_source_set,
};
use super::support::evidence_reports::{sealed_report_evidence_identity, sealed_report_identity};
use forge_query::facade::consumer_kit::{
    evidence_report_adoption_audit, EvidenceReportDeclaration, EvidenceReportScope,
    ForgeQueryEvidenceReportAdoptionFindingKind,
    ForgeQueryEvidenceReportAdoptionResidueClassification,
    ForgeQueryEvidenceReportAdoptionSourceSet,
};
use forge_query::facade::{ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope};
use topology::certification::milestone_one_runtime_builder;
use topology::runtime_support::{topology_runtime, TopologyRuntimeAdapters};

#[test]
fn reference_consumer_report_identity_uses_query_evidence_scheme() {
    let report_identity = sealed_report_evidence_identity(
        "worth-kernel.construction.phase-seven",
        "canonical-identity-smoke",
        |report| {
            report
                .shape_participating("surface", "reference-consumer")?
                .bool_participating("kit-owned", true)
        },
    )
    .expect("sealed identity");

    assert_consumer_evidence_report_identity(&report_identity);

    let report = EvidenceReportDeclaration::new(
        EvidenceReportScope::new("worth-kernel.construction.phase-seven")
            .expect("scope should be valid"),
        "canonical-identity-smoke",
    )
    .expect("report declaration")
    .shape_participating("surface", "reference-consumer")
    .expect("surface field")
    .bool_participating("kit-owned", true)
    .expect("kit-owned field")
    .seal()
    .expect("sealed report");

    assert!(
        report
            .report_identity()
            .terminal_projection_for_reporting()
            .starts_with("forge.query.evidence-identity.v1:"),
        "consumer report identities must be Query canonical evidence identities"
    );
    assert_eq!(report.indexed_field_count(), 2);
}

#[test]
fn branch_preview_report_preserves_semantics_after_kit_adoption() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.phase-seven.branch-preview-adoption".to_string(),
    )
    .expect("workspace");

    let report = prepare_branch_preview_basis_report(
        &mut workspace,
        PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
            sides: 5,
            radius: 1.0,
            height: 2.0,
        }),
    )
    .expect("basis preview report");

    assert!(report.parity_verified());
    assert!(
        report
            .report_digest()
            .starts_with("forge.query.evidence-identity.v1:"),
        "migrated worth-kernel report identity should come from the Query kit"
    );
}

#[test]
fn covered_report_surfaces_do_not_call_worth_kernel_digest_assembly() {
    let report = evidence_report_adoption_audit()
        .covering_sources(reference_consumer_adoption_source_set())
        .evaluate()
        .expect("reference consumer adoption sources parse");

    report.assert_clean();
    assert_eq!(
        report.report_identity().scope(),
        ForgeQueryEvidenceScope::ConsumerEvidenceReportAdoptionReport
    );
    assert_eq!(
        report.source_labels().len(),
        reference_consumer_adoption_source_count()
    );
}

#[test]
fn remaining_construction_digest_residue_is_defended_domain_identity() {
    let report = evidence_report_adoption_audit()
        .covering_sources(reference_consumer_adoption_source_set())
        .evaluate()
        .expect("reference consumer adoption sources parse");

    let inventory_identity = sealed_report_identity(
        "worth-kernel.construction.phase-seven",
        "defended-domain-digest-residue-inventory",
        |declaration| {
            declaration
                .shape_participating("residue-authority", "worth-domain-artifact-identity")?
                .identity_sequence_participating(
                    "classified-residue-row",
                    report.residue_rows().iter().map(|row| row.row_identity()),
                )
        },
    );

    assert!(
        inventory_identity.starts_with("forge.query.evidence-identity.v1:"),
        "defended residue inventory itself must be a Query evidence report identity"
    );
    assert_eq!(
        report.residue_rows().len(),
        expected_defended_residue_symbol_count(),
        "defended residue inventory must name every classified symbol exactly once"
    );

    for row in report.residue_rows() {
        assert_eq!(
            row.classification(),
            ForgeQueryEvidenceReportAdoptionResidueClassification::DefendedDomainArtifactIdentity
        );
        assert_eq!(
            row.row_identity().scope(),
            ForgeQueryEvidenceScope::ConsumerEvidenceReportAdoptionResidue
        );
    }

    assert_reference_consumer_adoption_inventory_consistent();
}

#[test]
fn adoption_audit_rejects_seeded_covered_digest_residue() {
    let report = evidence_report_adoption_audit()
        .covering_sources(
            ForgeQueryEvidenceReportAdoptionSourceSet::new("worth-kernel").source_file(
                "seeded-covered",
                "tests/support/seeded.rs",
                "fn seeded(parts: &[String]) { let _ = digest_owned_parts(parts); }",
                ForgeQueryEvidenceReportAdoptionResidueClassification::CoveredQueryEvidenceAdoption,
            ),
        )
        .evaluate()
        .expect("seeded covered source parses");

    assert_eq!(report.findings().len(), 1);
    assert_eq!(
        report.findings()[0].kind(),
        ForgeQueryEvidenceReportAdoptionFindingKind::CoveredSurfaceUsesWorthDigest
    );
}

#[test]
fn adoption_audit_rejects_seeded_unclassified_digest_residue() {
    let report = evidence_report_adoption_audit()
        .covering_sources(
            ForgeQueryEvidenceReportAdoptionSourceSet::new("worth-kernel").source_file(
                "seeded-unclassified",
                "tests/support/seeded_unclassified.rs",
                "fn seeded() { let _ = ConstructionDigestScope::ArtifactIdentity; }",
                ForgeQueryEvidenceReportAdoptionResidueClassification::Unclassified,
            ),
        )
        .evaluate()
        .expect("seeded unclassified source parses");

    assert_eq!(report.findings().len(), 1);
    assert_eq!(
        report.findings()[0].kind(),
        ForgeQueryEvidenceReportAdoptionFindingKind::UnclassifiedWorthDigestResidue
    );
}

fn assert_consumer_evidence_report_identity(identity: &ForgeQueryEvidenceIdentity) {
    assert_eq!(
        identity.scope(),
        ForgeQueryEvidenceScope::ConsumerEvidenceReport
    );
    assert!(
        identity
            .terminal_projection_for_reporting()
            .starts_with("forge.query.evidence-identity.v1:"),
        "consumer report identities must render through the canonical evidence scheme"
    );
}
