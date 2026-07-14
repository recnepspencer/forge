use super::{
    WorthQueryAdmittedBooleanExpressionTopology, WorthQueryAdmittedBooleanPredicateExpression,
    WorthQueryBooleanPredicateSelectivityRow, WorthQueryBooleanPredicateTopology,
    WorthQueryBooleanSelectivityAdmissionPosture, WorthQueryBooleanSelectivityBranch,
    WorthQueryBooleanSelectivityBranchKind, WorthQueryBooleanSelectivityShape,
    WorthQueryGraphReadAccessShape, WorthQueryPredicateAnchorPosture,
    WorthQueryPredicateSelectivityClass, WorthQueryTraversalPredicateOrderingPosture,
};

pub(crate) fn normalize_boolean_selectivity_for_access_shape(
    access_shape: WorthQueryGraphReadAccessShape,
    expression: WorthQueryAdmittedBooleanPredicateExpression,
) -> WorthQueryBooleanSelectivityShape {
    let branches = expression
        .branches()
        .iter()
        .map(|branch| {
            let rows = branch
                .predicate_leaves()
                .iter()
                .map(|leaf| {
                    WorthQueryBooleanPredicateSelectivityRow::new(
                        leaf.native_aspect_key().clone(),
                        leaf.native_field_key().clone(),
                        leaf.family(),
                        leaf.operator().clone(),
                        leaf.normalized_operand_values().to_vec(),
                        leaf.field_kind().clone(),
                        leaf.selectivity_class().clone(),
                    )
                })
                .collect::<Vec<_>>();
            WorthQueryBooleanSelectivityBranch::from_expression_branch(
                WorthQueryBooleanSelectivityBranchKind::from(branch.branch_kind()),
                branch.expression_path(),
                anchor_posture(rows.as_slice()),
                traversal_predicate_ordering_posture(rows.as_slice()),
                rows,
            )
        })
        .collect::<Vec<_>>();

    let mut predicate_rows = branches
        .iter()
        .flat_map(|branch| branch.predicate_rows().iter().cloned())
        .collect::<Vec<_>>();
    let predicate_count_before_dedup = predicate_rows.len();
    predicate_rows.sort_by_key(|row| row.digest_part());
    predicate_rows.dedup_by_key(|row| row.digest_part());
    let deduplicated_predicate_count = predicate_count_before_dedup - predicate_rows.len();

    let boolean_topology = boolean_topology(&expression);
    let anchor_posture = anchor_posture(predicate_rows.as_slice());
    let traversal_predicate_ordering_posture =
        traversal_predicate_ordering_posture(predicate_rows.as_slice());
    let admission_posture = admission_posture(predicate_rows.as_slice());

    WorthQueryBooleanSelectivityShape::new(
        access_shape,
        boolean_topology,
        anchor_posture,
        traversal_predicate_ordering_posture,
        admission_posture,
        expression,
        branches,
        predicate_rows,
        deduplicated_predicate_count,
    )
}

fn boolean_topology(
    expression: &WorthQueryAdmittedBooleanPredicateExpression,
) -> WorthQueryBooleanPredicateTopology {
    match expression.topology() {
        WorthQueryAdmittedBooleanExpressionTopology::Empty => {
            WorthQueryBooleanPredicateTopology::None
        }
        WorthQueryAdmittedBooleanExpressionTopology::ConjunctiveFlat => {
            WorthQueryBooleanPredicateTopology::ConjunctiveFlat
        }
    }
}

fn admission_posture(
    rows: &[WorthQueryBooleanPredicateSelectivityRow],
) -> WorthQueryBooleanSelectivityAdmissionPosture {
    if rows.iter().any(|row| {
        row.selectivity_class() == &WorthQueryPredicateSelectivityClass::TraversalPredicate
    }) {
        WorthQueryBooleanSelectivityAdmissionPosture::RequiresAccessCapabilityRegistration
    } else {
        WorthQueryBooleanSelectivityAdmissionPosture::InlineEligible
    }
}

fn anchor_posture(
    rows: &[WorthQueryBooleanPredicateSelectivityRow],
) -> WorthQueryPredicateAnchorPosture {
    let has_exact = rows
        .iter()
        .any(|row| row.selectivity_class().is_exact_anchor());
    let has_membership = rows.iter().any(|row| row.family() == "set-membership");
    let has_broad_or_risky = rows
        .iter()
        .any(|row| row.selectivity_class().is_broad_or_risky());
    match (
        rows.is_empty(),
        has_exact,
        has_membership,
        has_broad_or_risky,
    ) {
        (true, _, _, _) => WorthQueryPredicateAnchorPosture::NoPredicateAnchor,
        (false, true, _, true) | (false, false, true, true) => {
            WorthQueryPredicateAnchorPosture::MixedAnchorAndBroad
        }
        (false, true, _, false) => WorthQueryPredicateAnchorPosture::AnchoredByExactPredicate,
        (false, false, true, false) => {
            WorthQueryPredicateAnchorPosture::AnchoredByMembershipPredicate
        }
        (false, false, false, true) => WorthQueryPredicateAnchorPosture::BroadOnly,
        (false, false, false, false) => WorthQueryPredicateAnchorPosture::NoPredicateAnchor,
    }
}

fn traversal_predicate_ordering_posture(
    rows: &[WorthQueryBooleanPredicateSelectivityRow],
) -> WorthQueryTraversalPredicateOrderingPosture {
    let pre_traversal_count = rows
        .iter()
        .filter(|row| row.is_pre_traversal_eligible())
        .count();
    match (rows.len(), pre_traversal_count) {
        (0, _) => WorthQueryTraversalPredicateOrderingPosture::NoPredicate,
        (total, eligible) if total == eligible => {
            WorthQueryTraversalPredicateOrderingPosture::PreTraversalEligible
        }
        (_, 0) => WorthQueryTraversalPredicateOrderingPosture::PostTraversalFilterRequired,
        _ => WorthQueryTraversalPredicateOrderingPosture::Mixed,
    }
}
