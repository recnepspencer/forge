use super::{
    authorities::{ordering_field_authorities, predicate_field_authorities, relation_authority},
    operator_mapping::traversal_operators_for_relation,
    WorthQueryGraphReadAccessComplexityContract, WorthQueryGraphReadAccessInvalidationBasis,
    WorthQueryGraphReadAccessMemoryEstimateBasis, WorthQueryGraphReadAccessRebuildBasis,
    WorthQueryGraphReadAccessRequirementDerivationError, WorthQueryGraphReadAccessRequirementKind,
    WorthQueryGraphReadAccessRequirementRow, WorthQueryGraphReadAccessRequirementSet,
};
use crate::runtime::{
    WorthQueryAdmittedGraphReadRelation, WorthQueryBooleanSelectivityShape,
    WorthQueryGraphReadAccessShape, WorthQueryGraphReadLifecycleClass,
    WorthQueryGraphReadOrderingPosture, WorthQueryGraphReadPredicateFamily,
    WorthQueryGraphReadRelationshipProofBindingPosture, WorthQueryGraphReadResultPressure,
    WorthQueryGraphReadTraversalOperator,
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
    validate_requirement_derivation_inputs(access_shape, selectivity_shape)?;
    Ok(derive_compatible_graph_read_access_requirement_set(
        access_shape,
        selectivity_shape,
    ))
}

fn derive_compatible_graph_read_access_requirement_set(
    access_shape: &WorthQueryGraphReadAccessShape,
    selectivity_shape: &WorthQueryBooleanSelectivityShape,
) -> WorthQueryGraphReadAccessRequirementSet {
    let mut rows = Vec::new();
    rows.extend(traversal_requirement_rows(access_shape));
    rows.extend(predicate_requirement_rows(access_shape, selectivity_shape));
    rows.extend(ordering_requirement_rows(access_shape));
    rows.extend(proof_requirement_rows(access_shape));
    rows.extend(lifecycle_requirement_rows(access_shape));
    WorthQueryGraphReadAccessRequirementSet::new(
        access_shape.operation_resolution().read_graph_digest(),
        access_shape.digest().as_str(),
        selectivity_shape.digest().as_str(),
        rows,
    )
}

