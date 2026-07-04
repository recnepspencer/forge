use super::super::request::EvidenceLookupExecutionRequest;
use crate::workload_platform::evidence_ledger::{
    receipt_backed_event_ledger_touch_authority_for_admission_tests,
    receipt_backed_touch_authority_for_admission_tests, BooleanEvidenceRowAuthority,
    BooleanEvidenceStageKind, CompleteWorkloadEvidenceLedger, SelectedLookupSliceLedger,
    SelectedLookupSliceLedgerAssembly, WorkloadEvidenceStage,
};
use crate::workload_platform::evidence_lookup_family_catalog::{
    current_evidence_lookup_family_catalog, EvidenceLookupFamilyCatalogCloseout,
    EvidenceLookupProjectionFactFamily, EvidenceLookupQueryImportEvidence,
    EvidenceLookupStageReceiptFamilyIdentity,
};
use crate::workload_platform::evidence_lookup_index_product::EvidenceLookupIndexProduct;
use crate::workload_platform::evidence_lookup_input_admission::{
    admit_evidence_lookup_input, real_projection_consumption_receipt,
    EvidenceLookupInputAdmissionRequest, EvidenceLookupQueryAdmissionEvidenceSet,
    EvidenceLookupStageReceiptAdmission,
};
use crate::workload_platform::evidence_lookup_plan_selection::{
    select_evidence_lookup_plan, EvidenceLookupSelectedPlan,
};
use crate::workload_platform::spatial_compiled_product_consumer_cutover::lower_evidence_lookup_index_product;
use forge_query::facade::consumer_kit::ForgeQueryGraphObligationSupportPin;
use forge_query::facade::runtime::{
    ForgeQueryGraphObligationKind, ForgeQueryGraphObligationSupportLane,
};
use forge_query::facade::ProjectionConsumptionReceipt;

pub(super) struct ExecutionSubject {
    catalog: EvidenceLookupFamilyCatalogCloseout,
    stage: WorkloadEvidenceStage,
    receipt_family: EvidenceLookupStageReceiptFamilyIdentity,
    authority: crate::workload_platform::evidence_ledger::SpatialGeometryEvidenceTouchAuthority,
}

impl ExecutionSubject {
    pub(super) fn sparse_event_ledger() -> Self {
        Self {
            catalog: current_evidence_lookup_family_catalog().expect("catalog closes"),
            stage: WorkloadEvidenceStage::BooleanEventLedger,
            receipt_family: EvidenceLookupStageReceiptFamilyIdentity::boolean_event_ledger(),
            authority: receipt_backed_event_ledger_touch_authority_for_admission_tests(),
        }
    }

    pub(super) fn dense_projection_consumption() -> Self {
        Self {
            catalog: current_evidence_lookup_family_catalog().expect("catalog closes"),
            stage: WorkloadEvidenceStage::BooleanOperandAProjectionConsumption,
            receipt_family:
                EvidenceLookupStageReceiptFamilyIdentity::boolean_operand_projection_consumption(),
            authority: receipt_backed_touch_authority_for_admission_tests(
                BooleanEvidenceStageKind::OperandAProjectionConsumption,
                "phase-6-projection-consumption-receipt",
            ),
        }
    }

    pub(super) fn selected_plan(&self) -> EvidenceLookupSelectedPlan {
        let admitted =
            admit_evidence_lookup_input(&self.catalog, self.request()).expect("input admits");
        select_evidence_lookup_plan(&self.catalog, &admitted).expect("plan selects")
    }

    pub(super) fn index_product(
        &self,
        selected_plan: &EvidenceLookupSelectedPlan,
    ) -> EvidenceLookupIndexProduct {
        lower_evidence_lookup_index_product(
            selected_plan,
            &selected_lookup_slice_for_plan(selected_plan),
        )
        .expect("index product admits")
    }

    pub(super) fn execution_request<'a>(
        &'a self,
        selected_plan: &'a EvidenceLookupSelectedPlan,
        index_product: &'a EvidenceLookupIndexProduct,
    ) -> EvidenceLookupExecutionRequest<'a> {
        EvidenceLookupExecutionRequest::new(selected_plan, index_product)
    }

    pub(super) fn execution_request_with_projection_receipt<'a>(
        &'a self,
        selected_plan: &'a EvidenceLookupSelectedPlan,
        index_product: &'a EvidenceLookupIndexProduct,
        projection_receipt: &'a ProjectionConsumptionReceipt,
    ) -> EvidenceLookupExecutionRequest<'a> {
        self.execution_request(selected_plan, index_product)
            .with_projection_consumption_receipt(
                projection_required_family_identity(selected_plan),
                EvidenceLookupProjectionFactFamily::SpatialTouchOperandProjection,
                projection_receipt,
            )
    }

    fn request(&self) -> EvidenceLookupInputAdmissionRequest<'_> {
        let request =
            EvidenceLookupInputAdmissionRequest::from_spatial_touch_authority(&self.authority)
                .with_stage_receipt_identity(
                    EvidenceLookupStageReceiptAdmission::from_spatial_touch_authority(
                        &self.authority,
                        self.receipt_family.clone(),
                    ),
                );
        match query_import_for_stage(&self.catalog, self.stage) {
            Some(EvidenceLookupQueryImportEvidence::ConsumerKitSupportPin { support_pin }) => {
                request.with_query_import_evidence(
                    EvidenceLookupQueryAdmissionEvidenceSet::from_support_pin(support_pin),
                )
            }
            Some(EvidenceLookupQueryImportEvidence::ProjectionConsumptionReceipt {
                fact_family,
                ..
            }) => request.with_query_import_evidence(
                EvidenceLookupQueryAdmissionEvidenceSet::from_projection_consumption_receipt(
                    &real_projection_consumption_receipt(),
                    fact_family,
                ),
            ),
            Some(query_import) => request.with_query_import_evidence(
                EvidenceLookupQueryAdmissionEvidenceSet::from_query_import_evidence(&query_import),
            ),
            None => request,
        }
    }
}

