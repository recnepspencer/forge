use forge_foundational::{aspects, AspectContract, AspectValue, InternedString, ScalarAspectType};
use forge_proof::TransitionOutcome;
use forge_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StorePhysicalBoundaryWitness,
};
use forge_store_authority::require_current_store_authority;
use forge_store_budgets::S8PreExecutionBudgetEnvelope;
use forge_store_contracts::{
    DurableArtifactFamilyId, StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
};
use forge_store_layout_indexes::{
    access_execution::S8ExecutedAccessReceipt,
    access_lowering::access_lowering,
    access_lowering::S8ExecutionReadyAccessReceipt,
    access_planning::{access_planning, deterministic_plan_selection},
    layout_families::layout_declarations,
};
use forge_store_physical_format::PhysicalEpoch;
use forge_store_security::{
    admit_store_security_scope, StoreAdmittedSecurityScope, StoreAuthenticityRequirement,
    StoreAuthenticityRequirementClass, StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture,
    StoreSecurityScopeAdmissionExpectation, StoreSecurityScopeAdmissionRequest, StoreTenantScope,
};

pub fn execute_s8_layout_runtime_receipt() -> S8ExecutedAccessReceipt {
    let ready = ready_exact_point_plan(31);
    let witness = super::deterministic_baseline_btree_witness()
        .execute_separator_directed_lookup(
            ready.selected().budget_receipt().plan_binding(),
            super::baseline_btree_probe_slot(),
        )
        .expect("deterministic B-tree lookup should execute");
    let observed = access_lowering()
        .admit_executed_counters(&ready, &witness)
        .expect("real lower-family executed witness should admit");

    access_lowering()
        .execute_ready(ready, observed)
        .expect("admitted B-tree counters should execute")
}

fn ready_exact_point_plan(epoch: u64) -> S8ExecutionReadyAccessReceipt {
    let security_scope = admitted_scope(
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let declaration = layout_declarations()
        .declaration(DurableArtifactFamilyId::PhysicalPage)
        .unwrap();
    let classification = layout_declarations().classify_family(declaration);
    let lifecycle = layout_declarations()
        .require_strategy_lifecycle(
            layout_declarations()
                .require_production_authority(classification)
                .unwrap(),
        )
        .unwrap();
    let scope = layout_declarations()
        .require_scope_partition(
            layout_declarations().declare_derived_accuracy_class(
                layout_declarations().declare_authority_role(classification),
            ),
            security_scope.witnesses(),
        )
        .unwrap();
    let key_domain = layout_declarations()
        .declare_physical_key_domain(scope)
        .unwrap();
    let coverage = access_planning()
        .bootstrap_exact_root_epoch_coverage(
            lifecycle.declaration(),
            PhysicalEpoch::from_raw(epoch).unwrap(),
        )
        .unwrap();
    let selected = deterministic_plan_selection()
        .select_with_budget(
            lifecycle,
            key_domain,
            access_planning()
                .require_exact_point_access(coverage)
                .unwrap(),
            S8PreExecutionBudgetEnvelope::foreground_default(),
        )
        .unwrap();
    let lowered = access_lowering().lower_selected(selected).into_lowered();
    access_lowering()
        .admit_ready(lowered)
        .into_ready()
        .expect("exact point plan should be ready")
}

fn admitted_scope(
    key_scope: StoreKeyScope,
    tenant_scope: StoreTenantScope,
    authenticity_requirement: StoreAuthenticityRequirement,
    custody_posture: StoreCustodyPosture,
) -> StoreAdmittedSecurityScope {
    let current_authority = require_current_store_authority(boundary_fact(
        "store.s8.execution",
        "physical-format-integration",
    ));
    let expectation = StoreSecurityScopeAdmissionExpectation::new(
        key_scope,
        tenant_scope,
        authenticity_requirement,
        custody_posture,
    );
    let request = StoreSecurityScopeAdmissionRequest::new(
        &current_authority,
        key_scope,
        StoreKeyVersionPosture::Current,
        tenant_scope,
        authenticity_requirement,
        custody_posture,
        expectation,
    );

    match admit_store_security_scope(request) {
        TransitionOutcome::Success(admitted) => admitted,
        outcome => panic!("security scope admission should succeed: {outcome:?}"),
    }
}

fn boundary_fact(identity_key: &str, value: &str) -> StoreAspectBoundaryFact {
    let key = aspects().vocabulary().key(identity_key).unwrap();
    let contract = aspects()
        .contract()
        .for_key(key.clone())
        .identified_by(aspects().vocabulary().identity(1))
        .at_revision(aspects().vocabulary().revision(1))
        .scalar(ScalarAspectType::String);
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
        .expect("test physical authority scope should be valid"),
    )
    .expect("test physical boundary witness should admit")
}

#[cfg(test)]
mod tests {
    use super::execute_s8_layout_runtime_receipt;
    use forge_store_budgets::CounterEvidenceStrength;

    #[test]
    fn deterministic_btree_execution_produces_exact_strategy_counters() {
        let executed = execute_s8_layout_runtime_receipt();
        assert_eq!(executed.amplification_receipt().page_touches(), 2);
        assert_eq!(executed.amplification_receipt().index_probes(), 2);
        assert_eq!(
            executed.performance_receipt().counter_strength(),
            CounterEvidenceStrength::Exact,
        );
        assert!(executed.planned_vs_observed().parity_holds());
    }
}
