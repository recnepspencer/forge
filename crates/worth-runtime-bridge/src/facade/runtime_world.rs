pub use crate::correspondence::RuntimeWorldCorrespondenceInspectionCounters;
use crate::correspondence::RuntimeWorldCorrespondenceInspectionLedger;
use crate::correspondence::{
    admit_installed_basis, compare_current_basis, AdmittedRuntimeWorldCorrespondenceBasis,
    RuntimeWorldCorrespondenceAdmissionDenial,
};
use std::sync::Arc;

use super::{BridgeInstalledSemanticCorrespondence, RuntimeBridge};

/// Narrow Bridge admission port for the Runtime World composition owner.
///
/// It admits an already-installed Bridge correspondence and exposes no
/// mapping, Signal, or Relational construction surface.
#[derive(Clone)]
pub struct RuntimeWorldCorrespondencePort {
    runtime: RuntimeBridge,
    inspection: Arc<RuntimeWorldCorrespondenceInspectionLedger>,
}

impl std::fmt::Debug for RuntimeWorldCorrespondencePort {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("RuntimeWorldCorrespondencePort")
            .field("runtime", &self.runtime)
            .finish()
    }
}

impl RuntimeWorldCorrespondencePort {
    pub fn admit_installed_basis(
        &self,
        installed: &BridgeInstalledSemanticCorrespondence,
    ) -> Result<AdmittedRuntimeWorldCorrespondenceBasis, RuntimeWorldCorrespondenceAdmissionDenial>
    {
        admit_installed_basis(&self.runtime, installed, &self.inspection)
    }

    pub fn compare_current_exact(
        &self,
        admitted: &AdmittedRuntimeWorldCorrespondenceBasis,
    ) -> Result<(), RuntimeWorldCorrespondenceAdmissionDenial> {
        compare_current_basis(&self.runtime, admitted, &self.inspection)
    }

    pub fn inspection_counters(&self) -> RuntimeWorldCorrespondenceInspectionCounters {
        self.inspection.snapshot()
    }
}

impl RuntimeBridge {
    /// Borrow the Bridge's exact installed-correspondence admission seam.
    pub fn runtime_world_correspondence_port(&self) -> RuntimeWorldCorrespondencePort {
        RuntimeWorldCorrespondencePort {
            runtime: self.clone(),
            inspection: Arc::new(RuntimeWorldCorrespondenceInspectionLedger::default()),
        }
    }
}
