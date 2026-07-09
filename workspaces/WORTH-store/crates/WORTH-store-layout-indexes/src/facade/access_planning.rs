use crate::access_shape::{
    access_shapes, S8AccessLaneClassification, S8AccessShapeContract,
    S8AccessShapeUnsupportedDenial, S8DegradedExactScanRequest, S8FullDeclaredScanBasis,
};
use crate::artifact_family::PhysicalArtifactFamilyDeclaration;
use crate::materialization::{
    S8CoverageGapWitness, S8LayoutCoverageWitness, S8LayoutMaterializationState,
    S8MaterializationDenial, S8PhysicalAbsenceProof, S8PhysicalCoverageBasis,
};
use crate::planning::S8AccessPlanSelection;
use worth_store_blob_chunks::BlobGeneration;
use worth_store_physical_format::PhysicalEpoch;
use worth_store_recovery_physics::{CheckpointCoveredLsnRange, LogSequenceNumber};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccessPlanningFacade;

impl AccessPlanningFacade {
    pub const fn selection(&self) -> S8AccessPlanSelection {
        S8AccessPlanSelection
    }

    pub fn exact_root_epoch_coverage(
        &self,
        materialization: S8LayoutMaterializationState,
        epoch: PhysicalEpoch,
    ) -> Result<S8LayoutCoverageWitness, S8MaterializationDenial> {
        S8LayoutCoverageWitness::exact_through(
            materialization,
            S8PhysicalCoverageBasis::root_epoch(epoch).watermark(),
        )
    }

    pub fn exact_wal_lsn_coverage(
        &self,
        materialization: S8LayoutMaterializationState,
        lsn: LogSequenceNumber,
    ) -> Result<S8LayoutCoverageWitness, S8MaterializationDenial> {
        S8LayoutCoverageWitness::exact_through(
            materialization,
            S8PhysicalCoverageBasis::wal_lsn(lsn).watermark(),
        )
    }

    pub fn exact_blob_generation_coverage(
        &self,
        materialization: S8LayoutMaterializationState,
        generation: BlobGeneration,
    ) -> Result<S8LayoutCoverageWitness, S8MaterializationDenial> {
        S8LayoutCoverageWitness::exact_through(
            materialization,
            S8PhysicalCoverageBasis::blob_generation(generation).watermark(),
        )
    }

    pub fn exact_checkpoint_coverage(
        &self,
        materialization: S8LayoutMaterializationState,
        range: CheckpointCoveredLsnRange,
    ) -> Result<S8LayoutCoverageWitness, S8MaterializationDenial> {
        S8LayoutCoverageWitness::exact_through(
            materialization,
            S8PhysicalCoverageBasis::checkpoint_frontier(range).watermark(),
        )
    }

    pub fn stale_root_epoch_coverage(
        &self,
        declaration: &'static PhysicalArtifactFamilyDeclaration,
        epoch: PhysicalEpoch,
    ) -> Result<S8LayoutCoverageWitness, S8MaterializationDenial> {
        S8LayoutCoverageWitness::exact_through(
            S8LayoutMaterializationState::stale(declaration.family()),
            S8PhysicalCoverageBasis::root_epoch(epoch).watermark(),
        )
    }

    pub fn lagged_wal_lsn_coverage(
        &self,
        declaration: &'static PhysicalArtifactFamilyDeclaration,
        lower_bound: LogSequenceNumber,
        upper_bound: LogSequenceNumber,
    ) -> Result<S8LayoutCoverageWitness, S8MaterializationDenial> {
        S8LayoutCoverageWitness::lagged(
            S8LayoutMaterializationState::lagged(declaration.family()),
            S8PhysicalCoverageBasis::wal_lsn(lower_bound).watermark(),
            S8PhysicalCoverageBasis::wal_lsn(upper_bound).watermark(),
        )
    }

    pub fn partial_wal_lsn_coverage(
        &self,
        declaration: &'static PhysicalArtifactFamilyDeclaration,
        lower_bound: LogSequenceNumber,
        upper_bound: LogSequenceNumber,
        gap: CheckpointCoveredLsnRange,
    ) -> Result<S8LayoutCoverageWitness, S8MaterializationDenial> {
        let gap = S8CoverageGapWitness::physical_range(
            declaration.family(),
            S8PhysicalCoverageBasis::checkpoint_frontier(gap).basis_kind(),
            gap.range().start().get(),
            gap.range().end_exclusive().get(),
        );

        S8LayoutCoverageWitness::partially_covered(
            S8LayoutMaterializationState::partially_covered(declaration.family()),
            S8PhysicalCoverageBasis::wal_lsn(lower_bound).watermark(),
            S8PhysicalCoverageBasis::wal_lsn(upper_bound).watermark(),
            gap,
        )
    }

    pub fn quarantined_wal_lsn_coverage(
        &self,
        declaration: &'static PhysicalArtifactFamilyDeclaration,
        lower_bound: LogSequenceNumber,
        upper_bound: LogSequenceNumber,
        gap: CheckpointCoveredLsnRange,
    ) -> Result<S8LayoutCoverageWitness, S8MaterializationDenial> {
        let gap = S8CoverageGapWitness::physical_range(
            declaration.family(),
            S8PhysicalCoverageBasis::checkpoint_frontier(gap).basis_kind(),
            gap.range().start().get(),
            gap.range().end_exclusive().get(),
        );

        S8LayoutCoverageWitness::partially_covered(
            S8LayoutMaterializationState::quarantined(declaration.family()),
            S8PhysicalCoverageBasis::wal_lsn(lower_bound).watermark(),
            S8PhysicalCoverageBasis::wal_lsn(upper_bound).watermark(),
            gap,
        )
    }

    pub fn require_exact_point_access(
        &self,
        coverage: S8LayoutCoverageWitness,
    ) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
        access_shapes().point_lookup(coverage)
    }

    pub fn require_exact_range_access(
        &self,
        coverage: S8LayoutCoverageWitness,
    ) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
        access_shapes().range_lookup(coverage)
    }

    pub fn require_exact_prefix_access(
        &self,
        coverage: S8LayoutCoverageWitness,
    ) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
        access_shapes().prefix_lookup(coverage)
    }

    pub fn prove_exact_index_absence(
        &self,
        coverage: S8LayoutCoverageWitness,
    ) -> Result<S8PhysicalAbsenceProof, S8MaterializationDenial> {
        S8PhysicalAbsenceProof::exact_index(coverage)
    }

    pub fn prove_degraded_bounded_scan_absence(
        &self,
        coverage: S8LayoutCoverageWitness,
    ) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
        access_shapes().explicit_degraded_exact_scan(
            S8DegradedExactScanRequest::new(
                coverage
                    .require_exact()
                    .map_err(S8AccessShapeUnsupportedDenial::MaterializationDenied)?,
            )
            .with_budget_rows(8_192),
        )
    }

    pub fn require_full_declared_scan(
        &self,
        coverage: S8LayoutCoverageWitness,
        lane: S8AccessLaneClassification,
    ) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
        access_shapes().full_declared_scan(
            coverage,
            lane,
            S8FullDeclaredScanBasis::DeclaredFullTraversal,
        )
    }
}

pub const fn access_planning() -> AccessPlanningFacade {
    AccessPlanningFacade
}
