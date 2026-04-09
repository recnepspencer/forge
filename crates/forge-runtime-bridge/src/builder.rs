use std::sync::Arc;

use crate::adapter::{
    BridgeSourceAdapter, CommittedPatchSource, ContinuityLineageSource, InvalidationSink,
    SnapshotReadSource, SnapshotReaderPool, TruthBranchHeadSource,
};
use crate::diagnostics::DiagnosticSink;
use crate::error::{BridgeBuildError, BridgeBuildErrorKind};
use crate::facade::RuntimeBridge;
use crate::mapping::{
    BridgeAspectRegistration, BridgeMappingRegistration, FrozenAspectMappingRegistry,
    FrozenMappingRegistry,
};
use crate::merge::{AdmittedMergeRegistry, MergeHistoryDeclaration};
use crate::policy::BridgeRuntimePolicy;
use crate::source::{AdmittedSourceRegistry, SourceDeclaration};
use crate::structural::{AdmittedStructuralRegistry, StructuralIdentityDeclaration};

#[derive(Clone)]
pub struct MissingCommittedPatchSource;

#[derive(Clone)]
pub struct PresentCommittedPatchSource(Arc<dyn CommittedPatchSource>);

#[derive(Clone)]
pub struct MissingSnapshotReadSource;

#[derive(Clone)]
pub struct PresentSnapshotReadSource(Arc<dyn SnapshotReadSource>);

#[derive(Clone)]
pub struct MissingSignalSink;

#[derive(Clone)]
pub struct PresentSignalSink(Arc<dyn InvalidationSink>);

#[derive(Clone)]
pub struct MissingTruthBranchHeadSource;

#[derive(Clone)]
pub struct PresentTruthBranchHeadSource(Arc<dyn TruthBranchHeadSource>);

#[derive(Clone)]
pub struct MissingMappingRegistrations;

#[derive(Clone)]
pub struct PresentMappingRegistrations(Vec<BridgeMappingRegistration>);

#[derive(Clone)]
pub struct RuntimeBridgeBuilder<
    PatchState = MissingCommittedPatchSource,
    SnapshotState = MissingSnapshotReadSource,
    SignalState = MissingSignalSink,
    BranchHeadState = MissingTruthBranchHeadSource,
    MappingState = MissingMappingRegistrations,
> {
    policy: BridgeRuntimePolicy,
    committed_patch_source: PatchState,
    snapshot_read_source: SnapshotState,
    signal_sink: SignalState,
    truth_branch_head_source: BranchHeadState,
    continuity_lineage_source: Option<Arc<dyn ContinuityLineageSource>>,
    snapshot_reader_pool: Option<Arc<dyn SnapshotReaderPool>>,
    source_adapter_registrations: Vec<Arc<dyn BridgeSourceAdapter>>,
    source_declarations: Vec<SourceDeclaration>,
    structural_declarations: Vec<StructuralIdentityDeclaration>,
    merge_declarations: Vec<MergeHistoryDeclaration>,
    diagnostic_sink: Option<Arc<dyn DiagnosticSink>>,
    mapping_registrations: MappingState,
    aspect_registrations: Vec<BridgeAspectRegistration>,
}

impl Default
    for RuntimeBridgeBuilder<
        MissingCommittedPatchSource,
        MissingSnapshotReadSource,
        MissingSignalSink,
        MissingTruthBranchHeadSource,
        MissingMappingRegistrations,
    >
{
    fn default() -> Self {
        Self::new()
    }
}

