use worth_proof::TransitionOutcome;
use worth_store_security::{
    admit_store_security_scope, StoreAuthenticityRequirement, StoreAuthenticityRequirementClass,
    StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture,
    StoreSecurityScopeAdmissionExpectation, StoreSecurityScopeAdmissionRequest, StoreTenantScope,
};

use crate::{AdmittedBlobCustody, BlobCustodyPurpose};

use super::current_authority;

pub(crate) fn admitted_blob_custody(
    case: &str,
    purpose: BlobCustodyPurpose,
) -> AdmittedBlobCustody {
    let authority = current_authority(case, purpose.label());
    let authenticity = StoreAuthenticityRequirement::required(
        StoreAuthenticityRequirementClass::AuthenticatedBackupCapsule,
    );
    let request = StoreSecurityScopeAdmissionRequest::new(
        &authority,
        StoreKeyScope::BackupExportEnvelope,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::BackupRestoreBoundary,
        authenticity,
        StoreCustodyPosture::ExportPrepared,
        StoreSecurityScopeAdmissionExpectation::new(
            StoreKeyScope::BackupExportEnvelope,
            StoreTenantScope::BackupRestoreBoundary,
            authenticity,
            StoreCustodyPosture::ExportPrepared,
        ),
    );
    let admitted = match admit_store_security_scope(request) {
        TransitionOutcome::Success(admitted) => admitted,
        outcome => panic!("blob custody security scope should admit: {outcome:?}"),
    };
    AdmittedBlobCustody::from_security_receipt(purpose, admitted.receipt())
        .expect("blob custody receipt should admit")
}
