#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ResolvedRepairRegion {
    integrity: worth_store_physical_integrity::IntegrityRepairRegion,
    pub(super) source: std::path::PathBuf,
}

impl ResolvedRepairRegion {
    pub(super) fn new(
        integrity: worth_store_physical_integrity::IntegrityRepairRegion,
        source: std::path::PathBuf,
    ) -> Self {
        Self { integrity, source }
    }
    pub(super) const fn integrity(&self) -> worth_store_physical_integrity::IntegrityRepairRegion {
        self.integrity
    }
}

impl PartialOrd for ResolvedRepairRegion {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ResolvedRepairRegion {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.integrity
            .cmp(&other.integrity)
            .then_with(|| self.source.cmp(&other.source))
    }
}
