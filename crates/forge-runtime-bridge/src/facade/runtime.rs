use super::*;

mod debug;
mod historical_and_replay;
mod merge;
mod policy;
mod routing_and_bulk;
mod source;
mod speculation;
mod stream;
mod structural;
mod subscription;
mod writeback;

#[derive(Clone)]
pub struct RuntimeBridge {
    pub(crate) policy: BridgeRuntimePolicy,
    pub(crate) diagnostics: BridgeDiagnosticsFacade,
    pub(crate) diagnostic_sink: Arc<dyn DiagnosticSink>,
    pub(crate) committed_patch_source: Arc<dyn CommittedPatchSource>,
    pub(crate) snapshot_read_source: Arc<dyn SnapshotReadSource>,
    pub(crate) snapshot_reader_pool: Option<Arc<dyn SnapshotReaderPool>>,
    pub(crate) signal_sink: Arc<dyn InvalidationSink>,
    pub(crate) truth_branch_head_source: Option<Arc<dyn TruthBranchHeadSource>>,
    pub(crate) continuity_lineage_source: Option<Arc<dyn ContinuityLineageSource>>,
    pub(crate) writeback_authority: Option<Arc<dyn TruthWritebackAuthority>>,
    pub(crate) source_registry: AdmittedSourceRegistry,
    pub(crate) source_adapter: Option<Arc<dyn BridgeSourceAdapter>>,
    pub(crate) structural_registry: AdmittedStructuralRegistry,
    pub(crate) merge_registry: AdmittedMergeRegistry,
    pub(crate) mapping_registry: FrozenMappingRegistry,
    pub(crate) aspect_registry: FrozenAspectMappingRegistry,
    pub(crate) subscription_family_registry: FrozenSubscriptionFamilyRegistry,
}

impl std::fmt::Debug for RuntimeBridge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RuntimeBridge")
            .field("policy", &self.policy)
            .field(
                "has_truth_branch_head_source",
                &self.truth_branch_head_source.is_some(),
            )
            .field(
                "has_continuity_lineage_source",
                &self.continuity_lineage_source.is_some(),
            )
            .field(
                "has_writeback_authority",
                &self.writeback_authority.is_some(),
            )
            .field(
                "has_snapshot_reader_pool",
                &self.snapshot_reader_pool.is_some(),
            )
            .field("has_source_adapter", &self.source_adapter.is_some())
            .field(
                "source_contract_count",
                &self.source_registry.contracts().len(),
            )
            .field(
                "structural_contract_count",
                &self.structural_registry.contracts().len(),
            )
            .field(
                "merge_contract_count",
                &self.merge_registry.contracts().len(),
            )
            .field(
                "subscription_family_count",
                &self.subscription_family_registry.registrations().len(),
            )
            .finish()
    }
}
