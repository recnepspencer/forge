use super::{
    UiInoperableIntentCandidate, UiIntentAffinityPosture, UiIntentOperabilityDecision,
    UiIntentOperabilityDecisionInput, UiIntentOperabilityOutcome, UiIntentOperabilityProof,
};

pub(crate) fn evaluate_intent_operability(
    candidate: super::super::payload::UiPreparedIntentPayload,
    generation: &crate::runtime::WorthUiActiveApplicationGenerationIdentity,
    mounted: &crate::mounting::WorthUiMountedSessionState,
) -> UiIntentOperabilityOutcome {
    let basis = candidate.operability_basis();
    let decision = UiIntentOperabilityDecision::new(UiIntentOperabilityDecisionInput {
        contract_identity: basis.contract_identity().into(),
        support: basis.support(),
        mutability: basis.mutability(),
        readiness: basis.readiness(),
        occupancy: basis.occupancy().posture(),
        policy: basis.policy(),
        affinity: affinity(&candidate, generation, mounted),
        confirmation: basis.confirmation(),
        selected_dependencies_visited: 7,
    });
    if decision.is_operable() {
        UiIntentOperabilityOutcome::Operable(UiIntentOperabilityProof::new(candidate, decision))
    } else {
        UiIntentOperabilityOutcome::Inoperable(UiInoperableIntentCandidate::new(
            candidate, decision,
        ))
    }
}

fn affinity(
    candidate: &super::super::payload::UiPreparedIntentPayload,
    current: &crate::runtime::WorthUiActiveApplicationGenerationIdentity,
    mounted: &crate::mounting::WorthUiMountedSessionState,
) -> UiIntentAffinityPosture {
    let basis = candidate.input_basis();
    if basis.generation().session_identity() != current.session_identity() {
        return UiIntentAffinityPosture::WrongWorld;
    }
    if basis.generation().prepared_generation() != current.prepared_generation()
        || mounted.has_active_presentation_attempt()
        || mounted.view().current_frame() != Some(basis.publication_frame())
    {
        return UiIntentAffinityPosture::RebindRequired;
    }
    if crate::runtime::interaction::targeting::require_current_target(mounted, basis.target())
        .is_err()
    {
        return UiIntentAffinityPosture::Stale;
    }
    UiIntentAffinityPosture::Current
}
