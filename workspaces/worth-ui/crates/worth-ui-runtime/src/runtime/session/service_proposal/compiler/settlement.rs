#[derive(Debug)]
pub(in crate::runtime) struct UiServiceProposalSettlement {
    batch: super::UiServiceProposalStagedBatch,
    publication: super::UiServiceProposalPublicationReceipt,
    acknowledged: super::super::UiServiceFamilyParticipation,
    retained_receipts: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiServiceProposalPublicationDenial {
    ForeignProposal,
    BatchDigestMismatch,
    ReceiptCapacityExceeded,
    Census(super::super::UiServiceProposalCensusDenial),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::runtime) enum UiServiceProposalSettlementDenial {
    ForeignProposal,
    BatchDigestMismatch,
    PublicationDispositionMismatch,
    NonParticipatingFamily,
    OwnerScopeMismatch,
    DuplicateOwnerAcknowledgement,
    ReceiptCapacityExceeded,
    IncompleteOwnerSettlement,
    Census(super::super::UiServiceProposalCensusDenial),
    Occupancy(super::super::UiServiceProposalOccupancyDenial),
    Cancellation(super::super::UiServiceProposalCancellationDenial),
}

impl UiServiceProposalSettlement {
    pub(super) fn validate_publication(
        batch: &super::UiServiceProposalStagedBatch,
        publication: super::UiServiceProposalPublicationReceipt,
    ) -> Result<u16, UiServiceProposalPublicationDenial> {
        if publication.proposal() != batch.identity() {
            return Err(UiServiceProposalPublicationDenial::ForeignProposal);
        }
        if publication.batch_digest() != batch.digest() {
            return Err(UiServiceProposalPublicationDenial::BatchDigestMismatch);
        }
        batch
            .retained_receipts()
            .checked_add(1)
            .ok_or(UiServiceProposalPublicationDenial::ReceiptCapacityExceeded)
    }

    pub(super) fn from_validated_publication(
        batch: super::UiServiceProposalStagedBatch,
        publication: super::UiServiceProposalPublicationReceipt,
        retained_receipts: u16,
    ) -> Self {
        Self {
            batch,
            publication,
            acknowledged: super::super::UiServiceFamilyParticipation::EMPTY,
            retained_receipts,
        }
    }

    pub(super) fn accept_owner_acknowledgement(
        &mut self,
        acknowledgement: super::UiServiceProposalOwnerAcknowledgement,
    ) -> Result<(), UiServiceProposalSettlementDenial> {
        if acknowledgement.proposal() != self.batch.identity() {
            return Err(UiServiceProposalSettlementDenial::ForeignProposal);
        }
        if acknowledgement.batch_digest() != self.batch.digest() {
            return Err(UiServiceProposalSettlementDenial::BatchDigestMismatch);
        }
        if acknowledgement.disposition() != self.publication.disposition() {
            return Err(UiServiceProposalSettlementDenial::PublicationDispositionMismatch);
        }
        let family = acknowledgement.family();
        let Some(proposal) = self
            .batch
            .candidate()
            .family_proposals()
            .iter()
            .find(|proposal| proposal.family() == family)
        else {
            return Err(UiServiceProposalSettlementDenial::NonParticipatingFamily);
        };
        if proposal.scope() != acknowledgement.scope() {
            return Err(UiServiceProposalSettlementDenial::OwnerScopeMismatch);
        }
        if self.acknowledged.contains(family) {
            return Err(UiServiceProposalSettlementDenial::DuplicateOwnerAcknowledgement);
        }
        let retained_receipts = self
            .retained_receipts
            .checked_add(1)
            .ok_or(UiServiceProposalSettlementDenial::ReceiptCapacityExceeded)?;
        let acknowledged = self
            .acknowledged
            .with_family(family)
            .map_err(|_| UiServiceProposalSettlementDenial::DuplicateOwnerAcknowledgement)?;
        self.acknowledged = acknowledged;
        self.retained_receipts = retained_receipts;
        Ok(())
    }

    pub(in crate::runtime) const fn publication(
        &self,
    ) -> super::UiServiceProposalPublicationReceipt {
        self.publication
    }

    pub(in crate::runtime) fn is_complete(&self) -> bool {
        self.acknowledged.count()
            == self
                .batch
                .candidate()
                .demand()
                .participating_families()
                .count()
    }

    pub(super) fn terminal_parts(
        self,
    ) -> Result<super::staging::UiServiceProposalTerminalParts, UiServiceProposalSettlementDenial>
    {
        if !self.is_complete() {
            return Err(UiServiceProposalSettlementDenial::IncompleteOwnerSettlement);
        }
        let mut parts = self.batch.into_terminal_parts();
        parts.retained_receipts = self.retained_receipts;
        Ok(parts)
    }

    #[cfg(test)]
    pub(super) fn into_shutdown_terminal_parts(
        self,
    ) -> super::staging::UiServiceProposalTerminalParts {
        let owners_requiring_discard = self
            .batch
            .candidate()
            .demand()
            .participating_families()
            .without(self.acknowledged);
        let mut parts = self.batch.into_terminal_parts();
        parts.retained_receipts = self.retained_receipts;
        parts.owners_requiring_discard = owners_requiring_discard;
        parts
    }

    pub(super) fn candidate(&self) -> &super::UiServiceProposalCandidate {
        self.batch.candidate()
    }

    pub(super) fn leases(&self) -> &[super::super::UiServiceProposalOccupancyLease] {
        self.batch.leases()
    }

    pub(super) const fn retained_receipt_count(&self) -> u16 {
        self.retained_receipts
    }
}
