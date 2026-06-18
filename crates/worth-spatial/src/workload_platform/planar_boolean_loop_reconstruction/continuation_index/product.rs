use std::collections::BTreeMap;

use crate::workload_platform::planar_boolean_events::PlanarBooleanSourceIntervalSense;

use super::construction::build_fragment_continuation_index;
use super::counters::PlanarBooleanFragmentContinuationCounters;
use super::denial::PlanarBooleanFragmentContinuationDenial;
use super::input::PlanarBooleanFragmentContinuationIndexInput;
use super::neighborhood::PlanarBooleanFragmentContinuationNeighborhoodView;
use super::ordering::{continuation_order_key, PlanarBooleanContinuationOrderingBasis};
use super::row::PlanarBooleanFragmentContinuationRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanFragmentContinuationIndex {
    continuation_index_identity: String,
    request_identity: String,
    source_provenance_bundle_identity: String,
    split_vertex_identity_set_identity: String,
    fragment_set_identity: String,
    overlap_chain_set_identity: String,
    rows: Vec<PlanarBooleanFragmentContinuationRow>,
    ordering_basis: PlanarBooleanContinuationOrderingBasis,
    neighborhood_offsets: BTreeMap<String, BTreeMap<String, NeighborhoodOffsets>>,
    counters: PlanarBooleanFragmentContinuationCounters,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct NeighborhoodOffsets {
    forward: Vec<usize>,
    reversed: Vec<usize>,
}

impl PlanarBooleanFragmentContinuationIndex {
    pub fn admit(
        input: PlanarBooleanFragmentContinuationIndexInput<'_>,
    ) -> Result<Self, PlanarBooleanFragmentContinuationDenial> {
        build_fragment_continuation_index(input)
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        continuation_index_identity: String,
        request_identity: String,
        source_provenance_bundle_identity: String,
        split_vertex_identity_set_identity: String,
        fragment_set_identity: String,
        overlap_chain_set_identity: String,
        rows: Vec<PlanarBooleanFragmentContinuationRow>,
        ordering_basis: PlanarBooleanContinuationOrderingBasis,
        counters: PlanarBooleanFragmentContinuationCounters,
    ) -> Self {
        let mut neighborhood_offsets =
            BTreeMap::<String, BTreeMap<String, NeighborhoodOffsets>>::new();
        for (offset, row) in rows.iter().enumerate() {
            neighborhood_offsets
                .entry(row.split_vertex_identity().to_string())
                .or_default()
                .entry(row.source_loop_identity().to_string())
                .or_default()
                .push(row.source_sense(), offset);
        }
        Self {
            continuation_index_identity,
            request_identity,
            source_provenance_bundle_identity,
            split_vertex_identity_set_identity,
            fragment_set_identity,
            overlap_chain_set_identity,
            rows,
            ordering_basis,
            neighborhood_offsets,
            counters,
        }
    }

    pub fn continuation_index_identity(&self) -> &str {
        &self.continuation_index_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn source_provenance_bundle_identity(&self) -> &str {
        &self.source_provenance_bundle_identity
    }

    pub fn split_vertex_identity_set_identity(&self) -> &str {
        &self.split_vertex_identity_set_identity
    }

    pub fn fragment_set_identity(&self) -> &str {
        &self.fragment_set_identity
    }

    pub fn overlap_chain_set_identity(&self) -> &str {
        &self.overlap_chain_set_identity
    }

    pub fn rows(&self) -> &[PlanarBooleanFragmentContinuationRow] {
        &self.rows
    }

    pub fn ordering_basis(&self) -> &PlanarBooleanContinuationOrderingBasis {
        &self.ordering_basis
    }

    pub fn ordered_rows_with_basis(
        &self,
    ) -> impl Iterator<
        Item = (
            &PlanarBooleanFragmentContinuationRow,
            super::ordering::PlanarBooleanContinuationOrderingKey<'_>,
        ),
    > {
        self.rows
            .iter()
            .map(|row| (row, continuation_order_key(row)))
    }

    pub fn counters(&self) -> PlanarBooleanFragmentContinuationCounters {
        self.counters
    }

    pub fn neighborhood(
        &self,
        split_vertex_identity: &str,
        source_loop_identity: &str,
        source_sense: PlanarBooleanSourceIntervalSense,
    ) -> Option<PlanarBooleanFragmentContinuationNeighborhoodView<'_>> {
        let offsets = self
            .neighborhood_offsets
            .get(split_vertex_identity)
            .and_then(|by_loop| by_loop.get(source_loop_identity))
            .map(|offsets| offsets.for_sense(source_sense))?;
        let first_row = offsets.first().and_then(|offset| self.rows.get(*offset))?;
        Some(PlanarBooleanFragmentContinuationNeighborhoodView::new(
            self,
            first_row.split_vertex_identity(),
            first_row.source_loop_identity(),
            source_sense,
            offsets,
        ))
    }

    pub fn continuations_for_neighborhood(
        &self,
        split_vertex_identity: &str,
        source_loop_identity: &str,
        source_sense: PlanarBooleanSourceIntervalSense,
    ) -> impl Iterator<Item = &PlanarBooleanFragmentContinuationRow> {
        self.neighborhood(split_vertex_identity, source_loop_identity, source_sense)
            .into_iter()
            .flat_map(|neighborhood| neighborhood.rows())
    }
}

impl NeighborhoodOffsets {
    fn push(&mut self, source_sense: PlanarBooleanSourceIntervalSense, offset: usize) {
        match source_sense {
            PlanarBooleanSourceIntervalSense::Forward => self.forward.push(offset),
            PlanarBooleanSourceIntervalSense::Reversed => self.reversed.push(offset),
        }
    }

    fn for_sense(&self, source_sense: PlanarBooleanSourceIntervalSense) -> &[usize] {
        match source_sense {
            PlanarBooleanSourceIntervalSense::Forward => &self.forward,
            PlanarBooleanSourceIntervalSense::Reversed => &self.reversed,
        }
    }
}
