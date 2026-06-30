use super::consumer_class::KernelCompiledProductConsumerResponsibility;
use super::error::{
    KernelCompiledProductConsumerDependencyError, KernelCompiledProductConsumerDependencyErrorKind,
};
use super::family_class::KernelCompiledProductFamilyClass;
use super::future_cutover_lane::KernelCompiledProductFutureCutoverLane;
use super::proof_basis::KernelCompiledProductProofBasis;
use super::query_boundary_lane::KernelCompiledProductQueryBoundaryLane;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum KernelCompiledProductConsumerClusterIdentity {
    LookupConsumedWorkload,
    LookupConsumedBatchExecution,
    RetainedReplayBatchExecutionCarryForward,
    ReplayUndoBoundary,
    OrdinaryConsumerCutoverSummary,
    ConflictPublicCloseout,
    ConflictPublicCloseoutSeed,
    SpatialEvidenceLookupPublicCloseout,
    ReplayUndoPublicCloseoutReadModel,
    KernelConflictPublicCloseoutBoundaryTraceability,
}

impl KernelCompiledProductConsumerClusterIdentity {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LookupConsumedWorkload => "lookup-consumed-workload",
            Self::LookupConsumedBatchExecution => "lookup-consumed-batch-execution",
            Self::RetainedReplayBatchExecutionCarryForward => {
                "retained-replay-batch-execution-carry-forward"
            }
            Self::ReplayUndoBoundary => "replay-undo-boundary",
            Self::OrdinaryConsumerCutoverSummary => "ordinary-consumer-cutover-summary",
            Self::ConflictPublicCloseout => "conflict-public-closeout",
            Self::ConflictPublicCloseoutSeed => "conflict-public-closeout-seed",
            Self::SpatialEvidenceLookupPublicCloseout => "spatial-evidence-lookup-public-closeout",
            Self::ReplayUndoPublicCloseoutReadModel => "replay-undo-public-closeout-read-model",
            Self::KernelConflictPublicCloseoutBoundaryTraceability => {
                "kernel-conflict-public-closeout-boundary-traceability"
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KernelCompiledProductConsumerDependencyRow {
    cluster_identity: KernelCompiledProductConsumerClusterIdentity,
    current_source_path: &'static str,
    current_consumer_surface: &'static str,
    responsibility: KernelCompiledProductConsumerResponsibility,
    family_class: KernelCompiledProductFamilyClass,
    future_cutover_lane: KernelCompiledProductFutureCutoverLane,
    proof_basis: KernelCompiledProductProofBasis,
    query_boundary_lane: Option<KernelCompiledProductQueryBoundaryLane>,
    reason: &'static str,
}

impl KernelCompiledProductConsumerDependencyRow {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn new(
        cluster_identity: KernelCompiledProductConsumerClusterIdentity,
        current_source_path: &'static str,
        current_consumer_surface: &'static str,
        responsibility: KernelCompiledProductConsumerResponsibility,
        family_class: KernelCompiledProductFamilyClass,
        future_cutover_lane: KernelCompiledProductFutureCutoverLane,
        proof_basis: KernelCompiledProductProofBasis,
        query_boundary_lane: Option<KernelCompiledProductQueryBoundaryLane>,
        reason: &'static str,
    ) -> Result<Self, KernelCompiledProductConsumerDependencyError> {
        if current_source_path.is_empty() {
            return Err(KernelCompiledProductConsumerDependencyError::new(
                KernelCompiledProductConsumerDependencyErrorKind::MissingCurrentSourcePath,
                format!(
                    "cluster `{}` must name the current source path it is freezing",
                    cluster_identity.as_str()
                ),
            ));
        }
        if current_consumer_surface.is_empty() {
            return Err(KernelCompiledProductConsumerDependencyError::new(
                KernelCompiledProductConsumerDependencyErrorKind::MissingCurrentConsumerSurface,
                format!(
                    "cluster `{}` must name the current consumer surface it is freezing",
                    cluster_identity.as_str()
                ),
            ));
        }
        if responsibility == KernelCompiledProductConsumerResponsibility::QueryBacked
            && query_boundary_lane.is_none()
        {
            return Err(KernelCompiledProductConsumerDependencyError::new(
                KernelCompiledProductConsumerDependencyErrorKind::QueryBackedConsumerMissingRealQueryLane,
                format!(
                    "query-backed cluster `{}` must bind a real Query boundary lane",
                    cluster_identity.as_str()
                ),
            ));
        }
        if responsibility != KernelCompiledProductConsumerResponsibility::QueryBacked
            && query_boundary_lane.is_some()
        {
            return Err(KernelCompiledProductConsumerDependencyError::new(
                KernelCompiledProductConsumerDependencyErrorKind::NonQueryConsumerNamedAsQueryLane,
                format!(
                    "non-query cluster `{}` must not claim a Query boundary lane",
                    cluster_identity.as_str()
                ),
            ));
        }
        Ok(Self {
            cluster_identity,
            current_source_path,
            current_consumer_surface,
            responsibility,
            family_class,
            future_cutover_lane,
            proof_basis,
            query_boundary_lane,
            reason,
        })
    }

    pub const fn cluster_identity(&self) -> KernelCompiledProductConsumerClusterIdentity {
        self.cluster_identity
    }

    pub const fn current_source_path(&self) -> &'static str {
        self.current_source_path
    }

    pub const fn current_consumer_surface(&self) -> &'static str {
        self.current_consumer_surface
    }

    pub const fn responsibility(&self) -> KernelCompiledProductConsumerResponsibility {
        self.responsibility
    }

    pub const fn family_class(&self) -> KernelCompiledProductFamilyClass {
        self.family_class
    }

    pub const fn future_cutover_lane(&self) -> KernelCompiledProductFutureCutoverLane {
        self.future_cutover_lane
    }

    pub const fn proof_basis(&self) -> &KernelCompiledProductProofBasis {
        &self.proof_basis
    }

    pub const fn query_boundary_lane(&self) -> Option<KernelCompiledProductQueryBoundaryLane> {
        self.query_boundary_lane
    }

    pub const fn reason(&self) -> &'static str {
        self.reason
    }
}
