use super::{S8CoverageBasisKind, S8MaterializationDenial, S8MaterializationStateClass};
use crate::facade::{
    access_planning,
    layout_declarations,
};
use crate::key_domain::tests_support::{
    published_blob_evidence_bundle, published_blob_import_declaration,
};
use crate::{
    S8AccessAuthorityPosture, S8AccessShape, S8AccessShapeUnsupportedDenial,
    S8AccessStaleDisposition, S8ExpectedCounterClass,
};
use worth_store_physical_format::PhysicalEpoch;
use worth_store_recovery_physics::{CheckpointCoveredLsnRange, LogSequenceNumber};

fn format_family() -> &'static crate::PhysicalArtifactFamilyDeclaration {
    layout_declarations().seed_family()
}

#[test]
fn exact_absence_requires_exact_coverage() {
    let family = format_family().family();
    let stale = access_planning()
        .stale_root_epoch_coverage(
            format_family(),
            PhysicalEpoch::from_raw(7).expect("epoch fixture should be valid"),
        )
        .expect("coverage should build");

    assert_eq!(
        access_planning().prove_exact_index_absence(stale),
        Err(S8MaterializationDenial::LayoutCoverageIsStale {
            family,
            basis_kind: S8CoverageBasisKind::RootEpoch,
        })
    );
}

#[test]
fn partial_coverage_localizes_gap() {
    let gap =
        CheckpointCoveredLsnRange::new(LogSequenceNumber::new(11), LogSequenceNumber::new(19))
            .expect("gap fixture should be valid");
    let partial = access_planning()
        .partial_wal_lsn_coverage(
            format_family(),
            LogSequenceNumber::new(10),
            LogSequenceNumber::new(20),
            gap,
        )
        .expect("partial coverage should build");

    assert_eq!(
        access_planning().prove_exact_index_absence(partial),
        Err(S8MaterializationDenial::LayoutCoverageIsPartial {
            gap: partial.gap().expect("partial coverage retains its gap"),
        })
    );
}

#[test]
fn exact_through_basis_survives_range_and_prefix_completeness() {
    let coverage = access_planning()
        .exact_root_epoch_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                format_family().family(),
            ),
            PhysicalEpoch::from_raw(31).expect("epoch fixture should be valid"),
        )
        .expect("exact coverage should build");

    let range = access_planning()
        .require_exact_range_access(coverage)
        .expect("range should be exact");
    let prefix = access_planning()
        .require_exact_prefix_access(coverage)
        .expect("prefix should be exact");

    assert_eq!(range.shape(), S8AccessShape::RangeLookup);
    assert_eq!(range.coverage(), Some(coverage));
    assert_eq!(
        range.expected_counters(),
        S8ExpectedCounterClass::RangeLookup
    );
    assert_eq!(prefix.shape(), S8AccessShape::PrefixLookup);
    assert_eq!(prefix.coverage(), Some(coverage));
    assert_eq!(
        prefix.expected_counters(),
        S8ExpectedCounterClass::PrefixLookup
    );
}

#[test]
fn checkpoint_and_blob_generation_coverages_are_first_class_public_lanes() {
    let checkpoint =
        CheckpointCoveredLsnRange::new(LogSequenceNumber::new(21), LogSequenceNumber::new(29))
            .expect("checkpoint fixture should be valid");
    let checkpoint_coverage = access_planning()
        .exact_checkpoint_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                format_family().family(),
            ),
            checkpoint,
        )
        .expect("checkpoint coverage should admit");
    let checkpoint_absence = access_planning()
        .prove_exact_index_absence(checkpoint_coverage)
        .expect("checkpoint exact coverage should support exact absence");

    assert_eq!(checkpoint_absence.coverage(), checkpoint_coverage);
    assert_eq!(
        checkpoint_coverage.upper_bound().basis_kind(),
        S8CoverageBasisKind::CheckpointFrontier
    );
    assert_eq!(checkpoint_coverage.upper_bound().start_inclusive(), 21);
    assert_eq!(checkpoint_coverage.upper_bound().value(), 29);

    let blob_bundle = published_blob_evidence_bundle();
    let blob_coverage = access_planning()
        .exact_blob_generation_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                format_family().family(),
            ),
            blob_bundle.export_generation(),
        )
        .expect("blob generation coverage should admit");
    let blob_absence = access_planning()
        .prove_exact_index_absence(blob_coverage)
        .expect("blob generation exact coverage should support exact absence");

    assert_eq!(blob_absence.coverage(), blob_coverage);
    assert_eq!(
        blob_coverage.upper_bound().basis_kind(),
        S8CoverageBasisKind::BlobGeneration
    );
}

