use crate::workload_platform::evidence_ledger::{
    BooleanEvidenceStageKind, CompleteWorkloadEvidenceLedger,
    SpatialGeometryEvidenceTouchAuthority, WorkloadEvidenceStage, WorkloadEvidenceSupport,
};
use crate::workload_platform::evidence_lookup_execution::{
    EvidenceLookupExecutionOutcome, EvidenceLookupExecutionReceipt,
};
use crate::workload_platform::evidence_lookup_plan_selection::{
    EvidenceLookupPlanRowOutcome, EvidenceLookupSelectedPlan,
};

use super::denial::{
    PlanarBooleanEventLedgerLookupExecutionDenial,
    PlanarBooleanEventLedgerLookupExecutionDenialKind,
};
use super::packet::PlanarBooleanEventLedgerLookupExecutionPacket;
use crate::workload_platform::planar_boolean_events::PlanarBooleanEventLedgerReceipt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanEventLedgerLookupExecutionWitness {
    event_ledger_identity: String,
    spatial_touch_digest: String,
    stage_index_identity: String,
    selected_plan_digest: String,
    execution_receipt_digest: String,
    lookup_product_output_digest: String,
    evidence_ledger_basis_digest: String,
}

impl PlanarBooleanEventLedgerLookupExecutionWitness {
    pub fn admit(
        event_ledger: &PlanarBooleanEventLedgerReceipt,
        evidence_ledger: &CompleteWorkloadEvidenceLedger,
    ) -> Result<Self, PlanarBooleanEventLedgerLookupExecutionDenial> {
        PlanarBooleanEventLedgerLookupExecutionPacket::admit(event_ledger, evidence_ledger)
            .map(|packet| packet.witness().clone())
    }

    pub(crate) fn certify(
        event_ledger: &PlanarBooleanEventLedgerReceipt,
        spatial_touch_authority: &SpatialGeometryEvidenceTouchAuthority,
        selected_plan: &EvidenceLookupSelectedPlan,
        execution_receipt: &EvidenceLookupExecutionReceipt,
    ) -> Result<Self, PlanarBooleanEventLedgerLookupExecutionDenial> {
        require_event_ledger_authority(event_ledger, spatial_touch_authority)?;
        require_selected_plan(selected_plan, spatial_touch_authority, event_ledger)?;
        require_execution_receipt(
            execution_receipt,
            selected_plan,
            spatial_touch_authority,
            event_ledger,
        )?;
        Ok(Self {
            event_ledger_identity: event_ledger.event_ledger_identity().to_string(),
            spatial_touch_digest: spatial_touch_authority.digest().as_str().to_string(),
            stage_index_identity: spatial_touch_authority.stage_index_identity().to_string(),
            selected_plan_digest: selected_plan.selected_plan_digest().to_string(),
            execution_receipt_digest: execution_receipt.execution_receipt_digest().to_string(),
            lookup_product_output_digest: execution_receipt
                .lookup_product_output_digest()
                .to_string(),
            evidence_ledger_basis_digest: execution_receipt
                .evidence_ledger_basis_digest()
                .to_string(),
        })
    }

    pub fn event_ledger_identity(&self) -> &str {
        &self.event_ledger_identity
    }

    pub fn spatial_touch_digest(&self) -> &str {
        &self.spatial_touch_digest
    }

