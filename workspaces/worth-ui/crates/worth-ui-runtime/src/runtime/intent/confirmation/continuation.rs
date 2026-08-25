use std::sync::Arc;

use super::super::UiIntentAttemptLineage;
use super::state::{
    UiIntentConfirmationSlotState, UiIntentConfirmationState, UiIntentConfirmationTerminal,
    UiIntentConfirmationTerminalKind,
};
use super::{
    UiIntentConfirmationCancellationReason, UiIntentConfirmationChallenge,
    UiIntentConfirmationLookupCost, UiIntentConfirmationStop, UiIntentConfirmationStopReason,
    UiIntentConfirmationTimeBasisKind,
};

#[must_use]
pub enum UiIntentConfirmationContinuation {
    AdmissionReady(UiConfirmedIntentCandidate),
    Stopped(UiIntentConfirmationStop),
}

#[must_use]
pub struct UiConfirmedIntentCandidate {
    candidate: super::super::payload::UiPreparedIntentPayload,
    confirmation_decision: super::super::operability::UiIntentOperabilityDecision,
    lineage: UiIntentAttemptLineage,
}

pub(crate) struct UiIntentConfirmationContinuationContext<'state> {
    pub(crate) catalog: &'state crate::declaration::UiIntentCatalog,
    pub(crate) definitions: &'state crate::capability::FrozenIntentDefinitionCapabilities,
    pub(crate) generation: &'state crate::runtime::WorthUiActiveApplicationGenerationIdentity,
    pub(crate) mounted: &'state crate::mounting::WorthUiMountedSessionState,
    pub(crate) application_facts: &'state super::super::payload::UiIntentApplicationFactState,
    pub(crate) occupancy: &'state super::super::operability::UiIntentOccupancyState,
}

pub(crate) fn continue_confirmation(
    state: &mut UiIntentConfirmationState,
    route: super::super::UiResolvedConfirmationIntentRoute,
    context: UiIntentConfirmationContinuationContext<'_>,
) -> UiIntentConfirmationContinuation {
    let (_, route_definition, route_declaration, source, _route_resolution, _) = route.into_parts();
    let declaration = route_declaration.identity().as_str();
    let (pending, terminal, inspected) = matching_slots(state, declaration, route_definition);
    if pending.len() > 1 {
        settle_ambiguous(state, &pending);
        state.record_stopped();
        return stopped(
            UiIntentConfirmationStopReason::AmbiguousPendingChallenges {
                declaration: declaration.into(),
                observed: pending.len(),
            },
            inspected,
        );
    }
    let Some(slot) = pending.first().copied() else {
        state.record_stopped();
        return terminal_stop(state, declaration, terminal, inspected);
    };
    let challenge = take_pending(state, slot);
    let stop = validate_challenge(
        &challenge,
        route_definition,
        &route_declaration,
        &source,
        &context,
    );
    if let Some(reason) = stop {
        let terminal_kind = if matches!(reason, UiIntentConfirmationStopReason::Expired { .. }) {
            state.record_expired();
            UiIntentConfirmationTerminalKind::Expired
        } else {
            UiIntentConfirmationTerminalKind::Stopped
        };
        set_terminal(state, slot, &challenge, terminal_kind);
        state.record_stopped();
        return stopped(reason, inspected);
    }
    let UiIntentConfirmationChallenge {
        candidate,
        decision,
        lineage,
        slot_identity,
        ..
    } = challenge;
    let terminal = UiIntentConfirmationTerminal {
        declaration: candidate.declaration_identity().into(),
        definition: candidate.definition_id(),
        lineage,
        slot_identity,
        kind: UiIntentConfirmationTerminalKind::Continued,
    };
    state.slots[slot].state = UiIntentConfirmationSlotState::Terminal(terminal);
    state.record_continued();
    UiIntentConfirmationContinuation::AdmissionReady(UiConfirmedIntentCandidate {
        candidate,
        confirmation_decision: decision,
        lineage,
    })
}

fn matching_slots(
    state: &UiIntentConfirmationState,
    declaration: &str,
    definition: crate::capability::UiIntentId,
) -> (Vec<usize>, Vec<usize>, usize) {
    let mut pending = Vec::new();
    let mut terminal = Vec::new();
    for (index, slot) in state.slots.iter().enumerate() {
        match &slot.state {
            UiIntentConfirmationSlotState::Pending(challenge)
                if challenge.candidate.declaration_identity() == declaration
                    && challenge.candidate.definition_id() == definition =>
            {
                pending.push(index);
            }
            UiIntentConfirmationSlotState::Terminal(marker)
                if marker.declaration.as_ref() == declaration
                    && marker.definition == definition =>
            {
                terminal.push(index);
            }
            UiIntentConfirmationSlotState::Vacant
            | UiIntentConfirmationSlotState::Pending(_)
            | UiIntentConfirmationSlotState::Terminal(_) => {}
        }
    }
    (pending, terminal, state.slots.len())
}

