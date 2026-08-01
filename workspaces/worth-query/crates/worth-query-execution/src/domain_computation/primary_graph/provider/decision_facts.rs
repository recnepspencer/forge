use std::sync::Arc;

use super::WorthQueryPrimaryGraphProvider;
use crate::domain_computation::{
    WorthQueryDecisionFactAdmission, WorthQueryDecisionFactComparisonAdmission,
    WorthQueryDecisionFactComparisonEvidence, WorthQueryDecisionFactEvidence,
    WorthQueryDecisionFactEvidenceView, WorthQueryDecisionFactProvider,
    WorthQueryDecisionFactRequestView, WorthQueryDecisionReadSetFailure,
    WorthQueryProviderSessionView,
};

impl WorthQueryDecisionFactProvider for Arc<WorthQueryPrimaryGraphProvider> {
    fn observe_decision_fact(
        &self,
        session: WorthQueryProviderSessionView<'_>,
        request: WorthQueryDecisionFactRequestView<'_>,
        admission: WorthQueryDecisionFactAdmission,
    ) -> Result<WorthQueryDecisionFactEvidence, WorthQueryDecisionReadSetFailure> {
        let sessions = self
            .sessions
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let attempt = sessions
            .application_attempts
            .get(session.identity())
            .ok_or_else(provider_rejected)?;
        if !attempt.facts.contains_key(request.locator().identity()) {
            return Err(provider_rejected());
        }
        admission.observe(format!(
            "application-observed:{}",
            request.locator().identity()
        ))
    }

    fn compare_decision_fact(
        &self,
        session: WorthQueryProviderSessionView<'_>,
        evidence: WorthQueryDecisionFactEvidenceView<'_>,
        admission: WorthQueryDecisionFactComparisonAdmission,
    ) -> Result<WorthQueryDecisionFactComparisonEvidence, WorthQueryDecisionReadSetFailure> {
        let (fact, branch_id) = {
            let sessions = self
                .sessions
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            sessions
                .application_attempts
                .get(session.identity())
                .and_then(|attempt| {
                    attempt
                        .facts
                        .get(evidence.locator().identity())
                        .cloned()
                        .map(|fact| (fact, attempt.branch_id.clone()))
                })
                .ok_or_else(provider_rejected)?
        };
        let fresh = self.graph.with_runtime_mut(|runtime| {
            let snapshot = runtime.snapshots().snapshot();
            let fresh =
                snapshot.branch_id == branch_id && fact.remains_equal_in(runtime, &snapshot);
            runtime.snapshots().release_snapshot(&snapshot);
            fresh
        });
        admission.observe_current_version(if fresh {
            evidence.physical_version_evidence().to_owned()
        } else {
            format!("application-stale:{}", evidence.locator().identity())
        })
    }
}

fn provider_rejected() -> WorthQueryDecisionReadSetFailure {
    WorthQueryDecisionReadSetFailure::new(
        crate::domain_computation::WorthQueryDecisionReadSetDenialKind::ProviderRejected,
        "primary provider has no exact session-owned application fact",
    )
}
