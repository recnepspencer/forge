use crate::workload_platform::evidence_ledger::{
    CompleteWorkloadEvidenceLedger, SpatialGeometryEvidenceTouchAuthority, WorkloadEvidenceStage,
};
use crate::workload_platform::evidence_lookup_execution::{
    execute_evidence_lookup, EvidenceLookupExecutionReceipt, EvidenceLookupExecutionRequest,
};
use crate::workload_platform::evidence_lookup_family_catalog::{
    EvidenceLookupFamilyCatalogCloseout, EvidenceLookupFamilyDeclaration,
    EvidenceLookupProjectionFactFamily, EvidenceLookupQueryImportEvidence,
};
use crate::workload_platform::evidence_lookup_index_product::{
    admit_evidence_lookup_index_product, EvidenceLookupIndexProduct,
};
use crate::workload_platform::evidence_lookup_input_admission::{
    admit_evidence_lookup_input, current_projection_consumption_receipt,
    EvidenceLookupAdmittedInput, EvidenceLookupInputAdmissionError,
    EvidenceLookupInputAdmissionRequest, EvidenceLookupQueryAdmissionEvidenceSet,
    EvidenceLookupStageReceiptAdmission,
};
use crate::workload_platform::evidence_lookup_plan_selection::{
    select_evidence_lookup_plan, EvidenceLookupSelectedPlan,
};

use super::current_world::{
    current_complete_ledger_for_authority, current_spatial_touch_authority,
};
use super::{EvidenceLookupCoveredStageCutoverProof, EvidenceLookupStageCutoverError};

#[derive(Debug)]
pub(crate) struct EvidenceLookupCurrentCoveredStageCutoverPath {
    spatial_touch_authority: SpatialGeometryEvidenceTouchAuthority,
    stage_receipt_identity: String,
    admitted_input: EvidenceLookupAdmittedInput,
    selected_plan: EvidenceLookupSelectedPlan,
    index_product: EvidenceLookupIndexProduct,
    execution_receipt: EvidenceLookupExecutionReceipt,
}

impl EvidenceLookupCurrentCoveredStageCutoverPath {
    #[cfg(any(test, feature = "test-support-lowering"))]
    pub(crate) fn spatial_touch_authority(&self) -> &SpatialGeometryEvidenceTouchAuthority {
        &self.spatial_touch_authority
    }

    #[cfg(any(test, feature = "test-support-lowering"))]
    pub(crate) fn stage_receipt_identity(&self) -> &str {
        &self.stage_receipt_identity
    }

    pub(crate) fn admitted_input(&self) -> &EvidenceLookupAdmittedInput {
        &self.admitted_input
    }

    pub(crate) fn selected_plan(&self) -> &EvidenceLookupSelectedPlan {
        &self.selected_plan
    }

    pub(crate) fn index_product(&self) -> &EvidenceLookupIndexProduct {
        &self.index_product
    }

    pub(crate) fn execution_receipt(&self) -> &EvidenceLookupExecutionReceipt {
        &self.execution_receipt
    }

    pub(crate) fn prove_for_family(
        &self,
        family_identity: &str,
    ) -> Result<EvidenceLookupCoveredStageCutoverProof, EvidenceLookupStageCutoverError> {
        EvidenceLookupCoveredStageCutoverProof::prove(
            self.selected_plan.stage(),
            &self.spatial_touch_authority,
            &self.stage_receipt_identity,
            family_identity,
            &self.selected_plan,
            &self.execution_receipt,
        )
    }
}

pub(crate) fn admit_current_family_stage_cutover_path(
    catalog: &EvidenceLookupFamilyCatalogCloseout,
    family: &EvidenceLookupFamilyDeclaration,
    stage: WorkloadEvidenceStage,
) -> Result<EvidenceLookupCurrentCoveredStageCutoverPath, EvidenceLookupCurrentPathError> {
    let projection_receipt = current_projection_consumption_receipt();
    let query_evidence = match family.query_posture().imported_evidence().cloned() {
        Some(EvidenceLookupQueryImportEvidence::ConsumerKitSupportPin { support_pin }) => Some(
            EvidenceLookupQueryAdmissionEvidenceSet::from_support_pin(support_pin),
        ),
        Some(EvidenceLookupQueryImportEvidence::ProjectionConsumptionReceipt {
            fact_family,
            ..
        }) => Some(
            EvidenceLookupQueryAdmissionEvidenceSet::from_projection_consumption_receipt(
                &projection_receipt,
                fact_family,
            ),
        ),
        Some(imported_evidence) => Some(
            EvidenceLookupQueryAdmissionEvidenceSet::from_query_import_evidence(&imported_evidence),
        ),
        None => None,
    };

    admit_current_family_stage_cutover_path_with_query_evidence(
        catalog,
        family,
        stage,
        query_evidence.as_ref(),
        Some(&projection_receipt),
    )
}

