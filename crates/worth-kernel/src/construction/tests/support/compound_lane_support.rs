use std::collections::BTreeSet;

use crate::construction::tests::support::compound_ordering::{
    required_compound_adversarial_lane_name_set, PrimitiveConstructionAdversarialAuthoringOrderLane,
};
use crate::construction::tests::support::compound_row_support::{
    exhaustion_reason, grazing_kind, motion_kind, realization_strategy, rejection_class,
    rejection_locality, row_digest, stability_class,
};
use crate::construction::tests::support::compound_runtime::{
    compound_parity_registry, PrimitiveConstructionCompoundAdversarialLanes,
    PrimitiveConstructionCompoundRow,
};
use crate::construction::tests::support::evidence_reports::sealed_report_identity;

pub(crate) fn compound_canonical_rows(
    lanes: &PrimitiveConstructionCompoundAdversarialLanes,
) -> &[PrimitiveConstructionCompoundRow] {
    lanes
        .iter()
        .find(|(lane, _)| *lane == PrimitiveConstructionAdversarialAuthoringOrderLane::Canonical)
        .map(|(_, rows)| rows.as_slice())
        .unwrap_or(&[])
}

pub(crate) fn compound_lane_names(
    lanes: &PrimitiveConstructionCompoundAdversarialLanes,
) -> Vec<String> {
    lanes
        .iter()
        .map(|(lane, _)| lane.as_str().to_string())
        .collect()
}

pub(crate) fn compound_row_for<'a>(
    lanes: &'a PrimitiveConstructionCompoundAdversarialLanes,
    scenario_id: &str,
) -> Option<&'a PrimitiveConstructionCompoundRow> {
    compound_canonical_rows(lanes)
        .iter()
        .find(|row| row.scenario_id() == scenario_id)
}

pub(crate) fn compound_required_scenario_coverage_verified(
    lanes: &PrimitiveConstructionCompoundAdversarialLanes,
) -> bool {
    let scenario_ids = compound_canonical_rows(lanes)
        .iter()
        .map(|row| row.scenario_id().to_string())
        .collect::<BTreeSet<_>>();
    let required_scenario_ids = compound_parity_registry()
        .required_scenario_inventory()
        .scenario_ids()
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    scenario_ids == required_scenario_ids
}

pub(crate) fn compound_required_lane_coverage_verified(
    lanes: &PrimitiveConstructionCompoundAdversarialLanes,
) -> bool {
    lanes
        .iter()
        .map(|(lane, _)| lane.as_str())
        .collect::<BTreeSet<_>>()
        == required_compound_adversarial_lane_name_set()
}

pub(crate) fn compound_lane_digest_uniqueness_verified(
    lanes: &PrimitiveConstructionCompoundAdversarialLanes,
) -> bool {
    lanes
        .iter()
        .map(|(_, rows)| lane_digest(rows.iter().map(row_digest)))
        .collect::<BTreeSet<_>>()
        .len()
        == lanes.len()
}

pub(crate) fn compound_stable_scenario_coverage_verified(
    lanes: &PrimitiveConstructionCompoundAdversarialLanes,
) -> bool {
    compound_canonical_rows(lanes)
        .iter()
        .all(|canonical| compound_scenario_stable_across_orders(lanes, canonical.scenario_id()))
}

pub(crate) fn compound_authoring_order_parity_verified(
    lanes: &PrimitiveConstructionCompoundAdversarialLanes,
) -> bool {
    !lanes.is_empty()
        && compound_required_lane_coverage_verified(lanes)
        && compound_lane_digest_uniqueness_verified(lanes)
        && compound_required_scenario_coverage_verified(lanes)
        && compound_stable_scenario_coverage_verified(lanes)
}

pub(crate) fn compound_normalized_matrix_digest(
    lanes: &PrimitiveConstructionCompoundAdversarialLanes,
) -> String {
    lanes
        .iter()
        .find(|(lane, _)| *lane == PrimitiveConstructionAdversarialAuthoringOrderLane::Canonical)
        .map(|(_, rows)| {
            normalized_matrix_digest(
                rows.iter()
                    .map(|row| (row.scenario_id().to_string(), row_digest(row))),
            )
        })
        .unwrap_or_default()
}

pub(crate) fn compound_scenario_ids(
    lanes: &PrimitiveConstructionCompoundAdversarialLanes,
) -> Vec<String> {
    compound_canonical_rows(lanes)
        .iter()
        .map(|row| row.scenario_id().to_string())
        .collect()
}

pub(crate) fn compound_report_digest(
    lanes: &PrimitiveConstructionCompoundAdversarialLanes,
) -> String {
    let parts = lanes
        .iter()
        .flat_map(|(lane, rows)| {
            std::iter::once(lane.as_str().to_string())
                .chain(std::iter::once(lane_digest(rows.iter().map(row_digest))))
                .chain(std::iter::once(normalized_matrix_digest(
                    rows.iter()
                        .map(|row| (row.scenario_id().to_string(), row_digest(row))),
                )))
                .chain(rows.iter().map(row_digest))
        })
        .collect::<Vec<_>>();
    sealed_report_identity(
        "worth-kernel.construction.compound-lane",
        "compound-authoring-order",
        |report| report.value_sequence_participating("lane-parts", parts),
    )
}

pub(crate) fn compound_scenario_stable_across_orders(
    lanes: &PrimitiveConstructionCompoundAdversarialLanes,
    scenario_id: &str,
) -> bool {
    let Some(canonical) = compound_row_for(lanes, scenario_id) else {
        return false;
    };
    let lane_rows = lanes
        .iter()
        .filter_map(|(_, rows)| rows.iter().find(|row| row.scenario_id() == scenario_id))
        .collect::<Vec<_>>();
    lane_rows.len() == lanes.len()
        && lane_rows
            .iter()
            .all(|row| row_digest(row) == row_digest(canonical))
        && lane_rows
            .iter()
            .all(|row| row.topology_class() == canonical.topology_class())
        && lane_rows
            .iter()
            .all(|row| row.row_class() == canonical.row_class())
        && lane_rows
            .iter()
            .all(|row| realization_strategy(row) == realization_strategy(canonical))
        && lane_rows
            .iter()
            .all(|row| stability_class(row) == stability_class(canonical))
        && lane_rows
            .iter()
            .all(|row| exhaustion_reason(row) == exhaustion_reason(canonical))
        && lane_rows
            .iter()
            .all(|row| rejection_class(row) == rejection_class(canonical))
        && lane_rows
            .iter()
            .all(|row| rejection_locality(row) == rejection_locality(canonical))
        && lane_rows
            .iter()
            .all(|row| motion_kind(row) == motion_kind(canonical))
        && lane_rows
            .iter()
            .all(|row| grazing_kind(row) == grazing_kind(canonical))
}

fn lane_digest(row_digests: impl IntoIterator<Item = String>) -> String {
    sealed_report_identity(
        "worth-kernel.construction.compound-lane",
        "lane-row-inventory",
        |report| report.value_sequence_participating("row-identities", row_digests),
    )
}

fn normalized_matrix_digest(row_pairs: impl IntoIterator<Item = (String, String)>) -> String {
    let mut parts = row_pairs
        .into_iter()
        .map(|(scenario_id, row_digest)| format!("{scenario_id}:{row_digest}"))
        .collect::<Vec<_>>();
    parts.sort();
    sealed_report_identity(
        "worth-kernel.construction.compound-lane",
        "normalized-matrix",
        |report| report.value_sequence_participating("scenario-row-identities", parts),
    )
}
