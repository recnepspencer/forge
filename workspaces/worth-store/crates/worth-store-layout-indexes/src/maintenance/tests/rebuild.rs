use crate::maintenance::{
    layout_parity_verification, layout_rebuild_admission, layout_rebuild_candidate_readmission,
    layout_rebuild_execution,
};
use crate::maintenance::{DerivedIndexParityView, DerivedIndexRebuildAdmissionView};
use crate::strategy::tests_support::{admit_btree_page_strategy, admit_persisted_lsm_strategy};
use crate::{
    DerivedIndexCandidateDeclaration, DerivedIndexCandidateReadmissionReceipt,
    DerivedIndexCostEnvelopeParity, DerivedIndexCounterShapeParity, DerivedIndexParityBasis,
    DerivedIndexRebuildDenied, DerivedIndexRebuildReceipt, DerivedIndexRebuildRequest,
    DerivedIndexRebuildSourceInput, DerivedIndexResultIdentity, LayoutCorruptionView,
};
use worth_store_wal::BlobWalRecordKind;

use super::rebuild_support::{
    root_rebuild_setup, root_rebuilt_parity_basis_with_value, wal_rebuild_setup,
};
use crate::maintenance::test_support::{
    root_manifest_source_witness, root_manifest_source_witness_rows, wal_replay_source_witness,
};

#[test]
fn derived_projection_rebuilds_to_visible_parity_from_root_manifest_authority() {
    let strategy = admit_btree_page_strategy();
    let source_witness = root_manifest_source_witness(7, 11);
    let (rebuild_shape, materialization) =
        root_rebuild_setup(strategy.admitted_family(), &source_witness);
    let request = DerivedIndexRebuildRequest::new(
        strategy.admitted_family(),
        strategy.admitted_key_domain(),
        strategy.family(),
        rebuild_shape,
        materialization,
        DerivedIndexRebuildSourceInput::PhysicalRootManifest {
            source: source_witness.clone(),
        },
    );

    let plan = layout_rebuild_admission()
        .admit_plan(request)
        .into_admitted()
        .unwrap();
    assert!(matches!(
        plan.corruption().view(),
        LayoutCorruptionView::RebuildRequired(_)
    ));

    let rebuilt = layout_rebuild_execution().execute(plan).into_rebuilt();
    let counters = rebuilt.counters();
    assert_eq!(counters.source_artifacts_read(), 1);
    assert_eq!(counters.source_rows_read(), 1);
    assert_eq!(counters.candidate_rows_written(), 1);
    assert_eq!(
        counters.source_bytes_read(),
        counters.candidate_bytes_written()
    );
    assert_eq!(
        rebuilt.plan().result_identity(),
        DerivedIndexResultIdentity::PhysicalRoot {
            reference: source_witness.witness().root_reference(),
            authority: source_witness.store_authority_identity(),
        }
    );

    let parity_outcome = layout_parity_verification().verify(readmit_generated(rebuilt));
    let parity_counters = parity_outcome.counters();
    assert_eq!(parity_counters.coverage_comparisons(), 1);
    assert_eq!(parity_counters.key_comparisons(), 1);
    assert_eq!(parity_counters.value_comparisons(), 1);
    assert_eq!(
        parity_counters.counter_shape_comparisons(),
        source_witness.witness().counter_shape().len() as u64
    );
    assert!(parity_counters.bytes_compared() > 0);
    let parity = parity_outcome
        .into_verified()
        .expect("expected parity witness");
    assert!(parity.parity_holds());
    assert_eq!(
        parity.value_identity(),
        crate::maintenance::DerivedIndexIdentityParity::Exact
    );
    assert_eq!(
        parity.cost_envelope(),
        DerivedIndexCostEnvelopeParity::SourceArtifactDoesNotProveDeclaredEnvelope
    );
    assert_eq!(
        parity.counter_shape(),
        DerivedIndexCounterShapeParity::ExactDeterministicPhysicalShape
    );
}

