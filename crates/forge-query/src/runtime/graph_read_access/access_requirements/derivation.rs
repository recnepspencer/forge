use super::{
    authorities::{ordering_field_authorities, predicate_field_authorities, relation_authority},
    operator_mapping::traversal_operators_for_relation,
    ForgeQueryGraphReadAccessComplexityContract, ForgeQueryGraphReadAccessInvalidationBasis,
    ForgeQueryGraphReadAccessMemoryEstimateBasis, ForgeQueryGraphReadAccessRebuildBasis,
    ForgeQueryGraphReadAccessRequirementDerivationError, ForgeQueryGraphReadAccessRequirementKind,
    ForgeQueryGraphReadAccessRequirementRow, ForgeQueryGraphReadAccessRequirementSet,
};
use crate::runtime::{
    ForgeQueryAdmittedGraphReadRelation, ForgeQueryBooleanSelectivityShape,
    ForgeQueryGraphReadAccessShape, ForgeQueryGraphReadLifecycleClass,
    ForgeQueryGraphReadOrderingPosture, ForgeQueryGraphReadPredicateFamily,
    ForgeQueryGraphReadRelationshipProofBindingPosture, ForgeQueryGraphReadResultPressure,
    ForgeQueryGraphReadTraversalOperator,
};

pub(crate) fn derive_graph_read_access_requirement_set(
    access_shape: &ForgeQueryGraphReadAccessShape,
    selectivity_shape: &ForgeQueryBooleanSelectivityShape,
) -> ForgeQueryGraphReadAccessRequirementSet {
    match try_derive_graph_read_access_requirement_set(access_shape, selectivity_shape) {
        Ok(requirement_set) => requirement_set,
        Err(error) => panic!(
            "trusted graph read access requirement derivation received mismatched proof artifacts: {}",
            error.as_str()
        ),
    }
}

pub(crate) fn try_derive_graph_read_access_requirement_set(
    access_shape: &ForgeQueryGraphReadAccessShape,
    selectivity_shape: &ForgeQueryBooleanSelectivityShape,
) -> Result<
    ForgeQueryGraphReadAccessRequirementSet,
    ForgeQueryGraphReadAccessRequirementDerivationError,
> {
    validate_requirement_derivation_inputs(access_shape, selectivity_shape)?;
    Ok(derive_compatible_graph_read_access_requirement_set(
        access_shape,
        selectivity_shape,
    ))
}

fn derive_compatible_graph_read_access_requirement_set(
    access_shape: &ForgeQueryGraphReadAccessShape,
    selectivity_shape: &ForgeQueryBooleanSelectivityShape,
) -> ForgeQueryGraphReadAccessRequirementSet {
    let mut rows = Vec::new();
    rows.extend(traversal_requirement_rows(access_shape));
    rows.extend(predicate_requirement_rows(access_shape, selectivity_shape));
    rows.extend(ordering_requirement_rows(access_shape));
    rows.extend(proof_requirement_rows(access_shape));
    rows.extend(lifecycle_requirement_rows(access_shape));
    ForgeQueryGraphReadAccessRequirementSet::new(
        access_shape.operation_resolution().read_graph_digest(),
        access_shape.digest().as_str(),
        selectivity_shape.digest().as_str(),
        rows,
    )
}

fn validate_requirement_derivation_inputs(
    access_shape: &ForgeQueryGraphReadAccessShape,
    selectivity_shape: &ForgeQueryBooleanSelectivityShape,
) -> Result<(), ForgeQueryGraphReadAccessRequirementDerivationError> {
    let access_shape_read_graph_digest = access_shape.operation_resolution().read_graph_digest();
    if access_shape_read_graph_digest != selectivity_shape.read_graph_digest() {
        return Err(
            ForgeQueryGraphReadAccessRequirementDerivationError::ReadGraphDigestMismatch {
                access_shape_read_graph_digest: access_shape_read_graph_digest.to_string(),
                selectivity_shape_read_graph_digest: selectivity_shape
                    .read_graph_digest()
                    .to_string(),
            },
        );
    }
    if access_shape.digest().as_str() != selectivity_shape.access_shape_digest() {
        return Err(
            ForgeQueryGraphReadAccessRequirementDerivationError::AccessShapeDigestMismatch {
                access_shape_digest: access_shape.digest().as_str().to_string(),
                selectivity_shape_access_shape_digest: selectivity_shape
                    .access_shape_digest()
                    .to_string(),
            },
        );
    }
    Ok(())
}

