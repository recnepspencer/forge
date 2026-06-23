use forge_query::facade::runtime::{
    ForgeQueryGraphObligationKind, ForgeQueryGraphObligationSupportLane,
    ForgeQueryGraphObligationSupportMatrix, ForgeQueryGraphTouchSelector,
};
use forge_query::{
    graph_obligation_consumer_kit, ForgeQueryBoundaryAuditSourceSet,
    ForgeQueryGraphObligationConsumerKitErrorKind,
    ForgeQueryGraphObligationConsumerRegistrationDeclaration,
    ForgeQueryGraphObligationLocalCeremonyAudit, ForgeQueryGraphObligationResidueManifest,
    ForgeQueryGraphObligationResidueRow, ForgeQueryGraphObligationSelectorCoverageDeclaration,
    ForgeQueryGraphObligationSupportPin,
};

use super::*;
use crate::query_adoption::{
    spatial_query_adoption_inventory, WorthSpatialQueryAdoptionClassification,
};

#[test]
fn adoption_proof_uses_query_consumer_kit_execution_authority() {
    let proof = spatial_query_graph_obligation_adoption_proof()
        .expect("spatial Query consumer-kit adoption proof");

    assert!(proof.execution_proof().has_real_executor_rows());
    assert_eq!(proof.support_pin().row_count(), 1);
    assert_eq!(
        proof.local_ceremony_audit().evaluated_source_count(),
        expected_local_ceremony_source_labels().len()
    );
    assert_eq!(
        proof.local_ceremony_audit().audited_source_labels(),
        expected_local_ceremony_source_labels()
    );
    assert_eq!(proof.residue_manifest().rows().len(), 2);
    assert!(proof
        .residue_manifest()
        .rows()
        .iter()
        .any(|row| row.class() == "worth-spatial-broad-collection-selector"));
    assert_eq!(proof.execution_proof().selected_obligation_count(), 1);
    assert_eq!(proof.execution_proof().rows().len(), 1);
    assert!(!proof.manifest().manifest_digest().is_empty());
    assert!(!proof.execution_proof().proof_digest().is_empty());
}

