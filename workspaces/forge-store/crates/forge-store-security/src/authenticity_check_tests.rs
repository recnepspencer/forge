use forge_proof::TransitionOutcome;
use forge_store_physical_format::{
    PhysicalFrameKind, PhysicalGeneration, PhysicalGenerationAuthority, PhysicalHeaderKind,
    PhysicalPageId, PhysicalSegmentId,
};

use crate::security_scope_test_support::{
    current_authority, platform_authenticity_requirement, platform_request,
};
use crate::{
    admit_store_authenticity_witness_observation, admit_store_security_scope,
    StoreAdmittedSecurityScope, StoreAuthenticityCheck, StoreAuthenticityCheckDenialKind,
    StoreAuthenticityPhysicalIdentity, StoreAuthenticityRequirement,
    StoreAuthenticityRequirementClass, StoreAuthenticityResultKind, StoreAuthenticityWitnessInput,
    StoreAuthenticityWitnessObservationDeclaration, StoreCustodyPosture, StoreKeyScope,
    StoreKeyVersionPosture, StoreSecurityScopeAdmissionExpectation,
    StoreSecurityScopeAdmissionRequest, StoreTenantScope,
};

#[test]
fn admitted_check_is_the_only_path_that_produces_authenticity_result() {
    let admitted = admitted_scope("store.s51.authenticity.result", "page-0001");
    let authenticity_scope = admitted.witnesses().authenticity_scope();

    let result = StoreAuthenticityCheck::for_requirement(authenticity_scope.requirement())
        .with_security_scope(authenticity_scope)
        .with_physical_identity(physical_identity(1))
        .with_witness(admit_store_authenticity_witness_observation(
            authenticity_scope,
            physical_identity(1),
            StoreAuthenticityWitnessObservationDeclaration::verified(),
        ))
        .admit()
        .unwrap();

    assert_eq!(result.kind(), StoreAuthenticityResultKind::Verified);
    assert_eq!(result.requirement(), platform_authenticity_requirement());
    assert_eq!(result.scope_identity(), authenticity_scope.identity());
    assert_eq!(result.physical_identity(), physical_identity(1));
    assert_eq!(result.counters().requirement_checks(), 1);
    assert_eq!(result.counters().witness_observations(), 1);
    assert_eq!(result.counters().verified_results(), 1);
}

#[test]
fn required_authenticity_distinguishes_absent_stale_wrong_scope_and_failed() {
    let admitted = admitted_scope("store.s51.authenticity.denials", "page-0002");
    let wrong_scope = admitted_scope_with_requirement(
        "store.s51.authenticity.other",
        "page-0003",
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedManifest,
        ),
    );
    let authenticity_scope = admitted.witnesses().authenticity_scope();
    let wrong_authenticity_scope = wrong_scope.witnesses().authenticity_scope();

    assert_denial(
        authenticity_scope,
        StoreAuthenticityWitnessInput::absent(),
        StoreAuthenticityCheckDenialKind::MissingWitness,
        physical_identity(2),
    );
    assert_denial(
        authenticity_scope,
        admitted_witness(
            authenticity_scope,
            physical_identity(2),
            StoreAuthenticityWitnessObservationDeclaration::stale(),
        ),
        StoreAuthenticityCheckDenialKind::StaleWitness,
        physical_identity(2),
    );
    assert_denial(
        authenticity_scope,
        admitted_witness(
            wrong_authenticity_scope,
            physical_identity(2),
            StoreAuthenticityWitnessObservationDeclaration::verified(),
        ),
        StoreAuthenticityCheckDenialKind::WrongScope,
        physical_identity(2),
    );
    assert_denial(
        authenticity_scope,
        admitted_witness(
            authenticity_scope,
            physical_identity(2),
            StoreAuthenticityWitnessObservationDeclaration::failed(),
        ),
        StoreAuthenticityCheckDenialKind::Failed,
        physical_identity(2),
    );
    assert_denial(
        authenticity_scope,
        admitted_witness(
            authenticity_scope,
            physical_identity(3),
            StoreAuthenticityWitnessObservationDeclaration::verified(),
        ),
        StoreAuthenticityCheckDenialKind::WrongPhysicalIdentity,
        physical_identity(2),
    );
}