    pub fn stage_index_identity(&self) -> &str {
        &self.stage_index_identity
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn execution_receipt_digest(&self) -> &str {
        &self.execution_receipt_digest
    }

    pub fn lookup_product_output_digest(&self) -> &str {
        &self.lookup_product_output_digest
    }

    pub fn evidence_ledger_basis_digest(&self) -> &str {
        &self.evidence_ledger_basis_digest
    }
}

fn require_event_ledger_authority(
    event_ledger: &PlanarBooleanEventLedgerReceipt,
    spatial_touch_authority: &SpatialGeometryEvidenceTouchAuthority,
) -> Result<(), PlanarBooleanEventLedgerLookupExecutionDenial> {
    if spatial_touch_authority.boolean_stage() != BooleanEvidenceStageKind::EventLedger
        || spatial_touch_authority.evidence_stage() != WorkloadEvidenceStage::BooleanEventLedger
        || spatial_touch_authority.evidence_identity() != event_ledger.event_ledger_identity()
    {
        return Err(PlanarBooleanEventLedgerLookupExecutionDenial::new(
            PlanarBooleanEventLedgerLookupExecutionDenialKind::WitnessMismatch,
            "event-ledger lookup witness requires spatial touch authority for the same event-ledger receipt",
        ));
    }
    if spatial_touch_authority.support() != WorkloadEvidenceSupport::Admitted {
        return Err(PlanarBooleanEventLedgerLookupExecutionDenial::new(
            PlanarBooleanEventLedgerLookupExecutionDenialKind::WitnessMismatch,
            "event-ledger lookup witness requires admitted spatial touch authority",
        ));
    }
    Ok(())
}

fn require_selected_plan(
    selected_plan: &EvidenceLookupSelectedPlan,
    spatial_touch_authority: &SpatialGeometryEvidenceTouchAuthority,
    event_ledger: &PlanarBooleanEventLedgerReceipt,
) -> Result<(), PlanarBooleanEventLedgerLookupExecutionDenial> {
    if selected_plan.stage() != WorkloadEvidenceStage::BooleanEventLedger
        || selected_plan.spatial_touch_digest() != spatial_touch_authority.digest().as_str()
        || selected_plan.stage_receipt_digest() != event_ledger.event_ledger_identity()
    {
        return Err(PlanarBooleanEventLedgerLookupExecutionDenial::new(
            PlanarBooleanEventLedgerLookupExecutionDenialKind::WitnessMismatch,
            "event-ledger lookup witness requires a selected plan admitted for the same event-ledger receipt",
        ));
    }
    if !selected_plan.rows().iter().any(|row| {
        row.outcome() == EvidenceLookupPlanRowOutcome::Selected
            && row.stage_receipt_digest() == event_ledger.event_ledger_identity()
    }) {
        return Err(PlanarBooleanEventLedgerLookupExecutionDenial::new(
            PlanarBooleanEventLedgerLookupExecutionDenialKind::WitnessMismatch,
            "event-ledger lookup witness requires a selected lookup family for the event-ledger receipt",
        ));
    }
    Ok(())
}

fn require_execution_receipt(
    execution_receipt: &EvidenceLookupExecutionReceipt,
    selected_plan: &EvidenceLookupSelectedPlan,
    spatial_touch_authority: &SpatialGeometryEvidenceTouchAuthority,
    event_ledger: &PlanarBooleanEventLedgerReceipt,
) -> Result<(), PlanarBooleanEventLedgerLookupExecutionDenial> {
    if execution_receipt.selected_plan_digest() != selected_plan.selected_plan_digest()
        || execution_receipt.spatial_touch_digest() != spatial_touch_authority.digest().as_str()
        || execution_receipt.stage_receipt_digest() != event_ledger.event_ledger_identity()
    {
        return Err(PlanarBooleanEventLedgerLookupExecutionDenial::new(
            PlanarBooleanEventLedgerLookupExecutionDenialKind::WitnessMismatch,
            "event-ledger lookup witness requires execution receipt phase-chain identity parity",
        ));
    }
    if execution_receipt.outcome() != EvidenceLookupExecutionOutcome::IndexedHit {
        return Err(PlanarBooleanEventLedgerLookupExecutionDenial::new(
            PlanarBooleanEventLedgerLookupExecutionDenialKind::WitnessMismatch,
            "event-ledger lookup witness requires an indexed-hit execution receipt",
        ));
    }
    if !execution_receipt
        .lookup_product_output()
        .evidence_receipt_digests()
        .iter()
        .any(|digest| digest == event_ledger.event_ledger_identity())
    {
        return Err(PlanarBooleanEventLedgerLookupExecutionDenial::new(
            PlanarBooleanEventLedgerLookupExecutionDenialKind::WitnessMismatch,
            "event-ledger lookup witness requires execution output for the same event-ledger receipt",
        ));
    }
    Ok(())
}
