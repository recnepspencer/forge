use crate::workload_platform::evidence_ledger::{
    CompleteWorkloadEvidenceLedger, SelectedLookupSliceLedger, SelectedLookupSliceLedgerAssembly,
    SpatialGeometryEvidenceTouchAuthority, SpatialGeometryEvidenceTouchRequest,
    WorkloadEvidenceStage,
};
use crate::workload_platform::evidence_lookup_execution::{
    execute_evidence_lookup, EvidenceLookupExecutionReceipt, EvidenceLookupExecutionRequest,
};
use crate::workload_platform::evidence_lookup_family_catalog::{
    current_evidence_lookup_family_catalog, EvidenceLookupDiagnosticWitnessShape,
    EvidenceLookupStageReceiptFamilyIdentity,
};
use crate::workload_platform::evidence_lookup_index_product::EvidenceLookupIndexProduct;
use crate::workload_platform::evidence_lookup_input_admission::{
    admit_evidence_lookup_input, EvidenceLookupInputAdmissionRequest,
    EvidenceLookupStageReceiptAdmission,
};
use crate::workload_platform::evidence_lookup_plan_selection::{
    select_evidence_lookup_plan, EvidenceLookupPlanRowOutcome, EvidenceLookupSelectedPlan,
};
use crate::workload_platform::spatial_compiled_product_consumer_cutover::lower_evidence_lookup_index_product;

use super::denial::{
    PlanarBooleanEventLedgerLookupExecutionDenial,
    PlanarBooleanEventLedgerLookupExecutionDenialKind,
};
use super::witness::PlanarBooleanEventLedgerLookupExecutionWitness;
use crate::workload_platform::planar_boolean_events::PlanarBooleanEventLedgerReceipt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanEventLedgerLookupExecutionPacket {
    witness: PlanarBooleanEventLedgerLookupExecutionWitness,
    selected_family_identity: String,
    selected_family_declaration_digest: String,
    selected_family_diagnostic_witness_shape: EvidenceLookupDiagnosticWitnessShape,
    selected_plan: EvidenceLookupSelectedPlan,
    selected_lookup_slice: SelectedLookupSliceLedger,
    index_product: EvidenceLookupIndexProduct,
    execution_receipt: EvidenceLookupExecutionReceipt,
}

impl PlanarBooleanEventLedgerLookupExecutionPacket {
    pub fn admit(
        event_ledger: &PlanarBooleanEventLedgerReceipt,
        evidence_ledger: &CompleteWorkloadEvidenceLedger,
    ) -> Result<Self, PlanarBooleanEventLedgerLookupExecutionDenial> {
        let spatial_touch_authority =
            SpatialGeometryEvidenceTouchRequest::from_boolean_receipt(event_ledger)
                .with_complete_ledger(evidence_ledger)
                .admit()
                .map_err(|error| {
                    PlanarBooleanEventLedgerLookupExecutionDenial::new(
                        PlanarBooleanEventLedgerLookupExecutionDenialKind::SpatialTouchAuthority,
                        error.human_reason(),
                    )
                })?;
        let catalog = current_evidence_lookup_family_catalog().map_err(|error| {
            PlanarBooleanEventLedgerLookupExecutionDenial::new(
                PlanarBooleanEventLedgerLookupExecutionDenialKind::FamilyCatalog,
                format!("{:?}", error.kind()),
            )
        })?;
        let stage_receipt = EvidenceLookupStageReceiptAdmission::from_spatial_touch_authority(
            &spatial_touch_authority,
            EvidenceLookupStageReceiptFamilyIdentity::boolean_event_ledger(),
        );
        let selected_lookup_slice = SelectedLookupSliceLedgerAssembly::from_touch_authority(
            &spatial_touch_authority,
            &stage_receipt,
        )
        .assemble_selected_lookup_slice()
        .map_err(|error| {
            PlanarBooleanEventLedgerLookupExecutionDenial::new(
                PlanarBooleanEventLedgerLookupExecutionDenialKind::IndexProduct,
                error.human_reason(),
            )
        })?;
        deny_unrelated_boolean_residue(event_ledger, evidence_ledger, &spatial_touch_authority)?;
        let admitted_input = admit_evidence_lookup_input(
            &catalog,
            EvidenceLookupInputAdmissionRequest::from_spatial_touch_authority(
                &spatial_touch_authority,
            )
            .with_stage_receipt_identity(stage_receipt),
        )
        .map_err(|error| {
            PlanarBooleanEventLedgerLookupExecutionDenial::new(
                PlanarBooleanEventLedgerLookupExecutionDenialKind::InputAdmission,
                error.detail(),
            )
        })?;
        let selected_plan =
            select_evidence_lookup_plan(&catalog, &admitted_input).map_err(|error| {
                PlanarBooleanEventLedgerLookupExecutionDenial::new(
                    PlanarBooleanEventLedgerLookupExecutionDenialKind::PlanSelection,
                    error.detail(),
                )
            })?;
        let selected_row = selected_plan
            .rows()
            .iter()
            .find(|row| row.outcome() == EvidenceLookupPlanRowOutcome::Selected)
            .ok_or_else(|| {
                PlanarBooleanEventLedgerLookupExecutionDenial::new(
                    PlanarBooleanEventLedgerLookupExecutionDenialKind::WitnessMismatch,
                    "event-ledger lookup packet requires a selected lookup family row",
                )
            })?;
        let selected_family = catalog
            .family_by_identity(selected_row.family_identity())
            .ok_or_else(|| {
                PlanarBooleanEventLedgerLookupExecutionDenial::new(
                    PlanarBooleanEventLedgerLookupExecutionDenialKind::WitnessMismatch,
                    "event-ledger lookup packet requires the selected family declaration to remain present in the catalog",
                )
            })?;
        let index_product =
            lower_evidence_lookup_index_product(&selected_plan, &selected_lookup_slice).map_err(
                |error| {
                    PlanarBooleanEventLedgerLookupExecutionDenial::new(
                        PlanarBooleanEventLedgerLookupExecutionDenialKind::IndexProduct,
                        error.detail(),
                    )
                },
            )?;
        let execution_receipt = execute_evidence_lookup(&EvidenceLookupExecutionRequest::new(
            &selected_plan,
            &index_product,
        ))
        .map_err(|error| {
            PlanarBooleanEventLedgerLookupExecutionDenial::new(
                PlanarBooleanEventLedgerLookupExecutionDenialKind::Execution,
                error.detail(),
            )
        })?;
        let witness = PlanarBooleanEventLedgerLookupExecutionWitness::certify(
            event_ledger,
            &spatial_touch_authority,
            &selected_plan,
            &execution_receipt,
        )?;
        Ok(Self {
            witness,
            selected_family_identity: selected_family.identity().as_str().to_string(),
            selected_family_declaration_digest: selected_family.declaration_digest().to_string(),
            selected_family_diagnostic_witness_shape: selected_family.diagnostic_witness().clone(),
            selected_plan,
            selected_lookup_slice,
            index_product,
            execution_receipt,
        })
    }

