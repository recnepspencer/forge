use forge_store_security::{
    admitted_tenant_wal_checkpoint_security_scope_for_layout_access_test, StoreKeyVersionPosture,
    StoreLegacySecurityPosture,
};

use super::*;

#[test]
fn reopen_rejects_same_identity_artifact_redirected_outside_wal_root() {
    let access = crate::layout_access::WalLayoutAccess::s8();
    let security = admitted_tenant_wal_checkpoint_security_scope_for_layout_access_test();
    let metadata = crate::WalSecurityMetadataCarrier::for_wal_record(
        security.witnesses(),
        StoreKeyVersionPosture::Current,
        StoreLegacySecurityPosture::NativeScoped,
    );
    let key = access
        .admit_baseline_lsm_key(metadata, *b"redirect")
        .unwrap();
    let (envelope, anchor) =
        crate::layout_access::baseline_lsm_certification_execution::durable_record_binding(
            &access,
            key,
            480,
            BlobWalRecordKind::LsmValue,
        );
    let mut session = access.open_baseline_lsm_index(&anchor).unwrap();
    access
        .persist_baseline_lsm_record(&mut session, envelope, &anchor, key)
        .unwrap();
    drop(session);

    let redirected_root = std::env::current_dir()
        .unwrap()
        .join("target")
        .join(format!(
            "forge-store-redirect-hostile-{}",
            std::process::id()
        ));
    std::fs::create_dir_all(&redirected_root).unwrap();
    let redirected = redirected_root.join("forged-record");
    std::fs::copy(anchor.persisted_path(), &redirected).unwrap();
    let journal = anchor
        .persisted_path()
        .parent()
        .unwrap()
        .join("baseline-lsm-membership.journal");
    std::fs::write(
        journal,
        format!(
            "A {} {}\n",
            super::persisted_codec::hex(redirected.as_os_str().to_string_lossy().as_bytes()),
            anchor.persisted_bytes(),
        ),
    )
    .unwrap();
    assert_eq!(
        access.open_baseline_lsm_index(&anchor).unwrap_err(),
        super::super::BaselineLsmExecutionAdmissionDenial::DurableRecordBindingMismatch
    );
}
