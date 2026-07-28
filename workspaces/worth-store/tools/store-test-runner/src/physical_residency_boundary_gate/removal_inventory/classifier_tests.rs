use super::*;

#[test]
fn inventory_gate_rejects_an_unclassified_consumer_path() {
    let discovered = BTreeMap::from([(
        "crates/worth-store/src/physical_runtime/foreign.rs".to_owned(),
        BTreeSet::from(["c6-identifier".to_owned()]),
    )]);

    let denial = compare_inventory(&discovered, &BTreeMap::new())
        .expect_err("an unclassified consumer must be denied");

    assert!(denial.contains("unclassified consumers"));
}

#[test]
fn classifier_detects_legacy_manifest_and_direct_pool_edges() {
    let families = discover_families(
        "crates/example/Cargo.toml",
        r#"legacy = { features = ["legacy-s2-models"] }"#,
    );
    assert!(families.contains("legacy-s2-feature"));

    let indirect_alias = discover_families(
        "crates/example/Cargo.toml",
        r#"
[features]
bridge = ["worth-store-buffer-pool/legacy-s2-models"]
certification = ["bridge"]
"#,
    );
    assert!(
        indirect_alias.contains("legacy-s2-feature"),
        "an aggregate alias cannot hide a legacy dependency feature"
    );

    let certification = discover_families(
        "crates/worth-store-certification/src/new_consumer.rs",
        "use worth_store_buffer_pool::PhysicalResidencyCounters;",
    );
    assert!(
        certification.contains("direct-pool-consumer"),
        "certification is a consumer of Store truth, not a canonical pool owner"
    );
}

#[test]
fn classifier_detects_a_renamed_exact_count_residency_snapshot() {
    for source in [
        "pub struct PhysicalSubstrateReadinessSnapshot;",
        "pub use physical_substrate::PhysicalSubstrateReadinessSnapshot;",
        "fn snapshot() -> PhysicalSubstrateReadinessSnapshot { todo!() }",
    ] {
        assert!(
            discover_families(
                "crates/worth-store-readiness/src/renamed_snapshot.rs",
                source,
            )
            .contains("snapshot-residency-authority"),
            "renaming an exact-count snapshot cannot preserve the deleted authority graph: {source}"
        );
    }
}

#[test]
fn inventory_source_selection_includes_executable_docs_but_not_project_docs() {
    assert!(is_inventory_source(Path::new(
        "crates/example/src/compile_fail_proofs.md"
    )));
    assert!(is_inventory_source(Path::new(
        "crates/example/tests/ui/authority.md"
    )));
    assert!(!is_inventory_source(Path::new(
        "_docs/worth-store/architecture.md"
    )));
}

#[test]
fn classifier_detects_closeout_identifiers_and_deleted_path_resurrection() {
    for source in [
        "pub enum S2AcceptanceSuiteKind {}",
        "pub struct HarnessCloseoutEvidenceReport;",
        "pub struct HarnessCloseoutTranscriptEvidence;",
        "pub struct BoundedMemoryResidencySuite;",
        "mod bounded_memory_harness_closeout;",
        "mod bounded_memory_residency_suite;",
        "mod acceptance_suite_transcript;",
    ] {
        assert!(
            discover_families(
                "crates/worth-store-certification/src/orphaned_closeout.rs",
                source,
            )
            .contains("legacy-certification-closeout"),
            "the orphaned S.2 closeout family must remain discoverable: {source}"
        );
    }

    for path in [
        "crates/worth-store-certification/src/courtroom/memory/bounded_memory_residency_suite.rs",
        "crates/worth-store-certification/src/courtroom/physical_substrate/acceptance_suite_transcript.rs",
        "crates/worth-store-certification/src/scenario/memory/bounded_memory_harness_closeout.rs",
    ] {
        assert!(
            discover_families(path, "pub struct RenamedSyntheticAuthority;")
                .contains("legacy-certification-closeout"),
            "renamed contents cannot hide resurrection at {path}"
        );
    }
}

#[test]
fn classifier_detects_legacy_byte_guard_api_fragments() {
    for (fragment, family) in [
        ("ResidentFrameToken", "legacy-frame-table"),
        ("PinnedPageLease", "legacy-frame-table"),
        ("for_legacy_resident_frame", "legacy-frame-table"),
        ("PinnedFrameView", "legacy-record-view"),
        ("OwnedReadBuffer", "legacy-record-view"),
        ("for_owned_read_buffer", "legacy-record-view"),
        ("from_bounded_copy", "legacy-record-view"),
        ("from_pinned_frame", "legacy-record-view"),
        ("RecordViewEvidenceReport", "legacy-record-view"),
    ] {
        assert!(
            discover_families(
                "crates/worth-store-certification/src/legacy_api_consumer.rs",
                fragment,
            )
            .contains(family),
            "the public legacy API fragment must remain discoverable: {fragment}"
        );
    }

    for path in [
        "crates/worth-store-certification/src/evidence/cross_cutting/record_view_evidence.rs",
        "crates/worth-store-certification/src/evidence/cross_cutting/record_view_evidence_admission_tests.rs",
        "crates/worth-store-certification/src/evidence/cross_cutting/record_view_evidence_conflict_tests.rs",
        "crates/worth-store-certification/src/courtroom/harness/test_support/record_view_evidence_test_support.rs",
    ] {
        assert!(
            discover_families(path, "pub struct RenamedLegacyViewEvidence;")
                .contains("legacy-record-view"),
            "renamed contents cannot hide a legacy record-view evidence path: {path}"
        );
    }
}

