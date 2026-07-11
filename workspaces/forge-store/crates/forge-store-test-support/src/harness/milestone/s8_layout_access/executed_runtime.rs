use forge_foundational::{AspectContract, AspectValue, InternedString, ScalarAspectType, aspects};
use forge_proof::TransitionOutcome;
use forge_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StorePhysicalBoundaryWitness,
};
use forge_store_authority::require_current_store_authority;
use forge_store_budgets::S8PreExecutionBudgetEnvelope;
use forge_store_contracts::{
    AcceptedHandoffReadiness, DurableArtifactFamilyId, HandoffEvidenceDigestSet,
    ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE, ROADMAP_2_S1_SCOPE, StableDigest,
    StorePhysicalAuthorityWitness,
};
use forge_store_layout_indexes::{
    access_execution::S8ExecutedAccessReceipt,
    access_lowering::S8AccessLoweringOutcome,
    access_lowering::S8ExecutionReadyAccessReceipt,
    access_lowering::access_lowering,
    access_planning::{access_planning, deterministic_plan_selection},
    layout_families::layout_declarations,
};
use forge_store_physical_format::{
    PhysicalEpoch, PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId,
    PhysicalRecordSlot, PhysicalSegmentId, PlatformPhysicalAppendRequest, PlatformPhysicalFacade,
    PlatformPhysicalOpenRequest, SlotGenerationCell,
    layout_access::baseline_btree_counter_observation::BaselineBTreeExecutionWitness,
};
use forge_store_security::{
    StoreAdmittedSecurityScope, StoreAuthenticityRequirement, StoreAuthenticityRequirementClass,
    StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture,
    StoreSecurityScopeAdmissionExpectation, StoreSecurityScopeAdmissionRequest, StoreTenantScope,
    admit_store_security_scope,
};

pub(crate) fn execute_s8_layout_runtime_receipt() -> S8ExecutedAccessReceipt {
    let ready = ready_exact_point_plan(31);
    let witness = BaselineBTreeExecutionWitness::admit_published_layout(
        seed_baseline_btree_root_reference(),
        seed_baseline_btree_layout(),
    )
    .expect("seeded published layout should admit as a lower-family execution witness")
    .execute_separator_directed_lookup(ready.selected().budget_receipt().plan_binding(), slot(11));
    let observed = access_lowering()
        .admit_executed_counters(&ready, &witness)
        .expect("real lower-family executed witness should admit");

    match access_lowering().execute_ready(ready, observed) {
        S8AccessLoweringOutcome::Executed(executed) => executed,
        other => panic!("expected executed outcome, got {other:?}"),
    }
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
    let lowered = match access_lowering().lower_selected(selected) {
        S8AccessLoweringOutcome::Lowered(lowered) => lowered,
        other => panic!("expected lowered outcome, got {other:?}"),
    };
    match access_lowering().admit_ready(lowered) {
        S8AccessLoweringOutcome::Ready(ready) => ready,
        other => panic!("expected ready outcome, got {other:?}"),
    }
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

fn seed_baseline_btree_layout() -> forge_store_physical_format::PersistedPhysicalLayout {
    let mut facade =
        PlatformPhysicalFacade::open_s1(readiness(), PlatformPhysicalOpenRequest::s1_canonical())
            .expect("open S.1 physical facade");
    let _left = facade
        .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
            left_slot_cell(),
            &encode_leaf_record([slot(10), slot(11)], false, false),
        ))
        .expect("append left leaf");
    let _right = facade
        .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
            right_slot_cell(),
            &encode_leaf_record([slot(12), slot(13)], false, false),
        ))
        .expect("append right leaf");
    let _root = facade
        .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
            root_slot_cell(),
            &encode_root_record(slot(12), left_slot_cell(), right_slot_cell()),
        ))
        .expect("append root");
    facade
        .publish_physical_root()
        .expect("publish seeded root")
        .persisted_layout()
        .clone()
}

fn seed_baseline_btree_root_reference() -> forge_store_physical_format::PhysicalReference {
    forge_store_physical_format::PhysicalReferenceAuthority::s1()
        .admit_page_slot(root_slot_cell())
        .reference()
}

fn readiness() -> AcceptedHandoffReadiness {
    AcceptedHandoffReadiness::from_s0_artifacts(ROADMAP_2_S1_SCOPE, digest_set())
        .expect("S.1 handoff readiness")
}

fn digest_set() -> HandoffEvidenceDigestSet {
    HandoffEvidenceDigestSet::new(
        digest("backend"),
        digest("deferred"),
        digest("harness"),
        digest("terms"),
        digest("audit"),
        digest("complexity"),
        digest("provenance"),
    )
}

fn digest(name: &str) -> StableDigest {
    StableDigest::new(format!("sha256:{name}")).expect("non-empty digest")
}

fn root_slot_cell() -> SlotGenerationCell {
    PhysicalGenerationAuthority::s1()
        .slot_cell(segment(7), page(9), slot(1))
        .with_slot_generation(generation(17))
}

fn left_slot_cell() -> SlotGenerationCell {
    PhysicalGenerationAuthority::s1()
        .slot_cell(segment(7), page(11), slot(1))
        .with_slot_generation(generation(11))
}

fn right_slot_cell() -> SlotGenerationCell {
    PhysicalGenerationAuthority::s1()
        .slot_cell(segment(7), page(13), slot(1))
        .with_slot_generation(generation(13))
}

fn encode_leaf_record(
    slots: [PhysicalRecordSlot; 2],
    sibling_links_present: bool,
    tombstones_present: bool,
) -> [u8; 6] {
    let [first_low, first_high] = slots[0].get().to_le_bytes();
    let [second_low, second_high] = slots[1].get().to_le_bytes();
    [
        b'L',
        sibling_links_present as u8 | ((tombstones_present as u8) << 1),
        first_low,
        first_high,
        second_low,
        second_high,
    ]
}

fn encode_root_record(
    separator_slot: PhysicalRecordSlot,
    left_child: SlotGenerationCell,
    right_child: SlotGenerationCell,
) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(56);
    bytes.push(b'R');
    bytes.push(0);
    bytes.extend_from_slice(&separator_slot.get().to_le_bytes());
    encode_slot_cell(&mut bytes, left_child);
    encode_slot_cell(&mut bytes, right_child);
    bytes
}

fn encode_slot_cell(bytes: &mut Vec<u8>, cell: SlotGenerationCell) {
    bytes.extend_from_slice(&cell.segment_id().get().to_le_bytes());
    bytes.extend_from_slice(&cell.page_id().get().to_le_bytes());
    bytes.extend_from_slice(&cell.slot().get().to_le_bytes());
    bytes.extend_from_slice(&cell.generation().get().to_le_bytes());
}

fn segment(value: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(value).unwrap()
}

fn page(value: u64) -> PhysicalPageId {
    PhysicalPageId::from_raw(value).unwrap()
}

fn slot(value: u16) -> PhysicalRecordSlot {
    PhysicalRecordSlot::from_raw(value).unwrap()
}

fn generation(value: u64) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(value).unwrap()
}