    pub const fn witness(&self) -> &PlanarBooleanEventLedgerLookupExecutionWitness {
        &self.witness
    }

    pub fn selected_family_identity(&self) -> &str {
        &self.selected_family_identity
    }

    pub fn selected_family_declaration_digest(&self) -> &str {
        &self.selected_family_declaration_digest
    }

    pub const fn selected_family_diagnostic_witness_shape(
        &self,
    ) -> &EvidenceLookupDiagnosticWitnessShape {
        &self.selected_family_diagnostic_witness_shape
    }

    pub const fn selected_plan(&self) -> &EvidenceLookupSelectedPlan {
        &self.selected_plan
    }

    pub const fn selected_lookup_slice(&self) -> &SelectedLookupSliceLedger {
        &self.selected_lookup_slice
    }

    pub const fn index_product(&self) -> &EvidenceLookupIndexProduct {
        &self.index_product
    }

    pub const fn execution_receipt(&self) -> &EvidenceLookupExecutionReceipt {
        &self.execution_receipt
    }
}

fn deny_unrelated_boolean_residue(
    event_ledger: &PlanarBooleanEventLedgerReceipt,
    evidence_ledger: &CompleteWorkloadEvidenceLedger,
    spatial_touch_authority: &SpatialGeometryEvidenceTouchAuthority,
) -> Result<(), PlanarBooleanEventLedgerLookupExecutionDenial> {
    if let Some(unrelated_row) = evidence_ledger.rows().iter().find(|row| {
        if !row.stage().is_boolean_stage() {
            return false;
        }

        if row.stage() == spatial_touch_authority.evidence_stage()
            && row.evidence_identity() == spatial_touch_authority.evidence_identity()
        {
            return false;
        }

        if row.stage() == WorkloadEvidenceStage::BooleanSegmentPairEnumeration
            && row.evidence_identity() == event_ledger.segment_pair_enumeration_identity()
        {
            return false;
        }

        true
    }) {
        return Err(PlanarBooleanEventLedgerLookupExecutionDenial::new(
            PlanarBooleanEventLedgerLookupExecutionDenialKind::BroadBooleanResidue,
            format!(
                "ordinary event-ledger lookup cannot admit unrelated {} before family selection",
                unrelated_row.stage().human_name()
            ),
        ));
    }

    Ok(())
}