impl
    RuntimeBridgeBuilder<
        MissingCommittedPatchSource,
        MissingSnapshotReadSource,
        MissingSignalSink,
        MissingTruthBranchHeadSource,
        MissingMappingRegistrations,
    >
{
    pub fn new() -> Self {
        Self {
            policy: BridgeRuntimePolicy::default(),
            committed_patch_source: MissingCommittedPatchSource,
            snapshot_read_source: MissingSnapshotReadSource,
            signal_sink: MissingSignalSink,
            truth_branch_head_source: MissingTruthBranchHeadSource,
            continuity_lineage_source: None,
            snapshot_reader_pool: None,
            source_adapter_registrations: Vec::new(),
            source_declarations: Vec::new(),
            structural_declarations: Vec::new(),
            merge_declarations: Vec::new(),
            diagnostic_sink: None,
            mapping_registrations: MissingMappingRegistrations,
            aspect_registrations: Vec::new(),
        }
    }
}

impl<SnapshotState, SignalState, BranchHeadState, MappingState>
    RuntimeBridgeBuilder<
        MissingCommittedPatchSource,
        SnapshotState,
        SignalState,
        BranchHeadState,
        MappingState,
    >
{
    pub fn with_committed_patch_source<S>(
        self,
        source: S,
    ) -> RuntimeBridgeBuilder<
        PresentCommittedPatchSource,
        SnapshotState,
        SignalState,
        BranchHeadState,
        MappingState,
    >
    where
        S: CommittedPatchSource,
    {
        RuntimeBridgeBuilder {
            policy: self.policy,
            committed_patch_source: PresentCommittedPatchSource(Arc::new(source)),
            snapshot_read_source: self.snapshot_read_source,
            signal_sink: self.signal_sink,
            truth_branch_head_source: self.truth_branch_head_source,
            continuity_lineage_source: self.continuity_lineage_source,
            snapshot_reader_pool: self.snapshot_reader_pool,
            source_adapter_registrations: self.source_adapter_registrations,
            source_declarations: self.source_declarations,
            structural_declarations: self.structural_declarations,
            merge_declarations: self.merge_declarations,
            diagnostic_sink: self.diagnostic_sink,
            mapping_registrations: self.mapping_registrations,
            aspect_registrations: self.aspect_registrations,
        }
    }
}

impl<PatchState, SignalState, BranchHeadState, MappingState>
    RuntimeBridgeBuilder<
        PatchState,
        MissingSnapshotReadSource,
        SignalState,
        BranchHeadState,
        MappingState,
    >
{
    pub fn with_snapshot_read_source<S>(
        self,
        source: S,
    ) -> RuntimeBridgeBuilder<
        PatchState,
        PresentSnapshotReadSource,
        SignalState,
        BranchHeadState,
        MappingState,
    >
    where
        S: SnapshotReadSource,
    {
        RuntimeBridgeBuilder {
            policy: self.policy,
            committed_patch_source: self.committed_patch_source,
            snapshot_read_source: PresentSnapshotReadSource(Arc::new(source)),
            signal_sink: self.signal_sink,
            truth_branch_head_source: self.truth_branch_head_source,
            continuity_lineage_source: self.continuity_lineage_source,
            snapshot_reader_pool: self.snapshot_reader_pool,
            source_adapter_registrations: self.source_adapter_registrations,
            source_declarations: self.source_declarations,
            structural_declarations: self.structural_declarations,
            merge_declarations: self.merge_declarations,
            diagnostic_sink: self.diagnostic_sink,
            mapping_registrations: self.mapping_registrations,
            aspect_registrations: self.aspect_registrations,
        }
    }
}

