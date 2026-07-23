#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiOrdinaryHostOutput {
    target: WorthUiOrdinaryHostOutputTarget,
    touched_row_count: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiOrdinaryHostOutputTarget {
    RootShell,
    Component,
    ChildRange,
    Command,
    TokenSupport,
    StateSlot,
}

impl WorthUiOrdinaryHostOutput {
    pub fn new(target: WorthUiOrdinaryHostOutputTarget, touched_row_count: usize) -> Self {
        Self {
            target,
            touched_row_count,
        }
    }

    pub fn target(self) -> WorthUiOrdinaryHostOutputTarget {
        self.target
    }

    pub fn touched_row_count(self) -> usize {
        self.touched_row_count
    }

    pub fn meaning_digest(self) -> u64 {
        (u64::from(self.target as u8) ^ (self.touched_row_count as u64).rotate_left(29))
            .wrapping_mul(0x100000001b3)
    }
}
