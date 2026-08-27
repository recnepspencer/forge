#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::runtime) struct UiActiveDescendant {
    composite: super::UiFocusParticipantIdentity,
    scope: super::UiFocusScopeIdentity,
    descendant: super::UiFocusParticipantIdentity,
    descendant_incarnation: worth_ui_host_contract::UiMountIncarnation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::runtime) enum UiActiveDescendantDenial {
    NoSemanticFocus,
    CompositeMismatch,
    CompositeCannotBeItsOwnDescendant,
    UnknownDescendant,
    StaleDescendantIncarnation,
    ForeignScope,
    RevisionExhausted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::runtime) struct UiActiveDescendantTransitionReceipt {
    previous: Option<UiActiveDescendant>,
    current: Option<UiActiveDescendant>,
    revision: u64,
}

impl super::UiFocusRuntimeState {
    pub(in crate::runtime) fn move_active_descendant(
        &mut self,
        composite: super::UiFocusParticipantIdentity,
        descendant: super::UiFocusParticipantIdentity,
        descendant_incarnation: worth_ui_host_contract::UiMountIncarnation,
    ) -> Result<UiActiveDescendantTransitionReceipt, UiActiveDescendantDenial> {
        let current = self
            .current
            .ok_or(UiActiveDescendantDenial::NoSemanticFocus)?;
        if current.participant() != composite {
            return Err(UiActiveDescendantDenial::CompositeMismatch);
        }
        if composite == descendant {
            return Err(UiActiveDescendantDenial::CompositeCannotBeItsOwnDescendant);
        }
        let descendant = self
            .exact_participant(current.scope(), descendant, descendant_incarnation)
            .map_err(|denial| match denial {
                super::UiFocusRoutingDenial::StaleParticipantIncarnation => {
                    UiActiveDescendantDenial::StaleDescendantIncarnation
                }
                super::UiFocusRoutingDenial::UnknownParticipant => {
                    UiActiveDescendantDenial::UnknownDescendant
                }
                _ => UiActiveDescendantDenial::ForeignScope,
            })?;
        if descendant.scope() != current.scope() {
            return Err(UiActiveDescendantDenial::ForeignScope);
        }
        let next = UiActiveDescendant {
            composite,
            scope: current.scope(),
            descendant: descendant.identity(),
            descendant_incarnation: descendant.incarnation(),
        };
        let previous = self.active_descendant;
        self.revision = self
            .revision
            .checked_add(1)
            .ok_or(UiActiveDescendantDenial::RevisionExhausted)?;
        self.active_descendant = Some(next);
        Ok(UiActiveDescendantTransitionReceipt {
            previous,
            current: Some(next),
            revision: self.revision,
        })
    }

    pub(in crate::runtime) fn clear_active_descendant(
        &mut self,
        composite: super::UiFocusParticipantIdentity,
    ) -> Result<UiActiveDescendantTransitionReceipt, UiActiveDescendantDenial> {
        let current = self
            .current
            .ok_or(UiActiveDescendantDenial::NoSemanticFocus)?;
        if current.participant() != composite {
            return Err(UiActiveDescendantDenial::CompositeMismatch);
        }
        let previous = self.active_descendant;
        self.revision = self
            .revision
            .checked_add(1)
            .ok_or(UiActiveDescendantDenial::RevisionExhausted)?;
        self.active_descendant = None;
        Ok(UiActiveDescendantTransitionReceipt {
            previous,
            current: None,
            revision: self.revision,
        })
    }
}

impl UiActiveDescendant {
    pub(in crate::runtime) const fn composite(self) -> super::UiFocusParticipantIdentity {
        self.composite
    }

    pub(in crate::runtime) const fn descendant(self) -> super::UiFocusParticipantIdentity {
        self.descendant
    }

    pub(in crate::runtime) const fn scope(self) -> super::UiFocusScopeIdentity {
        self.scope
    }

    pub(in crate::runtime) const fn descendant_incarnation(
        self,
    ) -> worth_ui_host_contract::UiMountIncarnation {
        self.descendant_incarnation
    }
}

impl UiActiveDescendantTransitionReceipt {
    pub(in crate::runtime) const fn previous(self) -> Option<UiActiveDescendant> {
        self.previous
    }

    pub(in crate::runtime) const fn current(self) -> Option<UiActiveDescendant> {
        self.current
    }

    pub(in crate::runtime) const fn revision(self) -> u64 {
        self.revision
    }
}
