use std::collections::BTreeMap;

use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
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
            Some(1),
            "missing or duplicated request for seam {}",
            row.seam_key().as_str()
        );
        assert_eq!(
            receipt_counts.get(row.seam_key().as_str()).copied(),
            Some(1),
            "missing or duplicated receipt for seam {}",
            row.seam_key().as_str()
        );
        assert_eq!(
            envelope_counts.get(row.seam_key().as_str()).copied(),
            Some(1),
            "missing or duplicated envelope for seam {}",
            row.seam_key().as_str()
        );
        match row.route_kind() {
            ForgeQueryLowerRuntimeRouteKind::RoutePlanning => {
                assert_eq!(
                    route_plan_counts.get(row.seam_key().as_str()).copied(),
                    Some(1),
                    "missing or duplicated route plan for seam {}",
                    row.seam_key().as_str()
                );
                assert_eq!(
                    receipt_kinds.get(row.seam_key().as_str()),
                    Some(&ForgeQueryLowerRuntimeBoundaryExecutionKind::RoutePlan),
                    "unexpected route receipt kind for seam {}",
                    row.seam_key().as_str()
                );
            }
            ForgeQueryLowerRuntimeRouteKind::ReadmissionHandoff => {
                assert_eq!(
                    route_plan_counts
                        .get(row.seam_key().as_str())
                        .copied()
                        .unwrap_or(0),
                    0,
                    "readmission seam {} unexpectedly carried a route plan",
                    row.seam_key().as_str()
                );
                assert_eq!(
                    receipt_kinds.get(row.seam_key().as_str()),
                    Some(&ForgeQueryLowerRuntimeBoundaryExecutionKind::ReadmissionHandoff),
                    "unexpected readmission receipt kind for seam {}",
                    row.seam_key().as_str()
                );
            }
        }
    }

    let row_identities = crossings
        .rows()
        .iter()
        .map(|row| {
            ForgeQueryEvidenceIdentity::compose(
                ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
            )
            .field_shape(ForgeQueryEvidenceTag::new("seam"), row.seam_key().as_str())
            .field_shape(
                ForgeQueryEvidenceTag::new("route_kind"),
                row.route_kind().as_str(),
            )
            .field_usize(
                ForgeQueryEvidenceTag::new("request_count"),
                request_counts
                    .get(row.seam_key().as_str())
                    .copied()
                    .unwrap_or(0),
            )
            .field_usize(
                ForgeQueryEvidenceTag::new("route_plan_count"),
                route_plan_counts
                    .get(row.seam_key().as_str())
                    .copied()
                    .unwrap_or(0),
            )
            .field_usize(
                ForgeQueryEvidenceTag::new("receipt_count"),
                receipt_counts
                    .get(row.seam_key().as_str())
                    .copied()
                    .unwrap_or(0),
            )
            .field_usize(
                ForgeQueryEvidenceTag::new("envelope_count"),
                envelope_counts
                    .get(row.seam_key().as_str())
                    .copied()
                    .unwrap_or(0),
            )
            .seal()
        })
        .collect::<Vec<_>>();
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_evidence_identity_sequence(ForgeQueryEvidenceTag::new("rows"), &row_identities)
        .seal()
        .as_str()
        .to_string()
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
    let request_by_identity = surface
        .requests()
        .iter()
        .map(|request| {
            (
                request.request_identity().clone(),
                request.seam_key().as_str(),
            )
        })
        .collect::<Vec<_>>();
    let mut counts = BTreeMap::new();
    for receipt in surface.boundary_receipts() {
        let seam_key = request_by_identity
            .iter()
            .find_map(|(identity, seam_key)| {
                (identity == receipt.request_identity()).then_some(seam_key)
            })
            .unwrap_or_else(|| {
                panic!(
                    "receipt request {} must exist",
                    receipt.request_identity().as_ref()
                )
            });
        *counts.entry(*seam_key).or_insert(0usize) += 1;
    }
    counts
}

fn receipt_kind_by_seam(
    surface: &ForgeQueryLowerRuntimeRepresentativeSurface,
) -> BTreeMap<&'static str, ForgeQueryLowerRuntimeBoundaryExecutionKind> {
    let request_by_identity = surface
        .requests()
        .iter()
        .map(|request| {
            (
                request.request_identity().clone(),
                request.seam_key().as_str(),
            )
        })
        .collect::<Vec<_>>();
    let mut kinds = BTreeMap::new();
    for receipt in surface.boundary_receipts() {
        let seam_key = request_by_identity
            .iter()
            .find_map(|(identity, seam_key)| {
                (identity == receipt.request_identity()).then_some(seam_key)
            })
            .unwrap_or_else(|| {
                panic!(
                    "receipt request {} must exist",
                    receipt.request_identity().as_ref()
                )
            });
        kinds.insert(*seam_key, receipt.kind());
    }
    kinds
}
