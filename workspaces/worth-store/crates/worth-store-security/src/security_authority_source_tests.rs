use crate::{
    classify_app_org_id_as_security_scope_source, classify_iam_role_as_security_scope_source,
    classify_identity_provider_claim_as_security_scope_source,
    classify_kms_key_id_as_security_scope_source,
    classify_operator_identity_as_security_scope_source, reject_non_store_security_scope_source,
    StoreSecurityScopeDenialKind,
};

#[test]
fn security_scope_admission_rejects_adjacent_identity_and_operator_sources() {
    assert_eq!(
        reject_non_store_security_scope_source(
            classify_identity_provider_claim_as_security_scope_source()
        )
        .kind(),
        StoreSecurityScopeDenialKind::JwtSubjectIsNotTenantScope
    );
    assert_eq!(
        reject_non_store_security_scope_source(classify_app_org_id_as_security_scope_source())
            .kind(),
        StoreSecurityScopeDenialKind::ApplicationOrgIdIsNotTenantScope
    );
    assert_eq!(
        reject_non_store_security_scope_source(classify_kms_key_id_as_security_scope_source())
            .kind(),
        StoreSecurityScopeDenialKind::KmsKeyIdIsNotKeyScope
    );
    assert_eq!(
        reject_non_store_security_scope_source(classify_iam_role_as_security_scope_source()).kind(),
        StoreSecurityScopeDenialKind::IamRoleIsNotCustodyPosture
    );
    assert_eq!(
        reject_non_store_security_scope_source(
            classify_operator_identity_as_security_scope_source()
        )
        .kind(),
        StoreSecurityScopeDenialKind::OperatorIdentityIsNotRepairAuthority
    );
}
