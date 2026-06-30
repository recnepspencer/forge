use forge_query::facade::consumer_kit::ForgeQueryGraphObligationSupportPin;
use forge_query::facade::runtime::{
    ForgeQueryGraphObligationKind, ForgeQueryGraphObligationSupportLane,
};
use topology::derived_invalidation_family_catalog::DerivedTopologyProductFamilyIdentity;

use crate::workload_platform::evidence_ledger::WorkloadEvidenceStage;
use crate::workload_platform::evidence_lookup_inventory::{
    current_evidence_lookup_inventory, EvidenceLookupDisposition, EvidenceLookupReplacementPhase,
};

use super::declaration::{
    EvidenceLookupFamilyDeclaration, EvidenceLookupProductPosture,
    EvidenceLookupSpatialTouchAuthorityRequirement,
};
use super::error::{EvidenceLookupFamilyCatalogError, EvidenceLookupFamilyCatalogErrorKind};
use super::family_identity::EvidenceLookupFamilyIdentity;
use super::posture::{
    EvidenceLookupDiagnosticWitnessShape, EvidenceLookupEvidenceClass,
    EvidenceLookupEvidenceClassSet, EvidenceLookupFamilyIndexPosture,
    EvidenceLookupFamilyQueryPosture, EvidenceLookupProjectionFactFamily,
    EvidenceLookupTopologyInputPosture,
};
use super::source_pressure::EvidenceLookupFamilySourceInventoryPressure;
use super::stage_applicability::EvidenceLookupStageApplicability;
use super::stage_receipt_identity::EvidenceLookupStageReceiptFamilyIdentity;

pub(crate) fn current_family_declarations(
) -> Result<Vec<EvidenceLookupFamilyDeclaration>, EvidenceLookupFamilyCatalogError> {
    let inventory = current_evidence_lookup_inventory().map_err(|error| {
        EvidenceLookupFamilyCatalogError::with_message(
            EvidenceLookupFamilyCatalogErrorKind::MissingPhaseTwoInventoryPressure,
            format!("{:?}", error.kind()),
        )
    })?;
    let phase_two_migrate_row_count = inventory
        .rows()
        .iter()
        .filter(|row| {
            row.disposition() == EvidenceLookupDisposition::Migrate
                && row.replacement_phase() == EvidenceLookupReplacementPhase::PhaseTwoFamilyCatalog
        })
        .count();
    if phase_two_migrate_row_count == 0 {
        return Err(EvidenceLookupFamilyCatalogError::new(
            EvidenceLookupFamilyCatalogErrorKind::MissingPhaseTwoInventoryPressure,
        ));
    }
    let phase_two_pressure = EvidenceLookupFamilySourceInventoryPressure::phase_two_family_catalog(
        phase_two_migrate_row_count,
        inventory.closeout_digest(),
    );

    Ok(vec![
        boolean_overlap_stage_receipt_family(phase_two_pressure.clone())?,
        boolean_projection_consumption_family(phase_two_pressure.clone())?,
        boolean_event_ledger_family(phase_two_pressure)?,
    ])
}

fn boolean_overlap_stage_receipt_family(
    phase_two_pressure: EvidenceLookupFamilySourceInventoryPressure,
) -> Result<EvidenceLookupFamilyDeclaration, EvidenceLookupFamilyCatalogError> {
    EvidenceLookupFamilyDeclaration::builder()
        .identity(EvidenceLookupFamilyIdentity::declared(
            "spatial-touch.boolean.overlap-evidence.v1",
        ))
        .spatial_touch_authority(
            EvidenceLookupSpatialTouchAuthorityRequirement::SealedSpatialTouchAuthorityRequired,
        )
        .topology_input_posture(
            EvidenceLookupTopologyInputPosture::derived_product_receipt_required(
                DerivedTopologyProductFamilyIdentity::LoopCycles,
            ),
        )
        .stage_applicability(EvidenceLookupStageApplicability::matching_stages(
            vec![
                WorkloadEvidenceStage::BooleanSharedPlaneIdentity,
                WorkloadEvidenceStage::BooleanLocalFrameSelection,
            ],
            EvidenceLookupStageReceiptFamilyIdentity::boolean_common_plane(),
        )?)
        .evidence_classes(EvidenceLookupEvidenceClassSet::new(vec![
            EvidenceLookupEvidenceClass::BooleanStageReceipt,
            EvidenceLookupEvidenceClass::SpatialTouchEvidence,
            EvidenceLookupEvidenceClass::TopologyDerivedReceiptReference,
        ])?)
        .lookup_product_posture(EvidenceLookupProductPosture::DeclarationOnlySelectionRequired)
        .index_posture(EvidenceLookupFamilyIndexPosture::sparse_lookup_plan_required())
        .query_posture(
            EvidenceLookupFamilyQueryPosture::imported_support_pin_required(overlap_support_pin()),
        )
        .diagnostic_witness(
            EvidenceLookupDiagnosticWitnessShape::spatial_touch_stage_receipt_and_query_posture(),
        )
        .source_inventory_pressure(phase_two_pressure)
        .build()
}

