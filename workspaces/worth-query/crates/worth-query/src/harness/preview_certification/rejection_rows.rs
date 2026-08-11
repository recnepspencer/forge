use crate::harness::certification::RejectionCertificationRow;

use super::lane_builders::PreviewCertificationLanes;
use super::model::{
    PreviewCertificationLane, PreviewCertificationRejection, PreviewPerturbationClass,
    PreviewRejectionRow,
};
use super::rejection_evidence::PreviewRejectionEvidence;
use super::row_catalog::{PreviewRejectionRowSpec, PREVIEW_REJECTION_ROW_SPECS};

pub(super) fn rejection_rows(
    lanes: &PreviewCertificationLanes,
    evidence: &PreviewRejectionEvidence,
) -> Vec<PreviewRejectionRow> {
    PREVIEW_REJECTION_ROW_SPECS
        .iter()
        .map(|spec| {
            rejection_row(
                spec,
                &lanes.active,
                &lanes.parity,
                evidence,
                &lanes.preview_live,
                &lanes.parity_preview_live,
            )
        })
        .collect()
}

fn rejection_row(
    spec: &PreviewRejectionRowSpec,
    active_lane: &PreviewCertificationLane,
    parity_lane: &PreviewCertificationLane,
    evidence: &PreviewRejectionEvidence,
    preview_live_lane: &PreviewCertificationLane,
    parity_preview_live_lane: &PreviewCertificationLane,
) -> RejectionCertificationRow<
    PreviewPerturbationClass,
    PreviewCertificationLane,
    PreviewCertificationRejection,
> {
    let hostile_lane = match spec.runtime_failure_selector {
        Some(super::row_catalog::PreviewRuntimeFailureSelector::UnsupportedPreviewFamily) => {
            evidence.unsupported_preview_family.clone()
        }
        Some(super::row_catalog::PreviewRuntimeFailureSelector::InvalidBasis) => {
            evidence.invalid_basis.clone()
        }
        Some(super::row_catalog::PreviewRuntimeFailureSelector::BroadFallbackDenied) => {
            evidence.invalid_basis.clone()
        }
        Some(super::row_catalog::PreviewRuntimeFailureSelector::StaleLifecycle) => {
            evidence.stale_lifecycle.clone()
        }
        Some(super::row_catalog::PreviewRuntimeFailureSelector::DiscardedLifecycle) => {
            evidence.discarded_lifecycle.clone()
        }
        Some(super::row_catalog::PreviewRuntimeFailureSelector::PreviewLiveDriftDenied) => {
            evidence.preview_live_drift_denied.clone()
        }
        Some(super::row_catalog::PreviewRuntimeFailureSelector::PreviewLiveBroadFallbackDenied) => {
            evidence.preview_live_broad_fallback_denied.clone()
        }
        Some(
            super::row_catalog::PreviewRuntimeFailureSelector::WorkflowFoundationAuthorityDenied,
        ) => evidence.read_only_writeback_foundation_denied.clone(),
        Some(super::row_catalog::PreviewRuntimeFailureSelector::PromotionLinkageDenied) => {
            evidence.promotion_linkage_denied.clone()
        }
        Some(super::row_catalog::PreviewRuntimeFailureSelector::ReplayLinkageDenied) => {
            evidence.replay_linkage_denied.clone()
        }
        Some(super::row_catalog::PreviewRuntimeFailureSelector::ShapeMismatchDenied) => {
            evidence.shape_mismatch_denied.clone()
        }
        None => panic!(
            "preview rejection row {} has no runtime denial",
            spec.row_name
        ),
    };

    RejectionCertificationRow {
        row_name: spec.row_name,
        perturbation_class: spec.perturbation_class,
        control_lane: match spec.runtime_failure_selector {
            Some(super::row_catalog::PreviewRuntimeFailureSelector::PreviewLiveDriftDenied)
            | Some(
                super::row_catalog::PreviewRuntimeFailureSelector::PreviewLiveBroadFallbackDenied,
            ) => preview_live_lane.clone(),
            _ => active_lane.clone(),
        },
        hostile_lane,
        parity_lane: match spec.runtime_failure_selector {
            Some(super::row_catalog::PreviewRuntimeFailureSelector::PreviewLiveDriftDenied)
            | Some(
                super::row_catalog::PreviewRuntimeFailureSelector::PreviewLiveBroadFallbackDenied,
            ) => parity_preview_live_lane.clone(),
            _ => parity_lane.clone(),
        },
    }
}
