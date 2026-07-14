use std::collections::BTreeSet;

use crate::maintenance::{
    derived_index_parity_cases, derived_index_rebuild_admission_cases,
    derived_index_rebuild_execution_cases, layout_parity_verification, layout_rebuild_admission,
    layout_rebuild_candidate_readmission, layout_rebuild_execution,
    DerivedIndexCandidateReadmissionReceipt,
};
use crate::strategy::tests_support::{
    admit_btree_page_strategy, admit_persisted_lsm_strategy, admitted_page_key_bytes,
    root_manifest_scope, strategy_test_wal_security_scope_for_store,
};
use crate::strategy::AdmittedLayoutStrategy;
use crate::{
    access_shapes, DerivedIndexCandidateDeclaration, DerivedIndexParityBasis,
    DerivedIndexParityRow, DerivedIndexRebuildRequest, DerivedIndexRebuildSourceInput,
};
use worth_store_wal::BlobWalRecordKind;

use super::rebuild_support::root_rebuild_setup;
use crate::maintenance::test_support::{
    root_manifest_source_witness, wal_replay_source_witness_for_identity,
    wal_replay_source_witness_with_security,
};

#[test]
fn rebuild_admission_declares_exactly_ordinary_owner_cases() {
    let strategy = admit_btree_page_strategy();
    let source = root_manifest_source_witness(7, 11);
    let (shape, materialization) = root_rebuild_setup(strategy.admitted_family(), &source);
    let mut observed = BTreeSet::new();

    observed.insert(
        layout_rebuild_admission()
            .admit_plan(request(
                strategy,
                strategy.admitted_key_domain(),
                shape,
                materialization.clone(),
                DerivedIndexRebuildSourceInput::PhysicalRootManifest {
                    source: source.clone(),
                },
            ))
            .case_id()
            .as_str(),
    );
    let lsm_strategy = admit_persisted_lsm_strategy();
    let (lsm_shape, lsm_materialization) =
        super::rebuild_support::wal_rebuild_setup(lsm_strategy.admitted_family());
    let crate::LayoutMaterializationSourceKind::LsmReplacement(lsm_identity) =
        lsm_materialization.source().kind()
    else {
        panic!("persisted LSM materialization must retain replacement identity");
    };
    let current_security =
        worth_store_security::admitted_store_wal_checkpoint_security_scope_for_layout_partition_test();
    observed.insert(
        layout_rebuild_admission()
            .admit_plan(request(
                lsm_strategy,
                lsm_strategy.admitted_key_domain(),
                lsm_shape,
                lsm_materialization.clone(),
                DerivedIndexRebuildSourceInput::WalReplayRecord {
                    source_witness: wal_replay_source_witness_for_identity(
                        worth_store_wal::BlobWalRecordIdentity::new(
                            lsm_identity.sequence().checked_add(1).unwrap(),
                            BlobWalRecordKind::GenerationPublication,
                        )
                        .unwrap(),
                        current_security.witnesses(),
                    ),
                },
            ))
            .case_id()
            .as_str(),
    );
    let foreign_security =
        worth_store_security::admitted_tenant_wal_checkpoint_security_scope_for_layout_partition_test();
    observed.insert(
        layout_rebuild_admission()
            .admit_plan(request(
                lsm_strategy,
                lsm_strategy.admitted_key_domain(),
                lsm_shape,
                lsm_materialization.clone(),
                DerivedIndexRebuildSourceInput::WalReplayRecord {
                    source_witness: wal_replay_source_witness_with_security(
                        &lsm_materialization,
                        BlobWalRecordKind::GenerationPublication,
                        foreign_security.witnesses(),
                    ),
                },
            ))
            .case_id()
            .as_str(),
    );
    let foreign_authority =
        strategy_test_wal_security_scope_for_store("store.foreign.rebuild.matrix");
    observed.insert(
        layout_rebuild_admission()
            .admit_plan(request(
                lsm_strategy,
                lsm_strategy.admitted_key_domain(),
                lsm_shape,
                lsm_materialization.clone(),
                DerivedIndexRebuildSourceInput::WalReplayRecord {
                    source_witness: wal_replay_source_witness_with_security(
                        &lsm_materialization,
                        BlobWalRecordKind::GenerationPublication,
                        foreign_authority.witnesses(),
                    ),
                },
            ))
            .case_id()
            .as_str(),
    );
    observed.insert(
        layout_rebuild_admission()
            .admit_plan(request(
                strategy,
                strategy.admitted_key_domain(),
                shape,
                materialization.clone(),
                DerivedIndexRebuildSourceInput::DiagnosticReport,
            ))
            .case_id()
            .as_str(),
    );
    observed.insert(
        layout_rebuild_admission()
            .admit_plan(request(
                strategy,
                strategy.admitted_key_domain(),
                access_shapes().point_lookup_declaration(),
                materialization.clone(),
                DerivedIndexRebuildSourceInput::PhysicalRootManifest {
                    source: source.clone(),
                },
            ))
            .case_id()
            .as_str(),
    );
    observed.insert(
        layout_rebuild_admission()
            .admit_plan(request(
                strategy,
                strategy.admitted_key_domain(),
                shape,
                materialization.clone(),
                DerivedIndexRebuildSourceInput::WalReplayRecord {
                    source_witness: wal_replay_source_witness_for_identity(
                        worth_store_wal::BlobWalRecordIdentity::new(
                            73,
                            BlobWalRecordKind::GenerationPublication,
                        )
                        .unwrap(),
                        worth_store_security::admitted_store_wal_checkpoint_security_scope_for_layout_partition_test()
                            .witnesses(),
                    ),
                },
            ))
            .case_id()
            .as_str(),
    );
    let (_, wrong_domain) = root_manifest_scope();
    observed.insert(
        layout_rebuild_admission()
            .admit_plan(request(
                strategy,
                wrong_domain,
                shape,
                materialization,
                DerivedIndexRebuildSourceInput::PhysicalRootManifest { source },
            ))
            .case_id()
            .as_str(),
    );

    let declared = derived_index_rebuild_admission_cases()
        .map(|case| case.as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(observed, declared);
}

#[test]
fn rebuild_execution_declares_exactly_ordinary_owner_cases() {
    let root_strategy = admit_btree_page_strategy();
    let root_source = root_manifest_source_witness(7, 11);
    let (root_shape, root_materialization) =
        root_rebuild_setup(root_strategy.admitted_family(), &root_source);
    let mut observed = BTreeSet::new();

    let plan = layout_rebuild_admission()
        .admit_plan(request(
            root_strategy,
            root_strategy.admitted_key_domain(),
            root_shape,
            root_materialization.clone(),
            DerivedIndexRebuildSourceInput::PhysicalRootManifest {
                source: root_source.clone(),
            },
        ))
        .into_admitted()
        .unwrap();
    observed.insert(layout_rebuild_execution().execute(plan).case_id().as_str());

    let declared = derived_index_rebuild_execution_cases()
        .map(|case| case.as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(observed, declared);
}

#[test]
fn parity_verification_declares_exactly_ordinary_owner_cases() {
    let mut observed = BTreeSet::new();
    observed.insert(
        layout_parity_verification()
            .verify(root_receipt(11, None, None, None))
            .case_id()
            .as_str(),
    );
    observed.insert(
        layout_parity_verification()
            .verify(root_receipt(12, None, None, None))
            .case_id()
            .as_str(),
    );
    observed.insert(
        layout_parity_verification()
            .verify(root_receipt(11, Some(vec![999]), None, None))
            .case_id()
            .as_str(),
    );
    observed.insert(
        layout_parity_verification()
            .verify(root_receipt(11, None, Some("copied-value"), None))
            .case_id()
            .as_str(),
    );
    let strategy = admit_btree_page_strategy();
    let wrong_coverage = crate::materialization::test_support::materialization_observations()
        .exact_root_epoch_coverage(
            crate::materialization::LayoutMaterializationState::exact(
                strategy.admitted_family().declaration().family(),
            ),
            worth_store_physical_format::PhysicalEpoch::from_raw(999).unwrap(),
        )
        .unwrap();
    observed.insert(
        layout_parity_verification()
            .verify(root_receipt(11, None, None, Some(wrong_coverage)))
            .case_id()
            .as_str(),
    );

    let declared = derived_index_parity_cases()
        .map(|case| case.as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(observed, declared);
}

fn root_receipt(
    page: u64,
    counter_shape: Option<Vec<u64>>,
    value_override: Option<&str>,
    coverage_override: Option<crate::LayoutCoverageWitness>,
) -> DerivedIndexCandidateReadmissionReceipt {
    let strategy = admit_btree_page_strategy();
    let source = root_manifest_source_witness(7, 11);
    let (shape, materialization) = root_rebuild_setup(strategy.admitted_family(), &source);
    let plan = layout_rebuild_admission()
        .admit_plan(request(
            strategy,
            strategy.admitted_key_domain(),
            shape,
            materialization,
            DerivedIndexRebuildSourceInput::PhysicalRootManifest {
                source: source.clone(),
            },
        ))
        .into_admitted()
        .unwrap();
    let admitted_coverage =
        coverage_override.unwrap_or_else(|| plan.rebuild_scope().authority_coverage().clone());
    let counters = counter_shape.unwrap_or_else(|| source.witness().counter_shape().to_vec());
    let value = value_override.unwrap_or_else(|| source.witness().rows()[0].value_fingerprint());
    let basis = DerivedIndexParityBasis::new(
        vec![DerivedIndexParityRow::new(
            admitted_page_key_bytes(7, page),
            value,
        )],
        admitted_coverage,
        true,
        counters,
    )
    .unwrap();
    let execution = layout_rebuild_execution().execute(plan).into_rebuilt();
    layout_rebuild_candidate_readmission().readmit(
        execution,
        DerivedIndexCandidateDeclaration::from_canonical_basis(basis),
    )
}

fn request(
    strategy: AdmittedLayoutStrategy,
    key_domain: crate::AdmittedPhysicalKeyDomain,
    shape: crate::access::shape::AccessShapeContract,
    materialization: crate::AdmittedLayoutMaterialization,
    source: DerivedIndexRebuildSourceInput,
) -> DerivedIndexRebuildRequest {
    DerivedIndexRebuildRequest::new(
        strategy.admitted_family(),
        key_domain,
        strategy.family(),
        shape,
        materialization,
        source,
    )
}
