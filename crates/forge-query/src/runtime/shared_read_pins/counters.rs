#![cfg(test)]

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(in crate::runtime) struct ForgeQuerySharedReadCounters {
    committed_read_hot_path_lock_count: usize,
    orphaned_generation_count: usize,
    unretired_pin_count: usize,
}

impl ForgeQuerySharedReadCounters {
    pub(in crate::runtime) fn new(
        committed_read_hot_path_lock_count: usize,
        orphaned_generation_count: usize,
        unretired_pin_count: usize,
    ) -> Self {
        Self {
            committed_read_hot_path_lock_count,
            orphaned_generation_count,
            unretired_pin_count,
        }
    }

    pub(in crate::runtime) fn committed_read_hot_path_lock_count(self) -> usize {
        self.committed_read_hot_path_lock_count
    }

    pub(in crate::runtime) fn orphaned_generation_count(self) -> usize {
        self.orphaned_generation_count
    }

    pub(in crate::runtime) fn unretired_pin_count(self) -> usize {
        self.unretired_pin_count
    }
}
