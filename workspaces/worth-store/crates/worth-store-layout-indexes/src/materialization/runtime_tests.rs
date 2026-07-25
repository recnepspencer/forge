use super::{CoverageBasisKind, MaterializationDenial, MaterializationStateClass};
use crate::observation::AccessShape;
use crate::{
    access_planning, AccessAuthorityPosture, AccessStaleDisposition, ExpectedCounterClass,
};
use worth_store_physical_format::PhysicalEpoch;
use worth_store_recovery_physics::{CheckpointCoveredLsnRange, LogSequenceNumber};

fn format_family() -> &'static crate::PhysicalArtifactFamilyDeclaration {
    crate::layout_declarations().seed_family()
}

fn owner_sources(path: &str) -> String {
    fn collect(path: &std::path::Path, sources: &mut Vec<std::path::PathBuf>) {
        for entry in std::fs::read_dir(path).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                collect(&path, sources);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                sources.push(path);
            }
        }
    }

    let mut sources = Vec::new();
    collect(
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path),
        &mut sources,
    );
    sources.sort();
    sources
        .into_iter()
        .map(|source| std::fs::read_to_string(source).unwrap())
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn checkpoint_exactness_preserves_full_range_identity() {
    let left = access_planning()
        .exact_checkpoint_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                format_family().family(),
            ),
            CheckpointCoveredLsnRange::new(LogSequenceNumber::new(21), LogSequenceNumber::new(29))
                .expect("left checkpoint fixture should be valid"),
        )
        .expect("left checkpoint coverage should admit");
    let right = access_planning()
        .exact_checkpoint_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                format_family().family(),
            ),
            CheckpointCoveredLsnRange::new(LogSequenceNumber::new(24), LogSequenceNumber::new(29))
                .expect("right checkpoint fixture should be valid"),
        )
        .expect("right checkpoint coverage should admit");

    assert_ne!(left, right);
    assert_eq!(left.upper_bound().value(), right.upper_bound().value());
    assert_ne!(
        left.upper_bound().start_inclusive(),
        right.upper_bound().start_inclusive()
    );
}

#[test]
fn bounded_scan_absence_is_separate_from_exact_index_absence() {
    let coverage = access_planning()
        .exact_root_epoch_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                format_family().family(),
            ),
            PhysicalEpoch::from_raw(17).expect("epoch fixture should be valid"),
        )
        .expect("exact coverage should build");
    coverage
        .require_exact()
        .expect("degraded scan fixture coverage should be exact");
    let degraded = crate::access_shapes()
        .explicit_degraded_exact_scan(
            crate::DegradedExactScanRequest::new().with_budget_rows(8_192),
        )
        .expect("bounded exact scan should stay admitted as degraded access");

    assert_eq!(degraded.shape(), AccessShape::DegradedExactScan);
    assert_eq!(
        degraded.authority_posture(),
        AccessAuthorityPosture::ExplicitDegradedExactScan
    );
    assert_eq!(
        degraded.stale_disposition(),
        AccessStaleDisposition::ExplicitDegradedFallback
    );
    assert_eq!(
        degraded.expected_counters(),
        ExpectedCounterClass::DegradedExactScan
    );
    assert_eq!(degraded.budget_rows(), Some(8_192));
}

#[test]
fn lagged_and_quarantined_states_deny_exact_completeness() {
    let lagged = access_planning()
        .lagged_wal_lsn_coverage(
            format_family(),
            LogSequenceNumber::new(40),
            LogSequenceNumber::new(44),
        )
        .expect("lagged coverage should build");
    assert!(matches!(
        lagged.require_exact(),
        Err(MaterializationDenial::LayoutCoverageIsLagged {
            family,
            basis_kind: CoverageBasisKind::WalLsn,
        }) if family == format_family().family()
    ));

    let quarantined = access_planning()
        .quarantined_wal_lsn_coverage(
            format_family(),
            LogSequenceNumber::new(49),
            LogSequenceNumber::new(53),
            CheckpointCoveredLsnRange::new(LogSequenceNumber::new(50), LogSequenceNumber::new(52))
                .expect("quarantine fixture should be valid"),
        )
        .expect("quarantined coverage should build");
    assert_eq!(
        quarantined.require_exact(),
        Err(MaterializationDenial::LayoutRangeIsQuarantined {
            gap: quarantined
                .gap()
                .expect("quarantined coverage retains its gap"),
        })
    );
    assert_eq!(
        quarantined.state().class(),
        MaterializationStateClass::Quarantined
    );
}

