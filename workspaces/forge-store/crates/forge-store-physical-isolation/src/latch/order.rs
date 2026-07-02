use std::cmp::Ordering;

use super::{LatchAcquisitionStep, PhysicalLatchKey, PhysicalLatchMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalLatchAcquisitionOrder;

impl CanonicalLatchAcquisitionOrder {
    pub fn sort_steps(steps: &mut [LatchAcquisitionStep]) {
        steps.sort_by(Self::compare_steps);
    }

    pub fn is_canonical(steps: &[LatchAcquisitionStep]) -> bool {
        steps
            .windows(2)
            .all(|window| Self::compare_steps(&window[0], &window[1]) != Ordering::Greater)
    }

    pub fn compare_steps(left: &LatchAcquisitionStep, right: &LatchAcquisitionStep) -> Ordering {
        latch_key_order_tuple(left.key())
            .cmp(&latch_key_order_tuple(right.key()))
            .then_with(|| latch_mode_rank(left.mode()).cmp(&latch_mode_rank(right.mode())))
            .then_with(|| latch_action_rank(*left).cmp(&latch_action_rank(*right)))
    }
}

fn latch_key_order_tuple(key: PhysicalLatchKey) -> (u64, u8, u64, u64) {
    key.canonical_order_tuple()
}

fn latch_mode_rank(mode: PhysicalLatchMode) -> u8 {
    match mode {
        PhysicalLatchMode::Shared => 0,
        PhysicalLatchMode::Exclusive => 1,
    }
}

fn latch_action_rank(step: LatchAcquisitionStep) -> u8 {
    if step.is_upgrade() {
        1
    } else {
        0
    }
}
