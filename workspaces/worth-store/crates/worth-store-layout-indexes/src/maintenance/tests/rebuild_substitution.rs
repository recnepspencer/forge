use crate::maintenance::test_support::{root_manifest_source_witness, wal_replay_source_witness};
use crate::maintenance::{
    layout_parity_verification, layout_rebuild_admission, layout_rebuild_candidate_readmission,
    layout_rebuild_execution, DerivedIndexParityView,
};
use crate::strategy::tests_support::{
    admit_btree_page_strategy, admit_persisted_lsm_strategy, admitted_page_key_bytes,
};
use crate::{
    DerivedIndexCandidateDeclaration, DerivedIndexParityBasis, DerivedIndexParityRow,
    DerivedIndexRebuildReceipt, DerivedIndexRebuildRequest, DerivedIndexRebuildSourceInput,
};
use worth_store_wal::BlobWalRecordKind;

use super::rebuild_support::{
    root_rebuild_setup, root_rebuilt_parity_basis_with_value, wal_rebuild_setup,
    wal_rebuilt_parity_basis_with_value,
};

#[test]
fn parity_denies_value_substitution_against_root_manifest_authority() {
    let strategy = admit_btree_page_strategy();
    let source = root_manifest_source_witness(7, 11);
    let (shape, materialization) = root_rebuild_setup(strategy.admitted_family(), &source);
    let coverage = materialization.coverage().clone();
    let request = DerivedIndexRebuildRequest::new(
        strategy.admitted_family(),
        strategy.admitted_key_domain(),
        strategy.family(),
        shape,
        materialization,
        DerivedIndexRebuildSourceInput::PhysicalRootManifest {
            source: source.clone(),
        },
    );
    let execution = execute(request);
    let rebuilt = readmit(
        execution,
        root_rebuilt_parity_basis_with_value(coverage, source.witness(), "rebuilt-page-mismatch"),
    );
    assert!(matches!(
        layout_parity_verification().verify(rebuilt).view(),
        DerivedIndexParityView::Denied(crate::DerivedIndexParityDenied::ValueIdentityMismatch)
    ));
}

#[test]
fn parity_denies_value_substitution_against_wal_authority() {
    let strategy = admit_persisted_lsm_strategy();
    let (shape, materialization) = wal_rebuild_setup(strategy.admitted_family());
    let coverage = materialization.coverage().clone();
    let source =
        wal_replay_source_witness(&materialization, BlobWalRecordKind::GenerationPublication);
    let request = DerivedIndexRebuildRequest::new(
        strategy.admitted_family(),
        strategy.admitted_key_domain(),
        strategy.family(),
        shape,
        materialization,
        DerivedIndexRebuildSourceInput::WalReplayRecord {
            source_witness: source.clone(),
        },
    );
    let rebuilt = readmit(
        execute(request),
        wal_rebuilt_parity_basis_with_value(coverage, &source, "rebuilt-wal-mismatch"),
    );
    assert!(matches!(
        layout_parity_verification().verify(rebuilt).view(),
        DerivedIndexParityView::Denied(crate::DerivedIndexParityDenied::ValueIdentityMismatch)
    ));
}

#[test]
fn parity_denies_rebuilt_counter_shape_substitution() {
    let strategy = admit_btree_page_strategy();
    let source = root_manifest_source_witness(7, 11);
    let (shape, materialization) = root_rebuild_setup(strategy.admitted_family(), &source);
    let coverage = materialization.coverage().clone();
    let request = DerivedIndexRebuildRequest::new(
        strategy.admitted_family(),
        strategy.admitted_key_domain(),
        strategy.family(),
        shape,
        materialization,
        DerivedIndexRebuildSourceInput::PhysicalRootManifest {
            source: source.clone(),
        },
    );
    let basis = DerivedIndexParityBasis::new(
        vec![DerivedIndexParityRow::new(
            admitted_page_key_bytes(7, 11),
            source.witness().rows()[0].value_fingerprint(),
        )],
        coverage,
        true,
        vec![999],
    )
    .unwrap();
    let rebuilt = readmit(execute(request), basis);
    assert!(matches!(
        layout_parity_verification().verify(rebuilt).view(),
        DerivedIndexParityView::Denied(crate::DerivedIndexParityDenied::CounterShapeMismatch)
    ));
}

fn execute(request: DerivedIndexRebuildRequest) -> DerivedIndexRebuildReceipt {
    let plan = layout_rebuild_admission()
        .admit_plan(request)
        .into_admitted()
        .unwrap();
    layout_rebuild_execution().execute(plan).into_rebuilt()
}

fn readmit(
    execution: DerivedIndexRebuildReceipt,
    basis: DerivedIndexParityBasis,
) -> crate::DerivedIndexCandidateReadmissionReceipt {
    layout_rebuild_candidate_readmission().readmit(
        execution,
        DerivedIndexCandidateDeclaration::from_canonical_basis(basis),
    )
}
