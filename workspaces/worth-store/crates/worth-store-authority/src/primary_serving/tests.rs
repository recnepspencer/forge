use std::collections::HashMap;
use std::sync::Mutex;

use worth_foundational::{
    aspects, AspectContract, AspectKey, AspectValue, InternedString, ScalarAspectType,
};
use worth_proof::TransitionOutcome;
use worth_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StorePhysicalBoundaryWitness,
};
use worth_store_contracts::{
    StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
    ROADMAP_2_PRIMARY_PHYSICAL_BOUNDARY,
};

use crate::{
    require_current_store_authority, ControlStoreFencingAuthority, ControlStoreFencingPort,
    ControlStoreFencingProviderDenial, ControlStoreGeneration, ControlStoreSelectionCoordinates,
    ExternalFenceGrant, ExternalServeLeaseGrant, OperationalFencingAuthorityPort,
    OperationalFencingProviderDenial, PrimaryServeAdmissionDenial, PrimaryServeLeaseRequest,
    PrimaryServeOperation, PrimaryServingAuthority, PromotionFenceDenial,
    PromotionFenceOperationIdentity, PromotionFenceRecoveryRequest, PromotionFenceRequest,
    StoreCurrentAuthorityIdentity,
};

#[derive(Debug)]
struct TestProvider {
    provider_identity: [u8; 32],
    fences: Mutex<HashMap<PromotionFenceOperationIdentity, ExternalFenceGrant>>,
    substitute_recovery_provider: Mutex<bool>,
}

impl TestProvider {
    fn new(provider_identity: [u8; 32]) -> Self {
        Self {
            provider_identity,
            fences: Mutex::new(HashMap::new()),
            substitute_recovery_provider: Mutex::new(false),
        }
    }
}

impl ControlStoreFencingPort for TestProvider {
    fn selected_control_store(
        &self,
        _current_authority: StoreCurrentAuthorityIdentity,
    ) -> Result<ControlStoreSelectionCoordinates, ControlStoreFencingProviderDenial> {
        Ok(ControlStoreSelectionCoordinates::new(
            [3; 32],
            ControlStoreGeneration::initial(),
            [4; 32],
        ))
    }
}

impl OperationalFencingAuthorityPort for TestProvider {
    fn acquire_primary_serve_lease(
        &self,
        request: PrimaryServeLeaseRequest,
    ) -> Result<ExternalServeLeaseGrant, OperationalFencingProviderDenial> {
        Ok(ExternalServeLeaseGrant::from_provider(
            [5; 32],
            request.minimum_epoch_exclusive() + 1,
            request.requested_until_tick(),
            self.provider_identity,
        ))
    }

    fn renew_primary_serve_lease(
        &self,
        _current_token: [u8; 32],
        request: PrimaryServeLeaseRequest,
    ) -> Result<ExternalServeLeaseGrant, OperationalFencingProviderDenial> {
        self.acquire_primary_serve_lease(request)
    }

    fn revoke_and_advance_epoch(
        &self,
        old_lease_token: [u8; 32],
        minimum_epoch_exclusive: u64,
        operation_identity: PromotionFenceOperationIdentity,
    ) -> Result<ExternalFenceGrant, OperationalFencingProviderDenial> {
        let mut fences = self.fences.lock().unwrap();
        Ok(*fences.entry(operation_identity).or_insert_with(|| {
            ExternalFenceGrant::from_provider(
                old_lease_token,
                minimum_epoch_exclusive + 1,
                self.provider_identity,
                [7; 32],
                operation_identity,
            )
        }))
    }

    fn recover_fence(
        &self,
        operation_identity: PromotionFenceOperationIdentity,
    ) -> Result<Option<ExternalFenceGrant>, OperationalFencingProviderDenial> {
        let grant = self
            .fences
            .lock()
            .unwrap()
            .get(&operation_identity)
            .copied();
        if *self.substitute_recovery_provider.lock().unwrap() {
            return Ok(grant.map(|grant| {
                ExternalFenceGrant::from_provider(
                    grant.old_lease_token,
                    grant.new_epoch,
                    [9; 32],
                    grant.fence_identity,
                    grant.operation_identity,
                )
            }));
        }
        Ok(grant)
    }
}

