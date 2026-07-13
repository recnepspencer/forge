use forge_store_test_support::harness::physical_isolation::epoch_scope as support;

use forge_store_physical_isolation::{
    admit_seed_stable_read_plan, PhysicalReadPlanReleaseSemantics,
    PostProtectionPhysicalReadObservation, ProtectedPhysicalReferenceSet, PublishedReaderHazard,
    ReadPlanAdmissionScratchArena, TraversalAdmissionGuard, UnprotectedReadIntent,
};
use forge_store_test_support::{NativeAspectPhysicalReferenceDenial, NativeStoreAspectFixture};
use support::{current_root_from_authority, physical_authority_from_complete_closeout};

#[test]
fn native_aspect_fixture_reference_drives_stable_read_footprint() {
    let fixture = NativeStoreAspectFixture::segment_header("phase5-native-footprint", 89);
    let authority = physical_authority_from_complete_closeout();
    let root = current_root_from_authority(&authority);
    let reference_proof = fixture
        .derive_current_generation_segment_reference()
        .expect("segment-header fixture exposes physical segment generation");
    let reference = reference_proof.reference();
    let references = ProtectedPhysicalReferenceSet::from_current_generation_refs_with_scratch(
        [reference],
        ReadPlanAdmissionScratchArena::for_protected_reference_capacity(1),
    )
    .unwrap();
    let observed_references = references.clone();
    let intent = UnprotectedReadIntent::for_known_footprint(root, references, 4096)
        .with_release_semantics(PhysicalReadPlanReleaseSemantics::reader_releases_all());
    let hazard = PublishedReaderHazard::publish(&authority, intent).unwrap();
    let observed = PostProtectionPhysicalReadObservation::from_authority_after_hazard_publication(
        &authority,
        &hazard,
        root,
        observed_references,
    )
    .unwrap();
    let validated = hazard
        .observe_authority_after_publication(&authority, observed)
        .unwrap()
        .validate()
        .unwrap();
    let receipt = TraversalAdmissionGuard::from_validated_root(validated)
        .admit(ReadPlanAdmissionScratchArena::for_protected_reference_capacity(1))
        .unwrap();
    let plan = admit_seed_stable_read_plan(receipt.into_cursor().finish()).unwrap();

    assert_eq!(
        reference_proof.physical_witness(),
        reference_proof
            .boundary_fact()
            .authority_input()
            .physical_witness()
    );
    assert_eq!(reference_proof.generation().get(), 89);
    assert_eq!(
        fixture.current_generation_segment_reference(),
        Some(reference)
    );
    assert_eq!(plan.footprint().protected().references().len(), 1);
    assert_eq!(plan.counters().protected_references(), 1);

    let expected_release_basis = plan.reachability_barrier().footprint_basis();
    let handle = plan.into_execution_ready_handle();
    assert_eq!(handle.read_protected_reference(reference), Ok(()));
    assert_eq!(handle.release().footprint_basis(), expected_release_basis);
}

#[test]
fn scalar_native_aspect_fixture_cannot_mint_physical_footprint_reference() {
    let fixture = NativeStoreAspectFixture::scalar_string("not-a-segment-header");

    assert_eq!(
        fixture.derive_current_generation_segment_reference(),
        Err(NativeAspectPhysicalReferenceDenial::ValidatedValueIsNotStruct)
    );
    assert_eq!(fixture.current_generation_segment_reference(), None);
}
