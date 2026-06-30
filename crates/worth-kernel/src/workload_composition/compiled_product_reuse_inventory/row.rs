use super::classification::{
    CompiledProductReuseAuthorityKind, CompiledProductReuseDisposition, CompiledProductReuseOwner,
    CompiledProductReuseReplacementPhase, CompiledProductReuseSemanticCategory,
    CompiledProductReuseSemanticDistinction,
};
use super::source_scan::CompiledProductReuseScanPattern;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CompiledProductReuseSurfaceIdentity {
    BuildDerivedEquivalenceContract,
    BuildDerivedEquivalenceContractReport,
    CompareDerivedEquivalenceContracts,
    DerivedInvalidationPlannedDispositionFromUpdatePosture,
    HistoricalPathReuseDescriptorRetainedReuse,
    HistoricalCapabilityDescriptorRetainedReuse,
    ReuseEvidenceLookupIndexProduct,
    IndexProductDigest,
    ReplayParityReportFromRetainedProjectionMatch,
    ReplayParityReportRowCount,
    RetainedArtifactCaptureReceiptFromArtifacts,
    ReplayWorkloadWithCapturedRetainedWorkload,
    ReplayCaptureReceipt,
    LookupConsumedWorkloadCompositionAdmit,
    WorthWorkloadAdmitLookupConsumedWorkload,
    WorthWorkloadAdmitLookupConsumedBatchExecutionCluster,
    CurrentEvidenceLookupPublicCloseout,
    CurrentEvidenceLookupPublicCloseoutAssemblyInput,
    CurrentWorthWorkloadOrdinaryConsumerCutover,
    CurrentWorthTouchedGraphConflictPublicCloseout,
    CurrentWorthTouchedGraphConflictMilestoneFourteenSeed,
    ReplayUndoPublicCloseoutReadModelProjection,
    KernelConflictPublicCloseoutBoundaryTraceability,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompiledProductReuseInventoryRow {
    surface_identity: CompiledProductReuseSurfaceIdentity,
    source_path: &'static str,
    surface_name: &'static str,
    authority_surface: &'static str,
    semantic_category: CompiledProductReuseSemanticCategory,
    semantic_distinction: CompiledProductReuseSemanticDistinction,
    authority_kind: CompiledProductReuseAuthorityKind,
    disposition: CompiledProductReuseDisposition,
    owner: CompiledProductReuseOwner,
    replacement_phase: CompiledProductReuseReplacementPhase,
    blocker: &'static str,
    removal_trigger: &'static str,
    ordinary_path: bool,
    certification_only: bool,
    cap: Option<usize>,
    scan_pattern: CompiledProductReuseScanPattern,
    secondary_scan_pattern: Option<CompiledProductReuseScanPattern>,
}

impl CompiledProductReuseInventoryRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn new(
        surface_identity: CompiledProductReuseSurfaceIdentity,
        source_path: &'static str,
        surface_name: &'static str,
        authority_surface: &'static str,
        semantic_category: CompiledProductReuseSemanticCategory,
        semantic_distinction: CompiledProductReuseSemanticDistinction,
        authority_kind: CompiledProductReuseAuthorityKind,
        disposition: CompiledProductReuseDisposition,
        owner: CompiledProductReuseOwner,
        replacement_phase: CompiledProductReuseReplacementPhase,
        blocker: &'static str,
        removal_trigger: &'static str,
        ordinary_path: bool,
        certification_only: bool,
        cap: Option<usize>,
        scan_pattern: CompiledProductReuseScanPattern,
        secondary_scan_pattern: Option<CompiledProductReuseScanPattern>,
    ) -> Self {
        Self {
            surface_identity,
            source_path,
            surface_name,
            authority_surface,
            semantic_category,
            semantic_distinction,
            authority_kind,
            disposition,
            owner,
            replacement_phase,
            blocker,
            removal_trigger,
            ordinary_path,
            certification_only,
            cap,
            scan_pattern,
            secondary_scan_pattern,
        }
    }

    pub const fn surface_identity(&self) -> CompiledProductReuseSurfaceIdentity {
        self.surface_identity
    }

    pub const fn source_path(&self) -> &'static str {
        self.source_path
    }

    pub const fn surface_name(&self) -> &'static str {
        self.surface_name
    }

    pub const fn authority_surface(&self) -> &'static str {
        self.authority_surface
    }

    pub const fn semantic_category(&self) -> CompiledProductReuseSemanticCategory {
        self.semantic_category
    }

    pub const fn semantic_distinction(&self) -> CompiledProductReuseSemanticDistinction {
        self.semantic_distinction
    }

    pub const fn old_authority_kind(&self) -> CompiledProductReuseAuthorityKind {
        self.authority_kind
    }

    pub const fn disposition(&self) -> CompiledProductReuseDisposition {
        self.disposition
    }

    pub const fn owner(&self) -> CompiledProductReuseOwner {
        self.owner
    }

    pub const fn replacement_phase(&self) -> CompiledProductReuseReplacementPhase {
        self.replacement_phase
    }

    pub const fn blocker(&self) -> &'static str {
        self.blocker
    }

    pub const fn removal_trigger(&self) -> &'static str {
        self.removal_trigger
    }

    pub const fn ordinary_path(&self) -> bool {
        self.ordinary_path
    }

    pub const fn certification_only(&self) -> bool {
        self.certification_only
    }

    pub const fn cap(&self) -> Option<usize> {
        self.cap
    }

    pub const fn scan_pattern(&self) -> CompiledProductReuseScanPattern {
        self.scan_pattern
    }

    pub const fn secondary_scan_pattern(&self) -> Option<CompiledProductReuseScanPattern> {
        self.secondary_scan_pattern
    }
}
