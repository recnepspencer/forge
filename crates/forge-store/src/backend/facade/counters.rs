use crate::evidence::{Milestone7AccessStructureVerification, StoreCounterSnapshot};
use crate::media::DurableMediaReport;

use super::{dispatch_ref, StoreBackend};

impl StoreBackend {
    pub fn counter_snapshot(&self) -> StoreCounterSnapshot {
        dispatch_ref!(self, |backend| backend.counters().snapshot())
    }
    pub(crate) fn record_physical_chunk_export(&self, chunk_width: u64) {
        dispatch_ref!(self, |backend| backend.record_physical_chunk_export(chunk_width))
    }
    pub fn export_bundle(&self) -> crate::authority::AuthoritativeExportBundle {
        dispatch_ref!(self, |backend| backend.export_bundle())
    }
    pub fn durable_media_report(&self) -> DurableMediaReport {
        dispatch_ref!(self, |backend| backend.durable_media_report())
    }
    pub fn milestone_7_access_structure_verification(
        &self,
    ) -> Milestone7AccessStructureVerification {
        dispatch_ref!(self, |backend| backend.milestone_7_access_structure_verification())
    }
    pub fn milestone_6_access_structure_verification(
        &self,
    ) -> crate::evidence::Milestone6AccessStructureVerification {
        dispatch_ref!(self, |backend| backend.milestone_6_access_structure_verification())
    }
    pub fn record_durable_mode_selection(&self) {
        dispatch_ref!(self, |backend| backend.counters().record_durable_mode_selection())
    }
    pub fn record_embedded_mode_selection(&self) {
        dispatch_ref!(self, |backend| backend.counters().record_embedded_mode_selection())
    }
    pub fn record_hosted_runtime_start(&self) {
        dispatch_ref!(self, |backend| backend.counters().record_hosted_runtime_start())
    }
    pub fn record_hosted_runtime_stop(&self) {
        dispatch_ref!(self, |backend| backend.counters().record_hosted_runtime_stop())
    }
    pub fn record_external_commit_intake(&self) {
        dispatch_ref!(self, |backend| backend.counters().record_external_commit_intake())
    }
    pub fn record_external_checkpoint_intake(&self) {
        dispatch_ref!(self, |backend| backend.counters().record_external_checkpoint_intake())
    }
    #[cfg(test)]
    pub fn record_embedded_checkpoint_authority_rejection(&self) {
        dispatch_ref!(self, |backend| backend.counters().record_embedded_checkpoint_authority_rejection())
    }
    #[cfg(test)]
    pub fn record_mode_misuse_rejection(&self) {
        dispatch_ref!(self, |backend| backend.counters().record_mode_misuse_rejection())
    }
    pub fn record_durable_commit_acknowledged(&self) {
        dispatch_ref!(self, |backend| backend.counters().record_durable_commit_acknowledged())
    }
    pub fn record_support_artifact_recovery_gap(&self, count: u64) {
        dispatch_ref!(self, |backend| backend.counters().record_support_artifact_recovery_gap(count))
    }
}
