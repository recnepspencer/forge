use super::*;

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
}
