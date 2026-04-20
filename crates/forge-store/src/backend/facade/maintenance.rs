use crate::evidence::CanonicalizationMetrics;
use crate::failure::StoreError;

use super::{dispatch_mut, dispatch_ref, StoreBackend};

impl StoreBackend {
    pub fn plan_retention_candidates(
        &self,
        policy_class: crate::RetentionPolicyClass,
    ) -> Result<crate::RetentionPlanningReport, StoreError> {
        dispatch_ref!(self, |backend| backend.plan_retention_candidates(policy_class))
    }
    pub fn plan_retention_maintenance_batch(
        &self,
        policy_class: crate::RetentionPolicyClass,
    ) -> Result<crate::MaintenanceBatch, StoreError> {
        dispatch_ref!(self, |backend| backend.plan_retention_maintenance_batch(policy_class))
    }
    pub fn admit_maintenance_batch(
        &mut self,
        batch: crate::MaintenanceBatch,
    ) -> Result<crate::MaintenanceAdmissionReceipt, StoreError> {
        dispatch_mut!(self, |backend| backend.admit_maintenance_batch(batch))
    }
    pub fn publish_compaction_product(
        &mut self,
        plan: crate::CompactionPlan,
    ) -> Result<crate::CompactionPublicationReport, StoreError> {
        dispatch_mut!(self, |backend| backend.publish_compaction_product(plan))
    }
    pub fn verify_compaction_product(
        &mut self,
        product: crate::PublishedCompactionProduct,
    ) -> Result<crate::RetainedReadCostSurface, StoreError> {
        dispatch_mut!(self, |backend| backend.verify_compaction_product(product))
    }
    pub fn cutover_compaction_product(
        &mut self,
        product: crate::PublishedCompactionProduct,
    ) -> Result<crate::CompactionCutoverReport, StoreError> {
        dispatch_mut!(self, |backend| backend.cutover_compaction_product(product))
    }
    pub fn execute_derived_reclaim(
        &mut self,
        witness: crate::ReclaimEligibilityWitness,
    ) -> Result<crate::ReclaimExecutionReport, StoreError> {
        dispatch_mut!(self, |backend| backend.execute_derived_reclaim(witness))
    }
    pub fn execute_authoritative_reclaim(
        &mut self,
        range: crate::PolicyExpiredAuthorityRange,
    ) -> Result<crate::AuthoritativeReclaimReport, StoreError> {
        dispatch_mut!(self, |backend| backend.execute_authoritative_reclaim(range))
    }
    pub fn rebuild_reclaimed_derived_family(
        &mut self,
        rebuild_unit: crate::RetainedRangeRebuildUnit,
    ) -> Result<crate::RetainedRangeRebuildReport, StoreError> {
        dispatch_mut!(self, |backend| backend.rebuild_reclaimed_derived_family(rebuild_unit))
    }
    pub fn start_maintenance_declaration(
        &mut self,
        declaration: &crate::AdmittedMaintenanceDeclaration,
    ) -> Result<crate::CompletedMaintenance, crate::FailedMaintenance> {
        dispatch_mut!(self, |backend| backend.start_maintenance_declaration(declaration))
    }
    pub fn resume_maintenance_declaration(
        &mut self,
        declaration_id: &crate::MaintenanceDeclarationId,
    ) -> Result<crate::CompletedMaintenance, crate::FailedMaintenance> {
        dispatch_mut!(self, |backend| backend.resume_maintenance_declaration(declaration_id))
    }
    pub fn maintenance_status(
        &self,
        declaration_id: &crate::MaintenanceDeclarationId,
    ) -> Result<crate::MaintenanceStatusReport, StoreError> {
        dispatch_ref!(self, |backend| backend.maintenance_status(declaration_id))
    }
    pub fn milestone_10_counter_contract(&self) -> crate::Milestone10CounterContract {
        dispatch_ref!(self, |backend| backend.milestone_10_counter_contract())
    }
    pub fn milestone_10_complexity_surface(&self) -> crate::Milestone10ComplexitySurface {
        dispatch_ref!(self, |backend| backend.milestone_10_complexity_surface())
    }
    pub fn milestone_11_counter_contract(&self) -> crate::Milestone11CounterContract {
        dispatch_ref!(self, |backend| backend.milestone_11_counter_contract())
    }
    pub fn milestone_11_complexity_surface(&self) -> crate::Milestone11ComplexitySurface {
        dispatch_ref!(self, |backend| backend.milestone_11_complexity_surface())
    }
    pub fn milestone_13_counter_contract(&self) -> crate::Milestone13CounterContract {
        dispatch_ref!(self, |backend| backend.milestone_13_counter_contract())
    }
    pub fn milestone_13_complexity_surface(&self) -> crate::Milestone13ComplexitySurface {
        dispatch_ref!(self, |backend| backend.milestone_13_complexity_surface())
    }
    pub fn milestone_11_maintenance_report(&self) -> crate::Milestone11MaintenanceReport {
        dispatch_ref!(self, |backend| backend.milestone_11_maintenance_report())
    }
    pub fn maintenance_evidence(&self) -> crate::Milestone11MaintenanceReport {
        dispatch_ref!(self, |backend| backend.maintenance_evidence())
    }
    pub fn milestone_10_artifact_report(
        &self,
    ) -> Result<crate::Milestone10ArtifactReport, StoreError> {
        dispatch_ref!(self, |backend| backend.milestone_10_artifact_report())
    }
    pub fn record_canonicalization(&self, metrics: CanonicalizationMetrics) {
        dispatch_ref!(self, |backend| backend.record_canonicalization(metrics))
    }
}
