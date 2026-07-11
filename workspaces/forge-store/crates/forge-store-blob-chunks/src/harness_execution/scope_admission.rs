use forge_store_security::{
    admit_store_security_scope, StoreAuthenticityRequirement, StoreAuthenticityRequirementClass,
    StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture,
    StoreSecurityScopeAdmissionExpectation, StoreSecurityScopeAdmissionRequest, StoreTenantScope,
};

use crate::handoffs::BlobHarnessSecurityScopeClass;
use crate::{
    BlobChunkSecurityScope, BlobChunkSequenceAdmission, BlobChunkSize, BlobChunkingRuleAdmission,
};

use super::backend::{current_authority, physical_payload_for_bytes};
use super::transition_success::TransitionSuccess;

pub(super) fn blob_scope(
    case: &str,
    scope_class: BlobHarnessSecurityScopeClass,
) -> BlobChunkSecurityScope {
    let tenant_scope = match scope_class {
        BlobHarnessSecurityScopeClass::ScopePreserving => StoreTenantScope::TenantPhysicalBoundary,
        BlobHarnessSecurityScopeClass::CrossScopeDenied => {
            StoreTenantScope::MultiTenantPhysicalBoundary
        }
    };
    let authority = current_authority(case, "blob-harness-scope");
    let expectation = StoreSecurityScopeAdmissionExpectation::new(
        StoreKeyScope::BlobChunkEnvelope,
        tenant_scope,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedBlobChunk,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let admitted = admit_store_security_scope(StoreSecurityScopeAdmissionRequest::new(
        &authority,
        StoreKeyScope::BlobChunkEnvelope,
        StoreKeyVersionPosture::Current,
        tenant_scope,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedBlobChunk,
        ),
        StoreCustodyPosture::InternalStoreCustody,
        expectation,
    ))
    .success("security scope");
    BlobChunkSecurityScope::from_admitted_security_scope(admitted).expect("blob scope")
}

pub(super) fn integrity_proof_for_scope(
    case: &str,
    scope_class: BlobHarnessSecurityScopeClass,
    bytes: &[u8],
) -> crate::BlobChunkIntegrityProof {
    let scope = blob_scope(case, scope_class);
    let rule = BlobChunkingRuleAdmission::fixed_size(
        BlobChunkSize::from_bytes(bytes.len() as u64).expect("chunk bytes"),
    )
    .expect("rule");
    BlobChunkSequenceAdmission::start(scope, rule, bytes.len() as u64)
        .expect("start")
        .push_payload(0, physical_payload_for_bytes(bytes))
        .expect("payload")
        .finish()
        .expect("finish")
        .first_chunk()
        .clone()
}
