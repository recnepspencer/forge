use forge_proof::TransitionOutcome;
use forge_store_security::{
    admit_store_security_scope, readmit_trust_boundary_security_scope_declaration,
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreKeyVersionPosture, StoreSecurityScopeAdmissionDenial,
    StoreSecurityScopeAdmissionExpectation, StoreSecurityScopeAdmissionRequest, StoreTenantScope,
    StoreTrustBoundaryCrossing, StoreTrustBoundaryReadmissionTrigger,
};

use crate::BlobChunkSecurityMetadataWitness;

use super::authority::BlobImportReadmissionAuthority;
use super::counters::BlobImportReadmissionCounters;
use super::declaration::BlobImportDeclaration;
use super::denial::BlobImportReadmissionDenial;

pub(super) fn readmit_security_scope(
    authority: &BlobImportReadmissionAuthority,
    declaration: &BlobImportDeclaration,
    trigger: &StoreTrustBoundaryReadmissionTrigger,
    counters: BlobImportReadmissionCounters,
) -> Result<BlobChunkSecurityMetadataWitness, BlobImportReadmissionDenial> {
    let expectation = StoreSecurityScopeAdmissionExpectation::new(
        StoreKeyScope::BlobChunkEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedBlobChunk,
        ),
        StoreCustodyPosture::Readmitted,
    );
    let declaration = readmit_trust_boundary_security_scope_declaration(
        authority.current_authority(),
        declaration.chunk_scope(),
        StoreKeyVersionPosture::Current,
        expectation,
        trigger.clone(),
    )
    .map_err(|denial| map_security_denial(trigger, denial, counters))?;
    let admitted =
        match admit_store_security_scope(StoreSecurityScopeAdmissionRequest::from_raw_declaration(
            authority.current_authority(),
            declaration,
            expectation,
        )) {
            TransitionOutcome::Success(admitted) => admitted,
            TransitionOutcome::Denied(denial) => {
                return Err(map_security_denial(trigger, denial, counters));
            }
            TransitionOutcome::Stale(_) | TransitionOutcome::RebindRequired(_) => {
                return Err(stale_key_denial(counters));
            }
            TransitionOutcome::Deferred(_) | TransitionOutcome::Failed(_) => {
                return Err(wrong_tenant_denial(counters));
            }
        };
    BlobChunkSecurityMetadataWitness::from_admitted_security_scope(admitted)
        .map_err(|_| wrong_tenant_denial(counters))
}

fn map_security_denial(
    trigger: &StoreTrustBoundaryReadmissionTrigger,
    denial: StoreSecurityScopeAdmissionDenial,
    counters: BlobImportReadmissionCounters,
) -> BlobImportReadmissionDenial {
    match trigger.crossing() {
        StoreTrustBoundaryCrossing::KeyScopeGenerationChanged
        | StoreTrustBoundaryCrossing::BackupRestoreAfterKeyRotation => {
            return stale_key_denial(counters);
        }
        StoreTrustBoundaryCrossing::TenantScopeAuthorityChanged => {
            return wrong_tenant_denial(counters);
        }
        StoreTrustBoundaryCrossing::CustodyDomainChanged => {
            return custody_denial(counters);
        }
        _ => {}
    }
    match denial {
        StoreSecurityScopeAdmissionDenial::DeniedKeyVersionPosture => stale_key_denial(counters),
        StoreSecurityScopeAdmissionDenial::WrongTenantScope => wrong_tenant_denial(counters),
        StoreSecurityScopeAdmissionDenial::WrongCustodyPosture => custody_denial(counters),
        _ => wrong_tenant_denial(counters),
    }
}

fn stale_key_denial(counters: BlobImportReadmissionCounters) -> BlobImportReadmissionDenial {
    BlobImportReadmissionDenial::StaleKeyGeneration {
        counters: counters.record_stale_scope_denial(),
    }
}

fn wrong_tenant_denial(counters: BlobImportReadmissionCounters) -> BlobImportReadmissionDenial {
    BlobImportReadmissionDenial::WrongTenantAuthority {
        counters: counters.record_stale_scope_denial(),
    }
}

fn custody_denial(counters: BlobImportReadmissionCounters) -> BlobImportReadmissionDenial {
    BlobImportReadmissionDenial::CustodyDomainMismatch {
        counters: counters.record_stale_scope_denial(),
    }
}