#[test]
fn consumer_kit_rejects_incomplete_or_substituted_adoption_inputs() {
    let registration = spatial_graph_obligation_registration().expect("registration");
    let descriptor = spatial_graph_touch_descriptor().expect("descriptor");
    let operating_world = spatial_operating_world_descriptor();
    let matrix = ForgeQueryGraphObligationSupportMatrix::assembly_selection_foundation();

    let missing_registration = graph_obligation_consumer_kit(CONSUMER_NAME)
        .declare_selector_coverage(spatial_selector_coverage().expect("coverage"))
        .pin_support(spatial_support_pin())
        .against_support_matrix(matrix.clone())
        .audit_local_ceremony(spatial_local_ceremony_audit())
        .prove_execution_with(&descriptor, &operating_world);
    assert_eq!(
        missing_registration.unwrap_err().kind(),
        ForgeQueryGraphObligationConsumerKitErrorKind::MissingRegistrationDeclaration
    );

    let uncovered_selector = graph_obligation_consumer_kit(CONSUMER_NAME)
        .register_obligations(
            ForgeQueryGraphObligationConsumerRegistrationDeclaration::for_runtime_family(
                RUNTIME_FAMILY,
                [registration.clone()],
            )
            .expect("declaration"),
        )
        .declare_selector_coverage(
            ForgeQueryGraphObligationSelectorCoverageDeclaration::required([(
                "wrong collection",
                ForgeQueryGraphTouchSelector::collection("worth.spatial.wrong").expect("selector"),
            )]),
        )
        .pin_support(spatial_support_pin())
        .against_support_matrix(matrix.clone())
        .audit_local_ceremony(spatial_local_ceremony_audit())
        .prove_execution_with(&descriptor, &operating_world)
        .and_then(|kit| kit.prove_adoption_with_execution());
    assert_eq!(
        uncovered_selector.unwrap_err().kind(),
        ForgeQueryGraphObligationConsumerKitErrorKind::SelectorCoverageMismatch
    );

    let unsupported_pin = graph_obligation_consumer_kit(CONSUMER_NAME)
        .register_obligations(
            ForgeQueryGraphObligationConsumerRegistrationDeclaration::for_runtime_family(
                RUNTIME_FAMILY,
                [registration.clone()],
            )
            .expect("declaration"),
        )
        .declare_selector_coverage(spatial_selector_coverage().expect("coverage"))
        .pin_support(ForgeQueryGraphObligationSupportPin::supported([(
            ForgeQueryGraphObligationKind::BlockingInvariant,
            ForgeQueryGraphObligationSupportLane::GraphComposition,
        )]))
        .against_support_matrix(matrix.clone())
        .audit_local_ceremony(spatial_local_ceremony_audit())
        .prove_execution_with(&descriptor, &operating_world)
        .and_then(|kit| kit.prove_adoption_with_execution());
    assert_eq!(
        unsupported_pin.unwrap_err().kind(),
        ForgeQueryGraphObligationConsumerKitErrorKind::SupportPinDrift
    );

    let unevaluated_audit = graph_obligation_consumer_kit(CONSUMER_NAME)
        .register_obligations(
            ForgeQueryGraphObligationConsumerRegistrationDeclaration::for_runtime_family(
                RUNTIME_FAMILY,
                [registration.clone()],
            )
            .expect("declaration"),
        )
        .declare_selector_coverage(spatial_selector_coverage().expect("coverage"))
        .pin_support(spatial_support_pin())
        .against_support_matrix(matrix.clone())
        .audit_local_ceremony(ForgeQueryGraphObligationLocalCeremonyAudit::clean())
        .prove_execution_with(&descriptor, &operating_world)
        .and_then(|kit| kit.prove_adoption_with_execution());
    assert_eq!(
        unevaluated_audit.unwrap_err().kind(),
        ForgeQueryGraphObligationConsumerKitErrorKind::UnevaluatedLocalCeremonyAudit
    );

    let uncapped_residue = ForgeQueryGraphObligationResidueManifest::capped([
        ForgeQueryGraphObligationResidueRow::explicit(
            "uncapped local report",
            "worth-spatial",
            "phase-8",
            2,
            1,
            "legacy local report was not capped",
            "delete legacy local report",
            "remove",
        )
        .expect("row"),
    ]);
    assert_eq!(
        uncapped_residue.unwrap_err().kind(),
        ForgeQueryGraphObligationConsumerKitErrorKind::ResidueCapExceeded
    );

    let in_memory_only = graph_obligation_consumer_kit(CONSUMER_NAME)
        .register_obligations(
            ForgeQueryGraphObligationConsumerRegistrationDeclaration::for_runtime_family(
                RUNTIME_FAMILY,
                [registration],
            )
            .expect("declaration"),
        )
        .declare_selector_coverage(spatial_selector_coverage().expect("coverage"))
        .pin_support(spatial_support_pin())
        .against_support_matrix(matrix)
        .audit_local_ceremony(spatial_local_ceremony_audit())
        .prove_in_memory_selection(&descriptor, &operating_world)
        .expect("selection proof")
        .prove_adoption_with_execution();
    assert_eq!(
        in_memory_only.unwrap_err().kind(),
        ForgeQueryGraphObligationConsumerKitErrorKind::MissingInMemoryProof
    );
}

#[test]
fn spatial_public_status_exposes_execution_backed_adoption_status() {
    let status = current_spatial_query_consumer_kit_adoption_status()
        .expect("spatial Query consumer-kit adoption should be current");

    assert_eq!(status.support_requirement_count(), 1);
    assert_eq!(status.support_matched_required_count(), 1);
    assert_eq!(status.support_blocking_finding_count(), 0);
    assert_eq!(
        status.boundary_audit_source_count(),
        expected_local_ceremony_source_labels().len()
    );
    assert_eq!(status.boundary_audit_coverage_row_count(), 0);
    assert_eq!(status.workload_support_pin_row_count(), 1);
    assert!(status.hard_prohibition_audit_clean());
    assert_eq!(status.selected_obligation_count(), 1);
    assert_eq!(status.execution_row_count(), 1);
    assert_eq!(status.candidate_registration_count(), 1);
    assert_eq!(status.denied_row_count(), 0);
    assert_eq!(status.full_scan_count(), 0);
    assert_eq!(status.residue_row_count(), 2);
    assert!(!status.adoption_manifest_digest().is_empty());
    assert!(!status.execution_proof_digest().is_empty());
    assert!(!status.evidence_report_identity().is_empty());
    assert!(!status.evidence_digest_participation_identity().is_empty());
    assert!(!status.boundary_audit_report_identity().is_empty());
}

