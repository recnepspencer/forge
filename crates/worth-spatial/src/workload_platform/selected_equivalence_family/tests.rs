use crate::spatial_compiled_product_family::SpatialCompiledProductConsumer;
use crate::workload_platform::compiled_product_admission::{
    admit_spatial_compiled_product_input, SpatialCompiledProductAdmissionRequest,
};
use crate::workload_platform::evidence_lookup_index_product::tests::fixtures::{
    selected_lookup_slice_for_plan, IndexProductSubject,
};
use crate::workload_platform::selected_equivalence_family::{
    current_spatial_selected_equivalence_family_catalog, select_spatial_equivalence_family,
    SpatialOrderingNoisePosture,
};

#[test]
fn same_admitted_input_selects_same_equivalence_family() {
    let selected_plan = IndexProductSubject::sparse_event_ledger().select_plan();
    let ledger = selected_lookup_slice_for_plan(&selected_plan);
    let admitted = admit_spatial_compiled_product_input(
        &crate::spatial_compiled_product_family::current_spatial_compiled_product_family_catalog(),
        SpatialCompiledProductAdmissionRequest::for_evidence_lookup_ledger(
            SpatialCompiledProductConsumer::EvidenceLookupIndexProduct,
            &selected_plan,
            &ledger,
        ),
    )
    .expect("spatial admission");

    let catalog = current_spatial_selected_equivalence_family_catalog();
    let left = select_spatial_equivalence_family(&catalog, &admitted).expect("left family");
    let right = select_spatial_equivalence_family(&catalog, &admitted).expect("right family");

    assert_eq!(left.family_identity(), right.family_identity());
    assert_eq!(
        left.equivalence_basis_identity().identity_digest(),
        right.equivalence_basis_identity().identity_digest()
    );
    assert_eq!(
        left.reuse_basis_identity().identity_digest(),
        right.reuse_basis_identity().identity_digest()
    );
    assert_ne!(
        left.compatibility_basis_identity().identity_digest(),
        left.reuse_basis_identity().identity_digest()
    );
}

#[test]
fn ordering_noise_is_allowed_only_when_family_declares_it() {
    let evidence_plan = IndexProductSubject::sparse_event_ledger().select_plan();
    let evidence_ledger = selected_lookup_slice_for_plan(&evidence_plan);
    let evidence_admitted = admit_spatial_compiled_product_input(
        &crate::spatial_compiled_product_family::current_spatial_compiled_product_family_catalog(),
        SpatialCompiledProductAdmissionRequest::for_evidence_lookup_ledger(
            SpatialCompiledProductConsumer::EvidenceLookupIndexProduct,
            &evidence_plan,
            &evidence_ledger,
        ),
    )
    .expect("evidence lookup admission");
    let retained = crate::spatial_compiled_product_family::retained_and_projected_receipts(
        "phase-8-retained-replay-ordering-posture",
    );
    let historical = retained
        .0
        .historical_replay(&retained.0.replay_subject())
        .expect("historical replay");
    let replay_admitted = admit_spatial_compiled_product_input(
        &crate::spatial_compiled_product_family::current_spatial_compiled_product_family_catalog(),
        SpatialCompiledProductAdmissionRequest::for_retained_replay(
            &historical,
            &retained.0,
            &retained.1,
        ),
    )
    .expect("retained replay admission");

    let catalog = current_spatial_selected_equivalence_family_catalog();
    let evidence_family =
        select_spatial_equivalence_family(&catalog, &evidence_admitted).expect("evidence family");
    let replay_family =
        select_spatial_equivalence_family(&catalog, &replay_admitted).expect("replay family");

    assert_eq!(
        evidence_family.ordering_noise_posture(),
        SpatialOrderingNoisePosture::DeclaredBenignOrderingNoiseAllowed
    );
    assert_eq!(
        replay_family.ordering_noise_posture(),
        SpatialOrderingNoisePosture::ExactOrderingRequired
    );
}
