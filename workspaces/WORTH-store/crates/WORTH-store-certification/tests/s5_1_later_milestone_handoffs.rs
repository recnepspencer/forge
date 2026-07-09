use worth_foundational::{
    aspects, AspectContract, AspectKey, AspectValue, InternedString, ScalarAspectType,
};
use worth_proof::{TransitionOutcome, TransitionOutcome::*};
use worth_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StorePhysicalBoundaryWitness,
};
use worth_store_authority::{require_current_store_authority, StoreCurrentAuthorityWitness};
use worth_store_blob_chunks::{BlobChunkSecurityScope, S7BlobChunkSecurityHandoff};
use worth_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};
use worth_store_io_scheduler::{
    admit_s5_1_security_scope_for_s6_io_qos, S6IoQosSecurityScopeHandoff,
};
use worth_store_operations::{
    RepairBlastRadiusDeclaration, RepairBlastRadiusPlan, RepairBlastRadiusReadiness,
    S10RepairBlastRadiusHandoff,
};
use worth_store_readiness::{
    accept_s5_1_admitted_security_scope_readiness, S51AdmittedSecurityScopeReadiness,
    S51LaterMilestoneHandoffDenial, S51SecurityFoundationHandoff, S51SecurityFoundationNonClaim,
    S51SecurityScopeReadinessReservation,
};
use worth_store_security::{
    admit_store_security_scope, StoreAuthenticityRequirement, StoreCustodyPosture, StoreKeyScope,
    StoreKeyVersionPosture, StoreRepairPhysicalRegionDeclaration,
    StoreSecurityScopeAdmissionExpectation, StoreSecurityScopeAdmissionRequest, StoreTenantScope,
};

#[test]
fn phase_9_publishes_separate_downstream_handoffs_from_real_readiness() {
    let authority = current_authority("store.s51.phase9.handoffs");

    let s6 = S6IoQosSecurityScopeHandoff::from_s5_1_readiness(admitted_readiness(
        &authority,
        S51SecurityScopeReadinessReservation::io_qos(),
        StoreKeyScope::StoreManagedRoot,
        StoreTenantScope::StoreInternal,
        StoreAuthenticityRequirement::not_required(),
        StoreCustodyPosture::InternalStoreCustody,
    ))
    .expect("S.6 security-scope handoff should publish");
    let s6_admission = admit_s5_1_security_scope_for_s6_io_qos(s6);
    assert_eq!(
        s6_admission.permission().identity().key_scope(),
        StoreKeyScope::StoreManagedRoot
    );

    let s7 = S7BlobChunkSecurityHandoff::from_s5_1_readiness(admitted_readiness(
        &authority,
        S51SecurityScopeReadinessReservation::blob_chunk(),
        StoreKeyScope::BlobChunkEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            worth_store_security::StoreAuthenticityRequirementClass::AuthenticatedBlobChunk,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    ))
    .expect("S.7 blob handoff should publish");
    assert_eq!(
        s7.permission().identity().key_scope(),
        StoreKeyScope::BlobChunkEnvelope
    );
    let _blob_scope = BlobChunkSecurityScope::from_s7_handoff(s7);

    let repair = repair_readiness(&authority);
    let s10_repair = S10RepairBlastRadiusHandoff::from_repair_blast_radius_readiness(repair);
    assert_eq!(
        s10_repair.permission().identity().key_scope(),
        StoreKeyScope::RepairScopeEnvelope
    );
    let _repair = RepairBlastRadiusReadiness::from_s10_handoff(s10_repair);

    let s11 = S51SecurityFoundationHandoff::from_s5_1_readiness(admitted_readiness(
        &authority,
        S51SecurityScopeReadinessReservation::security_foundation(),
        StoreKeyScope::SecurityLifecycleFoundation,
        StoreTenantScope::SecurityLifecycleFoundation,
        StoreAuthenticityRequirement::not_required(),
        StoreCustodyPosture::InternalStoreCustody,
    ))
    .expect("S.11 lifecycle foundation handoff should publish");
    assert_eq!(
        s11.lifecycle_foundation_permission()
            .identity()
            .tenant_scope(),
        StoreTenantScope::SecurityLifecycleFoundation
    );
    assert_eq!(
        s11.non_claims(),
        [
            S51SecurityFoundationNonClaim::Encryption,
            S51SecurityFoundationNonClaim::KeyRotation,
            S51SecurityFoundationNonClaim::Audit,
            S51SecurityFoundationNonClaim::OperatorAuthorization,
        ]
    );
}

