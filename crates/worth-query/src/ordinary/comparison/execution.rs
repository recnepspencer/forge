use super::{
    WorthQueryComparisonChange, WorthQueryComparisonCompletion, WorthQueryComparisonCorrespondence,
    WorthQueryComparisonCorrespondencePosture, WorthQueryComparisonJourneyCounters,
    WorthQueryComparisonNextAction, WorthQueryComparisonOutcome, WorthQueryComparisonRequest,
    WorthQueryComparisonStop, WorthQueryComparisonStopSource,
};
use crate::correspondence::{
    resolve_correspondence_evidence, CorrespondenceEvaluationRequest,
    StructuralCandidateDiscoveryPlan, StructuralCandidateOrderingContract,
};
use crate::ordinary::history::WorthQueryHistoricalPathDeclaration;
use crate::ordinary::read::current;
use crate::runtime::{WorthQueryReadResult, WorthQueryWorkspace};

impl WorthQueryComparisonRequest {
    pub fn run(self, workspace: &mut WorthQueryWorkspace) -> WorthQueryComparisonOutcome {
        let counters = WorthQueryComparisonJourneyCounters::validate_pair();
        let actual = workspace.snapshot_identity();
        if self.context.current_snapshot() != &actual || self.context.retained_snapshot() != &actual
        {
            return stopped(
                WorthQueryComparisonStopSource::StaleBasisPair,
                WorthQueryComparisonNextAction::RefreshBasisPair,
                "one or both structurally bound comparison bases are stale",
                counters,
            );
        }

        let counters = counters.execute_current();
        let left = match self
            .declaration
            .read
            .clone()
            .using(current())
            .run(workspace)
            .into_result()
        {
            Ok(completion) => completion.into_result(),
            Err(stop) => {
                return stopped(
                    WorthQueryComparisonStopSource::CurrentExecution,
                    WorthQueryComparisonNextAction::ResolveAuthority,
                    format!("left comparison query stopped: {stop:?}"),
                    counters,
                );
            }
        };

        let counters = counters.execute_historical();
        let historical =
            WorthQueryHistoricalPathDeclaration::retained_from_read(self.declaration.read);
        let (right, right_materialization) = match historical
            .using(self.context.retained_context())
            .run(workspace)
            .into_result()
        {
            Ok(completion) => completion.into_parts(),
            Err(stop) => {
                return stopped(
                    WorthQueryComparisonStopSource::HistoricalExecution,
                    WorthQueryComparisonNextAction::ResolveAuthority,
                    format!("right comparison query stopped: {stop:?}"),
                    counters,
                );
            }
        };

        match self.declaration.intent {
            super::WorthQueryComparisonIntent::Diff => {
                complete_diff(left, right, right_materialization, counters)
            }
            super::WorthQueryComparisonIntent::StructuralCorrespondence => {
                structural_correspondence(left, right, self.declaration.candidate_budget, counters)
            }
            super::WorthQueryComparisonIntent::Lineage => {
                lineage_correspondence(left, right, counters)
            }
        }
    }
}

fn complete_diff(
    left: WorthQueryReadResult,
    right: WorthQueryReadResult,
    right_materialization: crate::historical::HistoricalMaterializationPathMetadata,
    counters: WorthQueryComparisonJourneyCounters,
) -> WorthQueryComparisonOutcome {
    let change = if left.rows() == right.rows() {
        WorthQueryComparisonChange::Unchanged
    } else {
        WorthQueryComparisonChange::Changed
    };
    WorthQueryComparisonOutcome::Completed(WorthQueryComparisonCompletion::new(
        left,
        right,
        right_materialization,
        change,
        counters,
    ))
}

fn structural_correspondence(
    _left: WorthQueryReadResult,
    right: WorthQueryReadResult,
    candidate_budget: usize,
    counters: WorthQueryComparisonJourneyCounters,
) -> WorthQueryComparisonOutcome {
    let mut candidates = right
        .rows()
        .iter()
        .map(|row| row.identity().evidence_identity().as_str().to_string())
        .collect::<Vec<_>>();
    candidates.sort();
    candidates.dedup();
    resolve_correspondence(
        CorrespondenceEvaluationRequest::structural_only(
            candidates,
            StructuralCandidateDiscoveryPlan::IndexBackedBounded,
            candidate_budget,
            StructuralCandidateOrderingContract::StableFingerprintOrder,
        ),
        counters.resolve_correspondence(),
    )
}

fn lineage_correspondence(
    left: WorthQueryReadResult,
    right: WorthQueryReadResult,
    counters: WorthQueryComparisonJourneyCounters,
) -> WorthQueryComparisonOutcome {
    let ([left_row], [right_row]) = (left.rows(), right.rows()) else {
        return stopped(
            WorthQueryComparisonStopSource::CorrespondenceDenied,
            WorthQueryComparisonNextAction::NarrowCandidates,
            "authoritative lineage comparison requires exactly one row on each basis",
            counters,
        );
    };
    let request = CorrespondenceEvaluationRequest::lineage_only(
        left_row.identity().evidence_identity().as_str(),
        right_row.identity().evidence_identity().as_str(),
        StructuralCandidateDiscoveryPlan::IndexBackedBounded,
        1,
    );
    resolve_correspondence(request, counters.resolve_correspondence())
}

fn resolve_correspondence(
    request: CorrespondenceEvaluationRequest,
    counters: WorthQueryComparisonJourneyCounters,
) -> WorthQueryComparisonOutcome {
    match resolve_correspondence_evidence(request) {
        Ok(correspondence) if correspondence.outcome().as_denied().is_none() => {
            let posture = if correspondence.outcome().as_lineage_continuity().is_some() {
                WorthQueryComparisonCorrespondencePosture::AuthoritativeContinuity
            } else {
                WorthQueryComparisonCorrespondencePosture::Advisory
            };
            WorthQueryComparisonOutcome::Correspondence(WorthQueryComparisonCorrespondence::new(
                correspondence,
                posture,
                counters,
            ))
        }
        Ok(correspondence) => stopped(
            WorthQueryComparisonStopSource::CorrespondenceDenied,
            WorthQueryComparisonNextAction::NarrowCandidates,
            correspondence
                .outcome()
                .as_denied()
                .map(|denial| denial.reason())
                .unwrap_or("correspondence was denied"),
            counters,
        ),
        Err(error) => stopped(
            WorthQueryComparisonStopSource::CorrespondenceDenied,
            WorthQueryComparisonNextAction::ResolveAuthority,
            format!("correspondence resolution failed: {error:?}"),
            counters,
        ),
    }
}

fn stopped(
    source: WorthQueryComparisonStopSource,
    next_action: WorthQueryComparisonNextAction,
    reason: impl Into<String>,
    counters: WorthQueryComparisonJourneyCounters,
) -> WorthQueryComparisonOutcome {
    WorthQueryComparisonOutcome::Stopped(WorthQueryComparisonStop::new(
        source,
        next_action,
        reason,
        counters,
    ))
}
