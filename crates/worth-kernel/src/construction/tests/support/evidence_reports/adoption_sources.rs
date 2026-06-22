use forge_query::facade::consumer_kit::{
    ForgeQueryEvidenceReportAdoptionResidueClassification,
    ForgeQueryEvidenceReportAdoptionSourceSet,
};

const COVERED_REPORT_SUPPORT_FILES: &[(&str, &str, &str)] = &[
    (
        "branch_preview_basis.rs",
        "tests/support/branch_preview_basis.rs",
        include_str!("../branch_preview_basis.rs"),
    ),
    (
        "projection_consumption.rs",
        "tests/support/projection_consumption.rs",
        include_str!("../projection_consumption.rs"),
    ),
    (
        "family_coverage.rs",
        "tests/support/family_coverage.rs",
        include_str!("../family_coverage.rs"),
    ),
    (
        "compound_parity_support.rs",
        "tests/support/compound_parity_support.rs",
        include_str!("../compound_parity_support.rs"),
    ),
    (
        "compound_parity_view.rs",
        "tests/support/compound_parity_view.rs",
        include_str!("../compound_parity_view.rs"),
    ),
    (
        "compound_lane_support.rs",
        "tests/support/compound_lane_support.rs",
        include_str!("../compound_lane_support.rs"),
    ),
    (
        "compound_row_support.rs",
        "tests/support/compound_row_support.rs",
        include_str!("../compound_row_support.rs"),
    ),
    (
        "corpus_replay_digest.rs",
        "tests/support/corpus_replay_digest.rs",
        include_str!("../corpus_replay_digest.rs"),
    ),
    (
        "corpus_replay_view.rs",
        "tests/support/corpus_replay_view.rs",
        include_str!("../corpus_replay_view.rs"),
    ),
    (
        "compound_runtime/rows/specialized_rows.rs",
        "tests/support/compound_runtime/rows/specialized_rows.rs",
        include_str!("../compound_runtime/rows/specialized_rows.rs"),
    ),
];

const DEFENDED_DOMAIN_DIGEST_FILES: &[(&str, &str, &str)] = &[
    (
        "digest_protocol.rs",
        "digest_protocol.rs",
        include_str!("../../../digest_protocol.rs"),
    ),
    ("digest.rs", "digest.rs", include_str!("../../../digest.rs")),
    (
        "result_surface/artifact.rs",
        "result_surface/artifact.rs",
        include_str!("../../../result_surface/artifact.rs"),
    ),
    (
        "phase_chain/request.rs",
        "phase_chain/request.rs",
        include_str!("../../../phase_chain/request.rs"),
    ),
    (
        "result_surface/geometry_recovery.rs",
        "result_surface/geometry_recovery.rs",
        include_str!("../../../result_surface/geometry_recovery.rs"),
    ),
    (
        "result_surface/outcome_rejection.rs",
        "result_surface/outcome_rejection.rs",
        include_str!("../../../result_surface/outcome_rejection.rs"),
    ),
    (
        "result_surface/rejection_facts.rs",
        "result_surface/rejection_facts.rs",
        include_str!("../../../result_surface/rejection_facts.rs"),
    ),
    (
        "phase_chain/admitted_scaffold/mod.rs",
        "phase_chain/admitted_scaffold/mod.rs",
        include_str!("../../../phase_chain/admitted_scaffold/mod.rs"),
    ),
    (
        "phase_chain/admitted_scaffold/birth_proof_support.rs",
        "phase_chain/admitted_scaffold/birth_proof_support.rs",
        include_str!("../../../phase_chain/admitted_scaffold/birth_proof_support.rs"),
    ),
    (
        "tests/support/compound_required_inventory.rs",
        "tests/support/compound_required_inventory.rs",
        include_str!("../compound_required_inventory.rs"),
    ),
    (
        "tests/support/prepared_outcome.rs",
        "tests/support/prepared_outcome.rs",
        include_str!("../prepared_outcome.rs"),
    ),
    (
        "tests/support/prepared_result.rs",
        "tests/support/prepared_result.rs",
        include_str!("../prepared_result.rs"),
    ),
    (
        "tests/support/realization/report_support.rs",
        "tests/support/realization/report_support.rs",
        include_str!("../realization/report_support.rs"),
    ),
    (
        "tests/support/realization/exhaustion_witness.rs",
        "tests/support/realization/exhaustion_witness.rs",
        include_str!("../realization/exhaustion_witness.rs"),
    ),
];

pub(crate) fn reference_consumer_adoption_source_set() -> ForgeQueryEvidenceReportAdoptionSourceSet
{
    let covered = COVERED_REPORT_SUPPORT_FILES.iter().fold(
        ForgeQueryEvidenceReportAdoptionSourceSet::new("worth-kernel"),
        |sources, (label, path, source)| {
            sources.source_file(
                *label,
                *path,
                *source,
                ForgeQueryEvidenceReportAdoptionResidueClassification::CoveredQueryEvidenceAdoption,
            )
        },
    );

    DEFENDED_DOMAIN_DIGEST_FILES
        .iter()
        .fold(covered, |sources, (label, path, source)| {
            sources.source_file(
                *label,
                *path,
                *source,
                ForgeQueryEvidenceReportAdoptionResidueClassification::DefendedDomainArtifactIdentity,
            )
        })
}

pub(crate) fn reference_consumer_adoption_source_count() -> usize {
    COVERED_REPORT_SUPPORT_FILES.len() + DEFENDED_DOMAIN_DIGEST_FILES.len()
}

pub(crate) fn expected_defended_residue_symbol_count() -> usize {
    DEFENDED_DOMAIN_DIGEST_FILES
        .iter()
        .map(|(_, _, source)| {
            [
                "digest_owned_parts",
                "digest_owned_parts_with_scope",
                "ConstructionDigestScope",
            ]
            .into_iter()
            .filter(|symbol| source.contains(symbol))
            .count()
        })
        .sum()
}

pub(crate) fn assert_reference_consumer_adoption_inventory_consistent() {
    for (path, _, source) in DEFENDED_DOMAIN_DIGEST_FILES {
        assert!(
            !COVERED_REPORT_SUPPORT_FILES
                .iter()
                .any(|(covered_path, _, _)| covered_path == path),
            "{path} is both covered Query evidence adoption and defended worth-domain residue"
        );
        assert!(
            !source.contains("forge.query.evidence-identity.v1"),
            "{path} must not smuggle Query evidence identity through worth-domain digest helpers"
        );
    }
}