#[test]
fn wrong_family_cannot_publish_s11_security_foundation_handoff() {
    let authority = current_authority("store.s51.phase9.wrong-family");
    let denial = S51SecurityFoundationHandoff::from_s5_1_readiness(admitted_readiness(
        &authority,
        S51SecurityScopeReadinessReservation::blob_chunk(),
        StoreKeyScope::SecurityLifecycleFoundation,
        StoreTenantScope::SecurityLifecycleFoundation,
        StoreAuthenticityRequirement::not_required(),
        StoreCustodyPosture::InternalStoreCustody,
    ))
    .expect_err("wrong family must not publish S.11 handoff");

    assert!(matches!(
        denial,
        S51LaterMilestoneHandoffDenial::WrongReadinessFamily { .. }
    ));
}

#[test]
fn correct_family_scope_replay_cannot_publish_s6_or_s11_with_changed_requirements() {
    let authority = current_authority("store.s51.phase9.changed-scope-replay");

    assert_later_handoff_denial(
        S6IoQosSecurityScopeHandoff::from_s5_1_readiness(admitted_readiness(
            &authority,
            S51SecurityScopeReadinessReservation::io_qos(),
            StoreKeyScope::TenantEnvelope,
            StoreTenantScope::StoreInternal,
            StoreAuthenticityRequirement::not_required(),
            StoreCustodyPosture::InternalStoreCustody,
        )),
        |denial| matches!(denial, S51LaterMilestoneHandoffDenial::WrongKeyScope { .. }),
    );
    assert_later_handoff_denial(
        S6IoQosSecurityScopeHandoff::from_s5_1_readiness(admitted_readiness(
            &authority,
            S51SecurityScopeReadinessReservation::io_qos(),
            StoreKeyScope::StoreManagedRoot,
            StoreTenantScope::TenantPhysicalBoundary,
            StoreAuthenticityRequirement::not_required(),
            StoreCustodyPosture::InternalStoreCustody,
        )),
        |denial| {
            matches!(
                denial,
                S51LaterMilestoneHandoffDenial::WrongTenantScope { .. }
            )
        },
    );
    assert_later_handoff_denial(
        S6IoQosSecurityScopeHandoff::from_s5_1_readiness(admitted_readiness(
            &authority,
            S51SecurityScopeReadinessReservation::io_qos(),
            StoreKeyScope::StoreManagedRoot,
            StoreTenantScope::StoreInternal,
            StoreAuthenticityRequirement::required(
                worth_store_security::StoreAuthenticityRequirementClass::AuthenticatedFrame,
            ),
            StoreCustodyPosture::InternalStoreCustody,
        )),
        |denial| {
            matches!(
                denial,
                S51LaterMilestoneHandoffDenial::WrongAuthenticityRequirement { .. }
            )
        },
    );

    assert_later_handoff_denial(
        S51SecurityFoundationHandoff::from_s5_1_readiness(admitted_readiness(
            &authority,
            S51SecurityScopeReadinessReservation::security_foundation(),
            StoreKeyScope::StoreManagedRoot,
            StoreTenantScope::SecurityLifecycleFoundation,
            StoreAuthenticityRequirement::not_required(),
            StoreCustodyPosture::InternalStoreCustody,
        )),
        |denial| matches!(denial, S51LaterMilestoneHandoffDenial::WrongKeyScope { .. }),
    );
    assert_later_handoff_denial(
        S51SecurityFoundationHandoff::from_s5_1_readiness(admitted_readiness(
            &authority,
            S51SecurityScopeReadinessReservation::security_foundation(),
            StoreKeyScope::SecurityLifecycleFoundation,
            StoreTenantScope::StoreInternal,
            StoreAuthenticityRequirement::not_required(),
            StoreCustodyPosture::InternalStoreCustody,
        )),
        |denial| {
            matches!(
                denial,
                S51LaterMilestoneHandoffDenial::WrongTenantScope { .. }
            )
        },
    );
    assert_later_handoff_denial(
        S51SecurityFoundationHandoff::from_s5_1_readiness(admitted_readiness(
            &authority,
            S51SecurityScopeReadinessReservation::security_foundation(),
            StoreKeyScope::SecurityLifecycleFoundation,
            StoreTenantScope::SecurityLifecycleFoundation,
            StoreAuthenticityRequirement::required(
                worth_store_security::StoreAuthenticityRequirementClass::AuthenticatedManifest,
            ),
            StoreCustodyPosture::InternalStoreCustody,
        )),
        |denial| {
            matches!(
                denial,
                S51LaterMilestoneHandoffDenial::WrongAuthenticityRequirement { .. }
            )
        },
    );
}

