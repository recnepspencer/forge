#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalWorkCapacity {
    commands: usize,
    scope_members_per_work: usize,
    total_scope_members: usize,
    semantic_bytes_per_work: usize,
    total_semantic_bytes: usize,
}

impl PhysicalWorkCapacity {
    pub fn new(
        commands: usize,
        scope_members_per_work: usize,
        total_scope_members: usize,
        semantic_bytes_per_work: usize,
        total_semantic_bytes: usize,
    ) -> Option<Self> {
        if commands == 0
            || scope_members_per_work == 0
            || total_scope_members < scope_members_per_work
            || semantic_bytes_per_work == 0
            || total_semantic_bytes < semantic_bytes_per_work
        {
            return None;
        }
        Some(Self {
            commands,
            scope_members_per_work,
            total_scope_members,
            semantic_bytes_per_work,
            total_semantic_bytes,
        })
    }

    pub const fn commands(self) -> usize {
        self.commands
    }
    pub const fn scope_members_per_work(self) -> usize {
        self.scope_members_per_work
    }
    pub const fn total_scope_members(self) -> usize {
        self.total_scope_members
    }
    pub const fn semantic_bytes_per_work(self) -> usize {
        self.semantic_bytes_per_work
    }
    pub const fn total_semantic_bytes(self) -> usize {
        self.total_semantic_bytes
    }
}

impl Default for PhysicalWorkCapacity {
    fn default() -> Self {
        Self {
            commands: 1_024,
            scope_members_per_work: 256,
            total_scope_members: 32_768,
            semantic_bytes_per_work: 1024 * 1024,
            total_semantic_bytes: 64 * 1024 * 1024,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PhysicalWorkCapacity;

    #[test]
    fn totals_cannot_be_smaller_than_per_work_limits() {
        assert!(PhysicalWorkCapacity::new(1, 2, 1, 1, 1).is_none());
        assert!(PhysicalWorkCapacity::new(1, 1, 1, 2, 1).is_none());
    }
}
