use crate::{authority::AuthoritativeExportBundle, failure::StoreError};

use super::ForgeStore;

impl ForgeStore {
    pub fn plan_retention_candidates(
        &self,
        policy_class: crate::RetentionPolicyClass,
    ) -> Result<crate::RetentionPlanningReport, StoreError> {
        self.backend.plan_retention_candidates(policy_class)
    }

    pub fn plan_retention_maintenance_batch(
        &self,
        policy_class: crate::RetentionPolicyClass,
    ) -> Result<crate::MaintenanceBatch, StoreError> {
        self.backend.plan_retention_maintenance_batch(policy_class)
    }

    pub fn admit_maintenance_batch(
        &mut self,
        batch: crate::MaintenanceBatch,
    ) -> Result<crate::MaintenanceAdmissionReceipt, StoreError> {
        self.backend.admit_maintenance_batch(batch)
    }

    pub fn publish_compaction_product(
        &mut self,
        plan: crate::CompactionPlan,
    ) -> Result<crate::CompactionPublicationReport, StoreError> {
        self.backend.publish_compaction_product(plan)
    }

    pub fn verify_compaction_product(
        &mut self,
        product: crate::PublishedCompactionProduct,
    ) -> Result<crate::RetainedReadCostSurface, StoreError> {
        self.backend.verify_compaction_product(product)
    }

    pub fn cutover_compaction_product(
        &mut self,
        product: crate::PublishedCompactionProduct,
    ) -> Result<crate::CompactionCutoverReport, StoreError> {
        self.backend.cutover_compaction_product(product)
    }

    pub fn execute_derived_reclaim(
        &mut self,
        witness: crate::ReclaimEligibilityWitness,
    ) -> Result<crate::ReclaimExecutionReport, StoreError> {
        self.backend.execute_derived_reclaim(witness)
    }

    pub fn execute_authoritative_reclaim(
        &mut self,
        range: crate::PolicyExpiredAuthorityRange,
    ) -> Result<crate::AuthoritativeReclaimReport, StoreError> {
        self.backend.execute_authoritative_reclaim(range)
    }

    pub fn rebuild_reclaimed_derived_family(
        &mut self,
        rebuild_unit: crate::RetainedRangeRebuildUnit,
    ) -> Result<crate::RetainedRangeRebuildReport, StoreError> {
        self.backend.rebuild_reclaimed_derived_family(rebuild_unit)
    }

    pub fn start_maintenance_declaration(
        &mut self,
        declaration: &crate::AdmittedMaintenanceDeclaration,
    ) -> Result<crate::CompletedMaintenance, crate::FailedMaintenance> {
        self.backend.start_maintenance_declaration(declaration)
    }

    pub fn resume_maintenance_declaration(
        &mut self,
        declaration_id: &crate::MaintenanceDeclarationId,
    ) -> Result<crate::CompletedMaintenance, crate::FailedMaintenance> {
        self.backend.resume_maintenance_declaration(declaration_id)
    }

    pub fn maintenance_status(
        &self,
        declaration_id: &crate::MaintenanceDeclarationId,
    ) -> Result<crate::MaintenanceStatusReport, StoreError> {
        self.backend.maintenance_status(declaration_id)
    }

    pub fn milestone_10_counter_contract(&self) -> crate::Milestone10CounterContract {
        self.backend.milestone_10_counter_contract()
    }

    pub fn milestone_10_complexity_surface(&self) -> crate::Milestone10ComplexitySurface {
        self.backend.milestone_10_complexity_surface()
    }

    pub fn milestone_11_counter_contract(&self) -> crate::Milestone11CounterContract {
        self.backend.milestone_11_counter_contract()
    }

    pub fn milestone_11_complexity_surface(&self) -> crate::Milestone11ComplexitySurface {
        self.backend.milestone_11_complexity_surface()
    }

    pub fn milestone_13_counter_contract(&self) -> crate::Milestone13CounterContract {
        self.backend.milestone_13_counter_contract()
    }

    pub fn milestone_13_complexity_surface(&self) -> crate::Milestone13ComplexitySurface {
        self.backend.milestone_13_complexity_surface()
    }

    pub fn milestone_13_artifact_report(
        &self,
    ) -> Result<crate::Milestone13ArtifactReport, StoreError> {
        self.backend.milestone_13_artifact_report()
    }

    pub fn milestone_11_maintenance_report(&self) -> crate::Milestone11MaintenanceReport {
        self.backend.milestone_11_maintenance_report()
    }

    pub fn maintenance_evidence(&self) -> crate::Milestone11MaintenanceReport {
        self.backend.maintenance_evidence()
    }

    pub fn milestone_10_artifact_report(
        &self,
    ) -> Result<crate::Milestone10ArtifactReport, StoreError> {
        self.backend.milestone_10_artifact_report()
    }

    pub fn milestone_10_certification_bundle(
        &self,
        control_export: &AuthoritativeExportBundle,
    ) -> Result<crate::Milestone10CertificationBundle, StoreError> {
        let primary_export = self.export_authoritative_records();
        let restored_export =
            Self::restore_from_authoritative_export(primary_export.clone().admit_restore())?
                .export_authoritative_records();
        let artifact_report = self.milestone_10_artifact_report()?;
        Ok(crate::Milestone10CertificationBundle::new(
            &primary_export,
            control_export,
            &restored_export,
            self.durable_media_report().backend_family(),
            artifact_report,
            self.milestone_10_complexity_surface(),
            self.milestone_10_counter_contract(),
            self.counters(),
        ))
    }

    pub fn milestone_11_certification_bundle(
        &self,
        control_export: &AuthoritativeExportBundle,
        failure_markers: &[String],
    ) -> crate::Milestone11CertificationBundle {
        let primary_export = self.export_authoritative_records();
        crate::Milestone11CertificationBundle::new(
            &primary_export,
            control_export,
            self.durable_media_report().backend_family(),
            self.milestone_11_maintenance_report(),
            self.milestone_11_complexity_surface(),
            self.milestone_11_counter_contract(),
            self.counters(),
            failure_markers,
        )
    }

    pub fn milestone_13_certification_bundle(
        &self,
        control_export: &AuthoritativeExportBundle,
    ) -> Result<crate::Milestone13CertificationBundle, StoreError> {
        let primary_export = self.export_authoritative_records();
        let artifact_report = self.milestone_13_artifact_report()?;
        Ok(crate::Milestone13CertificationBundle::new(
            &primary_export,
            control_export,
            self.durable_media_report().backend_family(),
            artifact_report,
            self.milestone_13_complexity_surface(),
            self.milestone_13_counter_contract(),
            self.counters(),
        ))
    }
}