pub(super) fn complete_ledger_for_plan(
    selected_plan: &EvidenceLookupSelectedPlan,
) -> CompleteWorkloadEvidenceLedger {
    let authority = authority_for_stage(selected_plan.stage());
    let stage_receipt = EvidenceLookupStageReceiptAdmission::from_spatial_touch_authority(
        &authority,
        receipt_family_for_stage(selected_plan.stage()),
    );
    let unrelated = receipt_backed_touch_authority_for_admission_tests(
        BooleanEvidenceStageKind::SharedPlaneIdentity,
        "phase-6-unrelated-shared-plane-receipt",
    );
    SelectedLookupSliceLedgerAssembly::from_touch_authority(&authority, &stage_receipt)
        .with_additional_boolean_receipt(&UnrelatedBooleanReceipt::from_touch_authority(&unrelated))
        .assemble()
        .expect("assembled lookup ledger closes")
}

pub(super) fn selected_lookup_slice_for_plan(
    selected_plan: &EvidenceLookupSelectedPlan,
) -> SelectedLookupSliceLedger {
    let authority = authority_for_stage(selected_plan.stage());
    let stage_receipt = EvidenceLookupStageReceiptAdmission::from_spatial_touch_authority(
        &authority,
        receipt_family_for_stage(selected_plan.stage()),
    );
    SelectedLookupSliceLedgerAssembly::from_touch_authority(&authority, &stage_receipt)
        .assemble_selected_lookup_slice()
        .expect("assembled selected lookup ledger closes")
}

pub(super) fn projection_required_family_identity(
    selected_plan: &EvidenceLookupSelectedPlan,
) -> String {
    selected_plan
        .rows()
        .iter()
        .find(|row| {
            row.query_posture()
                .requires_projection_consumption_receipt()
        })
        .expect("projection-required family exists")
        .family_identity()
        .to_string()
}

fn authority_for_stage(
    stage: WorkloadEvidenceStage,
) -> crate::workload_platform::evidence_ledger::SpatialGeometryEvidenceTouchAuthority {
    match stage {
        WorkloadEvidenceStage::BooleanEventLedger | WorkloadEvidenceStage::BooleanSplit => {
            receipt_backed_event_ledger_touch_authority_for_admission_tests()
        }
        WorkloadEvidenceStage::BooleanOperandAProjectionConsumption => {
            receipt_backed_touch_authority_for_admission_tests(
                BooleanEvidenceStageKind::OperandAProjectionConsumption,
                "phase-6-projection-consumption-receipt",
            )
        }
        other => panic!("unexpected execution fixture stage: {other:?}"),
    }
}

fn receipt_family_for_stage(
    stage: WorkloadEvidenceStage,
) -> EvidenceLookupStageReceiptFamilyIdentity {
    match stage {
        WorkloadEvidenceStage::BooleanEventLedger | WorkloadEvidenceStage::BooleanSplit => {
            EvidenceLookupStageReceiptFamilyIdentity::boolean_event_ledger()
        }
        WorkloadEvidenceStage::BooleanOperandAProjectionConsumption => {
            EvidenceLookupStageReceiptFamilyIdentity::boolean_operand_projection_consumption()
        }
        other => panic!("unexpected execution fixture stage: {other:?}"),
    }
}

fn query_import_for_stage(
    catalog: &EvidenceLookupFamilyCatalogCloseout,
    stage: WorkloadEvidenceStage,
) -> Option<EvidenceLookupQueryImportEvidence> {
    catalog
        .declarations()
        .iter()
        .find(|family| family.stage_applicability().stages().contains(&stage))
        .and_then(|family| family.query_posture().imported_evidence())
        .cloned()
}

#[allow(dead_code)]
fn real_support_pin() -> ForgeQueryGraphObligationSupportPin {
    ForgeQueryGraphObligationSupportPin::supported([(
        ForgeQueryGraphObligationKind::OperatingContextGate,
        ForgeQueryGraphObligationSupportLane::WorthKernelPhaseChain,
    )])
}

struct UnrelatedBooleanReceipt {
    boolean_stage: BooleanEvidenceStageKind,
    evidence_identity: String,
    support: crate::workload_platform::evidence_ledger::WorkloadEvidenceSupport,
    counters: crate::workload_platform::evidence_ledger::WorkloadEvidenceStageCounters,
}

impl UnrelatedBooleanReceipt {
    fn from_touch_authority(
        authority: &crate::workload_platform::evidence_ledger::SpatialGeometryEvidenceTouchAuthority,
    ) -> Self {
        Self {
            boolean_stage: authority.boolean_stage(),
            evidence_identity: authority.evidence_identity().to_string(),
            support: authority.support(),
            counters: authority.evidence_counters(),
        }
    }
}

impl crate::workload_platform::evidence_ledger::BooleanEvidenceReceipt for UnrelatedBooleanReceipt {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        self.boolean_stage
    }

    fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }

    fn evidence_support(
        &self,
    ) -> crate::workload_platform::evidence_ledger::WorkloadEvidenceSupport {
        self.support
    }

    fn evidence_counters(
        &self,
    ) -> crate::workload_platform::evidence_ledger::WorkloadEvidenceStageCounters {
        self.counters
    }
}

impl crate::trusted_boolean_evidence_authority::Seal for UnrelatedBooleanReceipt {}

impl BooleanEvidenceRowAuthority for UnrelatedBooleanReceipt {}