#[test]
fn local_ceremony_audit_covers_real_worth_spatial_sources() {
    let sources = spatial_local_ceremony_source_set();
    let labels = sources.source_labels();

    assert_eq!(labels, expected_local_ceremony_source_labels());
    for source in sources.sources() {
        assert_eq!(source.path(), Some(source.label()));
        assert!(!source.source().trim().is_empty());
    }

    let audit = ForgeQueryGraphObligationLocalCeremonyAudit::evaluate(&sources);

    assert!(audit.is_evaluated());
    assert!(audit.is_clean());
    assert_eq!(
        audit.evaluated_source_count(),
        expected_local_ceremony_source_labels().len()
    );
    assert_eq!(
        audit.audited_source_labels(),
        expected_local_ceremony_source_labels()
    );
}

#[test]
fn local_ceremony_audit_detects_seeded_bypass_in_real_source_scope() {
    let seeded_source = format!(
        "{QUERY_ADOPTION_RS}\nfn seeded_bypass_after_real_source<'a>() {{ let _ = ForgeQueryGraphObligationIndex::from_catalog(&catalog); }}\n"
    );
    let sources = ForgeQueryBoundaryAuditSourceSet::new("worth-spatial").source_file(
        QUERY_ADOPTION_SOURCE_LABEL,
        QUERY_ADOPTION_SOURCE_LABEL,
        seeded_source,
    );

    let audit = ForgeQueryGraphObligationLocalCeremonyAudit::evaluate(&sources);

    assert_eq!(audit.findings().len(), 1);
    let finding = &audit.findings()[0];
    assert_eq!(finding.source_label(), QUERY_ADOPTION_SOURCE_LABEL);
    assert_eq!(finding.source_path(), Some(QUERY_ADOPTION_SOURCE_LABEL));
    assert_eq!(
        finding.pattern(),
        "ForgeQueryGraphObligationIndex::from_catalog"
    );
}

#[test]
fn hard_break_deletes_local_reports_and_caps_remaining_support_projection_residue() {
    let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    assert!(!crate_root
        .join("src/query_adoption/evidence_reports.rs")
        .exists());
    assert!(!crate_root
        .join("src/query_adoption/boundary_audit.rs")
        .exists());

    let residue = spatial_query_graph_obligation_residue_manifest().expect("residue manifest");
    let support_projection = residue
        .rows()
        .iter()
        .find(|row| row.class() == "worth-spatial-runtime-facade-support-projection")
        .expect("support projection residue row");
    assert_eq!(support_projection.owner(), "worth-spatial");
    assert_eq!(support_projection.current_count(), 1);
    let broad_selector = residue
        .rows()
        .iter()
        .find(|row| row.class() == "worth-spatial-broad-collection-selector")
        .expect("broad selector residue row");
    assert_eq!(broad_selector.owner(), "worth-spatial");
    assert_eq!(broad_selector.current_count(), 1);

    let inventory = spatial_query_adoption_inventory();
    assert!(inventory.iter().any(|row| {
        row.classification() == WorthSpatialQueryAdoptionClassification::ExplicitResidue
            && row.replacement_surface() == "crates/worth-spatial/src/query_adoption/residue.rs"
    }));
    assert!(inventory.iter().all(|row| {
        row.replacement_surface() != "crates/worth-spatial/src/query_adoption/evidence_reports.rs"
            && row.replacement_surface()
                != "crates/worth-spatial/src/query_adoption/boundary_audit.rs"
    }));
}

fn expected_local_ceremony_source_labels() -> Vec<&'static str> {
    vec![
        QUERY_ADOPTION_SOURCE_LABEL,
        FACADE_QUERY_ADOPTION_SOURCE_LABEL,
        PERFORMANCE_COUNTERS_SOURCE_LABEL,
        RESIDUE_SOURCE_LABEL,
        SUPPORT_PROJECTION_SOURCE_LABEL,
    ]
}
