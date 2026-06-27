use crate::workload_platform::evidence_lookup_index_product::EvidenceLookupIndexProduct;
use crate::workload_platform::evidence_lookup_plan_selection::{
    EvidenceLookupPlanQuerySurface, EvidenceLookupPlanRowOutcome, EvidenceLookupSelectedPlan,
    EvidenceLookupSelectedPlanRow, EvidenceLookupSelectedStrategyKind,
};

use super::counters::EvidenceLookupExecutionCounters;
use super::error::{EvidenceLookupExecutionError, EvidenceLookupExecutionErrorKind};
use super::outcome::EvidenceLookupExecutionOutcome;
use super::receipt::{EvidenceLookupExecutionReceipt, EvidenceLookupExecutionReceiptParts};
use super::request::EvidenceLookupExecutionRequest;

pub fn execute_evidence_lookup(
    request: &EvidenceLookupExecutionRequest<'_>,
) -> Result<EvidenceLookupExecutionReceipt, EvidenceLookupExecutionError> {
    let selected_plan = request.selected_plan();
    let index_product = request.index_product();
    validate_phase_chain(selected_plan, index_product)?;
    validate_query_artifact_scope(request)?;

    let selected_rows = selected_rows(selected_plan);
    let query_gate_outcome = query_gate_outcome(request, &selected_rows);
    let candidate_rows = if query_gate_outcome.is_some() {
        Vec::new()
    } else {
        candidate_rows(request)
    };
    let outcome = query_gate_outcome.unwrap_or_else(|| {
        classify_execution_outcome(selected_plan, &selected_rows, &candidate_rows)
    });
    let evidence_receipt_digests = match outcome {
        EvidenceLookupExecutionOutcome::IndexedHit
        | EvidenceLookupExecutionOutcome::IndexedMiss
        | EvidenceLookupExecutionOutcome::BoundedRebuild => candidate_rows
            .iter()
            .map(|row| row.evidence_identity().to_string())
            .collect(),
        EvidenceLookupExecutionOutcome::RequiredQuerySupport
        | EvidenceLookupExecutionOutcome::MissingProjectionConsumptionFact
        | EvidenceLookupExecutionOutcome::DeniedBeforeExecution
        | EvidenceLookupExecutionOutcome::CappedResidue => Vec::new(),
    };
    let counters = counters_for_outcome(
        request,
        selected_rows.len(),
        selected_plan.counters().selected_spatial_region_count(),
        candidate_rows.len(),
        outcome,
        index_product,
    );
    Ok(EvidenceLookupExecutionReceipt::from_parts(
        EvidenceLookupExecutionReceiptParts {
            selected_plan_digest: selected_plan.selected_plan_digest().to_string(),
            index_product_digest: index_product.index_product_digest().to_string(),
            spatial_touch_digest: selected_plan.spatial_touch_digest().to_string(),
            stage_receipt_digest: selected_plan.stage_receipt_digest().to_string(),
            evidence_ledger_basis_digest: index_product.evidence_ledger_basis_digest().to_string(),
            topology_support_digest: index_product.topology_support_digest().to_string(),
            topology_support_state:
                super::receipt::EvidenceLookupExecutionTopologySupportState::from_selected_plan(
                    selected_plan,
                ),
            query_support_digest: index_product.query_support_digest().to_string(),
            query_surface_contract_rows: index_product.query_surface_contract_rows().to_vec(),
            index_lifecycle_posture: index_product.lifecycle_posture(),
            index_disposal_posture: index_product.disposal_posture(),
            outcome,
            counters,
            evidence_receipt_digests,
        },
    ))
}

fn validate_phase_chain(
    selected_plan: &EvidenceLookupSelectedPlan,
    index_product: &EvidenceLookupIndexProduct,
) -> Result<(), EvidenceLookupExecutionError> {
    if selected_plan.selected_plan_digest() != index_product.selected_plan_digest() {
        return Err(EvidenceLookupExecutionError::new(
            EvidenceLookupExecutionErrorKind::PlanIndexDigestMismatch,
            "lookup execution requires an index product admitted from the same selected plan",
        ));
    }
    if selected_plan.spatial_touch_digest() != index_product.spatial_touch_digest() {
        return Err(EvidenceLookupExecutionError::new(
            EvidenceLookupExecutionErrorKind::SpatialTouchDigestMismatch,
            "lookup execution requires matching spatial touch authority across plan and index product",
        ));
    }
    if selected_plan.stage_receipt_digest() != index_product.stage_receipt_digest() {
        return Err(EvidenceLookupExecutionError::new(
            EvidenceLookupExecutionErrorKind::StageReceiptDigestMismatch,
            "lookup execution requires matching stage receipt identity across plan and index product",
        ));
    }
    Ok(())
}

fn validate_query_artifact_scope(
    request: &EvidenceLookupExecutionRequest<'_>,
) -> Result<(), EvidenceLookupExecutionError> {
    for family_identity in request.query_artifacts().projection_receipt_families() {
        let Some(row) = request
            .selected_plan()
            .rows()
            .iter()
            .find(|row| row.family_identity() == family_identity.as_str())
        else {
            return Err(EvidenceLookupExecutionError::new(
                EvidenceLookupExecutionErrorKind::UnexpectedExecutionQueryArtifactFamily,
                family_identity.as_str(),
            ));
        };
        if !row
            .query_posture()
            .requires_projection_consumption_receipt()
        {
            return Err(EvidenceLookupExecutionError::new(
                EvidenceLookupExecutionErrorKind::UnexpectedExecutionQueryArtifactFamily,
                family_identity.as_str(),
            ));
        }
    }
    Ok(())
}