fn repair_readiness(authority: &StoreCurrentAuthorityWitness) -> RepairBlastRadiusReadiness {
    let declaration = RepairBlastRadiusDeclaration::native(
        authority,
        StoreRepairPhysicalRegionDeclaration::raw("phase9-region-a"),
        StoreKeyVersionPosture::Current,
        StoreCustodyPosture::InternalStoreCustody,
    )
    .expect("repair declaration should prepare");
    RepairBlastRadiusPlan::declare(declaration)
        .admit_with_store_blast_radius()
        .expect("repair readiness should admit")
}

fn admitted_readiness(
    authority: &StoreCurrentAuthorityWitness,
    reservation: S51SecurityScopeReadinessReservation,
    key_scope: StoreKeyScope,
    tenant_scope: StoreTenantScope,
    authenticity: StoreAuthenticityRequirement,
    custody: StoreCustodyPosture,
) -> S51AdmittedSecurityScopeReadiness {
    let expectation =
        StoreSecurityScopeAdmissionExpectation::new(key_scope, tenant_scope, authenticity, custody);
    let request = StoreSecurityScopeAdmissionRequest::new(
        authority,
        key_scope,
        StoreKeyVersionPosture::Current,
        tenant_scope,
        authenticity,
        custody,
        expectation,
    );

    match admit_store_security_scope(request) {
        Success(admitted) => accept_s5_1_admitted_security_scope_readiness(reservation, admitted),
        outcome => panic!("security scope should admit: {outcome:?}"),
    }
}

fn assert_later_handoff_denial<T: std::fmt::Debug>(
    outcome: Result<T, S51LaterMilestoneHandoffDenial>,
    matches_expected: impl FnOnce(&S51LaterMilestoneHandoffDenial) -> bool,
) {
    let denial = outcome.expect_err("handoff publication should deny");
    assert!(matches_expected(&denial));
    match denial {
        S51LaterMilestoneHandoffDenial::WrongReadinessFamily { counters, .. }
        | S51LaterMilestoneHandoffDenial::WrongKeyScope { counters, .. }
        | S51LaterMilestoneHandoffDenial::WrongTenantScope { counters, .. }
        | S51LaterMilestoneHandoffDenial::WrongAuthenticityRequirement { counters, .. }
        | S51LaterMilestoneHandoffDenial::WrongCustodyPosture { counters, .. }
        | S51LaterMilestoneHandoffDenial::UnsupportedSecurityFoundationClaim { counters } => {
            assert_eq!(counters.handoff_attempts(), 1);
            assert_eq!(counters.denied_count(), 1);
        }
    }
}

fn current_authority(label: &str) -> StoreCurrentAuthorityWitness {
    require_current_store_authority(boundary_fact(label, "current"))
}

fn boundary_fact(identity_key: &str, value: &str) -> StoreAspectBoundaryFact {
    let key = aspect_key(identity_key);
    let contract = scalar_string_contract(key.clone());
    let admitted_state = match aspects()
        .authoritative_state()
        .admit([validated_scalar_value(&contract, value)])
    {
        TransitionOutcome::Success(state) => state,
        outcome => panic!("state admission should succeed: {outcome:?}"),
    };

    StoreAspectBoundaryFact::from_admitted_state(
        StoreAspectIdentity::from_aspect_key(key),
        StoreAspectAuthorityInput::new(admitted_state, physical_witness()),
    )
    .expect("Store boundary fact should admit matching identity")
}

fn aspect_key(raw: &str) -> AspectKey {
    aspects().vocabulary().key(raw).unwrap()
}

fn scalar_string_contract(aspect_key: AspectKey) -> AspectContract {
    aspects()
        .contract()
        .for_key(aspect_key)
        .identified_by(aspects().vocabulary().identity(1))
        .at_revision(aspects().vocabulary().revision(1))
        .scalar(ScalarAspectType::String)
}

fn validated_scalar_value(
    contract: &AspectContract,
    raw_value: &str,
) -> worth_foundational::ContractValidatedAspectArtifact {
    match aspects()
        .validate()
        .against(contract)
        .value(AspectValue::String(InternedString::from(raw_value)))
    {
        TransitionOutcome::Success(value) => value,
        outcome => panic!("validation should succeed: {outcome:?}"),
    }
}

fn physical_witness() -> StorePhysicalBoundaryWitness {
    StorePhysicalBoundaryWitness::from_physical_authority(
        StorePhysicalAuthorityWitness::for_aspect_native_boundary(
            ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
        )
        .unwrap(),
    )
    .unwrap()
}
