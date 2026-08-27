#[must_use]
#[derive(Debug)]
pub(in crate::runtime) struct UiReservedServiceProposal {
    candidate: super::UiServiceProposalCandidate,
    leases: Box<[super::super::UiServiceProposalOccupancyLease]>,
    displacement: Option<super::super::UiServiceProposalDisplacement>,
}

pub(in crate::runtime) enum UiServiceProposalReservationOutcome {
    Reserved(UiReservedServiceProposal),
    Coalesced {
        incumbent: super::UiServiceProposalIdentity,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::runtime) enum UiServiceProposalReservationDenial {
    Occupancy(super::super::UiServiceProposalOccupancyDenial),
    Cancellation(super::super::UiServiceProposalCancellationDenial),
    Census(super::super::UiServiceProposalCensusDenial),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::runtime) struct UiServiceProposalBeforeEffectCancellationReceipt {
    proposal: super::UiServiceProposalIdentity,
    released_leases: u16,
}

impl UiReservedServiceProposal {
    pub(in crate::runtime) const fn identity(&self) -> super::UiServiceProposalIdentity {
        self.candidate.identity()
    }

    pub(in crate::runtime) fn leases(&self) -> &[super::super::UiServiceProposalOccupancyLease] {
        &self.leases
    }

    pub(in crate::runtime) const fn displacement(
        &self,
    ) -> Option<super::super::UiServiceProposalDisplacement> {
        self.displacement
    }

    pub(super) fn candidate(&self) -> &super::UiServiceProposalCandidate {
        &self.candidate
    }

    pub(super) fn from_parts(
        candidate: super::UiServiceProposalCandidate,
        leases: Box<[super::super::UiServiceProposalOccupancyLease]>,
        displacement: Option<super::super::UiServiceProposalDisplacement>,
    ) -> Self {
        Self {
            candidate,
            leases,
            displacement,
        }
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        super::UiServiceProposalCandidate,
        Box<[super::super::UiServiceProposalOccupancyLease]>,
        Option<super::super::UiServiceProposalDisplacement>,
    ) {
        (self.candidate, self.leases, self.displacement)
    }
}

impl UiServiceProposalBeforeEffectCancellationReceipt {
    pub(in crate::runtime) const fn proposal(self) -> super::UiServiceProposalIdentity {
        self.proposal
    }

    pub(in crate::runtime) const fn released_leases(self) -> u16 {
        self.released_leases
    }

    pub(super) const fn new(
        proposal: super::UiServiceProposalIdentity,
        released_leases: u16,
    ) -> Self {
        Self {
            proposal,
            released_leases,
        }
    }
}
