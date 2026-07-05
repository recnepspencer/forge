use crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapRegionIdentityLineageBundle;

#[derive(Clone, Copy)]
pub struct PlanarBooleanOverlapRegionLedgerAssemblyInput<'a> {
    identity_lineage: &'a PlanarBooleanOverlapRegionIdentityLineageBundle,
}

impl<'a> PlanarBooleanOverlapRegionLedgerAssemblyInput<'a> {
    pub fn new(identity_lineage: &'a PlanarBooleanOverlapRegionIdentityLineageBundle) -> Self {
        Self { identity_lineage }
    }

    pub fn from_identity_lineage(
        identity_lineage: &'a PlanarBooleanOverlapRegionIdentityLineageBundle,
    ) -> Self {
        Self::new(identity_lineage)
    }

    pub fn identity_lineage(self) -> &'a PlanarBooleanOverlapRegionIdentityLineageBundle {
        self.identity_lineage
    }
}
