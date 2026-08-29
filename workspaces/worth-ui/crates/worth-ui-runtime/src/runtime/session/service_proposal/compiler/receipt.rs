#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::runtime) enum UiServiceProposalPublicationDisposition {
    Accepted,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::runtime) struct UiServiceProposalPublicationReceipt {
    proposal: super::UiServiceProposalIdentity,
    batch_digest: u64,
    disposition: UiServiceProposalPublicationDisposition,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::runtime) struct UiServiceProposalOwnerAcknowledgement {
    proposal: super::UiServiceProposalIdentity,
    batch_digest: u64,
    family: crate::capability::UiRuntimeServiceFamily,
    scope: super::super::UiServiceProposalOccupancyScopeIdentity,
    disposition: UiServiceProposalPublicationDisposition,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::runtime) struct UiServiceProposalTerminalOwnerOutcome {
    proposal: super::UiServiceProposalIdentity,
    family: crate::capability::UiRuntimeServiceFamily,
    scope: super::super::UiServiceProposalOccupancyScopeIdentity,
    reason: super::UiServiceProposalTerminalReason,
}

/// A recorded view of the existing publication boundary, never a publisher.
#[cfg(test)]
pub(in crate::runtime) struct UiRecordedServiceProposalPublicationPort;

/// A recorded family-owner endpoint used only by lifecycle proofs.
#[cfg(test)]
pub(in crate::runtime) struct UiRecordedServiceProposalOwnerPort {
    family: crate::capability::UiRuntimeServiceFamily,
    scope: super::super::UiServiceProposalOccupancyScopeIdentity,
}

impl UiServiceProposalPublicationReceipt {
    pub(in crate::runtime) const fn from_staged_batch(
        batch: &super::UiServiceProposalStagedBatch,
        disposition: UiServiceProposalPublicationDisposition,
    ) -> Self {
        Self {
            proposal: batch.identity(),
            batch_digest: batch.digest(),
            disposition,
        }
    }

    pub(in crate::runtime) const fn proposal(self) -> super::UiServiceProposalIdentity {
        self.proposal
    }

    pub(in crate::runtime) const fn batch_digest(self) -> u64 {
        self.batch_digest
    }

    pub(in crate::runtime) const fn disposition(self) -> UiServiceProposalPublicationDisposition {
        self.disposition
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(in crate::runtime) fn recorded_foreign_fixture(
        proposal: super::UiServiceProposalIdentity,
        batch_digest: u64,
        disposition: UiServiceProposalPublicationDisposition,
    ) -> Self {
        Self {
            proposal,
            batch_digest,
            disposition,
        }
    }
}

impl UiServiceProposalOwnerAcknowledgement {
    pub(in crate::runtime) const fn from_family_owner(
        receipt: UiServiceProposalPublicationReceipt,
        family: crate::capability::UiRuntimeServiceFamily,
        scope: super::super::UiServiceProposalOccupancyScopeIdentity,
    ) -> Self {
        Self {
            proposal: receipt.proposal,
            batch_digest: receipt.batch_digest,
            family,
            scope,
            disposition: receipt.disposition,
        }
    }

    pub(in crate::runtime) const fn proposal(self) -> super::UiServiceProposalIdentity {
        self.proposal
    }

    pub(in crate::runtime) const fn batch_digest(self) -> u64 {
        self.batch_digest
    }

    pub(in crate::runtime) const fn family(self) -> crate::capability::UiRuntimeServiceFamily {
        self.family
    }

    pub(in crate::runtime) const fn scope(
        self,
    ) -> super::super::UiServiceProposalOccupancyScopeIdentity {
        self.scope
    }

    pub(in crate::runtime) const fn disposition(self) -> UiServiceProposalPublicationDisposition {
        self.disposition
    }

