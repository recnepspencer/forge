use worth_proof::TransitionOutcome;
use worth_store_authority::{require_current_store_authority, StoreCurrentAuthorityWitness};
use worth_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalReferenceScope,
    PhysicalSegmentId,
};
use worth_store_physical_integrity::{
    AuthorityDamageBoundary, ExecutedQuarantineFinding, PhysicalQuarantineAuthority,
    QuarantineHandoffPosture, QuarantineRecord, QuarantineSealRequest,
};
use worth_store_security::{
    admit_store_security_scope, StoreAdmittedSecurityScope, StoreAuthenticityRequirement,
    StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture,
    StoreSecurityScopeAdmissionExpectation, StoreSecurityScopeAdmissionRequest, StoreTenantScope,
};

use crate::NativeStoreAspectFixture;

#[derive(Debug, PartialEq, Eq)]
pub struct LayoutIntegrityAuthorityFixture {
    current_authority: StoreCurrentAuthorityWitness,
    security_scope: StoreAdmittedSecurityScope,
}

impl LayoutIntegrityAuthorityFixture {
    pub const fn current_authority(&self) -> &StoreCurrentAuthorityWitness {
        &self.current_authority
    }

    pub const fn security_scope(&self) -> &StoreAdmittedSecurityScope {
        &self.security_scope
    }
}

pub fn layout_integrity_authority(seed: &str) -> LayoutIntegrityAuthorityFixture {
    let aspect = NativeStoreAspectFixture::scalar_string(seed);
    let current_authority = require_current_store_authority(aspect.boundary_fact().clone());
    let key_scope = StoreKeyScope::StoreManagedRoot;
    let tenant_scope = StoreTenantScope::StoreInternal;
    let authenticity = StoreAuthenticityRequirement::not_required();
    let custody = StoreCustodyPosture::InternalStoreCustody;
    let expectation =
        StoreSecurityScopeAdmissionExpectation::new(key_scope, tenant_scope, authenticity, custody);
    let security_scope = match admit_store_security_scope(StoreSecurityScopeAdmissionRequest::new(
        &current_authority,
        key_scope,
        StoreKeyVersionPosture::Current,
        tenant_scope,
        authenticity,
        custody,
        expectation,
    )) {
        TransitionOutcome::Success(scope) => scope,
        outcome => panic!("layout integrity security scope must admit: {outcome:?}"),
    };
    LayoutIntegrityAuthorityFixture {
        current_authority,
        security_scope,
    }
}

pub fn authoritative_layout_quarantine_record(seed: &str) -> QuarantineRecord {
    seal(ExecutedQuarantineFinding::authoritative_quarantine(scope(
        seed,
    )))
}

pub fn audit_retained_layout_quarantine_record(seed: &str) -> QuarantineRecord {
    PhysicalQuarantineAuthority::seal(
        QuarantineSealRequest::from_executed_finding(
            ExecutedQuarantineFinding::authoritative_quarantine(scope(seed)),
        )
        .with_handoff_posture(QuarantineHandoffPosture::AuditRetentionOwnerRequired),
    )
    .expect("audit-retained physical finding must seal through quarantine authority")
}

pub fn unresolved_layout_authority_record(seed: &str) -> QuarantineRecord {
    seal(ExecutedQuarantineFinding::unresolved_authority(
        scope(seed),
        AuthorityDamageBoundary::BackendResidue,
    ))
}

fn seal(finding: ExecutedQuarantineFinding) -> QuarantineRecord {
    PhysicalQuarantineAuthority::seal(QuarantineSealRequest::from_executed_finding(finding))
        .expect("executed physical finding must seal through quarantine authority")
}

fn scope(seed: &str) -> PhysicalReferenceScope {
    let basis = seed_basis(seed);
    let segment = PhysicalSegmentId::from_raw(basis + 1).expect("fixture segment is nonzero");
    let page = PhysicalPageId::from_raw(basis + 11).expect("fixture page is nonzero");
    let generation =
        PhysicalGeneration::from_raw(basis + 5).expect("fixture generation is nonzero");
    PhysicalReferenceScope::derived_index(
        PhysicalGenerationAuthority::for_canonical_physical_format()
            .page_cell(segment, page)
            .with_page_generation(generation),
    )
}

fn seed_basis(seed: &str) -> u64 {
    seed.bytes().enumerate().fold(17_u64, |acc, (index, byte)| {
        acc + ((index as u64 + 1) * byte as u64)
    })
}
