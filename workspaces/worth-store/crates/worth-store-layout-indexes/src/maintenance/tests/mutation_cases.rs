use std::collections::BTreeSet;

use worth_store_budgets::PreExecutionBudgetEnvelope;
use worth_store_contracts::WalRecordFamily;
use worth_store_security::admitted_store_wal_checkpoint_security_scope_for_layout_partition_test;
use worth_store_wal::StoreWalRecordIdentity;

use crate::maintenance::{
    layout_lsm_maintenance, layout_mutation_admission, layout_mutation_admission_cases,
    IndexMaintenanceFailureOutcome, LayoutMutationAdmissionView, LsmRunPublicationAdmissionRequest,
};

#[test]
fn mutation_admission_declares_exactly_the_remaining_lsm_cases() {
    let security = admitted_store_wal_checkpoint_security_scope_for_layout_partition_test();
    let lsm = layout_lsm_maintenance()
        .admit_run_publication(LsmRunPublicationAdmissionRequest::new(
            security.witnesses(),
            WalRecordFamily::DurableMutationIntent,
            StoreWalRecordIdentity::new(43),
            PreExecutionBudgetEnvelope::maintenance_default(),
        ))
        .into_result()
        .unwrap();

    let planned = layout_mutation_admission().admit_lsm_append(lsm);
    assert!(matches!(
        planned.view(),
        LayoutMutationAdmissionView::Planned(_)
    ));
    let denied = layout_mutation_admission().deny_in_place_reachable_overwrite();
    assert!(matches!(
        denied.view(),
        LayoutMutationAdmissionView::Denied(
            IndexMaintenanceFailureOutcome::InPlaceReachableOverwriteUnsupported
        )
    ));

    assert_eq!(
        [planned.case_id(), denied.case_id()]
            .into_iter()
            .collect::<BTreeSet<_>>(),
        layout_mutation_admission_cases().collect::<BTreeSet<_>>()
    );
}

#[test]
fn lsm_mutation_plan_extracts_its_exact_admission() {
    let security = admitted_store_wal_checkpoint_security_scope_for_layout_partition_test();
    let lsm = layout_lsm_maintenance()
        .admit_run_publication(LsmRunPublicationAdmissionRequest::new(
            security.witnesses(),
            WalRecordFamily::DurableMutationIntent,
            StoreWalRecordIdentity::new(44),
            PreExecutionBudgetEnvelope::maintenance_default(),
        ))
        .into_result()
        .unwrap();
    let expected = lsm.selected().request_identity();

    let exact = layout_mutation_admission()
        .admit_lsm_append(lsm)
        .into_planned()
        .unwrap()
        .into_lsm_append();

    assert_eq!(exact.selected().request_identity(), expected);
}
