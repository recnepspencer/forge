use crate::workload_composition::CompiledProductReuseSurfaceIdentity;

use super::consumer_class::KernelCompiledProductConsumerResponsibility;
use super::dependency_row::{
    KernelCompiledProductConsumerClusterIdentity, KernelCompiledProductConsumerDependencyRow,
};
use super::error::KernelCompiledProductConsumerDependencyError;
use super::family_class::KernelCompiledProductFamilyClass;
use super::future_cutover_lane::KernelCompiledProductFutureCutoverLane;
use super::proof_basis::KernelCompiledProductProofBasis;
use super::query_boundary_lane::KernelCompiledProductQueryBoundaryLane;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct KernelCompiledProductConsumerCoverageTarget {
    cluster_identity: KernelCompiledProductConsumerClusterIdentity,
    current_source_path: &'static str,
    current_consumer_surface: &'static str,
    responsibility: KernelCompiledProductConsumerResponsibility,
    family_class: KernelCompiledProductFamilyClass,
    future_cutover_lane: KernelCompiledProductFutureCutoverLane,
    proof_basis: KernelCompiledProductProofBasis,
    query_boundary_lane: Option<KernelCompiledProductQueryBoundaryLane>,
    reason: &'static str,
    covered_reuse_surfaces: &'static [CompiledProductReuseSurfaceIdentity],
}

impl KernelCompiledProductConsumerCoverageTarget {
    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn new(
        cluster_identity: KernelCompiledProductConsumerClusterIdentity,
        current_source_path: &'static str,
        current_consumer_surface: &'static str,
        responsibility: KernelCompiledProductConsumerResponsibility,
        family_class: KernelCompiledProductFamilyClass,
        future_cutover_lane: KernelCompiledProductFutureCutoverLane,
        proof_basis: KernelCompiledProductProofBasis,
        query_boundary_lane: Option<KernelCompiledProductQueryBoundaryLane>,
        reason: &'static str,
        covered_reuse_surfaces: &'static [CompiledProductReuseSurfaceIdentity],
    ) -> Self {
        Self {
            cluster_identity,
            current_source_path,
            current_consumer_surface,
            responsibility,
            family_class,
            future_cutover_lane,
            proof_basis,
            query_boundary_lane,
            reason,
            covered_reuse_surfaces,
        }
    }

    pub(crate) fn lower_row(
        &self,
    ) -> Result<
        KernelCompiledProductConsumerDependencyRow,
        KernelCompiledProductConsumerDependencyError,
    > {
        KernelCompiledProductConsumerDependencyRow::new(
            self.cluster_identity,
            self.current_source_path,
            self.current_consumer_surface,
            self.responsibility,
            self.family_class,
            self.future_cutover_lane,
            self.proof_basis,
            self.query_boundary_lane,
            self.reason,
        )
    }

    pub(crate) const fn cluster_identity(&self) -> KernelCompiledProductConsumerClusterIdentity {
        self.cluster_identity
    }

    pub(crate) const fn covered_reuse_surfaces(&self) -> &[CompiledProductReuseSurfaceIdentity] {
        self.covered_reuse_surfaces
    }
}
