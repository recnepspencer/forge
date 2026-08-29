use crate::branch::RelationalBranchRootCheckpoint;
use crate::capabilities::RuntimeIdentitySource;
use crate::durability::data::DurabilityError;
use crate::history::data::{
    PositionedCanonicalCommit, RecordAllocationClass, RecordAllocationOrigin,
    RelationalCommitReceipt,
};
use crate::identity::data::PartitionId;
use crate::runtime::RelationalRuntime;

pub(super) struct CapturedCheckpointBasis {
    pub(super) latest_commit: Option<RelationalCommitReceipt>,
    pub(super) branch_cells: Vec<crate::branch::RelationalBranchCellCheckpoint>,
    pub(super) branch_roots: Vec<RelationalBranchRootCheckpoint>,
    pub(super) record_identity: CapturedRecordIdentity,
    pub(super) envelopes: Vec<PositionedCanonicalCommit>,
    pub(super) partitions: Vec<crate::storage::overlay::PartitionState>,
    pub(super) aspect_contracts: crate::schema::data::AspectContractPlanCatalog,
    pub(super) lineage_nodes: Vec<crate::lineage::data::LineageNode>,
    pub(super) index_definitions: Vec<crate::indexes::data::DerivedIndexDefinition>,
    pub(super) derived_index_artifacts: crate::indexes::data::DerivedIndexArtifacts,
    pub(super) symbol_table: crate::symbols::data::SymbolTableSnapshot,
    pub(super) runtime_name: String,
}

pub(super) struct CapturedRecordIdentity {
    pub(super) generation_high_water: Vec<(RecordAllocationClass, PartitionId, u64, u32)>,
    pub(super) reusable_slots: Vec<(RecordAllocationClass, PartitionId, usize)>,
    pub(super) append_frontiers: Vec<(RecordAllocationClass, PartitionId, usize)>,
    pub(super) pending_reservations: Vec<(
        RecordAllocationClass,
        PartitionId,
        u64,
        u32,
        RecordAllocationOrigin,
    )>,
}

impl CapturedCheckpointBasis {
    /// Select one immutable recovery basis while publication handoff is
    /// excluded. Expensive checkpoint reconstruction, encoding, and I/O occur
    /// only after the caller drops the admission guard.
    pub(super) fn capture(
        runtime: &RelationalRuntime,
        routes: &crate::runtime::RelationalCanonicalPublicationRoutes,
        selection: crate::runtime::PerformedCheckpointSelection,
    ) -> Result<Self, DurabilityError> {
        let envelopes = selection.positioned_snapshot();
        let captured = Self {
            latest_commit: envelopes
                .last()
                .map(|positioned| positioned.envelope().commit.clone()),
            branch_cells: runtime.history().branch_cells_snapshot(),
            branch_roots: runtime
                .history
                .branch_root_checkpoints()
                .map_err(|detail| {
                    DurabilityError::new(
                        crate::durability::data::RecoveryFailureClass::CorruptCheckpoint,
                        detail,
                    )
                })?,
            record_identity: CapturedRecordIdentity {
                generation_high_water: runtime.record_identity.generation_snapshot(),
                reusable_slots: runtime.record_identity.reusable_snapshot(),
                append_frontiers: runtime.record_identity.frontier_snapshot(),
                pending_reservations: runtime.record_identity.pending_snapshot(),
            },
            envelopes,
            partitions: runtime.materialize_partitions(),
            aspect_contracts: runtime
                .schema_contract_runtime
                .aspect_contract_plans
                .clone(),
            lineage_nodes: runtime.lineage_access().nodes_snapshot(),
            index_definitions: runtime.index_access().definitions_snapshot(),
            derived_index_artifacts:
                super::super::derived_index_artifacts::checkpoint_derived_index_artifacts(runtime),
            symbol_table: runtime.services.symbols.snapshot(),
            runtime_name: runtime.runtime_name().to_string(),
        };
        routes
            .validate_checkpoint_selection(&selection)
            .map_err(super::checkpointing::checkpoint_admission_error)?;
        Ok(captured)
    }
}