fn traversal_requirement_rows(
    access_shape: &ForgeQueryGraphReadAccessShape,
) -> Vec<ForgeQueryGraphReadAccessRequirementRow> {
    let relations = access_shape.operation_resolution().references().relations();
    let mut rows = Vec::new();
    for relation in relations {
        for operator in traversal_operators_for_relation(access_shape, relation) {
            rows.extend(operator_requirement_rows(
                operator,
                relation,
                access_shape.result_pressure().clone(),
                access_shape.fanout_posture().clone(),
                access_shape
                    .operation_resolution()
                    .references()
                    .schema_basis_digest(),
            ));
        }
    }
    if relations.is_empty() {
        rows.push(result_buffer_row(access_shape.result_pressure().clone()));
    }
    rows
}

fn operator_requirement_rows(
    operator: ForgeQueryGraphReadTraversalOperator,
    relation: &ForgeQueryAdmittedGraphReadRelation,
    result_pressure: ForgeQueryGraphReadResultPressure,
    fanout_posture: crate::runtime::ForgeQueryGraphReadFanoutPosture,
    schema_basis_digest: &str,
) -> Vec<ForgeQueryGraphReadAccessRequirementRow> {
    let mut rows = Vec::new();
    rows.extend(adjacency_rows_for_operator(
        &operator,
        relation,
        fanout_posture.clone(),
        schema_basis_digest,
    ));
    if requires_traversal_workset(&operator) {
        rows.push(
            relation_row(
                ForgeQueryGraphReadAccessRequirementKind::TraversalWorkset,
                ForgeQueryGraphReadAccessRebuildBasis::OperationResolutionProof,
                ForgeQueryGraphReadAccessInvalidationBasis::ReadGraphProofDelta,
                ForgeQueryGraphReadAccessComplexityContract::BoundedTraversalWorkset,
                ForgeQueryGraphReadAccessMemoryEstimateBasis::FrontierDepthBound,
                relation,
                fanout_posture.clone(),
                schema_basis_digest,
            )
            .with_traversal_operator(operator.clone()),
        );
        rows.push(
            relation_row(
                ForgeQueryGraphReadAccessRequirementKind::VisitedSet,
                ForgeQueryGraphReadAccessRebuildBasis::OperationResolutionProof,
                ForgeQueryGraphReadAccessInvalidationBasis::ReadGraphProofDelta,
                ForgeQueryGraphReadAccessComplexityContract::BoundedTraversalWorkset,
                ForgeQueryGraphReadAccessMemoryEstimateBasis::FrontierDepthBound,
                relation,
                fanout_posture.clone(),
                schema_basis_digest,
            )
            .with_traversal_operator(operator.clone()),
        );
    }
    if requires_dedup_set(&operator) {
        rows.push(
            relation_row(
                ForgeQueryGraphReadAccessRequirementKind::DedupSet,
                ForgeQueryGraphReadAccessRebuildBasis::OperationResolutionProof,
                ForgeQueryGraphReadAccessInvalidationBasis::ReadGraphProofDelta,
                ForgeQueryGraphReadAccessComplexityContract::BoundedTraversalWorkset,
                ForgeQueryGraphReadAccessMemoryEstimateBasis::FrontierDepthBound,
                relation,
                fanout_posture,
                schema_basis_digest,
            )
            .with_traversal_operator(operator),
        );
    }
    rows.push(result_buffer_row(result_pressure));
    rows
}

