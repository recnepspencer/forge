use super::super::participation::{
    PlanarBooleanLoopIslandOverlapParticipationMap, PlanarBooleanLoopOverlapParticipationMap,
    PlanarBooleanOverlapChainRegionLineageMap,
};

pub struct PlanarBooleanOverlapAdjacencyIndexInput<'a> {
    loop_participation_map: &'a PlanarBooleanLoopOverlapParticipationMap,
    island_participation_map: &'a PlanarBooleanLoopIslandOverlapParticipationMap,
    chain_lineage_map: &'a PlanarBooleanOverlapChainRegionLineageMap,
}

impl<'a> PlanarBooleanOverlapAdjacencyIndexInput<'a> {
    pub fn from_participation_products(
        loop_participation_map: &'a PlanarBooleanLoopOverlapParticipationMap,
        island_participation_map: &'a PlanarBooleanLoopIslandOverlapParticipationMap,
        chain_lineage_map: &'a PlanarBooleanOverlapChainRegionLineageMap,
    ) -> Self {
        Self {
            loop_participation_map,
            island_participation_map,
            chain_lineage_map,
        }
    }

    pub(crate) fn loop_participation_map(&self) -> &'a PlanarBooleanLoopOverlapParticipationMap {
        self.loop_participation_map
    }

    pub(crate) fn island_participation_map(
        &self,
    ) -> &'a PlanarBooleanLoopIslandOverlapParticipationMap {
        self.island_participation_map
    }

    pub(crate) fn chain_lineage_map(&self) -> &'a PlanarBooleanOverlapChainRegionLineageMap {
        self.chain_lineage_map
    }
}
