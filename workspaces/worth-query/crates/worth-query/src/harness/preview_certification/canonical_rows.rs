use crate::harness::certification::{CanonicalCertificationRow, ParityAnchor};

use super::lane_builders::PreviewCertificationLanes;
use super::model::{PreviewCertificationLane, PreviewCertificationRow, PreviewPerturbationClass};
use super::row_catalog::{PreviewCanonicalRowSpec, PREVIEW_CANONICAL_ROW_SPECS};

pub(super) fn canonical_rows(lanes: &PreviewCertificationLanes) -> Vec<PreviewCertificationRow> {
    PREVIEW_CANONICAL_ROW_SPECS
        .iter()
        .map(|spec| {
            canonical_row(
                spec,
                &lanes.active,
                &lanes.parity,
                &lanes.promotable,
                &lanes.promotion_parity,
                &lanes.preview_live,
                &lanes.parity_preview_live,
                &lanes.preview_live_rebind,
            )
        })
        .collect()
}

fn canonical_row(
    spec: &PreviewCanonicalRowSpec,
    active_lane: &PreviewCertificationLane,
    parity_lane: &PreviewCertificationLane,
    promotable_lane: &PreviewCertificationLane,
    promotion_parity_lane: &PreviewCertificationLane,
    preview_live_lane: &PreviewCertificationLane,
    parity_preview_live_lane: &PreviewCertificationLane,
    preview_live_rebind_lane: &PreviewCertificationLane,
) -> CanonicalCertificationRow<PreviewPerturbationClass, PreviewCertificationLane> {
    let control_lane = match spec.row_name {
        "preview-promotion-comparison-parity" | "preview-comparison-shape-proof-width" => {
            promotion_parity_lane.clone()
        }
        "preview-live-admission-parity" | "preview-live-drift-explicitness" => {
            preview_live_lane.clone()
        }
        "preview-workflow-foundation-admission"
        | "preview-workflow-foundation-no-rescan"
        | "preview-work-avoided-counter-parity" => promotable_lane.clone(),
        _ => active_lane.clone(),
    };
    CanonicalCertificationRow {
        row_name: spec.row_name,
        perturbation_class: spec.perturbation_class,
        hostile_expectation: spec.hostile_expectation,
        parity_anchor: ParityAnchor::Control,
        control_lane: control_lane.clone(),
        hostile_lane: match spec.hostile_lane_selector {
            super::row_catalog::PreviewLaneSelector::ParityExecution => parity_lane.clone(),
            super::row_catalog::PreviewLaneSelector::PromotionEligibleExecution => {
                promotable_lane.clone()
            }
            super::row_catalog::PreviewLaneSelector::PromotionParity => {
                promotion_parity_lane.clone()
            }
            super::row_catalog::PreviewLaneSelector::PreviewLiveAdmission => {
                parity_preview_live_lane.clone()
            }
            super::row_catalog::PreviewLaneSelector::PreviewLiveRebind => {
                preview_live_rebind_lane.clone()
            }
        },
        parity_lane: control_lane,
    }
}