fn boolean_event_ledger_family(
    phase_two_pressure: EvidenceLookupFamilySourceInventoryPressure,
) -> Result<EvidenceLookupFamilyDeclaration, EvidenceLookupFamilyCatalogError> {
    EvidenceLookupFamilyDeclaration::builder()
        .identity(EvidenceLookupFamilyIdentity::declared(
            "spatial-touch.boolean.event-ledger-evidence.v1",
        ))
        .spatial_touch_authority(
            EvidenceLookupSpatialTouchAuthorityRequirement::SealedSpatialTouchAuthorityRequired,
        )
        .topology_input_posture(EvidenceLookupTopologyInputPosture::not_required())
        .stage_applicability(EvidenceLookupStageApplicability::matching_stages(
            vec![
                WorkloadEvidenceStage::BooleanSplit,
                WorkloadEvidenceStage::BooleanEventLedger,
            ],
            EvidenceLookupStageReceiptFamilyIdentity::boolean_event_ledger(),
        )?)
        .evidence_classes(EvidenceLookupEvidenceClassSet::new(vec![
            EvidenceLookupEvidenceClass::BooleanStageReceipt,
            EvidenceLookupEvidenceClass::SpatialTouchEvidence,
        ])?)
        .lookup_product_posture(EvidenceLookupProductPosture::DeclarationOnlySelectionRequired)
        .index_posture(EvidenceLookupFamilyIndexPosture::sparse_lookup_plan_required())
        .query_posture(EvidenceLookupFamilyQueryPosture::not_required())
        .diagnostic_witness(
            EvidenceLookupDiagnosticWitnessShape::spatial_touch_stage_receipt_only(),
        )
        .source_inventory_pressure(phase_two_pressure)
        .build()
}

fn boolean_projection_consumption_family(
    phase_two_pressure: EvidenceLookupFamilySourceInventoryPressure,
) -> Result<EvidenceLookupFamilyDeclaration, EvidenceLookupFamilyCatalogError> {
    EvidenceLookupFamilyDeclaration::builder()
        .identity(EvidenceLookupFamilyIdentity::declared(
            "spatial-touch.boolean.projection-consumption-evidence.v1",
        ))
        .spatial_touch_authority(
            EvidenceLookupSpatialTouchAuthorityRequirement::SealedSpatialTouchAuthorityRequired,
        )
        .topology_input_posture(EvidenceLookupTopologyInputPosture::not_required())
        .stage_applicability(EvidenceLookupStageApplicability::matching_stages(
            vec![
                WorkloadEvidenceStage::BooleanOperandAProjectionConsumption,
                WorkloadEvidenceStage::BooleanOperandBProjectionConsumption,
            ],
            EvidenceLookupStageReceiptFamilyIdentity::boolean_operand_projection_consumption(),
        )?)
        .evidence_classes(EvidenceLookupEvidenceClassSet::new(vec![
            EvidenceLookupEvidenceClass::BooleanStageReceipt,
            EvidenceLookupEvidenceClass::SpatialTouchEvidence,
        ])?)
        .lookup_product_posture(EvidenceLookupProductPosture::DeclarationOnlySelectionRequired)
        .index_posture(EvidenceLookupFamilyIndexPosture::bounded_dense_lookup_plan_required())
        .query_posture(
            EvidenceLookupFamilyQueryPosture::imported_projection_consumption_required(
                EvidenceLookupProjectionFactFamily::SpatialTouchOperandProjection,
            ),
        )
        .diagnostic_witness(
            EvidenceLookupDiagnosticWitnessShape::spatial_touch_stage_receipt_and_query_posture(),
        )
        .source_inventory_pressure(phase_two_pressure)
        .build()
}

fn overlap_support_pin() -> ForgeQueryGraphObligationSupportPin {
    ForgeQueryGraphObligationSupportPin::supported([(
        ForgeQueryGraphObligationKind::OperatingContextGate,
        ForgeQueryGraphObligationSupportLane::WorthKernelPhaseChain,
    )])
}
