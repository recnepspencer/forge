use std::collections::BTreeMap;

pub(crate) struct UiPreparedFocusMountedReconciliation {
    participation: Option<UiPreparedFocusMountedParticipation>,
    nodes_visited: u32,
    installed: u32,
}

struct UiPreparedFocusMountedParticipation {
    participants: BTreeMap<super::UiFocusScopeIdentity, Vec<super::UiFocusParticipant>>,
    participant_index:
        BTreeMap<super::UiFocusParticipantIdentity, (super::UiFocusScopeIdentity, usize)>,
}

impl super::UiFocusRuntimeState {
    pub(crate) fn prepare_mounted_reconciliation(
        &self,
        snapshot: &crate::mounting::UiMountedFocusParticipationSnapshot,
    ) -> Result<UiPreparedFocusMountedReconciliation, super::UiFocusRoutingDenial> {
        if self
            .pending_portal
            .values()
            .any(|transition| transition.frame() == snapshot.frame())
        {
            let installed = u32::try_from(
                snapshot
                    .participants()
                    .iter()
                    .filter(|participant| {
                        participant.support()
                            != crate::capability::ComponentFocusSupport::NotFocusable
                    })
                    .count(),
            )
            .map_err(|_| super::UiFocusRoutingDenial::VisitCounterOverflow)?;
            return Ok(UiPreparedFocusMountedReconciliation {
                participation: None,
                nodes_visited: snapshot.nodes_visited(),
                installed,
            });
        }
        let participation = prepare_participation(snapshot)?;
        let installed = u32::try_from(participation.participant_index.len())
            .map_err(|_| super::UiFocusRoutingDenial::VisitCounterOverflow)?;
        Ok(UiPreparedFocusMountedReconciliation {
            participation: Some(participation),
            nodes_visited: snapshot.nodes_visited(),
            installed,
        })
    }

    pub(crate) fn commit_mounted_reconciliation(
        &mut self,
        prepared: UiPreparedFocusMountedReconciliation,
    ) -> Result<super::UiFocusReconciliationReceipt, super::UiFocusRoutingDenial> {
        let Some(participation) = prepared.participation else {
            return Ok(super::UiFocusReconciliationReceipt::new(
                None,
                prepared.nodes_visited,
                prepared.installed,
            ));
        };
        self.install_prepared_participation(participation);
        let transition = self.reconciliation_transition()?;
        Ok(super::UiFocusReconciliationReceipt::new(
            transition,
            prepared.nodes_visited,
            prepared.installed,
        ))
    }

    pub(crate) fn reconcile_mounted_participation(
        &mut self,
        snapshot: &crate::mounting::UiMountedFocusParticipationSnapshot,
    ) -> Result<super::UiFocusReconciliationReceipt, super::UiFocusRoutingDenial> {
        let prepared = self.prepare_mounted_reconciliation(snapshot)?;
        self.commit_mounted_reconciliation(prepared)
    }

    pub(super) fn install_mounted_participation(
        &mut self,
        snapshot: &crate::mounting::UiMountedFocusParticipationSnapshot,
    ) -> Result<u32, super::UiFocusRoutingDenial> {
        let prepared = prepare_participation(snapshot)?;
        let installed = u32::try_from(prepared.participant_index.len())
            .map_err(|_| super::UiFocusRoutingDenial::VisitCounterOverflow)?;
        self.install_prepared_participation(prepared);
        Ok(installed)
    }

    fn install_prepared_participation(&mut self, prepared: UiPreparedFocusMountedParticipation) {
        self.participants = prepared.participants;
        self.participant_index = prepared.participant_index;
        if self.active_descendant.is_some_and(|active| {
            self.exact_participant(
                active.scope(),
                active.descendant(),
                active.descendant_incarnation(),
            )
            .is_err()
        }) {
            self.active_descendant = None;
        }
    }

    fn reconciliation_transition(
        &mut self,
    ) -> Result<Option<super::UiFocusTransitionReceipt>, super::UiFocusRoutingDenial> {
        match self.current {
            Some(current) => match self.exact_current_successor(current) {
                Some(successor) => {
                    let next = super::UiSemanticKeyboardFocus::new(successor);
                    if current.exact_participant() == successor {
                        self.current = Some(next);
                        Ok(None)
                    } else {
                        self.apply_immediate(Some(next), super::UiFocusCause::RebindPreserved, 1)
                            .map(Some)
                    }
                }
                None => {
                    let fallback = self.first_in_scope(current.scope());
                    self.apply_immediate(
                        fallback.map(super::UiSemanticKeyboardFocus::new),
                        super::UiFocusCause::RebindFallback,
                        u32::from(fallback.is_some()),
                    )
                    .map(Some)
                }
            },
            None => Ok(None),
        }
    }
}

fn prepare_participation(
    snapshot: &crate::mounting::UiMountedFocusParticipationSnapshot,
) -> Result<UiPreparedFocusMountedParticipation, super::UiFocusRoutingDenial> {
    let mut participants = BTreeMap::<_, Vec<_>>::new();
    for participant in focusable_participants(snapshot) {
        participants
            .entry(participant.scope())
            .or_default()
            .push(participant);
    }
    for scoped in participants.values_mut() {
        scoped.sort_by_key(|participant| participant.mounted_order());
    }
    let mut participant_index = BTreeMap::new();
    for (scope, scoped) in &participants {
        for (index, participant) in scoped.iter().enumerate() {
            participant_index.insert(participant.identity(), (*scope, index));
        }
    }
    u32::try_from(participant_index.len())
        .map_err(|_| super::UiFocusRoutingDenial::VisitCounterOverflow)?;
    Ok(UiPreparedFocusMountedParticipation {
        participants,
        participant_index,
    })
}

pub(super) fn focusable_participants(
    snapshot: &crate::mounting::UiMountedFocusParticipationSnapshot,
) -> Vec<super::UiFocusParticipant> {
    snapshot
        .participants()
        .iter()
        .copied()
        .filter_map(super::UiFocusParticipant::from_mounted)
        .collect()
}
