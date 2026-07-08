use crate::strategy::S8StrategyDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8BTreeRebuildMigrationLaw;

impl S8BTreeRebuildMigrationLaw {
    pub(crate) const fn baseline() -> Self {
        Self
    }

    pub const fn verify_rebuild_from_authority(
        self,
        rebuilt_node_count: u16,
        authoritative_page_count: u16,
        ordering_preserved: bool,
    ) -> Result<(), S8StrategyDenial> {
        if rebuilt_node_count == authoritative_page_count && ordering_preserved {
            return Ok(());
        }
        Err(S8StrategyDenial::RebuildMigrationViolation)
    }
}
