use schema::facade::topology_authoring::{DerivedTopologyReadBasis, MilestoneOnePrimitiveCase};

use super::{
    admit_topology_compiled_product_input, TopologyCompiledProductAdmissionErrorKind,
    TopologyCompiledProductAdmissionRequest,
};
use crate::compiled_product_family::{
    current_topology_compiled_product_family_catalog, select_topology_compiled_product_family,
    TopologyCompiledProductConsumer,
};
use crate::derived_topology::invalidation_plan::selection::selection_test_fixtures::{
    admitted_legality_support, admitted_query_support, catalog_closeout,
    loop_cycles_touched_closure, unrelated_geometry_touched_closure,
};
use crate::derived_topology::invalidation_plan::selection::{
    DerivedInvalidationDensityPolicy, DerivedInvalidationSelectedPlan,
};
use crate::test_support::primitive_corpus::validated_topology::{
    build_test_runtime, committed_primitive_input,
};
#[test]
fn topology_equivalent_inputs_admit_to_same_identity() {
    let catalog = current_topology_compiled_product_family_catalog();
    let read_basis = real_read_basis("phase-six.admission.equivalent-inputs");
    let first = admit_topology_compiled_product_input(
        &catalog,
        TopologyCompiledProductAdmissionRequest::for_historical_read_basis(
            TopologyCompiledProductConsumer::DerivedEquivalenceContractProjection,
            read_basis.as_ref(),
        ),
    )
    .expect("first admitted input");
    let second = admit_topology_compiled_product_input(
        &catalog,
        TopologyCompiledProductAdmissionRequest::for_historical_read_basis(
            TopologyCompiledProductConsumer::DerivedEquivalenceContractProjection,
            read_basis.as_ref(),
        ),
    )
    .expect("second admitted input");

    assert_eq!(
        first.family_admitted_input().truth_basis_digest_hex(),
        second.family_admitted_input().truth_basis_digest_hex()
    );
    assert_eq!(
        first.family_admitted_input().locality_digest(),
        second.family_admitted_input().locality_digest()
    );
}

#[test]
fn topology_wrong_receipt_or_foreign_authority_is_rejected() {
    let catalog = current_topology_compiled_product_family_catalog();
    let read_basis = real_read_basis("phase-six.admission.truth-basis-mismatch");
    let mismatched =
        with_truth_basis_count(read_basis.as_ref(), read_basis.touched_aspects().len() + 1);
    let error = admit_topology_compiled_product_input(
        &catalog,
        TopologyCompiledProductAdmissionRequest::for_historical_read_basis(
            TopologyCompiledProductConsumer::DerivedEquivalenceContractProjection,
            &mismatched,
        ),
    )
    .expect_err("mismatched truth basis count must fail");

    assert_eq!(
        error.kind(),
        TopologyCompiledProductAdmissionErrorKind::InvalidTruthBasisCount
    );
}

#[test]
fn topology_selected_plan_mismatch_fails_before_selection() {
    let catalog = current_topology_compiled_product_family_catalog();
    let touched_closure = loop_cycles_touched_closure("admission.selected-plan");
    let foreign_touched_closure = unrelated_geometry_touched_closure();
    let read_basis = real_read_basis("phase-six.admission.selected-plan-mismatch");
    let selected_plan = DerivedInvalidationSelectedPlan::lower(
        &catalog_closeout(),
        &touched_closure,
        &admitted_query_support(),
        &admitted_legality_support(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .expect("selected plan");

    let error = admit_topology_compiled_product_input(
        &catalog,
        TopologyCompiledProductAdmissionRequest::for_selected_plan(
            TopologyCompiledProductConsumer::DerivedEquivalenceContractProjection,
            &read_basis,
            &foreign_touched_closure,
            &selected_plan,
        ),
    )
    .expect_err("foreign touched closure must fail before selection");

    assert!(matches!(
        error.kind(),
        TopologyCompiledProductAdmissionErrorKind::TouchedClosureNotBoundToSelectedPlan
            | TopologyCompiledProductAdmissionErrorKind::ReadBasisNotBoundToTouchedClosure
    ));
}

#[test]
fn admitted_input_selects_family_without_bypass() {
    let catalog = current_topology_compiled_product_family_catalog();
    let read_basis = real_read_basis("phase-six.admission.family-selection");
    let admitted = admit_topology_compiled_product_input(
        &catalog,
        TopologyCompiledProductAdmissionRequest::for_historical_read_basis(
            TopologyCompiledProductConsumer::DerivedEquivalenceCertificationParity,
            read_basis.as_ref(),
        ),
    )
    .expect("admitted input");
    let selected = select_topology_compiled_product_family(
        &catalog,
        admitted.clone().into_family_admitted_input(),
    )
    .expect("selected family");

    assert_eq!(
        selected.admitted_input().truth_basis_digest_hex(),
        admitted.source_authority_basis().truth_basis_digest_hex()
    );
}

fn real_read_basis(label: &str) -> Box<DerivedTopologyReadBasis> {
    let mut runtime = build_test_runtime().expect("phase 6 admission runtime");
    let committed = committed_primitive_input(
        &mut runtime,
        label,
        &MilestoneOnePrimitiveCase::SheetPatch { face_count: 3 },
    )
    .expect("committed primitive input");
    Box::new(committed.read_basis().clone())
}

fn with_truth_basis_count(
    read_basis: &DerivedTopologyReadBasis,
    touched_aspect_count: usize,
) -> DerivedTopologyReadBasis {
    let mut rebound = read_basis.clone();
    rebound.authority.truth_basis_identity.touched_aspect_count = touched_aspect_count;
    rebound
}
