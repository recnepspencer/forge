use std::collections::BTreeMap;

use super::progress::{UiObservationProgress, UiObservationProgressKey};
use super::{UiObservationProfile, UiObservationTurnDenial};

#[derive(Debug)]
pub(crate) struct UiObservationRuntimeState {
    next_turn: u64,
    active_turn: bool,
    last_owner_orders: BTreeMap<UiObservationProgressKey, u64>,
}

impl UiObservationRuntimeState {
    pub(crate) const fn new() -> Self {
        Self {
            next_turn: 1,
            active_turn: false,
            last_owner_orders: BTreeMap::new(),
        }
    }

    pub(crate) fn begin(
        &mut self,
        profile: UiObservationProfile,
    ) -> Result<(u64, UiObservationProfile), UiObservationTurnDenial> {
        if self.active_turn {
            return Err(UiObservationTurnDenial::TurnAlreadyActive);
        }
        let identity = self.next_turn;
        self.next_turn = self
            .next_turn
            .checked_add(1)
            .ok_or(UiObservationTurnDenial::IdentityExhausted)?;
        self.active_turn = true;
        Ok((identity, profile))
    }

    pub(super) fn is_historical(&self, progress: &UiObservationProgress) -> bool {
        self.last_owner_orders
            .get(progress.key())
            .is_some_and(|last| progress.owner_order() <= *last)
    }

    pub(super) fn finish_committed<'a>(
        &mut self,
        progress: impl IntoIterator<Item = &'a UiObservationProgress>,
    ) {
        for item in progress {
            self.last_owner_orders
                .insert(item.key().clone(), item.owner_order());
        }
        self.active_turn = false;
    }

    pub(super) fn abandon(&mut self) {
        self.active_turn = false;
    }
}