#[test]
fn parity_denies_coverage_substitution_against_root_manifest_authority() {
    let strategy = admit_btree_page_strategy();
    let source_witness = root_manifest_source_witness(7, 11);
    let (rebuild_shape, materialization) =
        root_rebuild_setup(strategy.admitted_family(), &source_witness);
    let plan = layout_rebuild_admission()
        .admit_plan(DerivedIndexRebuildRequest::new(
            strategy.admitted_family(),
            strategy.admitted_key_domain(),
            strategy.family(),
            rebuild_shape,
            materialization,
            DerivedIndexRebuildSourceInput::PhysicalRootManifest {
                source: source_witness.clone(),
            },
        ))
        .into_admitted()
        .unwrap();
    let substituted_coverage = crate::materialization::test_support::materialization_observations()
        .exact_root_epoch_coverage(
            crate::materialization::LayoutMaterializationState::exact(
                strategy.admitted_family().declaration().family(),
            ),
            worth_store_physical_format::PhysicalEpoch::from_raw(999).unwrap(),
        )
        .unwrap();
    let execution = layout_rebuild_execution().execute(plan).into_rebuilt();
    let rebuilt = readmit_candidate(
        execution,
        root_rebuilt_parity_basis_with_value(
            substituted_coverage,
            source_witness.witness(),
            source_witness.witness().rows()[0].value_fingerprint(),
        ),
    );

    let outcome = layout_parity_verification().verify(rebuilt);
    assert_eq!(outcome.counters().coverage_comparisons(), 1);
    assert_eq!(outcome.counters().key_comparisons(), 0);
    assert!(matches!(
        outcome.view(),
        DerivedIndexParityView::Denied(crate::DerivedIndexParityDenied::CoverageMismatch)
    ));
}

#[test]
fn rebuild_and_parity_counters_scale_exactly_with_source_rows() {
    const ROWS: u64 = 512;
    let strategy = admit_btree_page_strategy();
    let source_witness = root_manifest_source_witness_rows(7, ROWS);
    let (rebuild_shape, materialization) =
        root_rebuild_setup(strategy.admitted_family(), &source_witness);
    let request = DerivedIndexRebuildRequest::new(
        strategy.admitted_family(),
        strategy.admitted_key_domain(),
        strategy.family(),
        rebuild_shape,
        materialization,
        DerivedIndexRebuildSourceInput::PhysicalRootManifest {
            source: source_witness,
        },
    );
    let plan = layout_rebuild_admission()
        .admit_plan(request)
        .into_admitted()
        .unwrap();
    let receipt = layout_rebuild_execution().execute(plan).into_rebuilt();
    let rebuild_counters = receipt.counters();
    assert_eq!(rebuild_counters.source_rows_read(), ROWS);
    assert_eq!(rebuild_counters.candidate_rows_written(), ROWS);
    assert_eq!(rebuild_counters.canonical_row_order_comparisons(), ROWS - 1);
    assert_eq!(rebuild_counters.unique_key_comparisons(), ROWS - 1);
    assert_eq!(
        rebuild_counters.counter_shape_order_comparisons(),
        source_witness_counter_shape_comparisons(ROWS)
    );
    assert_eq!(
        rebuild_counters.source_bytes_read(),
        rebuild_counters.candidate_bytes_written()
    );

    let parity = layout_parity_verification().verify(readmit_generated(receipt));
    assert_eq!(parity.counters().authority_rows_materialized(), ROWS);
    assert_eq!(
        parity.counters().authority_bytes_materialized(),
        rebuild_counters.source_bytes_read()
    );
    assert_eq!(
        parity.counters().authority_row_order_comparisons(),
        ROWS - 1
    );
    assert_eq!(
        parity.counters().authority_unique_key_comparisons(),
        ROWS - 1
    );
    assert_eq!(
        parity
            .counters()
            .authority_counter_shape_order_comparisons(),
        source_witness_counter_shape_comparisons(ROWS)
    );
    assert_eq!(parity.counters().key_comparisons(), ROWS);
    assert_eq!(parity.counters().value_comparisons(), ROWS);
    assert!(parity.into_verified().is_ok());
}

fn source_witness_counter_shape_comparisons(rows: u64) -> u64 {
    let source = root_manifest_source_witness_rows(7, rows);
    (source.witness().counter_shape().len() as u64).saturating_sub(1)
}

