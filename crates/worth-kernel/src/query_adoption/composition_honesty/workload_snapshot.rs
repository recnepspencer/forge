use crate::workload_composition::{WorkloadCatalog, WorthWorkload};

use super::WorthKernelCompositionHonestyError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct WorthKernelRepresentativeWorkloadSnapshot {
    topology_receipt_identity: String,
    spatial_receipt_identities: Vec<String>,
    evidence_row_count: usize,
}

impl WorthKernelRepresentativeWorkloadSnapshot {
    pub(super) fn current() -> Result<Self, WorthKernelCompositionHonestyError> {
        let built = WorkloadCatalog::cube()
            .with_retained_replay_artifacts()
            .build()
            .map_err(WorthKernelCompositionHonestyError::RepresentativeWorkload)?;
        Ok(Self::from_workload(built.workload()))
    }

    fn from_workload(workload: &WorthWorkload) -> Self {
        Self {
            topology_receipt_identity: workload.topology().identity().name().to_string(),
            spatial_receipt_identities: vec![
                workload
                    .geometry_binding()
                    .identity()
                    .receipt_identity()
                    .to_string(),
                workload
                    .surface_support()
                    .identity()
                    .receipt_identity()
                    .to_string(),
                workload
                    .projection()
                    .identity()
                    .receipt_identity()
                    .to_string(),
                workload
                    .transform()
                    .identity()
                    .receipt_identity()
                    .to_string(),
                workload
                    .retained_replay()
                    .identity()
                    .receipt_identity()
                    .to_string(),
                workload
                    .diagnostics()
                    .identity()
                    .receipt_identity()
                    .to_string(),
                workload
                    .response()
                    .identity()
                    .receipt_identity()
                    .to_string(),
            ],
            evidence_row_count: workload.evidence_ledger().counters().rows(),
        }
    }

    pub(super) fn topology_receipt_identity(&self) -> &str {
        &self.topology_receipt_identity
    }

    pub(super) fn spatial_receipt_identities(&self) -> &[String] {
        &self.spatial_receipt_identities
    }

    pub(super) const fn evidence_row_count(&self) -> usize {
        self.evidence_row_count
    }

    #[cfg(test)]
    pub(super) fn with_missing_spatial_receipts(mut self) -> Self {
        self.spatial_receipt_identities.clear();
        self
    }
}
