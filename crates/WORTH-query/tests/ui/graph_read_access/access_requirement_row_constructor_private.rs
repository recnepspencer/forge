use worth_query::facade::runtime::{
    WorthQueryGraphReadAccessComplexityContract, WorthQueryGraphReadAccessInvalidationBasis,
    WorthQueryGraphReadAccessMemoryEstimateBasis, WorthQueryGraphReadAccessRebuildBasis,
    WorthQueryGraphReadAccessRequirementKind, WorthQueryGraphReadAccessRequirementRow,
};

fn main() {
    let _ = WorthQueryGraphReadAccessRequirementRow {
        kind: WorthQueryGraphReadAccessRequirementKind::DirectionalAdjacency,
        rebuild_basis: WorthQueryGraphReadAccessRebuildBasis::AuthoritativeRelationTruth,
        relation_name: None,
        relation_authority: None,
        relation_direction: None,
        relation_depth: None,
        fanout_posture: None,
        predicate_family: None,
        predicate_field_authorities: Vec::new(),
        ordering_posture: None,
        ordering_field_authorities: Vec::new(),
        traversal_operator: None,
        lifecycle_class: None,
        result_pressure: None,
        invalidation_basis: WorthQueryGraphReadAccessInvalidationBasis::AuthoritativeRelationDelta,
        complexity_contract: WorthQueryGraphReadAccessComplexityContract::DirectionalRelationLookup,
        memory_estimate_basis: WorthQueryGraphReadAccessMemoryEstimateBasis::RelationDegreeBound,
    };
}
