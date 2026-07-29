use std::collections::BTreeMap;

use super::progress::{UiObservationProgress, UiObservationProgressKey};
use super::{UiObservationProfile, UiObservationTurnDenial};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum UiObservationOrderPosture {
    Fresh,
    Duplicate,
    Historical,
}

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

    pub(super) fn order_posture(
        &self,
        progress: &UiObservationProgress,
    ) -> UiObservationOrderPosture {
        match self.last_owner_orders.get(progress.key()) {
            Some(last) if progress.owner_order() == *last => UiObservationOrderPosture::Duplicate,
            Some(last) if progress.owner_order() < *last => UiObservationOrderPosture::Historical,
            Some(_) | None => UiObservationOrderPosture::Fresh,
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owner_order_distinguishes_duplicate_historical_and_fresh_progress() {
        let mut state = UiObservationRuntimeState::new();
        let current = UiObservationProgress::committed_scroll_extent(7);
        assert_eq!(
            state.order_posture(&current),
            UiObservationOrderPosture::Fresh
        );
        state.finish_committed([&current]);

        let duplicate = UiObservationProgress::committed_scroll_extent(7);
        let historical = UiObservationProgress::committed_scroll_extent(6);
        let successor = UiObservationProgress::committed_scroll_extent(8);
        assert_eq!(
            state.order_posture(&duplicate),
            UiObservationOrderPosture::Duplicate
        );
        assert_eq!(
            state.order_posture(&historical),
            UiObservationOrderPosture::Historical
        );
        assert_eq!(
            state.order_posture(&successor),
            UiObservationOrderPosture::Fresh
        );
    }
}
