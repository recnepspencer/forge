#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(in crate::runtime) struct UiServiceProposalCensus {
    proposals: u16,
    occupancy_leases: u16,
    cancellation_records: u16,
    stage_receipts: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiServiceProposalCensusDenial {
    Overflow,
    Underflow,
}

impl UiServiceProposalCensus {
    pub(in crate::runtime) const fn zero() -> Self {
        Self {
            proposals: 0,
            occupancy_leases: 0,
            cancellation_records: 0,
            stage_receipts: 0,
        }
    }

    pub(in crate::runtime) const fn is_zero(self) -> bool {
        self.proposals == 0
            && self.occupancy_leases == 0
            && self.cancellation_records == 0
            && self.stage_receipts == 0
    }

    pub(in crate::runtime) const fn entries(self) -> [(&'static str, u16); 4] {
        [
            ("proposals", self.proposals),
            ("occupancy_leases", self.occupancy_leases),
            ("cancellation_records", self.cancellation_records),
            ("stage_receipts", self.stage_receipts),
        ]
    }

    pub(super) const fn exactly_tracks_live(
        self,
        proposals: u16,
        occupancy_leases: u16,
        cancellation_records: u16,
    ) -> bool {
        self.proposals == proposals
            && self.occupancy_leases == occupancy_leases
            && self.cancellation_records == cancellation_records
    }

    pub(super) fn record_stage_receipt(&mut self) -> Result<(), UiServiceProposalCensusDenial> {
        increment(&mut self.stage_receipts)
    }

    pub(super) fn release_stage_receipts(
        &mut self,
        count: u16,
    ) -> Result<(), UiServiceProposalCensusDenial> {
        self.stage_receipts = self
            .stage_receipts
            .checked_sub(count)
            .ok_or(UiServiceProposalCensusDenial::Underflow)?;
        Ok(())
    }

    pub(super) fn with_reservation(
        self,
        new_leases: u16,
        displaced_leases: u16,
        displaced_proposal: bool,
    ) -> Result<Self, UiServiceProposalCensusDenial> {
        let displaced = u16::from(displaced_proposal);
        Ok(Self {
            proposals: checked_replace(self.proposals, displaced, 1)?,
            occupancy_leases: checked_replace(self.occupancy_leases, displaced_leases, new_leases)?,
            cancellation_records: checked_replace(self.cancellation_records, displaced, 1)?,
            stage_receipts: self.stage_receipts,
        })
    }

    pub(super) fn with_terminal_release(
        self,
        leases: u16,
    ) -> Result<Self, UiServiceProposalCensusDenial> {
        Ok(Self {
            proposals: self
                .proposals
                .checked_sub(1)
                .ok_or(UiServiceProposalCensusDenial::Underflow)?,
            occupancy_leases: self
                .occupancy_leases
                .checked_sub(leases)
                .ok_or(UiServiceProposalCensusDenial::Underflow)?,
            cancellation_records: self
                .cancellation_records
                .checked_sub(1)
                .ok_or(UiServiceProposalCensusDenial::Underflow)?,
            stage_receipts: self.stage_receipts,
        })
    }

    pub(super) fn with_abandoned_before_effect(
        self,
        proposals: u16,
        leases: u16,
    ) -> Result<Self, UiServiceProposalCensusDenial> {
        Ok(Self {
            proposals: self
                .proposals
                .checked_sub(proposals)
                .ok_or(UiServiceProposalCensusDenial::Underflow)?,
            occupancy_leases: self
                .occupancy_leases
                .checked_sub(leases)
                .ok_or(UiServiceProposalCensusDenial::Underflow)?,
            cancellation_records: self
                .cancellation_records
                .checked_sub(proposals)
                .ok_or(UiServiceProposalCensusDenial::Underflow)?,
            stage_receipts: self.stage_receipts,
        })
    }

    pub(super) fn with_complete_release(
        self,
        leases: u16,
        receipts: u16,
    ) -> Result<Self, UiServiceProposalCensusDenial> {
        let mut released = self.with_terminal_release(leases)?;
        released.release_stage_receipts(receipts)?;
        Ok(released)
    }
}

fn checked_replace(
    current: u16,
    removed: u16,
    added: u16,
) -> Result<u16, UiServiceProposalCensusDenial> {
    current
        .checked_sub(removed)
        .ok_or(UiServiceProposalCensusDenial::Underflow)?
        .checked_add(added)
        .ok_or(UiServiceProposalCensusDenial::Overflow)
}

fn increment(value: &mut u16) -> Result<(), UiServiceProposalCensusDenial> {
    *value = value
        .checked_add(1)
        .ok_or(UiServiceProposalCensusDenial::Overflow)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{UiServiceProposalCensus, UiServiceProposalCensusDenial};

    #[test]
    fn proposal_census_returns_to_exact_zero() {
        let mut census = UiServiceProposalCensus::zero()
            .with_reservation(1, 0, false)
            .unwrap();
        census.record_stage_receipt().unwrap();
        census = census.with_complete_release(1, 1).unwrap();

        assert!(census.is_zero());
        assert_eq!(
            census.entries(),
            [
                ("proposals", 0),
                ("occupancy_leases", 0),
                ("cancellation_records", 0),
                ("stage_receipts", 0),
            ]
        );
    }

    #[test]
    fn census_rejects_underflow_without_wrapping() {
        let census = UiServiceProposalCensus::zero();
        assert_eq!(
            census.with_terminal_release(1),
            Err(UiServiceProposalCensusDenial::Underflow)
        );
        assert!(census.is_zero());
    }

    #[test]
    fn census_rejects_overflow_without_wrapping() {
        let census = UiServiceProposalCensus {
            proposals: u16::MAX,
            occupancy_leases: u16::MAX,
            cancellation_records: u16::MAX,
            stage_receipts: 0,
        };
        assert_eq!(
            census.with_reservation(1, 0, false),
            Err(UiServiceProposalCensusDenial::Overflow)
        );
        assert_eq!(census.proposals, u16::MAX);
    }
}