#[test]
fn empty_authoritative_manifest_rebuilds_empty_projection_without_inventing_corruption() {
    let strategy = admit_btree_page_strategy();
    let source = root_manifest_source_witness_rows(7, 0);
    let (rebuild_shape, materialization) = root_rebuild_setup(strategy.admitted_family(), &source);
    let request = DerivedIndexRebuildRequest::new(
        strategy.admitted_family(),
        strategy.admitted_key_domain(),
        strategy.family(),
        rebuild_shape,
        materialization,
        DerivedIndexRebuildSourceInput::PhysicalRootManifest { source },
    );

    let plan = layout_rebuild_admission()
        .admit_plan(request)
        .into_admitted()
        .expect("an empty authoritative manifest is a valid rebuild source");
    assert!(matches!(
        plan.corruption().view(),
        LayoutCorruptionView::RebuildRequired(_)
    ));
    let rebuilt = layout_rebuild_execution().execute(plan).into_rebuilt();
    assert_eq!(rebuilt.counters().source_rows_read(), 0);
    assert_eq!(rebuilt.counters().candidate_rows_written(), 0);
    assert!(layout_parity_verification()
        .verify(readmit_generated(rebuilt))
        .into_verified()
        .is_ok());
}

#[test]
fn unsuitable_wal_record_kind_is_denied_without_becoming_physical_corruption() {
    let strategy = admit_persisted_lsm_strategy();
    let (rebuild_shape, materialization) = wal_rebuild_setup(strategy.admitted_family());
    let source_witness =
        wal_replay_source_witness(&materialization, BlobWalRecordKind::RootCandidate);
    let request = DerivedIndexRebuildRequest::new(
        strategy.admitted_family(),
        strategy.admitted_key_domain(),
        strategy.family(),
        rebuild_shape,
        materialization,
        DerivedIndexRebuildSourceInput::WalReplayRecord {
            source_witness: source_witness.clone(),
        },
    );

    assert!(matches!(
        layout_rebuild_admission().admit_plan(request).view(),
        DerivedIndexRebuildAdmissionView::Denied(
            DerivedIndexRebuildDenied::SourceArtifactDoesNotMatchStrategy {
                source: "wal_replay_record_kind",
                ..
            }
        )
    ));
}

#[test]
fn derived_data_inputs_are_denied_as_rebuild_sources() {
    let strategy = admit_btree_page_strategy();
    let source = root_manifest_source_witness(7, 11);
    let (rebuild_shape, materialization) = root_rebuild_setup(strategy.admitted_family(), &source);

    for source in [
        DerivedIndexRebuildSourceInput::DerivedProjectionRows,
        DerivedIndexRebuildSourceInput::CertificationRows,
        DerivedIndexRebuildSourceInput::DiagnosticReport,
        DerivedIndexRebuildSourceInput::JsonProjection,
        DerivedIndexRebuildSourceInput::TerminalProjection,
    ] {
        let request = DerivedIndexRebuildRequest::new(
            strategy.admitted_family(),
            strategy.admitted_key_domain(),
            strategy.family(),
            rebuild_shape,
            materialization.clone(),
            source,
        );

        let outcome = layout_rebuild_admission().admit_plan(request);
        assert!(matches!(
            outcome.view(),
            DerivedIndexRebuildAdmissionView::Denied(
                DerivedIndexRebuildDenied::SourceInputIsNotAuthority { .. }
            )
        ));
    }
}

fn readmit_generated(
    execution: DerivedIndexRebuildReceipt,
) -> DerivedIndexCandidateReadmissionReceipt {
    let declaration = execution.candidate_declaration();
    layout_rebuild_candidate_readmission().readmit(execution, declaration)
}

fn readmit_candidate(
    execution: DerivedIndexRebuildReceipt,
    candidate: DerivedIndexParityBasis,
) -> DerivedIndexCandidateReadmissionReceipt {
    layout_rebuild_candidate_readmission().readmit(
        execution,
        DerivedIndexCandidateDeclaration::from_canonical_basis(candidate),
    )
}
