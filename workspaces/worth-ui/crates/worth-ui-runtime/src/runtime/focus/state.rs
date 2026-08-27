use std::collections::BTreeMap;

#[path = "state/observation.rs"]
mod observation;
#[path = "state/portal_proposal.rs"]
mod portal_proposal;

/// Sole owner of semantic keyboard focus for one active application session.
pub(crate) struct UiFocusRuntimeState {
    persistence: crate::runtime::UiServiceStatePersistencePosture,
    pub(super) participants: BTreeMap<super::UiFocusScopeIdentity, Vec<super::UiFocusParticipant>>,
    pub(super) participant_index:
        BTreeMap<super::UiFocusParticipantIdentity, (super::UiFocusScopeIdentity, usize)>,
    pub(super) current: Option<super::UiSemanticKeyboardFocus>,
    pub(super) active_descendant: Option<super::UiActiveDescendant>,
    window_focus: super::UiWindowFocus,
    modality: super::UiFocusVisibleModality,
    pub(super) pending_portal: BTreeMap<
        crate::runtime::session::service_proposal::UiServiceProposalIdentity,
        super::portal_transition::UiPreparedPortalFocusTransition,
    >,
    portal_restorations:
        BTreeMap<super::UiPortalFocusBoundaryIdentity, Option<super::UiFocusRestorationToken>>,
    pub(super) revision: u64,
}

impl UiFocusRuntimeState {
    pub(crate) const fn new_session_restore_candidate() -> Self {
        Self::new(crate::runtime::UiServiceStatePersistencePosture::SessionRestoreCandidate)
    }

    pub(in crate::runtime) const fn new(
        persistence: crate::runtime::UiServiceStatePersistencePosture,
    ) -> Self {
        Self {
            persistence,
            participants: BTreeMap::new(),
            participant_index: BTreeMap::new(),
            current: None,
            active_descendant: None,
            window_focus: super::UiWindowFocus::Unfocused,
            modality: super::UiFocusVisibleModality::Initial,
            pending_portal: BTreeMap::new(),
            portal_restorations: BTreeMap::new(),
            revision: 0,
        }
    }

    pub(in crate::runtime) const fn persistence(
        &self,
    ) -> crate::runtime::UiServiceStatePersistencePosture {
        self.persistence
    }

    pub(in crate::runtime) fn commit(
        &mut self,
        plan: super::UiFocusPlan,
    ) -> Result<super::UiFocusTransitionReceipt, super::UiFocusRoutingDenial> {
        if plan.expected_revision() != self.revision {
            return Err(super::UiFocusRoutingDenial::StalePlan);
        }
        self.apply_immediate(plan.next(), plan.cause(), plan.participants_visited())
    }

    pub(super) fn exact_current_successor(
        &self,
        current: super::UiSemanticKeyboardFocus,
    ) -> Option<super::UiFocusParticipant> {
        self.exact_participant(
            current.scope(),
            current.participant(),
            current.incarnation(),
        )
        .ok()
    }

    pub(super) fn exact_participant(
        &self,
        scope: super::UiFocusScopeIdentity,
        identity: super::UiFocusParticipantIdentity,
        incarnation: worth_ui_host_contract::UiMountIncarnation,
    ) -> Result<super::UiFocusParticipant, super::UiFocusRoutingDenial> {
        let Some((indexed_scope, index)) = self.participant_index.get(&identity).copied() else {
            return Err(super::UiFocusRoutingDenial::UnknownParticipant);
        };
        if indexed_scope != scope {
            return Err(super::UiFocusRoutingDenial::UnknownParticipant);
        }
        let participant = self.participants[&scope][index];
        if participant.incarnation() != incarnation {
            return Err(super::UiFocusRoutingDenial::StaleParticipantIncarnation);
        }
        Ok(participant)
    }

    pub(super) fn apply_immediate(
        &mut self,
        next: Option<super::UiSemanticKeyboardFocus>,
        cause: super::UiFocusCause,
        participants_visited: u32,
    ) -> Result<super::UiFocusTransitionReceipt, super::UiFocusRoutingDenial> {
        let previous = self.current;
        let outcome = super::routing::transition_outcome(previous, next);
        self.revision = self
            .revision
            .checked_add(1)
            .ok_or(super::UiFocusRoutingDenial::RevisionExhausted)?;
        self.current = next;
        if self.active_descendant.is_some_and(|active| {
            next.is_none_or(|focus| focus.participant() != active.composite())
        }) {
            self.active_descendant = None;
        }
        if cause == super::UiFocusCause::KeyboardTraversal {
            self.modality = super::UiFocusVisibleModality::Keyboard;
        }
        Ok(super::UiFocusTransitionReceipt::new(
            previous,
            next,
            cause,
            outcome,
            participants_visited,
            self.revision,
        ))
    }
}