#[test]
fn classifier_detects_unconditional_predecessor_files_and_publication_consumers() {
    for path in [
        "crates/worth-store-buffer-pool/src/access_policy_lifecycle.rs",
        "crates/worth-store-buffer-pool/src/budget.rs",
        "crates/worth-store-buffer-pool/src/budget_units.rs",
        "crates/worth-store-buffer-pool/src/renamed_predecessor.rs",
    ] {
        assert!(
            discover_families(path, "pub struct InnocuousName;")
                .contains("legacy-buffer-pool-predecessor"),
            "an identifier-free predecessor file must remain discoverable: {path}"
        );
    }

    assert!(
        discover_families(
            "crates/worth-store-buffer-pool/src/physical_residency/new_owner.rs",
            "pub struct InnocuousName;",
        )
        .is_empty(),
        "the canonical destination is not predecessor inventory"
    );

    for identifier in [
        "DirtyPublicationEvidence",
        "PageFlushRecoveryReceipt",
        "WalBeforeDataOrderingProof",
        "NoUndoPublicationProof",
        "ReopenedPageRecoveryEvidence",
        "StalePageRecoveryClassification",
        "RollbackImagePublicationDeclaration",
        "PageWritePolicyObservation",
        "WalCoveragePolicyAssessment",
        "NoUndoRecoveryPolicyAssessment",
        "UnadmittedDirtyPagePublicationDenial",
    ] {
        assert!(
            discover_families(
                "crates/worth-store-certification/src/recovery/renamed_consumer.rs",
                identifier,
            )
            .contains("legacy-page-publication-authority"),
            "a consumer cannot survive merely because its authority definition was deleted: {identifier}"
        );
    }

    for (path, source) in [
        (
            "crates/worth-store-certification/tests/scenarios/recovery/page_lsn_publication/page_generation_paths.rs",
            "pub fn generation_fixture() {}",
        ),
        (
            "crates/worth-store-certification/tests/suites/durability_recovery.rs",
            "#[path = \"../scenarios/recovery/page_lsn_publication/page_lsn_publication.rs\"]",
        ),
    ] {
        assert!(
            discover_families(path, source).contains("legacy-page-publication-authority"),
            "module closure and selectors must remain discoverable: {path}"
        );
    }
}

#[test]
fn classifier_detects_scheduler_capacity_bypasses_and_deleted_paths() {
    for fragment in [
        "BackgroundPacingCapability",
        "BackgroundPacingAuthority",
        "BackgroundPacingReady",
        "BackgroundPacingProgressionOutcome",
        "prove_background_pacing_current",
        "from_scheduler_capability",
        "with_pacing_admission",
        "io_readmission_satisfied",
        "admitted_compaction(",
    ] {
        assert!(
            discover_families(
                "crates/worth-store-blob-chunks/src/compaction/renamed_bypass.rs",
                fragment,
            )
            .contains("scheduler-capacity-publication"),
            "the deleted capacity or self-admission fragment must remain discoverable: {fragment}"
        );
    }

    for path in [
        "crates/worth-store-io-scheduler/src/background_pacing/capability.rs",
        "crates/worth-store-io-scheduler/src/background_pacing/proof.rs",
        "crates/worth-store-io-scheduler/src/background_pacing/tests/progression.rs",
        "crates/worth-store-blob-chunks/src/compaction/verification/pacing_admission.rs",
    ] {
        assert!(
            discover_families(path, "pub struct RenamedCapacityBypass;")
                .contains("scheduler-capacity-publication"),
            "renamed contents cannot hide deleted capacity source resurrection at {path}"
        );
    }
}

#[test]
fn classifier_distinguishes_live_self_admission_from_its_negative_proof() {
    let attempted_call = "BlobCompactionIntent::admitted_compaction(basis);";

    assert!(
        discover_families(
            "crates/worth-store-blob-chunks/src/compaction/bypass.rs",
            attempted_call,
        )
        .contains("scheduler-capacity-publication"),
        "live Rust must not regain a deleted self-admission constructor"
    );
    assert!(
        !discover_families(
            "crates/worth-store-blob-chunks/src/compaction/compile_fail_proofs.md",
            attempted_call,
        )
        .contains("scheduler-capacity-publication"),
        "the adversarial proof must be allowed to attempt the prohibited call"
    );
}
