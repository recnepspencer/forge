use crate::workload_platform::planar_boolean_events::PlanarBooleanSourceIntervalSense;

use super::product::PlanarBooleanFragmentContinuationIndex;
use super::row::PlanarBooleanFragmentContinuationRow;

#[derive(Clone, Copy)]
pub struct PlanarBooleanFragmentContinuationNeighborhoodView<'a> {
    index: &'a PlanarBooleanFragmentContinuationIndex,
    split_vertex_identity: &'a str,
    source_loop_identity: &'a str,
    source_sense: PlanarBooleanSourceIntervalSense,
    offsets: &'a [usize],
}

impl<'a> PlanarBooleanFragmentContinuationNeighborhoodView<'a> {
    pub(crate) fn new(
        index: &'a PlanarBooleanFragmentContinuationIndex,
        split_vertex_identity: &'a str,
        source_loop_identity: &'a str,
        source_sense: PlanarBooleanSourceIntervalSense,
        offsets: &'a [usize],
    ) -> Self {
        Self {
            index,
            split_vertex_identity,
            source_loop_identity,
            source_sense,
            offsets,
        }
    }

    pub fn neighborhood_identity(&self) -> &'a str {
        self.first_row()
            .expect("continuation neighborhood view requires at least one row")
            .neighborhood_identity()
    }

    pub fn split_vertex_identity(&self) -> &'a str {
        self.split_vertex_identity
    }

    pub fn source_loop_identity(&self) -> &'a str {
        self.source_loop_identity
    }

    pub fn source_sense(&self) -> PlanarBooleanSourceIntervalSense {
        self.source_sense
    }

    pub fn len(&self) -> usize {
        self.offsets.len()
    }

    pub fn is_empty(&self) -> bool {
        self.offsets.is_empty()
    }

    pub fn rows(&self) -> impl Iterator<Item = &'a PlanarBooleanFragmentContinuationRow> {
        self.offsets
            .iter()
            .filter_map(|offset| self.index.rows().get(*offset))
    }

    fn first_row(&self) -> Option<&'a PlanarBooleanFragmentContinuationRow> {
        self.offsets
            .first()
            .and_then(|offset| self.index.rows().get(*offset))
    }
}
