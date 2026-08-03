use worth_foundational::facade::CanonicalDigestWorkBudget;
use worth_query_admission::facade::graph_read_access::{
    derive_canonical_graph_read_access_requirements, WorthQueryCanonicalGraphReadPlanningInput,
    WorthQueryGraphReadPlanningIdentity, WorthQueryGraphReadPlanningOrderingField,
    WorthQueryGraphReadPlanningPredicateField, WorthQueryGraphReadPlanningRelation,
    WorthQueryGraphReadPlanningShape,
};
use worth_query_installation::facade::WorthQueryCanonicalWorkEvidence;

use super::{
    operator_mapping::traversal_operators_for_relation,
    WorthQueryGraphReadAccessRequirementDerivationError, WorthQueryGraphReadAccessRequirementSet,
};
use crate::runtime::{
    WorthQueryBooleanSelectivityShape, WorthQueryGraphReadAccessShape,
    WorthQueryGraphReadRelationshipProofBindingPosture,
};

pub(crate) fn derive_graph_read_access_requirement_set(
    access_shape: &WorthQueryGraphReadAccessShape,
    selectivity_shape: &WorthQueryBooleanSelectivityShape,
) -> WorthQueryGraphReadAccessRequirementSet {
    match try_derive_graph_read_access_requirement_set(access_shape, selectivity_shape) {
        Ok(requirement_set) => requirement_set,
        Err(error) => panic!(
            "trusted graph read access requirement derivation received mismatched proof artifacts: {}",
            error.as_str()
        ),
    }
}

pub(crate) fn try_derive_graph_read_access_requirement_set(
    access_shape: &WorthQueryGraphReadAccessShape,
    selectivity_shape: &WorthQueryBooleanSelectivityShape,
) -> Result<
    WorthQueryGraphReadAccessRequirementSet,
    WorthQueryGraphReadAccessRequirementDerivationError,
> {
    validate_inputs(access_shape, selectivity_shape)?;
    Ok(derive_canonical_graph_read_access_requirements(
        &planning_input(access_shape, selectivity_shape),
        canonical_budget(),
        WorthQueryCanonicalWorkEvidence::zero(),
    )
    .expect("installed graph-read proof identities fit the canonical admission budget"))
}

fn validate_inputs(
    access_shape: &WorthQueryGraphReadAccessShape,
    selectivity_shape: &WorthQueryBooleanSelectivityShape,
) -> Result<(), WorthQueryGraphReadAccessRequirementDerivationError> {
    let access_graph = access_shape.operation_resolution().read_graph_digest();
    if access_graph != selectivity_shape.read_graph_digest() {
        return Err(
            WorthQueryGraphReadAccessRequirementDerivationError::ReadGraphDigestMismatch {
                access_shape_read_graph_digest: access_graph.to_string(),
                selectivity_shape_read_graph_digest: selectivity_shape
                    .read_graph_digest()
                    .to_string(),
            },
        );
    }
    if access_shape.digest().as_str() != selectivity_shape.access_shape_digest() {
        return Err(
            WorthQueryGraphReadAccessRequirementDerivationError::AccessShapeDigestMismatch {
                access_shape_digest: access_shape.digest().as_str().to_string(),
                selectivity_shape_access_shape_digest: selectivity_shape
                    .access_shape_digest()
                    .to_string(),
            },
        );
    }
    Ok(())
}

fn planning_input(
    access_shape: &WorthQueryGraphReadAccessShape,
    selectivity_shape: &WorthQueryBooleanSelectivityShape,
) -> WorthQueryCanonicalGraphReadPlanningInput {
    let references = access_shape.operation_resolution().references();
    let identity = WorthQueryGraphReadPlanningIdentity::from_admitted_evidence(
        *access_shape
            .operation_resolution()
            .read_graph_canonical_digest(),
        *access_shape.digest().canonical_digest(),
        *selectivity_shape.digest().canonical_digest(),
        *references.schema_basis_digest(),
    );
    let shape = WorthQueryGraphReadPlanningShape::from_admitted_shape(
        references
            .relations()
            .iter()
            .map(|relation| {
                WorthQueryGraphReadPlanningRelation::from_admitted_reference(
                    relation.terminal_relation_projection_for_boundary(),
                    relation.direction().clone(),
                    relation.depth(),
                    traversal_operators_for_relation(access_shape, relation),
                )
            })
            .collect(),
        access_shape.fanout_posture().clone(),
        access_shape.result_pressure().clone(),
    )
    .with_predicates(
        access_shape.predicate_family().clone(),
        selectivity_shape
            .predicate_rows()
            .iter()
            .map(|row| {
                WorthQueryGraphReadPlanningPredicateField::from_admitted_field(
                    row.native_aspect_key().clone(),
                    row.native_field_key().clone(),
                    row.field_kind().as_str(),
                )
            })
            .collect(),
    )
    .with_ordering(
        access_shape.ordering_posture().clone(),
        references
            .orderings()
            .iter()
            .map(|row| {
                WorthQueryGraphReadPlanningOrderingField::from_admitted_field(
                    "root",
                    row.native_aspect_key().clone(),
                    row.native_field_key().clone(),
                    row.direction(),
                    row.kind().as_str(),
                )
            })
            .collect(),
    )
    .with_relationship_proof_required(
        access_shape.relationship_proof_posture()
            != &WorthQueryGraphReadRelationshipProofBindingPosture::NotRequired,
    );
    WorthQueryCanonicalGraphReadPlanningInput::from_admitted_evidence(identity, shape)
}

fn canonical_budget() -> CanonicalDigestWorkBudget {
    CanonicalDigestWorkBudget::new(4_096, 1024 * 1024)
        .expect("the graph-read requirement canonical budget is nonzero")
}