#[test]
fn external_fence_is_idempotent_and_recovers_with_exact_lease_binding() {
    let current = current_authority();
    let provider = TestProvider::new([6; 32]);
    let selected = ControlStoreFencingAuthority::for_current_store(&current, &provider)
        .select_generation()
        .unwrap();
    let serving =
        PrimaryServingAuthority::for_selected_control_generation(&current, selected, &provider)
            .unwrap();
    let lease = serving.acquire(4, 10, 100).unwrap();
    let operation = PromotionFenceOperationIdentity::admit([8; 32]).unwrap();
    let request = PromotionFenceRequest::for_old_primary(lease, 5, operation);

    let first = serving.fence_old_primary(request).unwrap();
    let repeated = serving.fence_old_primary(request).unwrap();
    let recovered = serving
        .recover_promotion_fence(PromotionFenceRecoveryRequest::new(operation, lease, 5))
        .unwrap();

    assert_eq!(first, repeated);
    assert_eq!(first, recovered);
}

#[test]
fn recovered_fence_rejects_provider_substitution() {
    let current = current_authority();
    let provider = TestProvider::new([6; 32]);
    let selected = ControlStoreFencingAuthority::for_current_store(&current, &provider)
        .select_generation()
        .unwrap();
    let serving =
        PrimaryServingAuthority::for_selected_control_generation(&current, selected, &provider)
            .unwrap();
    let lease = serving.acquire(4, 10, 100).unwrap();
    let operation = PromotionFenceOperationIdentity::admit([8; 32]).unwrap();
    serving
        .fence_old_primary(PromotionFenceRequest::for_old_primary(lease, 5, operation))
        .unwrap();
    *provider.substitute_recovery_provider.lock().unwrap() = true;

    assert_eq!(
        serving.recover_promotion_fence(PromotionFenceRecoveryRequest::new(operation, lease, 5)),
        Err(PromotionFenceDenial::ProviderIdentityChanged)
    );
}

#[test]
fn every_serving_operation_fails_closed_at_the_lease_expiry_boundary() {
    let current = current_authority();
    let provider = TestProvider::new([6; 32]);
    let selected = ControlStoreFencingAuthority::for_current_store(&current, &provider)
        .select_generation()
        .unwrap();
    let serving =
        PrimaryServingAuthority::for_selected_control_generation(&current, selected, &provider)
            .unwrap();
    let lease = serving.acquire(4, 10, 100).unwrap();

    for operation in [
        PrimaryServeOperation::ObserveAsCurrent,
        PrimaryServeOperation::Mutate,
        PrimaryServeOperation::Acknowledge,
    ] {
        let admitted = serving.admit(lease, operation, 99).unwrap();
        assert_eq!(admitted.operation(), operation);
        assert_eq!(admitted.admitted_at_tick(), 99);
        assert_eq!(
            serving.admit(lease, operation, 100),
            Err(PrimaryServeAdmissionDenial::LeaseExpired)
        );
        assert_eq!(
            serving.admit(lease, operation, 101),
            Err(PrimaryServeAdmissionDenial::LeaseExpired)
        );
    }
}

fn current_authority() -> crate::StoreCurrentAuthorityWitness {
    let key = aspect_key("worth.store.primary");
    let contract = scalar_string_contract(key.clone());
    let admitted_state = match aspects()
        .authoritative_state()
        .admit([validated_scalar_value(&contract, "current")])
    {
        TransitionOutcome::Success(state) => state,
        outcome => panic!("state admission should succeed: {outcome:?}"),
    };
    let physical = StorePhysicalAuthorityWitness::for_aspect_native_boundary_instance(
        ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
        ROADMAP_2_PRIMARY_PHYSICAL_BOUNDARY,
    )
    .unwrap();
    let boundary = StoreAspectBoundaryFact::from_admitted_state(
        StoreAspectIdentity::from_aspect_key(key),
        StoreAspectAuthorityInput::new(
            admitted_state,
            StorePhysicalBoundaryWitness::from_physical_authority(physical).unwrap(),
        ),
    )
    .unwrap();
    require_current_store_authority(boundary)
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
    value: &str,
) -> worth_foundational::ContractValidatedAspectArtifact {
    match aspects()
        .validate()
        .against(contract)
        .value(AspectValue::String(InternedString::from(value)))
    {
        TransitionOutcome::Success(value) => value,
        outcome => panic!("validation should succeed: {outcome:?}"),
    }
}
