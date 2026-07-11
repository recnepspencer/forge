use std::io::Write as _;

use forge_store_security::{StoreKeyScope, StoreTenantScope};

use super::persisted_index_tests::{
    membership_journal, persist_production_complete, persist_production_tail, production_anchor,
};
use super::*;

#[test]
fn reopen_rejects_equal_membership_manifest_from_another_security_scope() {
    let (access, first_key, anchor, mut session) = production_anchor(*b"scope001", 500);
    persist_production_tail(&access, &mut session, first_key, 501);
    let first_plan = BaselineLsmCompactionPlan::lower_from_persisted(&session, first_key).unwrap();
    let first_manifest = admitted_manifest(&access, &first_plan, 7, 500, 504);

    let other_key = BaselineLsmAdmittedKey {
        tenant_scope: StoreTenantScope::MultiTenantPhysicalBoundary,
        key_scope: StoreKeyScope::WalCheckpointEnvelope,
        canonical_key_bytes: *b"scope001",
    };
    persist_production_complete(&access, &mut session, other_key, 500);
    drop(session);
    append_retirement(&anchor, other_key, &first_manifest);

    assert_eq!(
        access.open_baseline_lsm_index(&anchor).unwrap_err(),
        super::super::BaselineLsmExecutionAdmissionDenial::ManifestMembershipMismatch
    );
}

#[test]
fn reopen_rejects_manifest_that_does_not_cover_authoritative_membership() {
    let (access, key, anchor, mut session) = production_anchor(*b"cover001", 520);
    persist_production_tail(&access, &mut session, key, 521);
    let plan = BaselineLsmCompactionPlan::lower_from_persisted(&session, key).unwrap();
    let incomplete = admitted_manifest(&access, &plan, 8, 521, 523);
    drop(session);
    append_retirement(&anchor, key, &incomplete);

    assert_eq!(
        access.open_baseline_lsm_index(&anchor).unwrap_err(),
        super::super::BaselineLsmExecutionAdmissionDenial::ManifestMembershipMismatch
    );
}

#[test]
fn reopen_rejects_correct_manifest_redirected_outside_store_root() {
    let (access, key, anchor, mut session) = production_anchor(*b"mroot001", 540);
    persist_production_tail(&access, &mut session, key, 541);
    let plan = BaselineLsmCompactionPlan::lower_from_persisted(&session, key).unwrap();
    let manifest = admitted_manifest(&access, &plan, 9, 540, 544);
    drop(session);

    let outside = std::env::current_dir()
        .unwrap()
        .join("target")
        .join("forge-store-foreign-manifest");
    std::fs::create_dir_all(outside.parent().unwrap()).unwrap();
    std::fs::copy(manifest.persisted_path(), &outside).unwrap();
    append_raw_retirement(&anchor, key, &outside, manifest.persisted_bytes());

    assert_eq!(
        access.open_baseline_lsm_index(&anchor).unwrap_err(),
        super::super::BaselineLsmExecutionAdmissionDenial::ManifestMembershipMismatch
    );
}

fn admitted_manifest(
    access: &crate::layout_projection::WalLayoutAccess,
    plan: &BaselineLsmCompactionPlan,
    checkpoint: u64,
    covered_lsn_start: u64,
    covered_lsn_end: u64,
) -> crate::AdmittedCheckpointPublicationReceipt {
    let scope = plan
        .manifest_scope(
            crate::StoreCheckpointRecordIdentity::new(checkpoint),
            covered_lsn_start,
            covered_lsn_end,
        )
        .unwrap();
    access
        .admit_baseline_lsm_manifest_durability(
            &crate::layout_projection::baseline_lsm_certification_execution::manifest_receipt(
                scope,
            ),
        )
        .unwrap()
}

fn append_retirement(
    anchor: &crate::AdmittedWalAppendReceipt,
    key: BaselineLsmAdmittedKey,
    manifest: &crate::AdmittedCheckpointPublicationReceipt,
) {
    append_raw_retirement(
        anchor,
        key,
        manifest.persisted_path(),
        manifest.persisted_bytes(),
    );
}

fn append_raw_retirement(
    anchor: &crate::AdmittedWalAppendReceipt,
    key: BaselineLsmAdmittedKey,
    manifest_path: &std::path::Path,
    manifest_bytes: u64,
) {
    writeln!(
        std::fs::OpenOptions::new()
            .append(true)
            .open(membership_journal(anchor))
            .unwrap(),
        "R {} {} {} {} {}",
        super::persisted_codec::tenant_code(key.tenant_scope()),
        super::persisted_codec::key_scope_code(key.key_scope()),
        super::persisted_codec::hex(&key.canonical_key_bytes()),
        super::persisted_codec::hex(manifest_path.as_os_str().to_string_lossy().as_bytes()),
        manifest_bytes,
    )
    .unwrap();
}
