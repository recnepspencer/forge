use super::append::{append_path, compaction_read};
use super::contract::S8AccessShapeContract;
use super::degraded::{explicit_degraded_exact_scan, S8DegradedExactScanRequest};
use super::denial::S8AccessShapeUnsupportedDenial;
use super::detail::{
    S8BatchPointBasis, S8BoundedScanBasis, S8FullDeclaredScanBasis, S8GroupedPrefixBasis,
    S8MultiRangeBasis, S8SortedBatchBasis, S8StreamingContinuationBasis,
};
use super::lane::S8AccessLaneClassification;
use super::point::{batch_point_lookup, point_lookup, sorted_batch_lookup};
use super::prefix::{grouped_prefix_lookup, prefix_lookup};
use super::quarantine::quarantine_read;
use super::range::{multi_range_lookup, range_lookup};
use super::rebuild::rebuild_read;
use super::repair::repair_read;
use super::scan::{bounded_scan, full_declared_scan, manifest_graph_walk};
use super::streaming::{
    chunk_tree_walk, coalesced_page_read, streaming_continuation_read, streaming_read,
};
use super::verifier::verifier_read;
use crate::maintenance::S8PhysicalMutationShape;
use crate::materialization::S8LayoutCoverageWitness;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8AccessShapesFacade;

impl S8AccessShapesFacade {
    pub fn point_lookup(
        &self,
        coverage: S8LayoutCoverageWitness,
    ) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
        point_lookup(coverage)
    }

    pub fn batch_point_lookup(
        &self,
        coverage: S8LayoutCoverageWitness,
        basis: S8BatchPointBasis,
    ) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
        batch_point_lookup(coverage, basis)
    }

    pub fn sorted_batch_lookup(
        &self,
        coverage: S8LayoutCoverageWitness,
        basis: S8SortedBatchBasis,
    ) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
        sorted_batch_lookup(coverage, basis)
    }

    pub fn range_lookup(
        &self,
        coverage: S8LayoutCoverageWitness,
    ) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
        range_lookup(coverage)
    }

    pub fn multi_range_lookup(
        &self,
        coverage: S8LayoutCoverageWitness,
        basis: S8MultiRangeBasis,
    ) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
        multi_range_lookup(coverage, basis)
    }

    pub fn prefix_lookup(
        &self,
        coverage: S8LayoutCoverageWitness,
    ) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
        prefix_lookup(coverage)
    }

    pub fn grouped_prefix_lookup(
        &self,
        coverage: S8LayoutCoverageWitness,
        basis: S8GroupedPrefixBasis,
    ) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
        grouped_prefix_lookup(coverage, basis)
    }

    pub fn coalesced_page_read(
        &self,
        coverage: S8LayoutCoverageWitness,
    ) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
        coalesced_page_read(coverage)
    }

    pub fn chunk_tree_walk(
        &self,
        coverage: S8LayoutCoverageWitness,
        lane: S8AccessLaneClassification,
    ) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
        chunk_tree_walk(coverage, lane)
    }

    pub fn manifest_graph_walk(
        &self,
        coverage: S8LayoutCoverageWitness,
        lane: S8AccessLaneClassification,
    ) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
        manifest_graph_walk(coverage, lane)
    }

    pub fn bounded_scan(
        &self,
        coverage: S8LayoutCoverageWitness,
        lane: S8AccessLaneClassification,
        basis: S8BoundedScanBasis,
    ) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
        bounded_scan(coverage, lane, basis)
    }

    pub fn full_declared_scan(
        &self,
        coverage: S8LayoutCoverageWitness,
        lane: S8AccessLaneClassification,
        basis: S8FullDeclaredScanBasis,
    ) -> super::scan::S8FullDeclaredScanOutcome {
        full_declared_scan(coverage, lane, basis)
    }

    pub fn streaming_read(
        &self,
        coverage: S8LayoutCoverageWitness,
        lane: S8AccessLaneClassification,
    ) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
        streaming_read(coverage, lane)
    }

    pub fn streaming_continuation_read(
        &self,
        coverage: S8LayoutCoverageWitness,
        lane: S8AccessLaneClassification,
        basis: S8StreamingContinuationBasis,
    ) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
        streaming_continuation_read(coverage, lane, basis)
    }

    pub fn append(
        &self,
        mutation_shape: S8PhysicalMutationShape,
    ) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
        append_path(mutation_shape)
    }

    pub fn compaction_read(
        &self,
        mutation_shape: S8PhysicalMutationShape,
    ) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
        compaction_read(mutation_shape)
    }

    pub fn rebuild_read(
        &self,
        coverage: S8LayoutCoverageWitness,
        lane: S8AccessLaneClassification,
    ) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
        rebuild_read(coverage, lane)
    }

    pub fn verifier_read(
        &self,
        coverage: S8LayoutCoverageWitness,
        lane: S8AccessLaneClassification,
    ) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
        verifier_read(coverage, lane)
    }

    pub fn repair_read(
        &self,
        coverage: S8LayoutCoverageWitness,
        lane: S8AccessLaneClassification,
    ) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
        repair_read(coverage, lane)
    }

    pub fn quarantine_read(
        &self,
        coverage: S8LayoutCoverageWitness,
        lane: S8AccessLaneClassification,
    ) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
        quarantine_read(coverage, lane)
    }

    pub fn explicit_degraded_exact_scan(
        &self,
        request: S8DegradedExactScanRequest,
    ) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
        explicit_degraded_exact_scan(request)
    }
}

pub const fn access_shapes() -> S8AccessShapesFacade {
    S8AccessShapesFacade
}