fn validate_requirement_derivation_inputs(
    access_shape: &WorthQueryGraphReadAccessShape,
    selectivity_shape: &WorthQueryBooleanSelectivityShape,
) -> Result<(), WorthQueryGraphReadAccessRequirementDerivationError> {
    let access_shape_read_graph_digest = access_shape.operation_resolution().read_graph_digest();
    if access_shape_read_graph_digest != selectivity_shape.read_graph_digest() {
        return Err(
            WorthQueryGraphReadAccessRequirementDerivationError::ReadGraphDigestMismatch {
                access_shape_read_graph_digest: access_shape_read_graph_digest.to_string(),
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

fn traversal_requirement_rows(
    access_shape: &WorthQueryGraphReadAccessShape,
) -> Vec<WorthQueryGraphReadAccessRequirementRow> {
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
    operator: WorthQueryGraphReadTraversalOperator,
    relation: &WorthQueryAdmittedGraphReadRelation,
    result_pressure: WorthQueryGraphReadResultPressure,
    fanout_posture: crate::runtime::WorthQueryGraphReadFanoutPosture,
    schema_basis_digest: &str,
) -> Vec<WorthQueryGraphReadAccessRequirementRow> {
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
                WorthQueryGraphReadAccessRequirementKind::TraversalWorkset,
                WorthQueryGraphReadAccessRebuildBasis::OperationResolutionProof,
                WorthQueryGraphReadAccessInvalidationBasis::ReadGraphProofDelta,
                WorthQueryGraphReadAccessComplexityContract::BoundedTraversalWorkset,
                WorthQueryGraphReadAccessMemoryEstimateBasis::FrontierDepthBound,
                relation,
                fanout_posture.clone(),
                schema_basis_digest,
            )
            .with_traversal_operator(operator.clone()),
        );
        rows.push(
            relation_row(
                WorthQueryGraphReadAccessRequirementKind::VisitedSet,
                WorthQueryGraphReadAccessRebuildBasis::OperationResolutionProof,
                WorthQueryGraphReadAccessInvalidationBasis::ReadGraphProofDelta,
                WorthQueryGraphReadAccessComplexityContract::BoundedTraversalWorkset,
                WorthQueryGraphReadAccessMemoryEstimateBasis::FrontierDepthBound,
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
                WorthQueryGraphReadAccessRequirementKind::DedupSet,
                WorthQueryGraphReadAccessRebuildBasis::OperationResolutionProof,
                WorthQueryGraphReadAccessInvalidationBasis::ReadGraphProofDelta,
                WorthQueryGraphReadAccessComplexityContract::BoundedTraversalWorkset,
                WorthQueryGraphReadAccessMemoryEstimateBasis::FrontierDepthBound,
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
    operator: &WorthQueryGraphReadTraversalOperator,
    relation: &WorthQueryAdmittedGraphReadRelation,
    fanout_posture: crate::runtime::WorthQueryGraphReadFanoutPosture,
    schema_basis_digest: &str,
) -> Vec<WorthQueryGraphReadAccessRequirementRow> {
    match operator {
        WorthQueryGraphReadTraversalOperator::BoundedAncestor
        | WorthQueryGraphReadTraversalOperator::SharedEndpoint
        | WorthQueryGraphReadTraversalOperator::SharedAttachment => {
            vec![relation_row(
                WorthQueryGraphReadAccessRequirementKind::ReverseAdjacency,
                WorthQueryGraphReadAccessRebuildBasis::AuthoritativeRelationTruth,
                WorthQueryGraphReadAccessInvalidationBasis::AuthoritativeRelationDelta,
                WorthQueryGraphReadAccessComplexityContract::ReverseRelationLookup,
                WorthQueryGraphReadAccessMemoryEstimateBasis::RelationDegreeBound,
                relation,
                fanout_posture,
                schema_basis_digest,
            )
            .with_traversal_operator(operator.clone())]
        }
        WorthQueryGraphReadTraversalOperator::FrontierSearch => vec![
            relation_row(
                WorthQueryGraphReadAccessRequirementKind::DirectionalAdjacency,
                WorthQueryGraphReadAccessRebuildBasis::AuthoritativeRelationTruth,
                WorthQueryGraphReadAccessInvalidationBasis::AuthoritativeRelationDelta,
                WorthQueryGraphReadAccessComplexityContract::DirectionalRelationLookup,
                WorthQueryGraphReadAccessMemoryEstimateBasis::RelationDegreeBound,
                relation,
                fanout_posture.clone(),
                schema_basis_digest,
            )
            .with_traversal_operator(operator.clone()),
            relation_row(
                WorthQueryGraphReadAccessRequirementKind::ReverseAdjacency,
                WorthQueryGraphReadAccessRebuildBasis::AuthoritativeRelationTruth,
                WorthQueryGraphReadAccessInvalidationBasis::AuthoritativeRelationDelta,
                WorthQueryGraphReadAccessComplexityContract::ReverseRelationLookup,
                WorthQueryGraphReadAccessMemoryEstimateBasis::RelationDegreeBound,
                relation,
                fanout_posture,
                schema_basis_digest,
            )
            .with_traversal_operator(operator.clone()),
        ],
        WorthQueryGraphReadTraversalOperator::DirectEdge
        | WorthQueryGraphReadTraversalOperator::SuccessorWalk
        | WorthQueryGraphReadTraversalOperator::BoundedDescendant
        | WorthQueryGraphReadTraversalOperator::AnchoredFrontier
        | WorthQueryGraphReadTraversalOperator::DeclarationTraversal => vec![relation_row(
            WorthQueryGraphReadAccessRequirementKind::DirectionalAdjacency,
            WorthQueryGraphReadAccessRebuildBasis::AuthoritativeRelationTruth,
            WorthQueryGraphReadAccessInvalidationBasis::AuthoritativeRelationDelta,
            WorthQueryGraphReadAccessComplexityContract::DirectionalRelationLookup,
            WorthQueryGraphReadAccessMemoryEstimateBasis::RelationDegreeBound,
            relation,
            fanout_posture,
            schema_basis_digest,
        )
        .with_traversal_operator(operator.clone())],
    }
}

fn predicate_requirement_rows(
    access_shape: &WorthQueryGraphReadAccessShape,
    selectivity_shape: &WorthQueryBooleanSelectivityShape,
) -> Vec<WorthQueryGraphReadAccessRequirementRow> {
    if access_shape.predicate_family() == &WorthQueryGraphReadPredicateFamily::None
        && selectivity_shape.predicate_rows().is_empty()
    {
        return Vec::new();
    }
    vec![WorthQueryGraphReadAccessRequirementRow::new(
        WorthQueryGraphReadAccessRequirementKind::PredicateSupport,
        WorthQueryGraphReadAccessRebuildBasis::SelectivityProof,
        WorthQueryGraphReadAccessInvalidationBasis::AuthoritativeFieldDelta,
        WorthQueryGraphReadAccessComplexityContract::CandidatePredicateSupport,
        WorthQueryGraphReadAccessMemoryEstimateBasis::PredicateCandidateSet,
    )
    .with_predicate_family(access_shape.predicate_family().clone())
    .with_predicate_field_authorities(predicate_field_authorities(access_shape, selectivity_shape))]
}

fn ordering_requirement_rows(
    access_shape: &WorthQueryGraphReadAccessShape,
) -> Vec<WorthQueryGraphReadAccessRequirementRow> {
    if access_shape.ordering_posture() == &WorthQueryGraphReadOrderingPosture::Unordered {
        return Vec::new();
    }
    vec![WorthQueryGraphReadAccessRequirementRow::new(
        WorthQueryGraphReadAccessRequirementKind::OrderingSupport,
        WorthQueryGraphReadAccessRebuildBasis::AuthoritativeFieldTruth,
        WorthQueryGraphReadAccessInvalidationBasis::AuthoritativeFieldDelta,
        WorthQueryGraphReadAccessComplexityContract::CandidateOrderingSupport,
        WorthQueryGraphReadAccessMemoryEstimateBasis::OrderedCandidateSet,
    )
    .with_ordering_posture(access_shape.ordering_posture().clone())
    .with_ordering_field_authorities(ordering_field_authorities(access_shape))]
}

fn proof_requirement_rows(
    access_shape: &WorthQueryGraphReadAccessShape,
) -> Vec<WorthQueryGraphReadAccessRequirementRow> {
    if access_shape.relationship_proof_posture()
        == &WorthQueryGraphReadRelationshipProofBindingPosture::NotRequired
    {
        return Vec::new();
    }
    vec![WorthQueryGraphReadAccessRequirementRow::new(
        WorthQueryGraphReadAccessRequirementKind::ProofSupport,
        WorthQueryGraphReadAccessRebuildBasis::ReadGraphProof,
        WorthQueryGraphReadAccessInvalidationBasis::ReadGraphProofDelta,
        WorthQueryGraphReadAccessComplexityContract::ProofEvidenceSupport,
        WorthQueryGraphReadAccessMemoryEstimateBasis::ProofEvidenceSet,
    )]
}

fn lifecycle_requirement_rows(
    access_shape: &WorthQueryGraphReadAccessShape,
) -> Vec<WorthQueryGraphReadAccessRequirementRow> {
    match access_shape.lifecycle_class() {
        WorthQueryGraphReadLifecycleClass::ReusableReadFamily => {
            vec![WorthQueryGraphReadAccessRequirementRow::new(
                WorthQueryGraphReadAccessRequirementKind::MaterializationLifecycle,
                WorthQueryGraphReadAccessRebuildBasis::RuntimeSupportRequired,
                WorthQueryGraphReadAccessInvalidationBasis::RuntimeLifecycleDelta,
                WorthQueryGraphReadAccessComplexityContract::LifecycleSupportAdmission,
                WorthQueryGraphReadAccessMemoryEstimateBasis::LifecycleManagedSupport,
            )
            .with_lifecycle_class(access_shape.lifecycle_class().clone())]
        }
    }
}

fn relation_row(
    kind: WorthQueryGraphReadAccessRequirementKind,
    rebuild_basis: WorthQueryGraphReadAccessRebuildBasis,
    invalidation_basis: WorthQueryGraphReadAccessInvalidationBasis,
    complexity_contract: WorthQueryGraphReadAccessComplexityContract,
    memory_estimate_basis: WorthQueryGraphReadAccessMemoryEstimateBasis,
    relation: &WorthQueryAdmittedGraphReadRelation,
    fanout_posture: crate::runtime::WorthQueryGraphReadFanoutPosture,
    schema_basis_digest: &str,
) -> WorthQueryGraphReadAccessRequirementRow {
    WorthQueryGraphReadAccessRequirementRow::new(
        kind,
        rebuild_basis,
        invalidation_basis,
        complexity_contract,
        memory_estimate_basis,
    )
    .with_relation(
        relation.terminal_relation_projection_for_boundary(),
        relation_authority(
            schema_basis_digest,
            relation.terminal_relation_projection_for_boundary(),
        ),
        relation.direction().clone(),
        relation.depth(),
    )
    .with_fanout_posture(fanout_posture)
}

fn result_buffer_row(
    result_pressure: WorthQueryGraphReadResultPressure,
) -> WorthQueryGraphReadAccessRequirementRow {
    WorthQueryGraphReadAccessRequirementRow::new(
        WorthQueryGraphReadAccessRequirementKind::ResultBuffer,
        WorthQueryGraphReadAccessRebuildBasis::OperationResolutionProof,
        WorthQueryGraphReadAccessInvalidationBasis::ReadGraphProofDelta,
        WorthQueryGraphReadAccessComplexityContract::ResultPressureBuffer,
        WorthQueryGraphReadAccessMemoryEstimateBasis::ResultPressureBound,
    )
    .with_result_pressure(result_pressure)
}

fn requires_traversal_workset(operator: &WorthQueryGraphReadTraversalOperator) -> bool {
    !matches!(operator, WorthQueryGraphReadTraversalOperator::DirectEdge)
}

fn requires_dedup_set(operator: &WorthQueryGraphReadTraversalOperator) -> bool {
    matches!(
        operator,
        WorthQueryGraphReadTraversalOperator::AnchoredFrontier
            | WorthQueryGraphReadTraversalOperator::SharedEndpoint
            | WorthQueryGraphReadTraversalOperator::SharedAttachment
            | WorthQueryGraphReadTraversalOperator::FrontierSearch
    )
}
