use super::super::lane::{WorkflowCertificationLane, WorkflowPerturbationClass};
use super::super::row_catalog::WorkflowCanonicalRowSpec;
use crate::harness::certification::{CanonicalCertificationRow, ParityAnchor};

pub(super) fn canonical_row(
    spec: &WorkflowCanonicalRowSpec,
    runtime_conflict: &WorkflowCertificationLane,
    runtime_merge: &WorkflowCertificationLane,
    runtime_merge_alt_budget: &WorkflowCertificationLane,
    runtime_mutation: &WorkflowCertificationLane,
    preview_foundation: &WorkflowCertificationLane,
    merge_lowering: &WorkflowCertificationLane,
    writeback_lowering: &WorkflowCertificationLane,
    conflict_inspection: &WorkflowCertificationLane,
    denied_conflict_inspection: &WorkflowCertificationLane,
    post_merge_inspection: &WorkflowCertificationLane,
    preview_merge_lowering: &WorkflowCertificationLane,
) -> CanonicalCertificationRow<WorkflowPerturbationClass, WorkflowCertificationLane> {
    let (control_lane, hostile_lane) = match spec.row_name {
        "workflow-declaration-family-explicitness" => {
            (runtime_conflict.clone(), runtime_merge.clone())
        }
        "workflow-basis-family-explicitness" => {
            (runtime_conflict.clone(), preview_foundation.clone())
        }
        "workflow-authority-target-explicitness" => {
            (runtime_merge.clone(), runtime_mutation.clone())
        }
        "workflow-preview-foundation-no-rediscovery" => {
            (preview_foundation.clone(), preview_foundation.clone())
        }
        "workflow-budget-class-explicitness" => {
            (runtime_merge.clone(), runtime_merge_alt_budget.clone())
        }
        "query-authored-mutation-lowering-parity" => {
            (runtime_mutation.clone(), runtime_mutation.clone())
        }
        "query-authored-merge-lowering-parity" => (merge_lowering.clone(), merge_lowering.clone()),
        "query-triggered-writeback-lowering-parity" => {
            (writeback_lowering.clone(), writeback_lowering.clone())
        }
        "conflict-inspection-explicitness" => (merge_lowering.clone(), conflict_inspection.clone()),
        "unsupported-deletion-topology-merge-class" => (
            conflict_inspection.clone(),
            denied_conflict_inspection.clone(),
        ),
        "post-merge-inspection-explicitness" => {
            (merge_lowering.clone(), post_merge_inspection.clone())
        }
        "workflow-freshness-explicitness" => {
            (merge_lowering.clone(), preview_merge_lowering.clone())
        }
        "workflow-prediction-width-explicitness" => {
            (runtime_merge.clone(), conflict_inspection.clone())
        }
        "workflow-realized-width-explicitness" => (runtime_merge.clone(), merge_lowering.clone()),
        "workflow-rediscovery-zero-parity" => (merge_lowering.clone(), merge_lowering.clone()),
        other => panic!("unexpected workflow canonical row {other}"),
    };

    CanonicalCertificationRow {
        row_name: spec.row_name,
        perturbation_class: spec.perturbation_class,
        hostile_expectation: spec.hostile_expectation,
        parity_anchor: ParityAnchor::Control,
        control_lane: control_lane.clone(),
        hostile_lane,
        parity_lane: control_lane,
    }
}
