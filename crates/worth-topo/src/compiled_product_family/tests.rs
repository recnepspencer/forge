use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;

use super::{
    current_topology_compiled_product_family_catalog, select_topology_compiled_product_family,
    TopologyCompiledProductConsumer,
};
use crate::certification::support::historical_query_snapshot::historical_query_snapshot_for_read_basis;
use crate::certification::support::read_basis_query_runtime::HistoricalReadBasisQueryRuntime;
use crate::derived_invalidation_compiled_product_admission::{
    admit_topology_compiled_product_input, TopologyCompiledProductAdmissionRequest,
};
use crate::test_support::primitive_corpus::validated_topology::{
    build_test_runtime, committed_primitive_input,
};

#[test]
fn one_family_declaration_serves_projection_and_certification_consumers() {
    let catalog = current_topology_compiled_product_family_catalog();
    let read_basis = real_read_basis();

    let projection_admitted = admit_topology_compiled_product_input(
        &catalog,
        TopologyCompiledProductAdmissionRequest::for_historical_read_basis(
            TopologyCompiledProductConsumer::DerivedEquivalenceContractProjection,
            &read_basis,
        ),
    )
    .expect("projection admitted input");
    let projection = select_topology_compiled_product_family(
        &catalog,
        projection_admitted.into_family_admitted_input(),
    )
    .expect("projection selection");
    let certification_admitted = admit_topology_compiled_product_input(
        &catalog,
        TopologyCompiledProductAdmissionRequest::for_historical_read_basis(
            TopologyCompiledProductConsumer::DerivedEquivalenceCertificationParity,
            &read_basis,
        ),
    )
    .expect("certification admitted input");
    let certification = select_topology_compiled_product_family(
        &catalog,
        certification_admitted.into_family_admitted_input(),
    )
    .expect("certification selection");

    assert_eq!(catalog.counters().family_count(), 1);
    assert_eq!(catalog.counters().declared_family_count(), 1);
    assert_eq!(catalog.counters().supported_consumer_count(), 2);
    assert_eq!(
        projection.declaration().identity(),
        certification.declaration().identity()
    );
    assert_eq!(
        projection.declaration().family_digest(),
        certification.declaration().family_digest()
    );
    assert_eq!(
        projection.declaration().authority_basis(),
        certification.declaration().authority_basis()
    );
    assert_eq!(
        projection.declaration().locality_footprint(),
        certification.declaration().locality_footprint()
    );
    assert_eq!(
        projection.declaration().prior_proof(),
        certification.declaration().prior_proof()
    );
    assert_eq!(
        projection.declaration().stage_identity(),
        certification.declaration().stage_identity()
    );
    assert_eq!(
        projection.declaration().validator_evidence_role(),
        certification.declaration().validator_evidence_role()
    );
    assert_eq!(
        projection.declaration().equivalence_policy(),
        certification.declaration().equivalence_policy()
    );
    assert_eq!(
        projection.declaration().equivalence_policy_name(),
        "topology-derived-equivalence"
    );
    assert_eq!(
        projection.declaration().equivalence_dimensions(),
        &[
            "compiled-product-identity",
            "materialized-topology",
            "interpreted-topology",
            "derived-validation",
        ]
    );
}

#[test]
fn real_read_basis_admission_and_lowering_match_declared_family_semantics() {
    let mut runtime = build_test_runtime().expect("phase 3 family test runtime");
    let committed = committed_primitive_input(
        &mut runtime,
        "phase-three.compiled-product-family",
        &MilestoneOnePrimitiveCase::SheetPatch { face_count: 3 },
    )
    .expect("committed primitive input");
    let catalog = current_topology_compiled_product_family_catalog();
    let admitted = admit_topology_compiled_product_input(
        &catalog,
        TopologyCompiledProductAdmissionRequest::for_historical_read_basis(
            TopologyCompiledProductConsumer::DerivedEquivalenceContractProjection,
            committed.read_basis(),
        ),
    )
    .expect("projection admitted input");
    let selected = select_topology_compiled_product_family(
        &catalog,
        admitted.clone().into_family_admitted_input(),
    )
    .expect("projection family selection");
    let mut query_runtime = HistoricalReadBasisQueryRuntime::open(
        &runtime,
        committed.read_basis().clone(),
        "phase-three.compiled-product-family.snapshot",
    )
    .expect("historical read-basis query runtime");
    let snapshot = historical_query_snapshot_for_read_basis(&mut query_runtime)
        .expect("historical query snapshot");
    let lowered = selected
        .compile_product_identity(
            snapshot.materialized(),
            snapshot.interpreted(),
            snapshot.validation(),
        )
        .expect("compiled-product lowering");

    assert_eq!(selected.declaration().identity(), lowered.family_identity());
    assert_eq!(
        selected.declaration().family_digest(),
        lowered.family_digest()
    );
    assert_eq!(
        selected.declaration().equivalence_policy_name(),
        "topology-derived-equivalence"
    );
    assert_eq!(
        selected.declaration().equivalence_dimensions(),
        &[
            "compiled-product-identity",
            "materialized-topology",
            "interpreted-topology",
            "derived-validation",
        ]
    );
    assert_eq!(
        lowered.compiled_product_identity().identity_digest(),
        lowered
            .reuse_decision_identity()
            .compiled_product_identity_digest()
    );
    assert_eq!(
        lowered.equivalence_policy_identity().identity_digest(),
        lowered
            .reuse_decision_identity()
            .equivalence_policy_identity_digest()
    );
}

fn real_read_basis() -> schema::facade::topology_authoring::DerivedTopologyReadBasis {
    let mut runtime = build_test_runtime().expect("phase 3 family shared runtime");
    committed_primitive_input(
        &mut runtime,
        "phase-three.compiled-product-family.shared-read-basis",
        &MilestoneOnePrimitiveCase::SheetPatch { face_count: 3 },
    )
    .expect("committed primitive input")
    .read_basis()
    .clone()
}
