use worth_proof::TransitionOutcome;
use serde::Serialize;

use super::declaration_candidate::PlacementDeclarationCandidate;
use super::declaration_classification::{
    classify_declaration_placement, PlacementClassificationOutcome,
};
use super::placement_category::WorkerPlacementCategory;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlacementDeclarationSummary {
    pub id: String,
    pub signal_kind: String,
    pub declaration_origin: String,
    pub category: WorkerPlacementCategory,
    pub proof_stage: String,
    pub outcome: String,
    pub reason: String,
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerPlacementSummary {
    pub total_declaration_count: u64,
    pub worker_executable_count: u64,
    pub main_thread_hosted_count: u64,
    pub unavailable_count: u64,
    pub denied_count: u64,
    pub raw_proof_count: u64,
    pub classified_outcome_count: u64,
    pub declarations: Vec<PlacementDeclarationSummary>,
}

pub(crate) fn project_worker_placement_summary(
    declarations: Vec<PlacementDeclarationCandidate>,
) -> WorkerPlacementSummary {
    let mut summary = WorkerPlacementSummary::default();
    for declaration in declarations {
        project_classified_declaration_into_summary(
            &mut summary,
            classify_declaration_placement(declaration),
        );
    }
    summary
}

fn project_classified_declaration_into_summary(
    summary: &mut WorkerPlacementSummary,
    outcome: PlacementClassificationOutcome,
) {
    summary.total_declaration_count = summary.total_declaration_count.saturating_add(1);
    summary.raw_proof_count = summary.raw_proof_count.saturating_add(1);
    summary.classified_outcome_count = summary.classified_outcome_count.saturating_add(1);
    append_classification_outcome_projection(summary, outcome);
}

fn append_classification_outcome_projection(
    summary: &mut WorkerPlacementSummary,
    outcome: PlacementClassificationOutcome,
) {
    match outcome {
        TransitionOutcome::Success(classified) => {
            increment_placement_category_count(summary, classified.category);
            let raw = classified.raw.payload();
            summary.declarations.push(PlacementDeclarationSummary {
                id: raw.id().to_owned(),
                signal_kind: raw.signal_kind().to_owned(),
                declaration_origin: raw.declaration_origin().to_owned(),
                category: classified.category,
                proof_stage: "placementClassified".to_owned(),
                outcome: "success".to_owned(),
                reason: classified.reason,
            });
        }
        TransitionOutcome::Denied(denial) => {
            summary.denied_count = summary.denied_count.saturating_add(1);
            increment_placement_category_count(summary, denial.category);
            let raw = denial.raw.payload();
            summary.declarations.push(PlacementDeclarationSummary {
                id: raw.id().to_owned(),
                signal_kind: raw.signal_kind().to_owned(),
                declaration_origin: raw.declaration_origin().to_owned(),
                category: denial.category,
                proof_stage: "rawDenied".to_owned(),
                outcome: "denied".to_owned(),
                reason: denial.reason,
            });
        }
        TransitionOutcome::Deferred(impossible)
        | TransitionOutcome::Stale(impossible)
        | TransitionOutcome::RebindRequired(impossible)
        | TransitionOutcome::Failed(impossible) => match impossible {},
    }
}

fn increment_placement_category_count(
    summary: &mut WorkerPlacementSummary,
    category: WorkerPlacementCategory,
) {
    match category {
        WorkerPlacementCategory::WorkerExecutable => {
            summary.worker_executable_count = summary.worker_executable_count.saturating_add(1);
        }
        WorkerPlacementCategory::MainThreadHosted => {
            summary.main_thread_hosted_count = summary.main_thread_hosted_count.saturating_add(1);
        }
        WorkerPlacementCategory::Unavailable => {
            summary.unavailable_count = summary.unavailable_count.saturating_add(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::core::WebSignalKind;
    use crate::runtime::placement::declaration_candidate::PlacementDeclarationOrigin;

    fn computed_declaration_candidate(
        origin: PlacementDeclarationOrigin,
    ) -> PlacementDeclarationCandidate {
        PlacementDeclarationCandidate {
            id: "derived".to_owned(),
            signal_kind: Some(WebSignalKind::Computed),
            origin,
            has_live_callback: false,
            callback_runtime_read_count: 0,
            host_capability_read_count: 0,
            is_unavailable: false,
        }
    }

    #[test]
    fn summary_counts_success_and_denial_outcomes() {
        let mut callback =
            computed_declaration_candidate(PlacementDeclarationOrigin::CallbackSignalTracked);
        callback.id = "callback".to_owned();
        callback.has_live_callback = true;

        let summary = project_worker_placement_summary(vec![
            computed_declaration_candidate(PlacementDeclarationOrigin::ExprSpec),
            callback,
        ]);

        assert_eq!(summary.total_declaration_count, 2);
        assert_eq!(summary.worker_executable_count, 1);
        assert_eq!(summary.main_thread_hosted_count, 1);
        assert_eq!(summary.denied_count, 1);
        assert_eq!(summary.raw_proof_count, 2);
        assert_eq!(summary.classified_outcome_count, 2);
        let denied = summary
            .declarations
            .iter()
            .find(|declaration| declaration.outcome == "denied")
            .expect("denied declaration summary exists");
        assert_eq!(denied.signal_kind, "computed");
        assert_eq!(denied.declaration_origin, "callbackSignalTracked");
    }
}
