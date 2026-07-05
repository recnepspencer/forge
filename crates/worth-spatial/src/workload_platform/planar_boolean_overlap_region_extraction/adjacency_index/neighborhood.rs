use super::product::PlanarBooleanOverlapRegionAdjacencyIndex;
use super::row::PlanarBooleanOverlapAdjacencyRow;

pub struct PlanarBooleanOverlapNeighborhoodView<'a> {
    index: &'a PlanarBooleanOverlapRegionAdjacencyIndex,
    anchor_identity: String,
    offsets: &'a [usize],
}

impl<'a> PlanarBooleanOverlapNeighborhoodView<'a> {
    pub(crate) fn new(
        index: &'a PlanarBooleanOverlapRegionAdjacencyIndex,
        anchor_identity: impl Into<String>,
        offsets: &'a [usize],
    ) -> Self {
        Self {
            index,
            anchor_identity: anchor_identity.into(),
            offsets,
        }
    }

    pub fn anchor_identity(&self) -> &str {
        &self.anchor_identity
    }

    pub fn rows(&self) -> impl Iterator<Item = &'a PlanarBooleanOverlapAdjacencyRow> {
        self.offsets
            .iter()
            .filter_map(|offset| self.index.rows().get(*offset))
    }
}
