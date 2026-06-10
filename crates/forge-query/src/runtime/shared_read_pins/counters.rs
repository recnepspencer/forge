#![cfg(test)]

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(in crate::runtime) struct ForgeQuerySharedReadCounters {
    orphaned_generation_count: usize,
    unretired_pin_count: usize,
}

impl ForgeQuerySharedReadCounters {
    pub(in crate::runtime) fn new(
        orphaned_generation_count: usize,
        unretired_pin_count: usize,
    ) -> Self {
        Self {
            orphaned_generation_count,
            unretired_pin_count,
        }
    }

    pub(in crate::runtime) fn orphaned_generation_count(self) -> usize {
        self.orphaned_generation_count
    }

    pub(in crate::runtime) fn unretired_pin_count(self) -> usize {
        self.unretired_pin_count
    }
}
