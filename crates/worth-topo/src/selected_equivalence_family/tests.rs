use crate::derived_invalidation_compiled_product_admission::{
    admit_topology_compiled_product_input, TopologyCompiledProductAdmissionRequest,
};
use crate::selected_equivalence_family::{
    current_topology_selected_equivalence_family_catalog, select_topology_equivalence_family,
    TopologyOrderingNoisePosture, TopologySelectedEquivalenceDimension,
};
use crate::test_support::primitive_corpus::validated_topology::{
    build_test_runtime, committed_primitive_input,
};
use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;

#[test]
fn same_admitted_input_selects_same_equivalence_family() {
    let mut runtime = build_test_runtime().expect("topology selected equivalence runtime");
    let committed = committed_primitive_input(
        &mut runtime,
        "phase-8-topology-selected-equivalence",
        &MilestoneOnePrimitiveCase::SheetPatch { face_count: 3 },
    )
    .expect("committed primitive input");
    let read_basis = committed.read_basis().clone();
    let admitted = admit_topology_compiled_product_input(
        &crate::compiled_product_family::current_topology_compiled_product_family_catalog(),
        TopologyCompiledProductAdmissionRequest::for_historical_read_basis(
            crate::compiled_product_family::TopologyCompiledProductConsumer::DerivedEquivalenceContractProjection,
            &read_basis,
        ),
    )
    .expect("topology admission");

    let catalog = current_topology_selected_equivalence_family_catalog();
    let left = select_topology_equivalence_family(&catalog, &admitted).expect("left family");
    let right = select_topology_equivalence_family(&catalog, &admitted).expect("right family");
    let declaration = catalog
        .declaration(left.family_identity())
        .expect("catalog should expose the selected family declaration");

    assert_eq!(left.family_identity(), right.family_identity());
    assert_eq!(
        left.equivalence_basis_identity().identity_digest(),
        right.equivalence_basis_identity().identity_digest()
    );
    assert_eq!(
        left.reuse_basis_identity().identity_digest(),
        right.reuse_basis_identity().identity_digest()
    );
    assert_eq!(declaration.identity(), left.family_identity());
    assert_eq!(
        catalog.family_for_compiled_product(
            admitted.family_admitted_input().family_identity()
        ),
        Some(declaration)
    );
    assert!(!catalog.catalog_digest().is_empty());
}

#[test]
fn topology_family_requires_exact_ordering() {
    let mut runtime = build_test_runtime().expect("topology selected equivalence runtime");
    let committed = committed_primitive_input(
        &mut runtime,
        "phase-8-topology-ordering-posture",
        &MilestoneOnePrimitiveCase::SheetPatch { face_count: 3 },
    )
    .expect("committed primitive input");
    let admitted = admit_topology_compiled_product_input(
        &crate::compiled_product_family::current_topology_compiled_product_family_catalog(),
        TopologyCompiledProductAdmissionRequest::for_historical_read_basis(
            crate::compiled_product_family::TopologyCompiledProductConsumer::DerivedEquivalenceContractProjection,
            committed.read_basis(),
        ),
    )
    .expect("topology admission");

    let selected = select_topology_equivalence_family(
        &current_topology_selected_equivalence_family_catalog(),
        &admitted,
    )
    .expect("selected family");

    assert_eq!(
        selected.ordering_noise_posture(),
        TopologyOrderingNoisePosture::ExactOrderingRequired
    );
}

#[test]
fn selected_family_carries_declared_comparator_contract() {
    let mut runtime = build_test_runtime().expect("topology selected equivalence runtime");
    let committed = committed_primitive_input(
        &mut runtime,
        "phase-8-topology-comparator-contract",
        &MilestoneOnePrimitiveCase::SheetPatch { face_count: 3 },
    )
    .expect("committed primitive input");
    let admitted = admit_topology_compiled_product_input(
        &crate::compiled_product_family::current_topology_compiled_product_family_catalog(),
        TopologyCompiledProductAdmissionRequest::for_historical_read_basis(
            crate::compiled_product_family::TopologyCompiledProductConsumer::DerivedEquivalenceContractProjection,
            committed.read_basis(),
        ),
    )
    .expect("topology admission");
    let selected = select_topology_equivalence_family(
        &current_topology_selected_equivalence_family_catalog(),
        &admitted,
    )
    .expect("selected family");

    let comparator = selected.comparator_contract();

    assert_eq!(comparator.family_identity(), selected.family_identity());
    assert_eq!(
        comparator.equivalence_policy_identity_digest(),
        selected.equivalence_policy_identity().identity_digest()
    );
    assert_eq!(
        comparator.equivalence_dimensions(),
        &[
            TopologySelectedEquivalenceDimension::SelectedEquivalenceBasisIdentity,
            TopologySelectedEquivalenceDimension::SelectedReuseBasisIdentity,
            TopologySelectedEquivalenceDimension::DerivedValidationDigest,
            TopologySelectedEquivalenceDimension::MaterializedTopologyDigest,
            TopologySelectedEquivalenceDimension::InterpretedTopologyDigest,
        ]
    );
    assert_eq!(
        comparator.compatibility_posture(),
        selected.compatibility_posture()
    );
    assert_eq!(
        comparator.freshness_requirement_posture(),
        selected.freshness_requirement_posture()
    );
    assert_eq!(comparator.ordering_noise_posture(), selected.ordering_noise_posture());
    assert_eq!(
        comparator.rendered_output_comparison_posture(),
        selected.rendered_output_comparison_posture()
    );
    assert_eq!(
        comparator
            .clone()
            .with_test_equivalence_dimensions(vec![
                TopologySelectedEquivalenceDimension::SelectedEquivalenceBasisIdentity,
            ])
            .equivalence_dimensions(),
        &[TopologySelectedEquivalenceDimension::SelectedEquivalenceBasisIdentity]
    );
}

#[test]
fn selected_equivalence_errors_expose_kind_and_detail() {
    let error = super::error::TopologySelectedEquivalenceFamilyError::new(
        super::error::TopologySelectedEquivalenceFamilyErrorKind::MissingDeclaredFamily,
        "fixture detail",
    );

    assert_eq!(
        error.kind(),
        super::error::TopologySelectedEquivalenceFamilyErrorKind::MissingDeclaredFamily
    );
    assert_eq!(error.detail(), "fixture detail");
}
