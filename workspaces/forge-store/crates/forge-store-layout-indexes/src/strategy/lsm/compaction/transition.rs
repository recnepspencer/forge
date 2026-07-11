/// Strategy-owned state change established by ordinary LSM compaction.
///
/// The type is public for lower Store consumers, but its successful value can
/// only be obtained from a WAL execution receipt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BaselineLsmCompactionTransition {
    tombstone_retention_admitted: bool,
}

impl BaselineLsmCompactionTransition {
    pub(super) const fn tombstone_retention_admitted() -> Self {
        Self {
            tombstone_retention_admitted: true,
        }
    }

    pub const fn is_tombstone_retention_admitted(self) -> bool {
        self.tombstone_retention_admitted
    }
}
