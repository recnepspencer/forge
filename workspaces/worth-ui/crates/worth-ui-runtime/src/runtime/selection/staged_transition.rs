#[must_use = "a staged Selection successor must settle with its compiled publication"]
pub(in crate::runtime) struct UiStagedDeclaredSelectionTransition {
    owner: super::UiStagedSelectionServiceProposal,
    predecessor_owner: Option<super::state::UiSelectionOwnerRecord>,
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
            predecessor_owner: predecessor
                .owner_record_for_staging(action.owner())
                .cloned(),
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

    pub(in crate::runtime) fn scroll_anchor_key_for(
        &self,
        target: worth_ui_host_contract::UiMountedInstanceIdentity,
    ) -> Option<crate::runtime::UiApplicationItemKey> {
        let action = self.owner.action();
        (action.mounted_target() == target)
            .then(|| action.request().application_item_key())
            .flatten()
    }

    #[cfg(test)]
    pub(in crate::runtime) const fn delta(&self) -> &super::UiSelectionDelta {
        &self.delta
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
        assert_eq!(
            current.owner_record_for_staging(self.owner.action().owner()),
            self.predecessor_owner.as_ref(),
            "Selection commit requires the exact staged owner predecessor"
        );
        let (reconciliation, delta) = if let Some(registration) = self.registration {
            let (reconciliation, delta) = current
                .synchronize_and_apply(registration, self.owner.action().request())
                .expect("staged Selection action retains its exact current owner");
            (Some(reconciliation), delta)
        } else {
            let delta = current
                .apply(
                    self.owner.action().owner(),
                    self.owner.action().incarnation(),
                    self.owner.action().request(),
                )
                .expect("staged Selection action retains its exact current owner");
            (None, delta)
        };
        assert!(
            reconciliation
                .as_ref()
                .zip(self.reconciliation.as_ref())
                .is_none_or(|(committed, staged)| committed.has_same_effect_as(staged)),
            "Selection commit must retain the staged catalog reconciliation effect"
        );
        assert!(
            delta.has_same_effect_as(&self.delta),
            "Selection commit must retain the staged compact delta effect"
        );
    }
}
