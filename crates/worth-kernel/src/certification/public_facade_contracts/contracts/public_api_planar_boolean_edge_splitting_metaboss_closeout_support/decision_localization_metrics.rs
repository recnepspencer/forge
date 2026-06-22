use std::collections::BTreeSet;

pub(crate) fn decision_phase_count(
    rows: &[worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitDecisionRow],
) -> usize {
    rows.iter()
        .map(|row| row.phase())
        .collect::<BTreeSet<_>>()
        .len()
}

pub(crate) fn decision_kind_count(
    rows: &[worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitDecisionRow],
) -> usize {
    rows.iter()
        .map(|row| row.kind())
        .collect::<BTreeSet<_>>()
        .len()
}

pub(crate) fn localized_decision_rows(
    rows: &[worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitDecisionRow],
) -> usize {
    rows.iter()
        .filter(|row| {
            !row.decision_identity().is_empty()
                && !row.affected_artifact_identity().is_empty()
                && !row.upstream_receipt_identity().is_empty()
        })
        .count()
}
