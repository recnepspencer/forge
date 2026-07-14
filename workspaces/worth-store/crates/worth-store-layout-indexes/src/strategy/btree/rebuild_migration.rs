use crate::strategy::StrategyDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BTreeRebuildMigrationLaw;

impl BTreeRebuildMigrationLaw {
    pub(crate) const fn baseline() -> Self {
        Self
    }

    pub const fn verify_rebuild_from_authority(
        self,
        rebuilt_node_count: u16,
        authoritative_page_count: u16,
        ordering_preserved: bool,
    ) -> Result<(), StrategyDenial> {
        if rebuilt_node_count == authoritative_page_count && ordering_preserved {
            return Ok(());
        }
        Err(StrategyDenial::RebuildMigrationViolation)
    }
}
