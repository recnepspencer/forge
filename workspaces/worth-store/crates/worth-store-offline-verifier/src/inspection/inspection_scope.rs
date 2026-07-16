#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OfflineInspectionScope;

impl OfflineInspectionScope {
    pub const fn all_physical_families() -> Self {
        Self
    }
    pub const fn includes(
        self,
        _family: worth_store_physical_format::OfflinePhysicalArtifactFamily,
    ) -> bool {
        true
    }
}
