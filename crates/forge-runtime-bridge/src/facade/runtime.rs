use super::*;

mod debug;
mod historical_and_replay;
mod routing_and_bulk;
mod stream;

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
    pub(crate) mapping_registry: FrozenMappingRegistry,
    pub(crate) aspect_registry: FrozenAspectMappingRegistry,
}

