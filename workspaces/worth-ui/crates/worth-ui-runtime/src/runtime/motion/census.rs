const ACTIVE_TRACK_LIMIT: u16 = 64;
const EXIT_RETENTION_LIMIT: u16 = 64;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct UiMotionResourceCensus {
    pub(super) staged_tracks: u16,
    pub(super) active_tracks: u16,
    pub(super) exit_retentions: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum UiMotionCensusDenial {
    CapacityExceeded,
    Underflow,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct UiMotionShutdownReport {
    abandoned_staged_tracks: u16,
    terminated_active_tracks: u16,
    cancelled_exit_retentions: u16,
    final_census: UiMotionResourceCensus,
}

impl UiMotionResourceCensus {
    pub(super) const fn zero() -> Self {
        Self {
            staged_tracks: 0,
            active_tracks: 0,
            exit_retentions: 0,
        }
    }

    pub(super) fn stage(&mut self) -> Result<(), UiMotionCensusDenial> {
        let total = self
            .staged_tracks
            .checked_add(self.active_tracks)
            .ok_or(UiMotionCensusDenial::CapacityExceeded)?;
        if total >= ACTIVE_TRACK_LIMIT {
            return Err(UiMotionCensusDenial::CapacityExceeded);
        }
        self.staged_tracks += 1;
        Ok(())
    }

    pub(super) fn discard_staged(&mut self) -> Result<(), UiMotionCensusDenial> {
        self.staged_tracks = self
            .staged_tracks
            .checked_sub(1)
            .ok_or(UiMotionCensusDenial::Underflow)?;
        Ok(())
    }

    pub(super) fn commit_staged(&mut self, replacing: bool) -> Result<(), UiMotionCensusDenial> {
        self.discard_staged()?;
        if !replacing {
            self.active_tracks = self
                .active_tracks
                .checked_add(1)
                .ok_or(UiMotionCensusDenial::CapacityExceeded)?;
        }
        Ok(())
    }

    pub(super) fn terminal(&mut self) -> Result<(), UiMotionCensusDenial> {
        self.active_tracks = self
            .active_tracks
            .checked_sub(1)
            .ok_or(UiMotionCensusDenial::Underflow)?;
        Ok(())
    }

    pub(super) fn reserve_exit_retention(&mut self) -> Result<(), UiMotionCensusDenial> {
        if self.exit_retentions >= EXIT_RETENTION_LIMIT {
            return Err(UiMotionCensusDenial::CapacityExceeded);
        }
        self.exit_retentions += 1;
        Ok(())
    }

    pub(super) fn release_exit_retention(&mut self) -> Result<(), UiMotionCensusDenial> {
        self.exit_retentions = self
            .exit_retentions
            .checked_sub(1)
            .ok_or(UiMotionCensusDenial::Underflow)?;
        Ok(())
    }

    pub(crate) const fn staged_tracks(self) -> u16 {
        self.staged_tracks
    }

    pub(crate) const fn active_tracks(self) -> u16 {
        self.active_tracks
    }

    pub(crate) const fn exit_retentions(self) -> u16 {
        self.exit_retentions
    }

    pub(crate) const fn is_zero(self) -> bool {
        self.staged_tracks == 0 && self.active_tracks == 0 && self.exit_retentions == 0
    }
}

impl UiMotionShutdownReport {
    pub(super) const fn from_census(census: UiMotionResourceCensus) -> Self {
        Self {
            abandoned_staged_tracks: census.staged_tracks,
            terminated_active_tracks: census.active_tracks,
            cancelled_exit_retentions: census.exit_retentions,
            final_census: UiMotionResourceCensus::zero(),
        }
    }

    pub(crate) const fn abandoned_staged_tracks(self) -> u16 {
        self.abandoned_staged_tracks
    }

    pub(crate) const fn terminated_active_tracks(self) -> u16 {
        self.terminated_active_tracks
    }

    pub(crate) const fn cancelled_exit_retentions(self) -> u16 {
        self.cancelled_exit_retentions
    }

    pub(crate) const fn final_census(self) -> UiMotionResourceCensus {
        self.final_census
    }
}