#[test]
fn reversed_lagged_intervals_are_denied_at_admission() {
    assert_eq!(
        access_planning().lagged_wal_lsn_coverage(
            format_family(),
            LogSequenceNumber::new(44),
            LogSequenceNumber::new(40),
        ),
        Err(MaterializationDenial::CoverageIntervalIsReversed {
            family: format_family().family(),
            basis_kind: CoverageBasisKind::WalLsn,
            lower_bound: 44,
            upper_bound: 40,
        })
    );
}

#[test]
fn ordinary_runtime_owners_cannot_assemble_coverage_from_scalar_watermarks() {
    for relative in [
        "src/read",
        "src/recovery/btree_replay",
        "src/maintenance/lsm",
    ] {
        let source = owner_sources(relative);
        for forbidden in [
            ".exact_root_epoch_coverage(",
            ".exact_wal_lsn_coverage(",
            ".admit_catalog_materialization(",
            ".watermark()",
        ] {
            assert!(
                !source.contains(forbidden),
                "ordinary owner {relative} retained raw materialization lane {forbidden}",
            );
        }
    }
}

#[test]
fn degraded_rebind_consumes_a_replacement_owner_selection() {
    let source = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/access/execution/degraded_scan/staged_runtime.rs"),
    )
    .unwrap();
    assert!(source.contains("replacement: SelectedDegradedExactScan"));
    assert!(source.contains("admission: super::DegradedScanRebindAdmission"));
    assert!(!source.contains("coverage: crate::materialization::LayoutCoverageWitness"));
    assert!(!source.contains("family: crate::AdmittedPhysicalArtifactFamily"));
    assert!(!source.contains("key_domain: crate::AdmittedPhysicalKeyDomain"));
}

#[test]
fn ordinary_lookup_coverage_is_derived_from_lower_owned_sources() {
    let source = owner_sources("src/read");
    assert!(!source.contains("coverage_epoch"));
    assert!(!source.contains("coverage_lsn"));
    assert!(source.contains("admit_btree_lookup_materialization"));
    assert!(source.contains("current_btree_materialization_frontier"));
    assert!(source.contains("request.current_catalog"));
    assert!(source.contains("request.current_source.as_ref().unwrap_or(&request.source)"));
    assert!(source.contains("&request.source"));
    assert!(!source.contains("request.source.replacement_output()"));
    let replay = owner_sources("src/recovery/btree_replay");
    assert!(!replay.contains("coverage_epoch"));
    assert!(replay.contains("request.physical_source.root_reference"));
    assert!(replay.contains("admit_btree_replay_materialization"));
    let lsm_replay = owner_sources("src/maintenance/lsm");
    assert!(!lsm_replay.contains("coverage_lsn"));
    assert!(lsm_replay.contains("request.source"));
    assert!(lsm_replay.contains("admit_lsm_replay_materialization"));
    assert!(!lsm_replay.contains("expected_output_identity"));
}

#[test]
fn persisted_lsm_membership_cannot_be_readmitted_under_another_store_family() {
    let replacement =
        crate::strategy::tests_support::certification_published_lsm_membership_replacement();
    let (foreign_family, _) = crate::strategy::tests_support::admit_strategy_scope(
        worth_store_contracts::DurableArtifactFamilyId::PublicationWalIntent,
        worth_store_security::StoreKeyScope::WalCheckpointEnvelope,
        worth_store_security::StoreTenantScope::StoreInternal,
        worth_store_security::StoreAuthenticityRequirement::required(
            worth_store_security::StoreAuthenticityRequirementClass::AuthenticatedWalRecord,
        ),
        worth_store_security::StoreCustodyPosture::InternalStoreCustody,
    );

    assert_eq!(
        crate::lsm_strategy().readmit_lookup_source(foreign_family, &replacement),
        Err(crate::BaselineLsmExecutionAdmissionDenial::RecordKeyScopeMismatch)
    );
}