fn adjacency_rows_for_operator(
    operator: &ForgeQueryGraphReadTraversalOperator,
    relation: &ForgeQueryAdmittedGraphReadRelation,
    fanout_posture: crate::runtime::ForgeQueryGraphReadFanoutPosture,
    schema_basis_digest: &str,
) -> Vec<ForgeQueryGraphReadAccessRequirementRow> {
    match operator {
        ForgeQueryGraphReadTraversalOperator::BoundedAncestor
        | ForgeQueryGraphReadTraversalOperator::SharedEndpoint
        | ForgeQueryGraphReadTraversalOperator::SharedAttachment => {
            vec![relation_row(
                ForgeQueryGraphReadAccessRequirementKind::ReverseAdjacency,
                ForgeQueryGraphReadAccessRebuildBasis::AuthoritativeRelationTruth,
                ForgeQueryGraphReadAccessInvalidationBasis::AuthoritativeRelationDelta,
                ForgeQueryGraphReadAccessComplexityContract::ReverseRelationLookup,
                ForgeQueryGraphReadAccessMemoryEstimateBasis::RelationDegreeBound,
                relation,
                fanout_posture,
                schema_basis_digest,
            )
            .with_traversal_operator(operator.clone())]
        }
        ForgeQueryGraphReadTraversalOperator::FrontierSearch => vec![
            relation_row(
                ForgeQueryGraphReadAccessRequirementKind::DirectionalAdjacency,
                ForgeQueryGraphReadAccessRebuildBasis::AuthoritativeRelationTruth,
                ForgeQueryGraphReadAccessInvalidationBasis::AuthoritativeRelationDelta,
                ForgeQueryGraphReadAccessComplexityContract::DirectionalRelationLookup,
                ForgeQueryGraphReadAccessMemoryEstimateBasis::RelationDegreeBound,
                relation,
                fanout_posture.clone(),
                schema_basis_digest,
            )
            .with_traversal_operator(operator.clone()),
            relation_row(
                ForgeQueryGraphReadAccessRequirementKind::ReverseAdjacency,
                ForgeQueryGraphReadAccessRebuildBasis::AuthoritativeRelationTruth,
                ForgeQueryGraphReadAccessInvalidationBasis::AuthoritativeRelationDelta,
                ForgeQueryGraphReadAccessComplexityContract::ReverseRelationLookup,
                ForgeQueryGraphReadAccessMemoryEstimateBasis::RelationDegreeBound,
                relation,
                fanout_posture,
                schema_basis_digest,
            )
            .with_traversal_operator(operator.clone()),
        ],
        ForgeQueryGraphReadTraversalOperator::DirectEdge
        | ForgeQueryGraphReadTraversalOperator::SuccessorWalk
        | ForgeQueryGraphReadTraversalOperator::BoundedDescendant
        | ForgeQueryGraphReadTraversalOperator::AnchoredFrontier
        | ForgeQueryGraphReadTraversalOperator::DeclarationTraversal => vec![relation_row(
            ForgeQueryGraphReadAccessRequirementKind::DirectionalAdjacency,
            ForgeQueryGraphReadAccessRebuildBasis::AuthoritativeRelationTruth,
            ForgeQueryGraphReadAccessInvalidationBasis::AuthoritativeRelationDelta,
            ForgeQueryGraphReadAccessComplexityContract::DirectionalRelationLookup,
            ForgeQueryGraphReadAccessMemoryEstimateBasis::RelationDegreeBound,
            relation,
            fanout_posture,
            schema_basis_digest,
        )
        .with_traversal_operator(operator.clone())],
    }
}

fn predicate_requirement_rows(
    access_shape: &ForgeQueryGraphReadAccessShape,
    selectivity_shape: &ForgeQueryBooleanSelectivityShape,
) -> Vec<ForgeQueryGraphReadAccessRequirementRow> {
    if access_shape.predicate_family() == &ForgeQueryGraphReadPredicateFamily::None
        && selectivity_shape.predicate_rows().is_empty()
    {
        return Vec::new();
    }
    vec![ForgeQueryGraphReadAccessRequirementRow::new(
        ForgeQueryGraphReadAccessRequirementKind::PredicateSupport,
        ForgeQueryGraphReadAccessRebuildBasis::SelectivityProof,
        ForgeQueryGraphReadAccessInvalidationBasis::AuthoritativeFieldDelta,
        ForgeQueryGraphReadAccessComplexityContract::CandidatePredicateSupport,
        ForgeQueryGraphReadAccessMemoryEstimateBasis::PredicateCandidateSet,
    )
    .with_predicate_family(access_shape.predicate_family().clone())
    .with_predicate_field_authorities(predicate_field_authorities(access_shape, selectivity_shape))]
}

fn ordering_requirement_rows(
    access_shape: &ForgeQueryGraphReadAccessShape,
) -> Vec<ForgeQueryGraphReadAccessRequirementRow> {
    if access_shape.ordering_posture() == &ForgeQueryGraphReadOrderingPosture::Unordered {
        return Vec::new();
    }
    vec![ForgeQueryGraphReadAccessRequirementRow::new(
        ForgeQueryGraphReadAccessRequirementKind::OrderingSupport,
        ForgeQueryGraphReadAccessRebuildBasis::AuthoritativeFieldTruth,
        ForgeQueryGraphReadAccessInvalidationBasis::AuthoritativeFieldDelta,
        ForgeQueryGraphReadAccessComplexityContract::CandidateOrderingSupport,
        ForgeQueryGraphReadAccessMemoryEstimateBasis::OrderedCandidateSet,
    )
    .with_ordering_posture(access_shape.ordering_posture().clone())
    .with_ordering_field_authorities(ordering_field_authorities(access_shape))]
}

