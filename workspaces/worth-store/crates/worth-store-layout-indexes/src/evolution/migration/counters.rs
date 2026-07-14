use worth_store_physical_isolation::PhysicalPublicationCounterSnapshot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutMigrationCounterSnapshot {
    physical_publication: PhysicalPublicationCounterSnapshot,
    target_bindings_published: u64,
}

impl LayoutMigrationCounterSnapshot {
    pub(super) const fn published(physical: PhysicalPublicationCounterSnapshot) -> Self {
        Self {
            physical_publication: physical,
            target_bindings_published: 1,
        }
    }

    pub const fn physical_publication(self) -> PhysicalPublicationCounterSnapshot {
        self.physical_publication
    }

    pub const fn target_bindings_published(self) -> u64 {
        self.target_bindings_published
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutRollbackCounterSnapshot {
    physical_publication: PhysicalPublicationCounterSnapshot,
    rollback_bindings_published: u64,
}

impl LayoutRollbackCounterSnapshot {
    pub(super) const fn published(physical: PhysicalPublicationCounterSnapshot) -> Self {
        Self {
            physical_publication: physical,
            rollback_bindings_published: 1,
        }
    }

    pub const fn physical_publication(self) -> PhysicalPublicationCounterSnapshot {
        self.physical_publication
    }

    pub const fn rollback_bindings_published(self) -> u64 {
        self.rollback_bindings_published
    }
}