impl<SignalState, MappingState>
    RuntimeBridgeBuilder<
        MissingCommittedPatchSource,
        MissingSnapshotReadSource,
        SignalState,
        MissingTruthBranchHeadSource,
        MappingState,
    >
{
    pub fn with_relational_source<S>(
        self,
        source: S,
    ) -> RuntimeBridgeBuilder<
        PresentCommittedPatchSource,
        PresentSnapshotReadSource,
        SignalState,
        MissingTruthBranchHeadSource,
        MappingState,
    >
    where
        S: CommittedPatchSource + SnapshotReadSource,
    {
        let source = Arc::new(source);
        RuntimeBridgeBuilder {
            policy: self.policy,
            committed_patch_source: PresentCommittedPatchSource(source.clone()),
            snapshot_read_source: PresentSnapshotReadSource(source.clone()),
            signal_sink: self.signal_sink,
            truth_branch_head_source: MissingTruthBranchHeadSource,
            continuity_lineage_source: self.continuity_lineage_source,
            snapshot_reader_pool: self.snapshot_reader_pool,
            source_adapter_registrations: self.source_adapter_registrations,
            source_declarations: self.source_declarations,
            structural_declarations: self.structural_declarations,
            merge_declarations: self.merge_declarations,
            diagnostic_sink: self.diagnostic_sink,
            mapping_registrations: self.mapping_registrations,
            aspect_registrations: self.aspect_registrations,
        }
    }
}

impl<PatchState, SnapshotState, BranchHeadState, MappingState>
    RuntimeBridgeBuilder<
        PatchState,
        SnapshotState,
        MissingSignalSink,
        BranchHeadState,
        MappingState,
    >
{
    pub fn with_signal_sink<S>(
        self,
        sink: S,
    ) -> RuntimeBridgeBuilder<
        PatchState,
        SnapshotState,
        PresentSignalSink,
        BranchHeadState,
        MappingState,
    >
    where
        S: InvalidationSink,
    {
        RuntimeBridgeBuilder {
            policy: self.policy,
            committed_patch_source: self.committed_patch_source,
            snapshot_read_source: self.snapshot_read_source,
            signal_sink: PresentSignalSink(Arc::new(sink)),
            truth_branch_head_source: self.truth_branch_head_source,
            continuity_lineage_source: self.continuity_lineage_source,
            snapshot_reader_pool: self.snapshot_reader_pool,
            source_adapter_registrations: self.source_adapter_registrations,
            source_declarations: self.source_declarations,
            structural_declarations: self.structural_declarations,
            merge_declarations: self.merge_declarations,
            diagnostic_sink: self.diagnostic_sink,
            mapping_registrations: self.mapping_registrations,
            aspect_registrations: self.aspect_registrations,
        }
    }
}