fn proof_requirement_rows(
    access_shape: &ForgeQueryGraphReadAccessShape,
) -> Vec<ForgeQueryGraphReadAccessRequirementRow> {
    if access_shape.relationship_proof_posture()
        == &ForgeQueryGraphReadRelationshipProofBindingPosture::NotRequired
    {
        return Vec::new();
    }
    vec![ForgeQueryGraphReadAccessRequirementRow::new(
        ForgeQueryGraphReadAccessRequirementKind::ProofSupport,
        ForgeQueryGraphReadAccessRebuildBasis::ReadGraphProof,
        ForgeQueryGraphReadAccessInvalidationBasis::ReadGraphProofDelta,
        ForgeQueryGraphReadAccessComplexityContract::ProofEvidenceSupport,
        ForgeQueryGraphReadAccessMemoryEstimateBasis::ProofEvidenceSet,
    )]
}

fn lifecycle_requirement_rows(
    access_shape: &ForgeQueryGraphReadAccessShape,
) -> Vec<ForgeQueryGraphReadAccessRequirementRow> {
    match access_shape.lifecycle_class() {
        ForgeQueryGraphReadLifecycleClass::ReusableReadFamily => {
            vec![ForgeQueryGraphReadAccessRequirementRow::new(
                ForgeQueryGraphReadAccessRequirementKind::MaterializationLifecycle,
                ForgeQueryGraphReadAccessRebuildBasis::RuntimeSupportRequired,
                ForgeQueryGraphReadAccessInvalidationBasis::RuntimeLifecycleDelta,
                ForgeQueryGraphReadAccessComplexityContract::LifecycleSupportAdmission,
                ForgeQueryGraphReadAccessMemoryEstimateBasis::LifecycleManagedSupport,
            )
            .with_lifecycle_class(access_shape.lifecycle_class().clone())]
        }
    }
}

fn relation_row(
    kind: ForgeQueryGraphReadAccessRequirementKind,
    rebuild_basis: ForgeQueryGraphReadAccessRebuildBasis,
    invalidation_basis: ForgeQueryGraphReadAccessInvalidationBasis,
    complexity_contract: ForgeQueryGraphReadAccessComplexityContract,
    memory_estimate_basis: ForgeQueryGraphReadAccessMemoryEstimateBasis,
    relation: &ForgeQueryAdmittedGraphReadRelation,
    fanout_posture: crate::runtime::ForgeQueryGraphReadFanoutPosture,
    schema_basis_digest: &str,
) -> ForgeQueryGraphReadAccessRequirementRow {
    ForgeQueryGraphReadAccessRequirementRow::new(
        kind,
        rebuild_basis,
        invalidation_basis,
        complexity_contract,
        memory_estimate_basis,
    )
    .with_relation(
        relation.relation(),
        relation_authority(schema_basis_digest, relation.relation()),
        relation.direction().clone(),
        relation.depth(),
    )
    .with_fanout_posture(fanout_posture)
}

fn result_buffer_row(
    result_pressure: ForgeQueryGraphReadResultPressure,
) -> ForgeQueryGraphReadAccessRequirementRow {
    ForgeQueryGraphReadAccessRequirementRow::new(
        ForgeQueryGraphReadAccessRequirementKind::ResultBuffer,
        ForgeQueryGraphReadAccessRebuildBasis::OperationResolutionProof,
        ForgeQueryGraphReadAccessInvalidationBasis::ReadGraphProofDelta,
        ForgeQueryGraphReadAccessComplexityContract::ResultPressureBuffer,
        ForgeQueryGraphReadAccessMemoryEstimateBasis::ResultPressureBound,
    )
    .with_result_pressure(result_pressure)
}

fn requires_traversal_workset(operator: &ForgeQueryGraphReadTraversalOperator) -> bool {
    !matches!(operator, ForgeQueryGraphReadTraversalOperator::DirectEdge)
}

fn requires_dedup_set(operator: &ForgeQueryGraphReadTraversalOperator) -> bool {
    matches!(
        operator,
        ForgeQueryGraphReadTraversalOperator::AnchoredFrontier
            | ForgeQueryGraphReadTraversalOperator::SharedEndpoint
            | ForgeQueryGraphReadTraversalOperator::SharedAttachment
            | ForgeQueryGraphReadTraversalOperator::FrontierSearch
    )
}
