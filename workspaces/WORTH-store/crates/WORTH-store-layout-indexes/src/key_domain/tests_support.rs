use crate::{layout_declarations, ArtifactScopePartitionWitness};
use worth_foundational::{aspects, AspectContract, AspectValue, InternedString, ScalarAspectType};
use worth_proof::TransitionOutcome;
use worth_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StorePhysicalBoundaryWitness,
};
use worth_store_authority::{require_current_store_authority, StoreCurrentAuthorityWitness};
use worth_store_blob_chunks::{
    certification_test_authority::{
        bridge_s7_export_trust_boundary, execute_s7_blob_harness,
        materialize_s7_executed_lifecycle_evidence, BlobHarnessExecutedWitness,
        BlobHarnessExecutionInput,
    },
    BlobGeneration, BlobHarnessAccessMode, BlobHarnessActorMix, BlobHarnessChunkSizeClass,
    BlobHarnessChunkTopology, BlobHarnessFailurePoint, BlobHarnessPlacementClass,
    BlobHarnessSecurityScopeClass, BlobHarnessSizeClass, BlobImportDeclaration, BlobObjectId,
    S7ExecutedLifecycleEvidenceBundle,
};
use worth_store_budgets::BlobHarnessEnvelopeProfile;
use worth_store_contracts::DurableArtifactFamilyId;
use worth_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};
use worth_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalRecordSlot,
    PhysicalReferenceAdmissionWitness, PhysicalReferenceAuthority, PhysicalRootReference,
    PhysicalSegmentId,
};
use worth_store_security::{
    admit_store_security_scope, StoreAdmittedSecurityScope, StoreAuthenticityRequirement,
    StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture,
    StoreSecurityScopeAdmissionExpectation, StoreSecurityScopeAdmissionRequest, StoreTenantScope,
};

pub(crate) fn admitted_scope(
    key_scope: StoreKeyScope,
    tenant_scope: StoreTenantScope,
    authenticity_requirement: StoreAuthenticityRequirement,
    custody_posture: StoreCustodyPosture,
) -> StoreAdmittedSecurityScope {
    let current_authority = current_authority("store.s8.key_domain", "test-current");
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
        outcome => panic!("test security scope should admit through production path: {outcome:?}"),
    }
}

pub(crate) fn admit_phase_four_scope(
    family_id: DurableArtifactFamilyId,
    security_scope: &StoreAdmittedSecurityScope,
) -> ArtifactScopePartitionWitness {
    let declaration = layout_declarations().declaration(family_id).unwrap();
    let classification = layout_declarations().classify_family(declaration);
    let role = layout_declarations().declare_authority_role(classification);
    let accuracy = layout_declarations().declare_derived_accuracy_class(role);

    layout_declarations()
        .require_scope_partition(accuracy, security_scope.witnesses())
        .unwrap()
}

pub(crate) fn published_blob_identity() -> (BlobObjectId, BlobGeneration) {
    let evidence = published_blob_evidence_bundle();
    (
        evidence.export_object_id().clone(),
        evidence.export_generation(),
    )
}

pub(crate) fn alternate_blob_identity() -> (BlobObjectId, BlobGeneration) {
    let evidence = alternate_blob_evidence_bundle();
    (
        evidence.export_object_id().clone(),
        evidence.export_generation(),
    )
}

pub(crate) fn published_blob_import_declaration() -> BlobImportDeclaration {
    bridge_s7_export_trust_boundary(&published_blob_harness_witness()).into_declaration()
}

pub(crate) fn alternate_blob_import_declaration() -> BlobImportDeclaration {
    bridge_s7_export_trust_boundary(&alternate_blob_harness_witness()).into_declaration()
}

pub(crate) fn published_blob_evidence_bundle() -> S7ExecutedLifecycleEvidenceBundle {
    materialize_s7_executed_lifecycle_evidence(published_blob_harness_witness())
}

pub(crate) fn alternate_blob_evidence_bundle() -> S7ExecutedLifecycleEvidenceBundle {
    materialize_s7_executed_lifecycle_evidence(alternate_blob_harness_witness())
}

fn published_blob_harness_witness() -> BlobHarnessExecutedWitness {
    blob_identity_harness_witness(
        BlobHarnessAccessMode::ReadOnlyReplay,
        BlobHarnessActorMix::SeedReplayOnly,
    )
}

fn alternate_blob_harness_witness() -> BlobHarnessExecutedWitness {
    blob_identity_harness_witness(
        BlobHarnessAccessMode::ExportBoundary,
        BlobHarnessActorMix::ExportImport,
    )
}

fn blob_identity_harness_witness(
    access_mode: BlobHarnessAccessMode,
    actor_mix: BlobHarnessActorMix,
) -> BlobHarnessExecutedWitness {
    let topology = BlobHarnessChunkTopology::from_classes(
        BlobHarnessSizeClass::LocalDeterministic,
        BlobHarnessChunkSizeClass::Fixed64KiB,
    )
    .expect("blob topology should admit");
    let witness = execute_s7_blob_harness(BlobHarnessExecutionInput::new(
        BlobHarnessEnvelopeProfile::Local,
        BlobHarnessSizeClass::LocalDeterministic,
        BlobHarnessPlacementClass::StoreLocal,
        BlobHarnessSecurityScopeClass::ScopePreserving,
        access_mode,
        BlobHarnessFailurePoint::NoFaultSeed,
        actor_mix,
        topology,
    ));

    witness
}

pub(crate) fn page_slot_reference_admission(
    segment: u64,
    page: u64,
    slot: u16,
    generation: u64,
) -> PhysicalReferenceAdmissionWitness {
    let cell = PhysicalGenerationAuthority::s1()
        .slot_cell(segment_id(segment), page_id(page), slot_id(slot))
        .with_slot_generation(physical_generation(generation));
    PhysicalReferenceAuthority::s1().admit_page_slot(cell)
}

pub(crate) fn root_reference(value: u64) -> PhysicalRootReference {
    PhysicalRootReference::from_raw(value).expect("test root reference is non-zero")
}

pub(crate) fn segment_id(value: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(value).expect("test segment id is non-zero")
}

pub(crate) fn page_id(value: u64) -> PhysicalPageId {
    PhysicalPageId::from_raw(value).expect("test page id is non-zero")
}

fn slot_id(value: u16) -> PhysicalRecordSlot {
    PhysicalRecordSlot::from_raw(value).expect("test slot is non-zero")
}

fn physical_generation(value: u64) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(value).expect("test generation is non-zero")
}

pub(crate) fn current_authority(identity_key: &str, value: &str) -> StoreCurrentAuthorityWitness {
    require_current_store_authority(boundary_fact(identity_key, value))
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
        .expect("test physical authority scope should be valid"),
    )
    .expect("test physical boundary witness should admit")
}