    #[cfg(test)]
    pub(in crate::runtime) const fn recorded_foreign_fixture(
        receipt: UiServiceProposalPublicationReceipt,
        family: crate::capability::UiRuntimeServiceFamily,
        scope: super::super::UiServiceProposalOccupancyScopeIdentity,
    ) -> Self {
        Self::from_family_owner(receipt, family, scope)
    }
}

#[cfg(test)]
impl UiRecordedServiceProposalPublicationPort {
    pub(in crate::runtime) const fn recorded_fixture() -> Self {
        Self
    }

    pub(in crate::runtime) const fn report(
        &self,
        batch: &super::UiServiceProposalStagedBatch,
        disposition: UiServiceProposalPublicationDisposition,
    ) -> UiServiceProposalPublicationReceipt {
        UiServiceProposalPublicationReceipt::from_staged_batch(batch, disposition)
    }
}

#[cfg(test)]
impl UiRecordedServiceProposalOwnerPort {
    pub(in crate::runtime) const fn recorded_fixture(
        family: crate::capability::UiRuntimeServiceFamily,
        scope: super::super::UiServiceProposalOccupancyScopeIdentity,
    ) -> Self {
        Self { family, scope }
    }

    pub(in crate::runtime) const fn acknowledge(
        &self,
        receipt: UiServiceProposalPublicationReceipt,
    ) -> UiServiceProposalOwnerAcknowledgement {
        UiServiceProposalOwnerAcknowledgement {
            proposal: receipt.proposal,
            batch_digest: receipt.batch_digest,
            family: self.family,
            scope: self.scope,
            disposition: receipt.disposition,
        }
    }

    pub(in crate::runtime) const fn terminal_outcome(
        &self,
        proposal: super::UiServiceProposalIdentity,
        reason: super::UiServiceProposalTerminalReason,
    ) -> UiServiceProposalTerminalOwnerOutcome {
        UiServiceProposalTerminalOwnerOutcome {
            proposal,
            family: self.family,
            scope: self.scope,
            reason,
        }
    }
}

impl UiServiceProposalTerminalOwnerOutcome {
    pub(in crate::runtime) const fn from_family_owner(
        proposal: super::UiServiceProposalIdentity,
        family: crate::capability::UiRuntimeServiceFamily,
        scope: super::super::UiServiceProposalOccupancyScopeIdentity,
        reason: super::UiServiceProposalTerminalReason,
    ) -> Self {
        Self {
            proposal,
            family,
            scope,
            reason,
        }
    }

    pub(in crate::runtime) const fn proposal(self) -> super::UiServiceProposalIdentity {
        self.proposal
    }

    pub(in crate::runtime) const fn family(self) -> crate::capability::UiRuntimeServiceFamily {
        self.family
    }

    pub(in crate::runtime) const fn scope(
        self,
    ) -> super::super::UiServiceProposalOccupancyScopeIdentity {
        self.scope
    }

    pub(in crate::runtime) const fn reason(self) -> super::UiServiceProposalTerminalReason {
        self.reason
    }
}

#[cfg(test)]
mod tests {
    use super::{
        UiRecordedServiceProposalOwnerPort, UiServiceProposalPublicationDisposition,
        UiServiceProposalPublicationReceipt,
    };

    #[test]
    fn owner_endpoint_copies_every_publication_binding_axis() {
        let proposal = super::super::UiServiceProposalIdentity::for_test(8);
        let receipt = UiServiceProposalPublicationReceipt::recorded_foreign_fixture(
            proposal,
            93,
            UiServiceProposalPublicationDisposition::Rejected,
        );
        let owner = UiRecordedServiceProposalOwnerPort::recorded_fixture(
            crate::capability::UiRuntimeServiceFamily::Portal,
            super::super::super::UiServiceProposalOccupancyScopeIdentity::for_test(4),
        );
        let acknowledgement = owner.acknowledge(receipt);
        assert_eq!(acknowledgement.proposal(), proposal);
        assert_eq!(acknowledgement.batch_digest(), 93);
        assert_eq!(acknowledgement.disposition(), receipt.disposition());
    }
}
