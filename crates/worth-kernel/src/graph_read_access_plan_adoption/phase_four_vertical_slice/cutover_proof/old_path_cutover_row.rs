use super::super::receipt_boundary::{
    WorthGraphReadAccessSliceReceiptProjection, WorthGraphReadAccessSliceReceiptStatus,
};
use super::super::slice_selection::WorthGraphReadAccessSelectedVerticalSlice;
use super::super::stable_digest;
use super::source_firewall_report::local_loop_firewall_region;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessSliceCutoverStatus {
    ReadyForDeletionAfterReceipt,
    CappedUntilQueryExecutionSurfaceExists,
    CappedUntilMigrationInventoryBindingExists,
}

impl WorthGraphReadAccessSliceCutoverStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadyForDeletionAfterReceipt => "ready_for_deletion_after_receipt",
            Self::CappedUntilQueryExecutionSurfaceExists => {
                "capped_until_query_execution_surface_exists"
            }
            Self::CappedUntilMigrationInventoryBindingExists => {
                "capped_until_migration_inventory_binding_exists"
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessSliceCutoverProof {
    selected_slice_digest: String,
    receipt_projection_digest: String,
    status: WorthGraphReadAccessSliceCutoverStatus,
    displaced_evidence_identity: String,
    deletion_target_identity: String,
    source_firewall_region: String,
    blocker: Option<String>,
    cutover_digest: String,
}

pub(crate) fn project_cutover_for_slice(
    selected_slice: &WorthGraphReadAccessSelectedVerticalSlice,
    receipt_projection: &WorthGraphReadAccessSliceReceiptProjection,
) -> WorthGraphReadAccessSliceCutoverProof {
    let status = match receipt_projection.status() {
        WorthGraphReadAccessSliceReceiptStatus::QueryReceiptObserved => {
            WorthGraphReadAccessSliceCutoverStatus::ReadyForDeletionAfterReceipt
        }
        WorthGraphReadAccessSliceReceiptStatus::QueryExecutionCapabilityGap => {
            WorthGraphReadAccessSliceCutoverStatus::CappedUntilQueryExecutionSurfaceExists
        }
    };
    let status = cap_cutover_until_inventory_binding_exists(status);
    let displaced_evidence_identity = displaced_evidence_identity_for_slice(selected_slice);
    let blocker = receipt_projection.blocker().map(str::to_string);
    let source_firewall_region = local_loop_firewall_region().to_string();
    let deletion_target_identity = format!(
        "phase-four-cutover/deletion-target/{source_firewall_region}/{displaced_evidence_identity}"
    );
    let cutover_digest = stable_digest(&[
        "worth_graph_read_access_slice_cutover_proof_v1".to_string(),
        format!("slice:{}", selected_slice.slice_digest()),
        format!("receipt:{}", receipt_projection.projection_digest()),
        format!("status:{}", status.as_str()),
        format!("displaced_evidence:{displaced_evidence_identity}"),
        format!("deletion_target:{deletion_target_identity}"),
        format!("firewall_region:{source_firewall_region}"),
        format!("blocker:{}", blocker.as_deref().unwrap_or("none")),
    ]);
    WorthGraphReadAccessSliceCutoverProof {
        selected_slice_digest: selected_slice.slice_digest().to_string(),
        receipt_projection_digest: receipt_projection.projection_digest().to_string(),
        status,
        displaced_evidence_identity,
        deletion_target_identity,
        source_firewall_region,
        blocker,
        cutover_digest,
    }
}

fn cap_cutover_until_inventory_binding_exists(
    receipt_status: WorthGraphReadAccessSliceCutoverStatus,
) -> WorthGraphReadAccessSliceCutoverStatus {
    match receipt_status {
        WorthGraphReadAccessSliceCutoverStatus::ReadyForDeletionAfterReceipt => {
            WorthGraphReadAccessSliceCutoverStatus::CappedUntilMigrationInventoryBindingExists
        }
        WorthGraphReadAccessSliceCutoverStatus::CappedUntilQueryExecutionSurfaceExists
        | WorthGraphReadAccessSliceCutoverStatus::CappedUntilMigrationInventoryBindingExists => {
            receipt_status
        }
    }
}

fn displaced_evidence_identity_for_slice(
    selected_slice: &WorthGraphReadAccessSelectedVerticalSlice,
) -> String {
    selected_slice
        .source_attempt_digest()
        .or_else(|| selected_slice.source_carried_gap_digest())
        .unwrap_or_else(|| selected_slice.source_posture_row_digest())
        .to_string()
}

impl WorthGraphReadAccessSliceCutoverProof {
    pub fn selected_slice_digest(&self) -> &str {
        &self.selected_slice_digest
    }

    pub fn receipt_projection_digest(&self) -> &str {
        &self.receipt_projection_digest
    }

    pub const fn status(&self) -> WorthGraphReadAccessSliceCutoverStatus {
        self.status
    }

    pub const fn old_path_is_deleted_or_capped(&self) -> bool {
        matches!(
            self.status,
            WorthGraphReadAccessSliceCutoverStatus::ReadyForDeletionAfterReceipt
                | WorthGraphReadAccessSliceCutoverStatus::CappedUntilQueryExecutionSurfaceExists
                | WorthGraphReadAccessSliceCutoverStatus::CappedUntilMigrationInventoryBindingExists
        )
    }

    pub fn old_path_identity(&self) -> &str {
        &self.deletion_target_identity
    }

    pub fn deletion_target_identity(&self) -> &str {
        &self.deletion_target_identity
    }

    pub fn displaced_evidence_identity(&self) -> &str {
        &self.displaced_evidence_identity
    }

    pub fn source_firewall_region(&self) -> &str {
        &self.source_firewall_region
    }

    pub fn blocker(&self) -> Option<&str> {
        self.blocker.as_deref()
    }

    pub fn cutover_digest(&self) -> &str {
        &self.cutover_digest
    }
}
