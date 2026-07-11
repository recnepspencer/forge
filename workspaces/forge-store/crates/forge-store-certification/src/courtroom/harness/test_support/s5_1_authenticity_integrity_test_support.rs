use super::pre_decode_physical_admission_test_support::{
    crc32c, with_pre_decode_admission, CountingSemanticDecoder,
};
use forge_foundational::{
    aspects, AspectContract, AspectKey, AspectValue, InternedString, ScalarAspectType,
};
use forge_proof::TransitionOutcome;
use forge_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StorePhysicalBoundaryWitness,
};
use forge_store_authority::{require_current_store_authority, StoreCurrentAuthorityWitness};
use forge_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};
use forge_store_physical_format::{PhysicalAuthenticityIdentity, PhysicalFrameKind};
use forge_store_physical_integrity::{
    AuthenticityPolicyDecodeCounters, AuthenticityPolicyPhysicalDecodeGate,
    DeclaredPhysicalChecksum, LogicalDecodeGateIdentity, PhysicalIntegrityAdmissionRequest,
    S3LogicalDecoder,
};
use forge_store_security::{
    admit_store_authenticity_witness_observation, admit_store_security_scope,
    StoreAdmittedSecurityScope, StoreAuthenticityCheck, StoreAuthenticityCheckDenialKind,
    StoreAuthenticityRequirement, StoreAuthenticityWitnessInput,
    StoreAuthenticityWitnessObservationDeclaration, StoreCurrentAuthenticityScopeWitness,
    StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture,
    StoreSecurityScopeAdmissionExpectation, StoreSecurityScopeAdmissionRequest, StoreTenantScope,
};

pub(crate) fn admitted_scope(identity_key: &str, value: &str) -> StoreAdmittedSecurityScope {
    let authority = current_authority(identity_key, value);
    let request = StoreSecurityScopeAdmissionRequest::platform_page_envelope(
        &authority,
        StoreKeyVersionPosture::Current,
        StoreCustodyPosture::InternalStoreCustody,
    );
    match admit_store_security_scope(request) {
        TransitionOutcome::Success(admitted) => admitted,
        outcome => panic!("security scope should admit: {outcome:?}"),
    }
}

pub(crate) fn admitted_scope_with_requirement(
    identity_key: &str,
    value: &str,
    requirement: StoreAuthenticityRequirement,
) -> StoreAdmittedSecurityScope {
    let authority = current_authority(identity_key, value);
    let request = StoreSecurityScopeAdmissionRequest::new(
        &authority,
        StoreKeyScope::PageEnvelope,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::TenantPhysicalBoundary,
        requirement,
        StoreCustodyPosture::InternalStoreCustody,
        StoreSecurityScopeAdmissionExpectation::new(
            StoreKeyScope::PageEnvelope,
            StoreTenantScope::TenantPhysicalBoundary,
            requirement,
            StoreCustodyPosture::InternalStoreCustody,
        ),
    );
    match admit_store_security_scope(request) {
        TransitionOutcome::Success(admitted) => admitted,
        outcome => panic!("custom security scope should admit: {outcome:?}"),
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum PolicyWitnessPosture {
    Verified,
    Absent,
}

#[derive(Debug)]
pub(crate) struct PolicyLaneObservation {
    pub(crate) identity: LogicalDecodeGateIdentity,
    pub(crate) counters: AuthenticityPolicyDecodeCounters,
    pub(crate) authenticity_result_present: bool,
    pub(crate) authenticity_denial_kind: Option<StoreAuthenticityCheckDenialKind>,
    pub(crate) decode_invocations: u32,
}

pub(crate) fn policy_lane_observation(
    authenticity_scope: &StoreCurrentAuthenticityScopeWitness,
    posture: PolicyWitnessPosture,
) -> PolicyLaneObservation {
    let mut observation = None;
    with_pre_decode_admission(
        b"checksum-valid-auth-policy-switch",
        |admission, validation, witness| {
            let checked = admission
                .admit_frame(PhysicalIntegrityAdmissionRequest::frame(
                    validation,
                    witness,
                    PhysicalFrameKind::RecordFrame,
                    DeclaredPhysicalChecksum::new(crc32c(b"checksum-valid-auth-policy-switch")),
                ))
                .unwrap();
            let identity = checked.gate_evidence().identity().clone();
            let physical_identity = authenticity_physical_identity(&identity);
            let authenticity =
                StoreAuthenticityCheck::for_requirement(authenticity_scope.requirement())
                    .with_security_scope(authenticity_scope)
                    .with_physical_identity(physical_identity)
                    .with_witness(policy_witness(
                        authenticity_scope,
                        physical_identity,
                        posture,
                    ))
                    .admit();
            let authenticity_denial_kind = authenticity.as_ref().err().map(|denial| denial.kind());
            let gate =
                AuthenticityPolicyPhysicalDecodeGate::admit_frame(checked, authenticity).unwrap();
            let mut decoder = CountingSemanticDecoder::default();

            decoder.decode(gate.logical_decode_gate());
            observation = Some(PolicyLaneObservation {
                identity,
                counters: gate.counters(),
                authenticity_result_present: gate.authenticity_result().is_some(),
                authenticity_denial_kind,
                decode_invocations: decoder.invocations,
            });
        },
    );
    observation.expect("policy lane should produce an observation")
}

fn policy_witness(
    scope: &StoreCurrentAuthenticityScopeWitness,
    physical_identity: PhysicalAuthenticityIdentity,
    posture: PolicyWitnessPosture,
) -> StoreAuthenticityWitnessInput<PhysicalAuthenticityIdentity> {
    match posture {
        PolicyWitnessPosture::Verified => admit_store_authenticity_witness_observation(
            scope,
            physical_identity,
            StoreAuthenticityWitnessObservationDeclaration::verified(),
        ),
        PolicyWitnessPosture::Absent => StoreAuthenticityWitnessInput::absent(),
    }
}

fn authenticity_physical_identity(
    identity: &LogicalDecodeGateIdentity,
) -> PhysicalAuthenticityIdentity {
    PhysicalAuthenticityIdentity::new(
        identity.header_kind(),
        identity.locality(),
        identity.checked_byte_count(),
        identity.checksum_value(),
        identity.checksum_algorithm(),
    )
}

fn current_authority(identity_key: &str, value: &str) -> StoreCurrentAuthorityWitness {
    require_current_store_authority(boundary_fact(identity_key, value))
}

fn boundary_fact(identity_key: &str, value: &str) -> StoreAspectBoundaryFact {
    let key = aspects().vocabulary().key(identity_key).unwrap();
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
) -> forge_foundational::ContractValidatedAspectArtifact {
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
