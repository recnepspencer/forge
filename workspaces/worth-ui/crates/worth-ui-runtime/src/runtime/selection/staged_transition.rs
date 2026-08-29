#[must_use = "a staged Selection successor must settle with its compiled publication"]
pub(in crate::runtime) struct UiStagedDeclaredSelectionTransition {
    owner: super::UiStagedSelectionServiceProposal,
    registration: Option<super::UiSelectionRegistration>,
    reconciliation: Option<super::UiSelectionReconciliationReceipt>,
    delta: super::UiSelectionDelta,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiDeclaredSelectionStagingDenial {
    OwnerMismatch,
    IncarnationMismatch,
    Selection(super::UiSelectionRequestDenial),
}

impl UiStagedDeclaredSelectionTransition {
    pub(in crate::runtime) fn prepare(
        proposal: crate::runtime::session::service_proposal::UiServiceProposalIdentity,
        action: crate::runtime::session::service_proposal::UiDeclaredFocusSelectionAction,
        registration: Option<super::UiSelectionRegistration>,
        predecessor: &super::UiSelectionRuntimeState,
    ) -> Result<Self, UiDeclaredSelectionStagingDenial> {
        if let Some(registration) = registration.as_ref() {
            if registration.owner() != action.owner() {
                return Err(UiDeclaredSelectionStagingDenial::OwnerMismatch);
            }
            if registration.incarnation() != action.incarnation() {
                return Err(UiDeclaredSelectionStagingDenial::IncarnationMismatch);
            }
        }
        let mut staged_owner = predecessor.clone();
        let (reconciliation, delta) = match registration.as_ref() {
            Some(registration) => {
                let (reconciliation, delta) = staged_owner
                    .synchronize_and_apply(registration.clone(), action.request())
                    .map_err(UiDeclaredSelectionStagingDenial::Selection)?;
                (Some(reconciliation), delta)
            }
            None => {
                let delta = staged_owner
                    .apply(action.owner(), action.incarnation(), action.request())
                    .map_err(UiDeclaredSelectionStagingDenial::Selection)?;
                (None, delta)
            }
        };
        Ok(Self {
            owner: super::UiStagedSelectionServiceProposal::prepare(proposal, action),
            registration,
            reconciliation,
            delta,
        })
    }

    pub(in crate::runtime) fn family_stage_receipt(
        &self,
    ) -> crate::runtime::session::service_proposal::UiServiceProposalStageReceipt {
        self.owner.family_stage_receipt()
    }

    pub(in crate::runtime) const fn delta(&self) -> &super::UiSelectionDelta {
        &self.delta
    }

    pub(in crate::runtime) const fn reconciliation(
        &self,
    ) -> Option<&super::UiSelectionReconciliationReceipt> {
        self.reconciliation.as_ref()
    }

    pub(in crate::runtime) fn settlement_acknowledgement(
        &self,
        publication: crate::runtime::session::service_proposal::UiServiceProposalPublicationReceipt,
    ) -> crate::runtime::session::service_proposal::UiServiceProposalOwnerAcknowledgement {
        crate::runtime::session::service_proposal::UiServiceProposalOwnerAcknowledgement::from_family_owner(
            publication,
            crate::capability::UiRuntimeServiceFamily::Selection,
            self.owner.scope(),
        )
    }

    pub(in crate::runtime) fn terminal_outcome(
        &self,
        reason: crate::runtime::session::service_proposal::UiServiceProposalTerminalReason,
    ) -> crate::runtime::session::service_proposal::UiServiceProposalTerminalOwnerOutcome {
        crate::runtime::session::service_proposal::UiServiceProposalTerminalOwnerOutcome::from_family_owner(
            self.owner.proposal(),
            crate::capability::UiRuntimeServiceFamily::Selection,
            self.owner.scope(),
            reason,
        )
    }

    pub(in crate::runtime) fn commit(self, current: &mut super::UiSelectionRuntimeState) {
        if let Some(registration) = self.registration {
            current
                .synchronize_and_apply(registration, self.owner.action().request())
                .expect("staged Selection action retains its exact current owner");
        } else {
            current
                .apply(
                    self.owner.action().owner(),
                    self.owner.action().incarnation(),
                    self.owner.action().request(),
                )
                .expect("staged Selection action retains its exact current owner");
        }
    }
}