impl<PatchState, SnapshotState, SignalState, BranchHeadState, MappingState>
    RuntimeBridgeBuilder<PatchState, SnapshotState, SignalState, BranchHeadState, MappingState>
{
    pub fn with_policy(mut self, policy: BridgeRuntimePolicy) -> Self {
        self.policy = policy;
        self
    }

    pub fn with_snapshot_reader_pool<P>(mut self, pool: P) -> Self
    where
        P: SnapshotReaderPool,
    {
        self.snapshot_reader_pool = Some(Arc::new(pool));
        self
    }

    pub fn with_source_adapter<S>(mut self, source_adapter: S) -> Self
    where
        S: BridgeSourceAdapter,
    {
        self.source_adapter_registrations
            .push(Arc::new(source_adapter));
        self
    }

    pub fn with_continuity_lineage_source<S>(mut self, source: S) -> Self
    where
        S: ContinuityLineageSource,
    {
        self.continuity_lineage_source = Some(Arc::new(source));
        self
    }

    pub fn with_truth_branch_head_source<S>(
        self,
        source: S,
    ) -> RuntimeBridgeBuilder<
        PatchState,
        SnapshotState,
        SignalState,
        PresentTruthBranchHeadSource,
        MappingState,
    >
    where
        S: TruthBranchHeadSource,
    {
        RuntimeBridgeBuilder {
            policy: self.policy,
            committed_patch_source: self.committed_patch_source,
            snapshot_read_source: self.snapshot_read_source,
            signal_sink: self.signal_sink,
            truth_branch_head_source: PresentTruthBranchHeadSource(Arc::new(source)),
            continuity_lineage_source: self.continuity_lineage_source,
            snapshot_reader_pool: self.snapshot_reader_pool,
            source_adapter_registrations: self.source_adapter_registrations,
            source_declarations: self.source_declarations,
            structural_declarations: self.structural_declarations,
            merge_declarations: self.merge_declarations,
            diagnostic_sink: self.diagnostic_sink,
            mapping_registrations: self.mapping_registrations,
            aspect_registrations: self.aspect_registrations,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn with_diagnostics_sink(mut self, sink: Arc<dyn DiagnosticSink>) -> Self {
        self.diagnostic_sink = Some(sink);
        self
    }

    pub fn register_aspect_mapping(mut self, registration: BridgeAspectRegistration) -> Self {
        self.aspect_registrations.push(registration);
        self
    }

    pub fn register_source(mut self, declaration: SourceDeclaration) -> Self {
        self.source_declarations.push(declaration);
        self
    }

    pub fn register_structural(mut self, declaration: StructuralIdentityDeclaration) -> Self {
        self.structural_declarations.push(declaration);
        self
    }

    pub fn register_merge(mut self, declaration: MergeHistoryDeclaration) -> Self {
        self.merge_declarations.push(declaration);
        self
    }
}

impl<PatchState, SnapshotState, SignalState, BranchHeadState>
    RuntimeBridgeBuilder<
        PatchState,
        SnapshotState,
        SignalState,
        BranchHeadState,
        MissingMappingRegistrations,
    >
{
    pub fn register_mapping(
        self,
        registration: BridgeMappingRegistration,
    ) -> RuntimeBridgeBuilder<
        PatchState,
        SnapshotState,
        SignalState,
        BranchHeadState,
        PresentMappingRegistrations,
    > {
        RuntimeBridgeBuilder {
            policy: self.policy,
            committed_patch_source: self.committed_patch_source,
            snapshot_read_source: self.snapshot_read_source,
            signal_sink: self.signal_sink,
            truth_branch_head_source: self.truth_branch_head_source,
            continuity_lineage_source: self.continuity_lineage_source,
            snapshot_reader_pool: self.snapshot_reader_pool,
            source_adapter_registrations: self.source_adapter_registrations,
            source_declarations: self.source_declarations,
            structural_declarations: self.structural_declarations,
            merge_declarations: self.merge_declarations,
            diagnostic_sink: self.diagnostic_sink,
            mapping_registrations: PresentMappingRegistrations(vec![registration]),
            aspect_registrations: self.aspect_registrations,
        }
    }
}

impl<PatchState, SnapshotState, SignalState, BranchHeadState>
    RuntimeBridgeBuilder<
        PatchState,
        SnapshotState,
        SignalState,
        BranchHeadState,
        PresentMappingRegistrations,
    >
{
    pub fn register_mapping(mut self, registration: BridgeMappingRegistration) -> Self {
        self.mapping_registrations.0.push(registration);
        self
    }
}

fn finalize_source_configuration(
    source_declarations: Vec<SourceDeclaration>,
    mut source_adapter_registrations: Vec<Arc<dyn BridgeSourceAdapter>>,
) -> Result<(AdmittedSourceRegistry, Option<Arc<dyn BridgeSourceAdapter>>), BridgeBuildError> {
    if source_adapter_registrations.len() > 1 {
        return Err(BridgeBuildError::new(
            BridgeBuildErrorKind::BuilderConfigurationConflict,
            "Bridge builder registered more than one source adapter for the same runtime.",
        ));
    }

    let source_registry = AdmittedSourceRegistry::freeze(source_declarations)?;
    let source_adapter = source_adapter_registrations.pop();

    if !source_registry.contracts().is_empty() && source_adapter.is_none() {
        return Err(BridgeBuildError::new(
            BridgeBuildErrorKind::MissingSourceAdapter,
            "Bridge builder registered source declarations but no source adapter.",
        ));
    }

    if let Some(adapter) = source_adapter.as_ref() {
        let declared_capabilities = adapter.declared_capabilities();
        let required_capabilities = source_registry.required_capabilities();
        if !declared_capabilities.contains_all(&required_capabilities) {
            return Err(BridgeBuildError::new(
                BridgeBuildErrorKind::SourceCapabilityMismatch,
                format!(
                    "Bridge source adapter capabilities `{}` do not satisfy required source capabilities `{}`.",
                    declared_capabilities.digest(),
                    required_capabilities.digest()
                ),
            ));
        }
    }

    Ok((source_registry, source_adapter))
}

fn finalize_structural_configuration(
    structural_declarations: Vec<StructuralIdentityDeclaration>,
) -> Result<AdmittedStructuralRegistry, BridgeBuildError> {
    AdmittedStructuralRegistry::freeze(structural_declarations)
}

fn finalize_merge_configuration(
    merge_declarations: Vec<MergeHistoryDeclaration>,
) -> Result<AdmittedMergeRegistry, BridgeBuildError> {
    AdmittedMergeRegistry::freeze(merge_declarations)
}

impl
    RuntimeBridgeBuilder<
        PresentCommittedPatchSource,
        PresentSnapshotReadSource,
        PresentSignalSink,
        MissingTruthBranchHeadSource,
        PresentMappingRegistrations,
    >
{
    pub fn build(self) -> Result<RuntimeBridge, BridgeBuildError> {
        let mapping_registry = FrozenMappingRegistry::freeze(self.mapping_registrations.0)?;
        let aspect_registry = FrozenAspectMappingRegistry::freeze(self.aspect_registrations)?;
        let (source_registry, source_adapter) = finalize_source_configuration(
            self.source_declarations,
            self.source_adapter_registrations,
        )?;
        let structural_registry = finalize_structural_configuration(self.structural_declarations)?;
        let merge_registry = finalize_merge_configuration(self.merge_declarations)?;
        Ok(RuntimeBridge::new(
            self.policy,
            self.committed_patch_source.0,
            self.snapshot_read_source.0,
            self.signal_sink.0,
            None,
            self.continuity_lineage_source,
            self.snapshot_reader_pool,
            source_registry,
            source_adapter,
            structural_registry,
            merge_registry,
            self.diagnostic_sink,
            mapping_registry,
            aspect_registry,
        ))
    }
}

impl
    RuntimeBridgeBuilder<
        PresentCommittedPatchSource,
        PresentSnapshotReadSource,
        PresentSignalSink,
        PresentTruthBranchHeadSource,
        PresentMappingRegistrations,
    >
{
    pub fn build(self) -> Result<RuntimeBridge, BridgeBuildError> {
        let mapping_registry = FrozenMappingRegistry::freeze(self.mapping_registrations.0)?;
        let aspect_registry = FrozenAspectMappingRegistry::freeze(self.aspect_registrations)?;
        let (source_registry, source_adapter) = finalize_source_configuration(
            self.source_declarations,
            self.source_adapter_registrations,
        )?;
        let structural_registry = finalize_structural_configuration(self.structural_declarations)?;
        let merge_registry = finalize_merge_configuration(self.merge_declarations)?;
        Ok(RuntimeBridge::new(
            self.policy,
            self.committed_patch_source.0,
            self.snapshot_read_source.0,
            self.signal_sink.0,
            Some(self.truth_branch_head_source.0),
            self.continuity_lineage_source,
            self.snapshot_reader_pool,
            source_registry,
            source_adapter,
            structural_registry,
            merge_registry,
            self.diagnostic_sink,
            mapping_registry,
            aspect_registry,
        ))
    }
}

impl<PatchState, SnapshotState, SignalState, BranchHeadState, MappingState> std::fmt::Debug
    for RuntimeBridgeBuilder<PatchState, SnapshotState, SignalState, BranchHeadState, MappingState>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RuntimeBridgeBuilder")
            .field("policy", &self.policy)
            .field(
                "aspect_registration_count",
                &self.aspect_registrations.len(),
            )
            .finish()
    }
}

#[cfg(test)]
mod tests;
