use std::sync::Arc;

use super::WorthQueryPrimaryGraphProvider;
use crate::domain_computation::{
    WorthQueryBoundInvariantExecutionView, WorthQueryInvariantExecutionDenialKind,
    WorthQueryInvariantExecutionFailure, WorthQueryInvariantExecutionProvider,
    WorthQueryInvariantProviderVerdict, WorthQueryInvariantStateLoadAdmission,
    WorthQueryInvariantStateLoadEvidence, WorthQueryInvariantStateLoadRequestView,
    WorthQueryInvariantStateLocator, WorthQueryInvariantStructuralCounters,
    WorthQueryInvariantVerdictAdmission, WorthQueryInvariantVerdictEvidence,
    WorthQueryProviderSessionView,
};

impl WorthQueryInvariantExecutionProvider for Arc<WorthQueryPrimaryGraphProvider> {
    fn load_invariant_state(
        &self,
        session: WorthQueryProviderSessionView<'_>,
        request: WorthQueryInvariantStateLoadRequestView<'_>,
        admission: WorthQueryInvariantStateLoadAdmission,
    ) -> Result<WorthQueryInvariantStateLoadEvidence, WorthQueryInvariantExecutionFailure> {
        let sessions = self
            .sessions
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let overlay_identity = sessions
            .session_overlays
            .get(session.identity())
            .ok_or_else(|| closure_failure("provider session has no staged overlay"))?;
        let overlay = sessions
            .overlays
            .get(overlay_identity)
            .ok_or_else(|| closure_failure("provider session overlay is unavailable"))?;
        let attempt = sessions
            .application_attempts
            .get(session.identity())
            .ok_or_else(|| closure_failure("provider session has no application attempt"))?;
        let expected = expected_locators(overlay)?;
        if attempt.expected_steps.len() != overlay.facts.len()
            || request.locators() != expected.as_slice()
        {
            return Err(closure_failure(
                "invariant state-load plan does not close over the proposed effects",
            ));
        }
        admission.admit(
            format!("primary-invariant-load:{overlay_identity}"),
            expected.clone(),
            WorthQueryInvariantStructuralCounters::new(
                expected.len(),
                expected.len() as u64,
                overlay.facts.len() as u64,
            ),
        )
    }

    fn execute_invariant(
        &self,
        session: WorthQueryProviderSessionView<'_>,
        execution: WorthQueryBoundInvariantExecutionView<'_>,
        admission: WorthQueryInvariantVerdictAdmission,
    ) -> Result<WorthQueryInvariantProviderVerdict, WorthQueryInvariantExecutionFailure> {
        let sessions = self
            .sessions
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let overlay_identity = sessions
            .session_overlays
            .get(session.identity())
            .ok_or_else(|| closure_failure("invariant execution lost its staged overlay"))?;
        let overlay = sessions
            .overlays
            .get(overlay_identity)
            .ok_or_else(|| closure_failure("invariant execution overlay is unavailable"))?;
        let attempt = sessions
            .application_attempts
            .get(session.identity())
            .ok_or_else(|| closure_failure("invariant execution lost its application attempt"))?;
        let expected = expected_locators(overlay)?;
        let load_evidence = execution.state_load_evidence();
        if attempt.expected_steps.len() != overlay.facts.len()
            || execution.state_load_plan().locators() != expected.as_slice()
            || load_evidence.loaded_fact_locators() != expected.as_slice()
            || load_evidence.physical_load_evidence()
                != format!("primary-invariant-load:{overlay_identity}")
        {
            return Err(closure_failure(
                "invariant execution evidence differs from the exact proposed state",
            ));
        }
        let evidence = WorthQueryInvariantVerdictEvidence::new(
            execution.requirement().slot(),
            "primary-application-installed-invariant",
            load_evidence.identity(),
            expected.len() as u64,
        )?;
        admission.passed(evidence)
    }
}

fn expected_locators(
    overlay: &super::WorthQueryPrimaryGraphOverlay,
) -> Result<Vec<WorthQueryInvariantStateLocator>, WorthQueryInvariantExecutionFailure> {
    let mut expected = overlay
        .facts
        .iter()
        .map(|fact| {
            WorthQueryInvariantStateLocator::new("application-proposed-state", fact.identity())
        })
        .collect::<Result<Vec<_>, _>>()?;
    expected.sort();
    let original_len = expected.len();
    expected.dedup();
    if expected.len() != original_len {
        return Err(closure_failure(
            "proposed effects contain duplicate invariant fact identities",
        ));
    }
    Ok(expected)
}

fn closure_failure(detail: &'static str) -> WorthQueryInvariantExecutionFailure {
    WorthQueryInvariantExecutionFailure::new(
        WorthQueryInvariantExecutionDenialKind::StateLoadClosureMismatch,
        detail,
    )
}