fn settle_ambiguous(state: &mut UiIntentConfirmationState, slots: &[usize]) {
    for slot in slots {
        let challenge = take_pending(state, *slot);
        set_terminal(
            state,
            *slot,
            &challenge,
            UiIntentConfirmationTerminalKind::Cancelled(
                UiIntentConfirmationCancellationReason::AmbiguousContinuation,
            ),
        );
        drop(challenge);
    }
    state.record_cancelled(slots.len());
}

fn take_pending(
    state: &mut UiIntentConfirmationState,
    slot: usize,
) -> UiIntentConfirmationChallenge {
    let previous = core::mem::replace(
        &mut state.slots[slot].state,
        UiIntentConfirmationSlotState::Vacant,
    );
    let UiIntentConfirmationSlotState::Pending(challenge) = previous else {
        unreachable!("selected confirmation slot is pending")
    };
    challenge
}

fn set_terminal(
    state: &mut UiIntentConfirmationState,
    slot: usize,
    challenge: &UiIntentConfirmationChallenge,
    kind: UiIntentConfirmationTerminalKind,
) {
    state.slots[slot].state = UiIntentConfirmationSlotState::Terminal(
        UiIntentConfirmationTerminal::from_challenge(challenge, kind),
    );
}

fn terminal_stop(
    state: &mut UiIntentConfirmationState,
    declaration: &str,
    terminal: Vec<usize>,
    inspected: usize,
) -> UiIntentConfirmationContinuation {
    if terminal.len() != 1 {
        return stopped(
            if terminal.is_empty() {
                UiIntentConfirmationStopReason::NoPendingChallenge {
                    declaration: declaration.into(),
                }
            } else {
                UiIntentConfirmationStopReason::AmbiguousPendingChallenges {
                    declaration: declaration.into(),
                    observed: terminal.len(),
                }
            },
            inspected,
        );
    }
    let slot = terminal[0];
    let previous = core::mem::replace(
        &mut state.slots[slot].state,
        UiIntentConfirmationSlotState::Vacant,
    );
    let UiIntentConfirmationSlotState::Terminal(marker) = previous else {
        unreachable!("selected confirmation terminal is present")
    };
    state.record_replay();
    let reason = match marker.kind {
        UiIntentConfirmationTerminalKind::Continued => {
            UiIntentConfirmationStopReason::AlreadyContinued
        }
        UiIntentConfirmationTerminalKind::Cancelled(reason) => {
            UiIntentConfirmationStopReason::LifecycleCancelled(reason)
        }
        UiIntentConfirmationTerminalKind::Expired => UiIntentConfirmationStopReason::AlreadyStopped,
        UiIntentConfirmationTerminalKind::Stopped => UiIntentConfirmationStopReason::AlreadyStopped,
    };
    let _terminal_identity = (marker.lineage, marker.slot_identity);
    stopped(reason, inspected)
}

