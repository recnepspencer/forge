use std::collections::BTreeMap;

use crate::identity::hash_parts;
use crate::lower_runtime_routing::{
    forge_query_lower_runtime_crossing_inventory, ForgeQueryLowerRuntimeBoundaryExecutionKind,
    ForgeQueryLowerRuntimeRouteKind,
};

use super::evidence::ForgeQueryLowerRuntimeRepresentativeSurface;

pub(super) fn admitted_crossing_cardinality_digest(
    surface: &ForgeQueryLowerRuntimeRepresentativeSurface,
) -> String {
    let crossings = forge_query_lower_runtime_crossing_inventory();
    let request_counts = count_by_seam(
        surface
            .requests()
            .iter()
            .map(|request| request.seam_key().as_str()),
    );
    let route_plan_counts = count_by_seam(
        surface
            .route_plans()
            .iter()
            .map(|plan| plan.eligibility().request().seam_key().as_str()),
    );
    let envelope_counts = count_by_seam(
        surface
            .envelopes()
            .iter()
            .map(|envelope| envelope.seam_key().as_str()),
    );
    let receipt_counts = count_receipts_by_seam(surface);
    let receipt_kinds = receipt_kind_by_seam(surface);

    for row in crossings.rows() {
        assert_eq!(
            request_counts.get(row.seam_key().as_str()).copied(),
            Some(1)
        );
        assert_eq!(
            receipt_counts.get(row.seam_key().as_str()).copied(),
            Some(1)
        );
        assert_eq!(
            envelope_counts.get(row.seam_key().as_str()).copied(),
            Some(1)
        );
        match row.route_kind() {
            ForgeQueryLowerRuntimeRouteKind::RoutePlanning => {
                assert_eq!(
                    route_plan_counts.get(row.seam_key().as_str()).copied(),
                    Some(1)
                );
                assert_eq!(
                    receipt_kinds.get(row.seam_key().as_str()),
                    Some(&ForgeQueryLowerRuntimeBoundaryExecutionKind::RoutePlan)
                );
            }
            ForgeQueryLowerRuntimeRouteKind::ReadmissionHandoff => {
                assert_eq!(
                    route_plan_counts
                        .get(row.seam_key().as_str())
                        .copied()
                        .unwrap_or(0),
                    0
                );
                assert_eq!(
                    receipt_kinds.get(row.seam_key().as_str()),
                    Some(&ForgeQueryLowerRuntimeBoundaryExecutionKind::ReadmissionHandoff)
                );
            }
        }
    }

    hash_parts(
        &crossings
            .rows()
            .iter()
            .map(|row| {
                format!(
                    "{}|{}|req:{}|plan:{}|receipt:{}|envelope:{}",
                    row.seam_key().as_str(),
                    row.route_kind().as_str(),
                    request_counts
                        .get(row.seam_key().as_str())
                        .copied()
                        .unwrap_or(0),
                    route_plan_counts
                        .get(row.seam_key().as_str())
                        .copied()
                        .unwrap_or(0),
                    receipt_counts
                        .get(row.seam_key().as_str())
                        .copied()
                        .unwrap_or(0),
                    envelope_counts
                        .get(row.seam_key().as_str())
                        .copied()
                        .unwrap_or(0)
                )
            })
            .collect::<Vec<_>>(),
    )
}

fn count_by_seam(seams: impl Iterator<Item = &'static str>) -> BTreeMap<&'static str, usize> {
    let mut counts = BTreeMap::new();
    for seam_key in seams {
        *counts.entry(seam_key).or_insert(0usize) += 1;
    }
    counts
}

fn count_receipts_by_seam(
    surface: &ForgeQueryLowerRuntimeRepresentativeSurface,
) -> BTreeMap<&'static str, usize> {
    let request_by_digest: BTreeMap<_, _> = surface
        .requests()
        .iter()
        .map(|request| {
            (
                request.request_digest().to_string(),
                request.seam_key().as_str(),
            )
        })
        .collect();
    let mut counts = BTreeMap::new();
    for receipt in surface.boundary_receipts() {
        let seam_key = request_by_digest
            .get(receipt.request_digest())
            .unwrap_or_else(|| panic!("receipt request {} must exist", receipt.request_digest()));
        *counts.entry(*seam_key).or_insert(0usize) += 1;
    }
    counts
}

fn receipt_kind_by_seam(
    surface: &ForgeQueryLowerRuntimeRepresentativeSurface,
) -> BTreeMap<&'static str, ForgeQueryLowerRuntimeBoundaryExecutionKind> {
    let request_by_digest: BTreeMap<_, _> = surface
        .requests()
        .iter()
        .map(|request| {
            (
                request.request_digest().to_string(),
                request.seam_key().as_str(),
            )
        })
        .collect();
    let mut kinds = BTreeMap::new();
    for receipt in surface.boundary_receipts() {
        let seam_key = request_by_digest
            .get(receipt.request_digest())
            .unwrap_or_else(|| panic!("receipt request {} must exist", receipt.request_digest()));
        kinds.insert(*seam_key, receipt.kind());
    }
    kinds
}
