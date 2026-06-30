use crate::workload_platform::evidence_ledger::WorkloadEvidenceStage;
use crate::workload_platform::evidence_lookup_diagnostics::{
    derive_evidence_lookup_diagnostics, EvidenceLookupDiagnosticCloseout,
};
use crate::workload_platform::evidence_lookup_execution::EvidenceLookupExecutionReceipt;
use crate::workload_platform::evidence_lookup_family_catalog::{
    EvidenceLookupFamilyCatalogCloseout, EvidenceLookupFamilyDeclaration,
};
use crate::workload_platform::evidence_lookup_index_product::EvidenceLookupIndexProduct;
use crate::workload_platform::evidence_lookup_input_admission::EvidenceLookupAdmittedInput;
use crate::workload_platform::evidence_lookup_plan_selection::EvidenceLookupSelectedPlan;
use crate::workload_platform::evidence_lookup_stage_cutover::current_path::admit_current_family_stage_cutover_path;

use super::error::{
    EvidenceLookupQuerySurfaceMatrixError, EvidenceLookupQuerySurfaceMatrixErrorKind,
};

pub(super) struct CurrentQuerySurfaceWitness {
    admitted_input: EvidenceLookupAdmittedInput,
    selected_plan: EvidenceLookupSelectedPlan,
    index_product: EvidenceLookupIndexProduct,
    execution_receipt: EvidenceLookupExecutionReceipt,
    diagnostics: EvidenceLookupDiagnosticCloseout,
}

impl CurrentQuerySurfaceWitness {
    pub(super) fn admitted_input(&self) -> &EvidenceLookupAdmittedInput {
        &self.admitted_input
    }

    pub(super) fn selected_plan(&self) -> &EvidenceLookupSelectedPlan {
        &self.selected_plan
    }

    pub(super) fn index_product(&self) -> &EvidenceLookupIndexProduct {
        &self.index_product
    }

    pub(super) fn execution_receipt(&self) -> &EvidenceLookupExecutionReceipt {
        &self.execution_receipt
    }

    pub(super) fn diagnostics(&self) -> &EvidenceLookupDiagnosticCloseout {
        &self.diagnostics
    }
}

pub(super) fn current_query_surface_witness(
    catalog: &EvidenceLookupFamilyCatalogCloseout,
    family: &EvidenceLookupFamilyDeclaration,
    stage: WorkloadEvidenceStage,
) -> Result<CurrentQuerySurfaceWitness, EvidenceLookupQuerySurfaceMatrixError> {
    let path = admit_current_family_stage_cutover_path(catalog, family, stage)
        .map_err(|error| build_error(family.identity().as_str(), stage, error.detail()))?;

    let diagnostics =
        derive_evidence_lookup_diagnostics(path.selected_plan(), path.execution_receipt())
            .map_err(|error| build_error(family.identity().as_str(), stage, error.detail()))?;

    Ok(CurrentQuerySurfaceWitness {
        admitted_input: path.admitted_input().clone(),
        selected_plan: path.selected_plan().clone(),
        index_product: path.index_product().clone(),
        execution_receipt: path.execution_receipt().clone(),
        diagnostics,
    })
}

fn build_error(
    family_identity: &str,
    stage: WorkloadEvidenceStage,
    detail: impl Into<String>,
) -> EvidenceLookupQuerySurfaceMatrixError {
    EvidenceLookupQuerySurfaceMatrixError::new(
        EvidenceLookupQuerySurfaceMatrixErrorKind::CurrentPathBuildFailure,
        format!(
            "current query surface path failed for family `{family_identity}` at stage `{}`: {}",
            stage.human_name(),
            detail.into()
        ),
    )
}