fn validate_challenge(
    challenge: &UiIntentConfirmationChallenge,
    route_definition: crate::capability::UiIntentId,
    route_declaration: &Arc<crate::declaration::UiCanonicalIntentDeclaration>,
    source: &crate::runtime::interaction::UiSemanticInteraction,
    context: &UiIntentConfirmationContinuationContext<'_>,
) -> Option<UiIntentConfirmationStopReason> {
    let candidate = &challenge.candidate;
    let candidate_generation = candidate.input_basis().generation();
    if candidate_generation.session_identity() != context.generation.session_identity() {
        return Some(UiIntentConfirmationStopReason::ApplicationWorldChanged);
    }
    if candidate_generation.prepared_generation() != context.generation.prepared_generation() {
        return Some(UiIntentConfirmationStopReason::ApplicationGenerationChanged);
    }
    if source.generation().session_identity() != context.generation.session_identity() {
        return Some(UiIntentConfirmationStopReason::ApplicationWorldChanged);
    }
    if source.generation().prepared_generation() != context.generation.prepared_generation() {
        return Some(UiIntentConfirmationStopReason::ApplicationGenerationChanged);
    }
    if route_definition != candidate.definition_id()
        || !Arc::ptr_eq(route_declaration, candidate.declaration_reference())
    {
        return Some(UiIntentConfirmationStopReason::ConfirmationRouteChanged);
    }
    let observed = match source.time_basis() {
        worth_ui_host_contract::UiHostObservationTimeBasis::HostMonotonicMillis(millis) => millis,
        worth_ui_host_contract::UiHostObservationTimeBasis::HostWallClockMicros(_) => {
            return Some(UiIntentConfirmationStopReason::MonotonicTimeRequired {
                observed: UiIntentConfirmationTimeBasisKind::HostWallClock,
            })
        }
        worth_ui_host_contract::UiHostObservationTimeBasis::PresentationRelativeTick(_) => {
            return Some(UiIntentConfirmationStopReason::MonotonicTimeRequired {
                observed: UiIntentConfirmationTimeBasisKind::PresentationRelative,
            })
        }
    };
    if observed < challenge.issued_at_millis {
        return Some(UiIntentConfirmationStopReason::MonotonicTimeRegressed {
            issued_at_millis: challenge.issued_at_millis,
            observed_millis: observed,
        });
    }
    if observed > challenge.expires_at_millis {
        return Some(UiIntentConfirmationStopReason::Expired {
            expires_at_millis: challenge.expires_at_millis,
            observed_millis: observed,
        });
    }
    let current_frame = context.mounted.view().current_frame();
    if source.target().frame_relation()
        != crate::runtime::interaction::UiPresentedTargetFrameRelation::Current
        || current_frame != Some(source.target().presentation().frame())
    {
        return Some(UiIntentConfirmationStopReason::ConfirmationPresentationStale);
    }
    if let Err(denial) = crate::runtime::interaction::targeting::admit_current_target(
        context.mounted,
        source.target(),
    ) {
        return Some(UiIntentConfirmationStopReason::ConfirmationTargetChanged(
            denial,
        ));
    }
    if source.target().presentation().frame() == candidate.input_basis().publication_frame() {
        return Some(UiIntentConfirmationStopReason::ConfirmationNotPresented);
    }
    let affinity = match crate::runtime::interaction::targeting::admit_current_target_incarnation(
        context.mounted,
        candidate.input_basis().target(),
    ) {
        Ok(affinity) => affinity,
        Err(denial) => return Some(UiIntentConfirmationStopReason::TargetChanged(denial)),
    };
    if affinity.graph_node() != candidate.graph_node()
        || !product_route_is_current(candidate, context.catalog, context.definitions)
    {
        return Some(UiIntentConfirmationStopReason::ProductRouteChanged);
    }
    if !candidate.payload_inputs_are_current(
        context.mounted,
        context.application_facts,
        context.generation,
    ) {
        return Some(UiIntentConfirmationStopReason::PayloadInputChanged);
    }
    match candidate.operability_dependencies_are_current(
        context.mounted,
        context.application_facts,
        context.generation,
    ) {
        Ok(()) => {}
        Err(super::super::operability::UiIntentOperabilityDependencyDrift::DeclaredDependency) => {
            return Some(UiIntentConfirmationStopReason::OperabilityDependencyChanged)
        }
        Err(super::super::operability::UiIntentOperabilityDependencyDrift::Policy) => {
            return Some(UiIntentConfirmationStopReason::PolicyChanged)
        }
        Err(super::super::operability::UiIntentOperabilityDependencyDrift::Confirmation) => {
            return Some(UiIntentConfirmationStopReason::ConfirmationPolicyChanged)
        }
    }
    if challenge.decision.confirmation().required_policy_identity()
        != Some(challenge.policy_identity.as_ref())
    {
        return Some(UiIntentConfirmationStopReason::ConfirmationPolicyChanged);
    }
    if !context
        .occupancy
        .is_current_observation(candidate.operability_basis().occupancy())
    {
        return Some(UiIntentConfirmationStopReason::OccupancyChanged);
    }
    None
}

fn product_route_is_current(
    candidate: &super::super::payload::UiPreparedIntentPayload,
    catalog: &crate::declaration::UiIntentCatalog,
    definitions: &crate::capability::FrozenIntentDefinitionCapabilities,
) -> bool {
    let Some((crate::declaration::UiIntentCatalogResolvedRoute::Product { declaration, .. }, _)) =
        catalog.lookup(candidate.graph_node(), candidate.interaction_family())
    else {
        return false;
    };
    Arc::ptr_eq(&declaration, candidate.declaration_reference())
        && definitions.definition_at(declaration.definition()).id() == candidate.definition_id()
}

fn stopped(
    reason: UiIntentConfirmationStopReason,
    slots_inspected: usize,
) -> UiIntentConfirmationContinuation {
    UiIntentConfirmationContinuation::Stopped(UiIntentConfirmationStop::new(
        reason,
        UiIntentConfirmationLookupCost::new(slots_inspected),
    ))
}

impl UiConfirmedIntentCandidate {
    pub const fn definition_id(&self) -> crate::capability::UiIntentId {
        self.candidate.definition_id()
    }

    pub fn declaration_identity(&self) -> &str {
        self.candidate.declaration_identity()
    }

    pub const fn lineage(&self) -> UiIntentAttemptLineage {
        self.lineage
    }

    pub const fn confirmation_decision(
        &self,
    ) -> &super::super::operability::UiIntentOperabilityDecision {
        &self.confirmation_decision
    }

    pub fn retained_payload_count(&self) -> usize {
        self.candidate.retained_payload_count()
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        super::super::payload::UiPreparedIntentPayload,
        super::super::operability::UiIntentOperabilityDecision,
        UiIntentAttemptLineage,
    ) {
        (self.candidate, self.confirmation_decision, self.lineage)
    }
}
