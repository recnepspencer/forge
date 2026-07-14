use super::append::{append_path, compaction_read};
use super::contract::AccessShapeContract;
use super::degraded::{explicit_degraded_exact_scan, DegradedExactScanRequest};
use super::denial::AccessShapeUnsupportedDenial;
use super::detail::FullDeclaredScanBasis;
#[cfg(test)]
use super::detail::{
    BoundedScanBasis, GroupedPrefixBasis, MultiRangeBasis, StreamingContinuationBasis,
};
use super::lane::AccessLaneClassification;
use super::point::point_lookup_declaration;
#[cfg(test)]
use super::prefix::grouped_prefix_lookup_declaration;
use super::prefix::prefix_lookup_declaration;
#[cfg(test)]
use super::quarantine::quarantine_read;
#[cfg(test)]
use super::range::multi_range_lookup_declaration;
use super::range::range_lookup_declaration;
#[cfg(test)]
use super::rebuild::rebuild_read;
use super::rebuild::rebuild_read_declaration;
#[cfg(test)]
use super::repair::repair_read;
use super::scan::full_declared_scan;
#[cfg(test)]
use super::scan::{bounded_scan, manifest_graph_walk};
#[cfg(test)]
use super::streaming::{
    chunk_tree_walk, coalesced_page_read, streaming_continuation_read, streaming_read,
};
#[cfg(test)]
use super::verifier::verifier_read;
use crate::maintenance::PhysicalMutationShape;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccessShapesFacade;

impl AccessShapesFacade {
    pub const fn point_lookup_declaration(&self) -> AccessShapeContract {
        point_lookup_declaration()
    }

    pub const fn range_lookup_declaration(&self) -> AccessShapeContract {
        range_lookup_declaration()
    }

    pub const fn prefix_lookup_declaration(&self) -> AccessShapeContract {
        prefix_lookup_declaration()
    }

    pub fn rebuild_read_declaration(
        &self,
        lane: AccessLaneClassification,
    ) -> Result<AccessShapeContract, AccessShapeUnsupportedDenial> {
        rebuild_read_declaration(lane)
    }

    #[cfg(test)]
    pub const fn multi_range_lookup_declaration(
        &self,
        basis: MultiRangeBasis,
    ) -> AccessShapeContract {
        multi_range_lookup_declaration(basis)
    }

    #[cfg(test)]
    pub const fn grouped_prefix_lookup_declaration(
        &self,
        basis: GroupedPrefixBasis,
    ) -> AccessShapeContract {
        grouped_prefix_lookup_declaration(basis)
    }

    #[cfg(test)]
    pub fn coalesced_page_read(&self) -> Result<AccessShapeContract, AccessShapeUnsupportedDenial> {
        coalesced_page_read()
    }

    #[cfg(test)]
    pub fn chunk_tree_walk(
        &self,
        lane: AccessLaneClassification,
    ) -> Result<AccessShapeContract, AccessShapeUnsupportedDenial> {
        chunk_tree_walk(lane)
    }

    #[cfg(test)]
    pub fn manifest_graph_walk(
        &self,
        lane: AccessLaneClassification,
    ) -> Result<AccessShapeContract, AccessShapeUnsupportedDenial> {
        manifest_graph_walk(lane)
    }

    #[cfg(test)]
    pub fn bounded_scan(
        &self,
        lane: AccessLaneClassification,
        basis: BoundedScanBasis,
    ) -> Result<AccessShapeContract, AccessShapeUnsupportedDenial> {
        bounded_scan(lane, basis)
    }

    pub fn full_declared_scan(
        &self,
        lane: AccessLaneClassification,
        basis: FullDeclaredScanBasis,
    ) -> super::scan::FullDeclaredScanOutcome {
        full_declared_scan(lane, basis)
    }

    #[cfg(test)]
    pub fn streaming_read(
        &self,
        lane: AccessLaneClassification,
    ) -> Result<AccessShapeContract, AccessShapeUnsupportedDenial> {
        streaming_read(lane)
    }

    #[cfg(test)]
    pub fn streaming_continuation_read(
        &self,
        lane: AccessLaneClassification,
        basis: StreamingContinuationBasis,
    ) -> Result<AccessShapeContract, AccessShapeUnsupportedDenial> {
        streaming_continuation_read(lane, basis)
    }

    pub fn append(
        &self,
        mutation_shape: PhysicalMutationShape,
    ) -> Result<AccessShapeContract, AccessShapeUnsupportedDenial> {
        append_path(mutation_shape)
    }

    pub fn compaction_read(
        &self,
        mutation_shape: PhysicalMutationShape,
    ) -> Result<AccessShapeContract, AccessShapeUnsupportedDenial> {
        compaction_read(mutation_shape)
    }

    #[cfg(test)]
    pub fn rebuild_read(
        &self,
        lane: AccessLaneClassification,
    ) -> Result<AccessShapeContract, AccessShapeUnsupportedDenial> {
        rebuild_read(lane)
    }

    #[cfg(test)]
    pub fn verifier_read(
        &self,
        lane: AccessLaneClassification,
    ) -> Result<AccessShapeContract, AccessShapeUnsupportedDenial> {
        verifier_read(lane)
    }

    #[cfg(test)]
    pub fn repair_read(
        &self,
        lane: AccessLaneClassification,
    ) -> Result<AccessShapeContract, AccessShapeUnsupportedDenial> {
        repair_read(lane)
    }

    #[cfg(test)]
    pub fn quarantine_read(
        &self,
        lane: AccessLaneClassification,
    ) -> Result<AccessShapeContract, AccessShapeUnsupportedDenial> {
        quarantine_read(lane)
    }

    pub fn explicit_degraded_exact_scan(
        &self,
        request: DegradedExactScanRequest,
    ) -> Result<AccessShapeContract, AccessShapeUnsupportedDenial> {
        explicit_degraded_exact_scan(request)
    }
}

pub const fn access_shapes() -> AccessShapesFacade {
    AccessShapesFacade
}