#[test]
fn required_authenticity_distinguishes_unavailable_and_unsupported() {
    let admitted = admitted_scope("store.s51.authenticity.posture", "page-0004");
    let authenticity_scope = admitted.witnesses().authenticity_scope();

    let unavailable = denial_for(
        authenticity_scope,
        StoreAuthenticityWitnessInput::unavailable(),
        physical_identity(4),
    );
    assert_eq!(
        unavailable.kind(),
        StoreAuthenticityCheckDenialKind::Unavailable
    );
    assert_eq!(unavailable.counters().unavailable_denials(), 1);

    let unsupported = denial_for(
        authenticity_scope,
        StoreAuthenticityWitnessInput::unsupported(),
        physical_identity(4),
    );
    assert_eq!(
        unsupported.kind(),
        StoreAuthenticityCheckDenialKind::Unsupported
    );
    assert_eq!(unsupported.counters().unsupported_denials(), 1);
}

fn admitted_scope(identity_key: &str, value: &str) -> StoreAdmittedSecurityScope {
    let authority = current_authority(identity_key, value);
    match admit_store_security_scope(platform_request(
        &authority,
        StoreKeyVersionPosture::Current,
    )) {
        TransitionOutcome::Success(admitted) => admitted,
        outcome => panic!("platform security scope should admit: {outcome:?}"),
    }
}

fn admitted_scope_with_requirement(
    identity_key: &str,
    value: &str,
    requirement: StoreAuthenticityRequirement,
) -> StoreAdmittedSecurityScope {
    let authority = current_authority(identity_key, value);
    let expectation = StoreSecurityScopeAdmissionExpectation::new(
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        requirement,
        StoreCustodyPosture::InternalStoreCustody,
    );
    let request = StoreSecurityScopeAdmissionRequest::new(
        &authority,
        StoreKeyScope::PageEnvelope,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::TenantPhysicalBoundary,
        requirement,
        StoreCustodyPosture::InternalStoreCustody,
        expectation,
    );
    match admit_store_security_scope(request) {
        TransitionOutcome::Success(admitted) => admitted,
        outcome => panic!("custom platform security scope should admit: {outcome:?}"),
    }
}

fn assert_denial(
    authenticity_scope: &crate::StoreCurrentAuthenticityScopeWitness,
    witness: StoreAuthenticityWitnessInput,
    expected: StoreAuthenticityCheckDenialKind,
    physical_identity: StoreAuthenticityPhysicalIdentity,
) {
    let denial = denial_for(authenticity_scope, witness, physical_identity);
    assert_eq!(denial.kind(), expected);
    assert_eq!(denial.requirement(), authenticity_scope.requirement());
    assert_eq!(denial.scope_identity(), authenticity_scope.identity());
}

fn denial_for(
    authenticity_scope: &crate::StoreCurrentAuthenticityScopeWitness,
    witness: StoreAuthenticityWitnessInput,
    physical_identity: StoreAuthenticityPhysicalIdentity,
) -> crate::StoreAuthenticityCheckDenial {
    StoreAuthenticityCheck::for_requirement(authenticity_scope.requirement())
        .with_security_scope(authenticity_scope)
        .with_physical_identity(physical_identity)
        .with_witness(witness)
        .admit()
        .unwrap_err()
}

fn admitted_witness(
    scope: &crate::StoreCurrentAuthenticityScopeWitness,
    physical_identity: StoreAuthenticityPhysicalIdentity,
    declaration: StoreAuthenticityWitnessObservationDeclaration,
) -> StoreAuthenticityWitnessInput {
    admit_store_authenticity_witness_observation(scope, physical_identity, declaration)
}

fn physical_identity(page: u64) -> StoreAuthenticityPhysicalIdentity {
    StoreAuthenticityPhysicalIdentity::new(
        PhysicalHeaderKind::Frame(PhysicalFrameKind::RecordFrame),
        PhysicalGenerationAuthority::s1()
            .page_cell(segment(1), page_id(page))
            .with_page_generation(generation(7))
            .owner(),
        128,
        0xC0FFEE,
        "crc32c",
    )
}

fn segment(value: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(value).unwrap()
}

fn page_id(value: u64) -> PhysicalPageId {
    PhysicalPageId::from_raw(value).unwrap()
}

fn generation(value: u64) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(value).unwrap()
}
