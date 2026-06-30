use super::{WorkloadCompositionError, WorthWorkload, WorthWorkloadParts};
use crate::workload_composition::BatchAdmissionExecutionReceipt;

impl WorthWorkload {
    pub(crate) fn with_batch_admission_execution(
        &self,
        batch_admission_execution: BatchAdmissionExecutionReceipt,
    ) -> Result<Self, WorkloadCompositionError> {
        WorthWorkload::compose(WorthWorkloadParts {
            topology: self.topology().clone(),
            geometry_binding: self.geometry_binding().clone(),
            surface_support: self.surface_support().clone(),
            projection: self.projection().clone(),
            transform: self.transform().clone(),
            retained_replay: self.retained_replay().clone(),
            batch_admission_execution: Some(batch_admission_execution),
            diagnostics: self.diagnostics().clone(),
            response: self.response().clone(),
            evidence_ledger: self.evidence_ledger().clone(),
        })
    }
}
