use worth_proof::TransitionOutcome;
use worth_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StorePhysicalBoundaryWitness,
};
use worth_store_security::{
    StoreAuthenticityRequirement, StoreAuthorityBoundSecurityScopeReceipt, StoreCustodyPosture,
    StoreKeyScope, StoreKeyVersionPosture, StoreSecurityScopeAdmissionExpectation,
    StoreSecurityScopeAdmissionRequest, StoreTenantScope,
};

use super::{contract, validated_value};

pub(in crate::physical_runtime) fn physical_witness() -> StorePhysicalBoundaryWitness {
    let authority =
        worth_store_contracts::StorePhysicalAuthorityWitness::for_aspect_native_boundary(
            worth_store_contracts::ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
        )
        .expect("record work is inside the aspect-native roadmap gate");
    StorePhysicalBoundaryWitness::from_physical_authority(authority)
        .expect("record work uses aspect-native physical authority")
}

pub(super) fn read_security_admission(
    witness: StorePhysicalBoundaryWitness,
) -> (
    StoreAuthorityBoundSecurityScopeReceipt,
    worth_store_io_scheduler::IoSchedulerSecurityScopeAdmission,
) {
    let (contract, identity, _) = contract("store.physical.record.root-read-basis", 1_301, witness);
    let value = validated_value(&contract, "record-root-read-admitted");
    let state = match worth_foundational::aspects()
        .authoritative_state()
        .admit([value])
    {
        TransitionOutcome::Success(state) => state,
        outcome => panic!("built-in record aspect state must admit: {outcome:?}"),
    };
    let authority_fact = StoreAspectBoundaryFact::from_admitted_state(
        identity,
        StoreAspectAuthorityInput::new(state, witness),
    )
    .expect("built-in record state contains exactly its declared identity");
    admit_scheduler_scope(&authority_fact)
}

pub(in crate::physical_runtime) fn admit_scheduler_scope(
    authority_fact: &StoreAspectBoundaryFact,
) -> (
    StoreAuthorityBoundSecurityScopeReceipt,
    worth_store_io_scheduler::IoSchedulerSecurityScopeAdmission,
) {
    let current = worth_store_authority::require_current_store_authority(authority_fact.clone());
    let authenticity = StoreAuthenticityRequirement::not_required();
    let expectation = StoreSecurityScopeAdmissionExpectation::new(
        StoreKeyScope::StoreManagedRoot,
        StoreTenantScope::StoreInternal,
        authenticity,
        StoreCustodyPosture::InternalStoreCustody,
    );
    let request = StoreSecurityScopeAdmissionRequest::new(
        &current,
        StoreKeyScope::StoreManagedRoot,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::StoreInternal,
        authenticity,
        StoreCustodyPosture::InternalStoreCustody,
        expectation,
    );
    match worth_store_security::admit_store_security_scope(request) {
        TransitionOutcome::Success(scope) => {
            let scheduler = worth_store_io_scheduler::admit_security_scope_for_scheduler(&scope)
                .expect("built-in record scope is the scheduler's Store-internal scope");
            (scope.authority_bound_receipt(), scheduler)
        }
        outcome => panic!("built-in record security scope must admit: {outcome:?}"),
    }
}