fn selected_rows(
    selected_plan: &EvidenceLookupSelectedPlan,
) -> Vec<&EvidenceLookupSelectedPlanRow> {
    selected_plan
        .rows()
        .iter()
        .filter(|row| row.outcome() == EvidenceLookupPlanRowOutcome::Selected)
        .collect()
}

fn candidate_rows<'a>(
    request: &'a EvidenceLookupExecutionRequest<'_>,
) -> Vec<&'a crate::workload_platform::evidence_ledger::WorkloadEvidenceRow> {
    request
        .index_product()
        .rows()
        .iter()
        .filter(|row| row.stage() == request.selected_plan().stage())
        .collect()
}

fn query_gate_outcome(
    request: &EvidenceLookupExecutionRequest<'_>,
    selected_rows: &[&EvidenceLookupSelectedPlanRow],
) -> Option<EvidenceLookupExecutionOutcome> {
    for row in selected_rows {
        if row.query_posture().surface()
            != EvidenceLookupPlanQuerySurface::ProjectionConsumptionReceipt
        {
            continue;
        }
        let Some(expected_fact_family) = row.query_posture().projection_fact_family() else {
            return Some(EvidenceLookupExecutionOutcome::DeniedBeforeExecution);
        };
        let Some(artifact) = request
            .query_artifacts()
            .projection_receipt(row.family_identity())
        else {
            return Some(EvidenceLookupExecutionOutcome::RequiredQuerySupport);
        };
        if artifact.fact_family() != expected_fact_family
            || artifact.receipt().receipt_digest().is_empty()
        {
            return Some(EvidenceLookupExecutionOutcome::MissingProjectionConsumptionFact);
        }
    }
    None
}

fn classify_execution_outcome(
    selected_plan: &EvidenceLookupSelectedPlan,
    selected_rows: &[&EvidenceLookupSelectedPlanRow],
    candidate_rows: &[&crate::workload_platform::evidence_ledger::WorkloadEvidenceRow],
) -> EvidenceLookupExecutionOutcome {
    if selected_rows.is_empty() {
        if selected_plan
            .rows()
            .iter()
            .any(|row| row.outcome() == EvidenceLookupPlanRowOutcome::RequiredQueryPosture)
        {
            return EvidenceLookupExecutionOutcome::RequiredQuerySupport;
        }
        if selected_plan
            .rows()
            .iter()
            .any(|row| row.outcome() == EvidenceLookupPlanRowOutcome::Denied)
        {
            return EvidenceLookupExecutionOutcome::DeniedBeforeExecution;
        }
        if selected_plan
            .rows()
            .iter()
            .any(|row| row.outcome() == EvidenceLookupPlanRowOutcome::CappedResidue)
        {
            return EvidenceLookupExecutionOutcome::CappedResidue;
        }
        return EvidenceLookupExecutionOutcome::IndexedMiss;
    }
    let has_bounded_dense = selected_rows.iter().any(|row| {
        row.strategy().is_some_and(|strategy| {
            strategy.kind() == EvidenceLookupSelectedStrategyKind::BoundedDenseIndexedLookupPlan
        })
    });
    if has_bounded_dense {
        return EvidenceLookupExecutionOutcome::BoundedRebuild;
    }
    if candidate_rows.is_empty() {
        return EvidenceLookupExecutionOutcome::IndexedMiss;
    }
    EvidenceLookupExecutionOutcome::IndexedHit
}

fn counters_for_outcome(
    request: &EvidenceLookupExecutionRequest<'_>,
    selected_family_count: usize,
    selected_region_count: usize,
    evidence_candidate_count: usize,
    outcome: EvidenceLookupExecutionOutcome,
    index_product: &EvidenceLookupIndexProduct,
) -> EvidenceLookupExecutionCounters {
    let evidence_access_allowed = matches!(
        outcome,
        EvidenceLookupExecutionOutcome::IndexedHit
            | EvidenceLookupExecutionOutcome::IndexedMiss
            | EvidenceLookupExecutionOutcome::BoundedRebuild
    );
    let indexed_hit_count = usize::from(outcome == EvidenceLookupExecutionOutcome::IndexedHit);
    let indexed_miss_count = usize::from(outcome == EvidenceLookupExecutionOutcome::IndexedMiss);
    EvidenceLookupExecutionCounters::new(
        selected_family_count,
        selected_region_count,
        if evidence_access_allowed {
            evidence_candidate_count
        } else {
            0
        },
        if evidence_access_allowed {
            evidence_candidate_count
        } else {
            0
        },
        if evidence_access_allowed {
            index_product.counters().selected_basis_row_count()
        } else {
            0
        },
        if evidence_access_allowed {
            index_product.counters().resident_byte_count()
        } else {
            0
        },
        indexed_hit_count,
        indexed_miss_count,
        0,
        request.query_artifacts().projection_receipt_count(),
    )
}
