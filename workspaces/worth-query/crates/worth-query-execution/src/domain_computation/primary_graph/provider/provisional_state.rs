use std::sync::Arc;

use super::{WorthQueryPrimaryGraphOverlay, WorthQueryPrimaryGraphProvider};
use crate::domain_computation::{
    WorthQueryProposedFact, WorthQueryProposedFactOrigin, WorthQueryProviderSessionView,
    WorthQueryProvisionalEffectAction, WorthQueryProvisionalEffectProgramView,
    WorthQueryProvisionalFailure, WorthQueryProvisionalGraphProvider,
    WorthQueryProvisionalOverlayAdmission, WorthQueryProvisionalOverlayEvidence,
    WorthQueryProvisionalOverlayEvidenceView,
};

impl WorthQueryProvisionalGraphProvider for Arc<WorthQueryPrimaryGraphProvider> {
    fn stage_provisional_overlay(
        &self,
        session: WorthQueryProviderSessionView<'_>,
        program: WorthQueryProvisionalEffectProgramView<'_>,
        admission: WorthQueryProvisionalOverlayAdmission,
    ) -> Result<WorthQueryProvisionalOverlayEvidence, WorthQueryProvisionalFailure> {
        let branch_id = {
            let sessions = self
                .sessions
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            let expected = sessions
                .application_attempts
                .get(session.identity())
                .ok_or_else(|| {
                    WorthQueryProvisionalFailure::invalid_program(
                        "provider session has no registered application attempt",
                    )
                })?;
            if expected.expected_steps != program.steps() {
                return Err(WorthQueryProvisionalFailure::invalid_program(
                    "lowered provider program differs from the registered application effects",
                ));
            }
            expected.branch_id.clone()
        };
        let facts = program
            .steps()
            .iter()
            .map(|step| proposed_fact(step.action()))
            .collect::<Result<Vec<_>, _>>()?;
        let mut sessions = self
            .sessions
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        sessions.next_overlay += 1;
        let identity = format!(
            "primary-overlay:{}:{}",
            program.generation(),
            sessions.next_overlay
        );
        sessions.overlays.insert(
            identity.clone(),
            WorthQueryPrimaryGraphOverlay {
                branch_id,
                facts: facts.clone(),
            },
        );
        sessions
            .session_overlays
            .insert(session.identity().to_owned(), identity.clone());
        admission.admit(identity, facts)
    }

    fn discard_provisional_overlay(
        &self,
        evidence: WorthQueryProvisionalOverlayEvidenceView<'_>,
    ) -> Result<(), WorthQueryProvisionalFailure> {
        let mut sessions = self
            .sessions
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        sessions
            .overlays
            .remove(evidence.physical_overlay_identity());
        sessions
            .session_overlays
            .retain(|_, overlay| overlay != evidence.physical_overlay_identity());
        Ok(())
    }
}

fn proposed_fact(
    action: &WorthQueryProvisionalEffectAction,
) -> Result<WorthQueryProposedFact, WorthQueryProvisionalFailure> {
    let (identity, origin, value) = match action {
        WorthQueryProvisionalEffectAction::Create { symbolic_identity } => (
            symbolic_identity.as_ref(),
            WorthQueryProposedFactOrigin::StagedCreation,
            "created",
        ),
        WorthQueryProvisionalEffectAction::Replace { target_identity } => (
            target_identity.as_ref(),
            WorthQueryProposedFactOrigin::StagedReplacement,
            "replaced",
        ),
        WorthQueryProvisionalEffectAction::Retire { target_identity } => (
            target_identity.as_ref(),
            WorthQueryProposedFactOrigin::StagedRetirement,
            "retired",
        ),
        WorthQueryProvisionalEffectAction::DeriveView { view_identity } => (
            view_identity.as_ref(),
            WorthQueryProposedFactOrigin::DerivedProvisionalView,
            "derived",
        ),
    };
    WorthQueryProposedFact::new(identity, origin, value)
}
