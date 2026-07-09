use std::collections::BTreeSet;

use super::{PinClass, RecordArena, RecordKind};

impl<K: RecordKind> RecordArena<K> {
    pub(crate) fn snapshot_pin_count(&self, slot: usize) -> Option<u32> {
        self.snapshot_pins.get(slot).copied()
    }

    pub(crate) fn branch_pin_count(&self, slot: usize) -> Option<u32> {
        self.branch_pins.get(slot).copied()
    }

    pub(crate) fn replay_pin_count(&self, slot: usize) -> Option<u32> {
        self.replay_pins.get(slot).copied()
    }

    pub(crate) fn increment_snapshot_pin(&mut self, slot: usize) -> Option<u32> {
        let count = self.snapshot_pins.get_mut(slot)?;
        *count = count.saturating_add(1);
        Some(*count)
    }

    pub(crate) fn decrement_snapshot_pin(&mut self, slot: usize) -> Option<u32> {
        let count = self.snapshot_pins.get_mut(slot)?;
        if *count == 0 {
            return None;
        }
        *count -= 1;
        Some(*count)
    }

    pub(crate) fn adjust_named_pin(&mut self, slot: usize, class: PinClass) -> Option<&mut u32> {
        match class {
            PinClass::Branch => self.branch_pins.get_mut(slot),
            PinClass::Replay => self.replay_pins.get_mut(slot),
        }
    }

    pub(crate) fn increment_named_pins_bulk(&mut self, slots: &BTreeSet<usize>, class: PinClass) {
        let pins = match class {
            PinClass::Branch => &mut self.branch_pins,
            PinClass::Replay => &mut self.replay_pins,
        };
        for &slot in slots {
            let Some(pin_count) = pins.get_mut(slot) else {
                continue;
            };
            *pin_count = pin_count.saturating_add(1);
        }
    }

    pub(crate) fn clear_all_pins(&mut self) {
        self.snapshot_pins.fill(0);
        self.branch_pins.fill(0);
        self.replay_pins.fill(0);
    }

    pub(crate) fn clear_named_pins(&mut self, class: PinClass) {
        match class {
            PinClass::Branch => self.branch_pins.fill(0),
            PinClass::Replay => self.replay_pins.fill(0),
        }
    }
}