pub(crate) fn admit_current_family_stage_cutover_path_with_query_evidence(
    catalog: &EvidenceLookupFamilyCatalogCloseout,
    family: &EvidenceLookupFamilyDeclaration,
    stage: WorkloadEvidenceStage,
    query_evidence: Option<&EvidenceLookupQueryAdmissionEvidenceSet>,
    projection_receipt: Option<&forge_query::facade::ProjectionConsumptionReceipt>,
) -> Result<EvidenceLookupCurrentCoveredStageCutoverPath, EvidenceLookupCurrentPathError> {
    let spatial_touch_authority = current_spatial_touch_authority(stage)?;
    let stage_receipt_identity = spatial_touch_authority.evidence_identity().to_string();
    let receipt_family = family
        .stage_applicability()
        .stage_receipt_family_identity()
        .clone();
    let mut request =
        EvidenceLookupInputAdmissionRequest::from_spatial_touch_authority(&spatial_touch_authority)
            .with_stage_receipt_identity(
                EvidenceLookupStageReceiptAdmission::from_spatial_touch_authority(
                    &spatial_touch_authority,
                    receipt_family,
                ),
            );
    if let Some(query_evidence) = query_evidence {
        request = request.with_query_import_evidence(query_evidence.clone());
    }

    let admitted_input = admit_evidence_lookup_input(catalog, request)
        .map_err(EvidenceLookupCurrentPathError::from)?;
    let selected_plan = select_evidence_lookup_plan(catalog, &admitted_input)
        .map_err(EvidenceLookupCurrentPathError::from)?;
    let index_product = admit_evidence_lookup_index_product(
        &selected_plan,
        &complete_ledger_for_plan(&spatial_touch_authority),
    )
    .map_err(EvidenceLookupCurrentPathError::from)?;

    let mut execution_request = EvidenceLookupExecutionRequest::new(&selected_plan, &index_product);
    if selected_plan.rows().iter().any(|row| {
        row.family_identity() == family.identity().as_str()
            && row
                .query_posture()
                .requires_projection_consumption_receipt()
    }) {
        let projection_receipt = projection_receipt.ok_or_else(|| {
            EvidenceLookupCurrentPathError::missing_projection_receipt(
                family.identity().as_str(),
                stage,
            )
        })?;
        execution_request = execution_request.with_projection_consumption_receipt(
            family.identity().as_str().to_string(),
            EvidenceLookupProjectionFactFamily::SpatialTouchOperandProjection,
            projection_receipt,
        );
    }
    let execution_receipt = execute_evidence_lookup(&execution_request)
        .map_err(EvidenceLookupCurrentPathError::from)?;

    Ok(EvidenceLookupCurrentCoveredStageCutoverPath {
        spatial_touch_authority,
        stage_receipt_identity,
        admitted_input,
        selected_plan,
        index_product,
        execution_receipt,
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct EvidenceLookupCurrentPathError {
    detail: String,
}

impl EvidenceLookupCurrentPathError {
    pub(crate) fn from_spatial_touch_denial(
        stage: WorkloadEvidenceStage,
        error: crate::workload_platform::evidence_ledger::SpatialGeometryEvidenceTouchDenial,
    ) -> Self {
        Self {
            detail: format!(
                "current covered-stage cutover path could not admit spatial touch authority for stage `{}`: {}",
                stage.human_name(),
                error.human_reason()
            ),
        }
    }

    fn missing_projection_receipt(family_identity: &str, stage: WorkloadEvidenceStage) -> Self {
        Self {
            detail: format!(
                "current covered-stage cutover path requires one projection consumption receipt for family `{family_identity}` at stage `{}`",
                stage.human_name()
            ),
        }
    }

    pub(crate) fn detail(&self) -> &str {
        &self.detail
    }
}

impl From<EvidenceLookupInputAdmissionError> for EvidenceLookupCurrentPathError {
    fn from(value: EvidenceLookupInputAdmissionError) -> Self {
        Self {
            detail: format!("input admission failed: {:?}", value.kind()),
        }
    }
}

impl
    From<crate::workload_platform::evidence_lookup_plan_selection::EvidenceLookupPlanSelectionError>
    for EvidenceLookupCurrentPathError
{
    fn from(
        value: crate::workload_platform::evidence_lookup_plan_selection::EvidenceLookupPlanSelectionError,
    ) -> Self {
        Self {
            detail: format!("plan selection failed: {:?}", value.kind()),
        }
    }
}

impl From<crate::workload_platform::evidence_lookup_index_product::EvidenceLookupIndexProductError>
    for EvidenceLookupCurrentPathError
{
    fn from(
        value: crate::workload_platform::evidence_lookup_index_product::EvidenceLookupIndexProductError,
    ) -> Self {
        Self {
            detail: format!("index product failed: {:?}", value.kind()),
        }
    }
}

impl From<crate::workload_platform::evidence_lookup_execution::EvidenceLookupExecutionError>
    for EvidenceLookupCurrentPathError
{
    fn from(
        value: crate::workload_platform::evidence_lookup_execution::EvidenceLookupExecutionError,
    ) -> Self {
        Self {
            detail: format!("execution failed: {:?}", value.kind()),
        }
    }
}

fn complete_ledger_for_plan(
    authority: &SpatialGeometryEvidenceTouchAuthority,
) -> CompleteWorkloadEvidenceLedger {
    current_complete_ledger_for_authority(authority)
}
