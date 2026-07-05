use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapChainRegionLineageMap, PlanarBooleanSharedAreaAdmissionBundle,
};

#[derive(Clone, Copy)]
pub struct PlanarBooleanPreRegionNormalizationInput<'a> {
    shared_area_admission: &'a PlanarBooleanSharedAreaAdmissionBundle,
    chain_lineage_map: &'a PlanarBooleanOverlapChainRegionLineageMap,
}

impl<'a> PlanarBooleanPreRegionNormalizationInput<'a> {
    pub fn new(
        shared_area_admission: &'a PlanarBooleanSharedAreaAdmissionBundle,
        chain_lineage_map: &'a PlanarBooleanOverlapChainRegionLineageMap,
    ) -> Self {
        Self {
            shared_area_admission,
            chain_lineage_map,
        }
    }

    pub fn from_shared_area_admission(
        shared_area_admission: &'a PlanarBooleanSharedAreaAdmissionBundle,
        chain_lineage_map: &'a PlanarBooleanOverlapChainRegionLineageMap,
    ) -> Self {
        Self::new(shared_area_admission, chain_lineage_map)
    }

    pub fn shared_area_admission(self) -> &'a PlanarBooleanSharedAreaAdmissionBundle {
        self.shared_area_admission
    }

    pub fn chain_lineage_map(self) -> &'a PlanarBooleanOverlapChainRegionLineageMap {
        self.chain_lineage_map
    }
}