#[test]
fn coverage_basis_witnesses_survive_reopen_and_certification_replay() {
    let root_epoch = PhysicalEpoch::from_raw(37).expect("epoch fixture should be valid");
    let root_from_open = access_planning()
        .exact_root_epoch_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                format_family().family(),
            ),
            root_epoch,
        )
        .expect("root epoch coverage should admit");
    let root_from_reopen = access_planning()
        .exact_root_epoch_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                format_family().family(),
            ),
            root_epoch,
        )
        .expect("reopened root epoch coverage should admit");

    assert_eq!(root_from_open, root_from_reopen);

    let wal_lsn = LogSequenceNumber::new(64);
    let wal_from_log = access_planning()
        .exact_wal_lsn_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                format_family().family(),
            ),
            wal_lsn,
        )
        .expect("wal lsn coverage should admit");
    let wal_from_replay = access_planning()
        .exact_wal_lsn_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                format_family().family(),
            ),
            wal_lsn,
        )
        .expect("replayed wal lsn coverage should admit");

    assert_eq!(wal_from_log, wal_from_replay);

    let blob_bundle = published_blob_evidence_bundle();
    let import_declaration = published_blob_import_declaration();
    let blob_from_lifecycle = access_planning()
        .exact_blob_generation_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                format_family().family(),
            ),
            blob_bundle.lifecycle_declaration().generation(),
        )
        .expect("lifecycle blob generation should admit");
    let blob_from_export = access_planning()
        .exact_blob_generation_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                format_family().family(),
            ),
            blob_bundle.export_generation(),
        )
        .expect("export blob generation should admit");
    let blob_from_replay = access_planning()
        .exact_blob_generation_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                format_family().family(),
            ),
            import_declaration.generation(),
        )
        .expect("replayed blob generation should admit");

    assert_eq!(blob_from_lifecycle, blob_from_export);
    assert_eq!(blob_from_lifecycle, blob_from_replay);
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

    let degraded = access_planning()
        .prove_degraded_bounded_scan_absence(coverage)
        .expect("bounded exact scan should stay admitted as degraded access");

    assert_eq!(degraded.shape(), S8AccessShape::DegradedExactScan);
    assert_eq!(degraded.coverage(), Some(coverage));
    assert_eq!(
        degraded.authority_posture(),
        S8AccessAuthorityPosture::ExplicitDegradedExactScan
    );
    assert_eq!(
        degraded.stale_disposition(),
        S8AccessStaleDisposition::ExplicitDegradedFallback
    );
    assert_eq!(
        degraded.expected_counters(),
        S8ExpectedCounterClass::DegradedExactScan
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
        access_planning().require_exact_range_access(lagged),
        Err(S8AccessShapeUnsupportedDenial::MaterializationDenied(
            S8MaterializationDenial::LayoutCoverageIsLagged {
            family,
            basis_kind: S8CoverageBasisKind::WalLsn,
        })) if family == format_family().family()
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
        access_planning().require_exact_prefix_access(quarantined),
        Err(S8AccessShapeUnsupportedDenial::MaterializationDenied(
            S8MaterializationDenial::LayoutRangeIsQuarantined {
                gap: quarantined
                    .gap()
                    .expect("quarantined coverage retains its gap"),
            }
        ))
    );
    assert_eq!(
        quarantined.state().class(),
        S8MaterializationStateClass::Quarantined
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
        Err(S8MaterializationDenial::CoverageIntervalIsReversed {
            family: format_family().family(),
            basis_kind: S8CoverageBasisKind::WalLsn,
            lower_bound: 44,
            upper_bound: 40,
        })
    );
}
