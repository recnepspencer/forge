use serde::{Deserialize, Serialize};

use crate::data::graph::SignalGraph;
use crate::data::telemetry::RuntimeTelemetry;
use crate::logic::transaction::{ReconstructabilityProof, ReconstructabilityRecord};

use super::authority::SignalCheckpointImage;
use super::diagnostics::SignalSnapshotDiagnostics;
use super::lifecycle::{SignalBranchId, SignalSnapshotId, SignalSnapshotMeta};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Versioned snapshot of `worth-signal` evaluation state.
///
/// This captures graph-local evaluation state, runtime diagnostics required for
/// deterministic replay/restore, branch metadata, and lineage/replay history.
/// It intentionally does not claim ownership of host-managed source truth.
pub struct SignalSnapshotV1 {
    pub meta: SignalSnapshotMeta,
    pub checkpoint_image: SignalCheckpointImage,
    #[serde(alias = "graph")]
    pub diagnostic_graph: SignalGraph,
    pub diagnostics: SignalSnapshotDiagnostics,
    pub graph_telemetry: RuntimeTelemetry,
    pub runtime_telemetry: Option<RuntimeTelemetry>,
    pub reconstructability: Option<ReconstructabilityRecord>,
}

impl SignalSnapshotV1 {
    /// Inspect snapshot metadata without restoring the snapshot.
    pub fn meta(&self) -> &SignalSnapshotMeta {
        &self.meta
    }

    /// Branch identity that owned the snapshot head when this snapshot was captured.
    pub fn branch_id(&self) -> SignalBranchId {
        self.meta.branch_id
    }

    /// Stable snapshot identifier for replay and lineage references.
    pub fn snapshot_id(&self) -> SignalSnapshotId {
        self.meta.snapshot_id
    }

    pub fn checkpoint_image(&self) -> &SignalCheckpointImage {
        &self.checkpoint_image
    }

    /// Rich diagnostics/inspection payload captured with the snapshot.
    ///
    /// This is not restore authority. Supported restore paths must consume the
    /// checkpoint image instead.
    pub fn diagnostic_graph(&self) -> &SignalGraph {
        &self.diagnostic_graph
    }

    pub(crate) fn authority_graph(&self) -> Result<SignalGraph, crate::data::error::SignalError> {
        SignalGraph::restore_from_checkpoint_image(&self.checkpoint_image)
    }

    pub fn reconstructability_proof(
        &self,
    ) -> Result<ReconstructabilityProof, crate::data::error::SignalError> {
        let record = self.reconstructability.as_ref().ok_or_else(|| {
            crate::data::error::SignalError::incompatible_snapshot(format!(
                "snapshot `{}` is missing reconstructability record",
                self.meta.snapshot_id.0
            ))
        })?;
        Ok(record.proof())
    }
}
