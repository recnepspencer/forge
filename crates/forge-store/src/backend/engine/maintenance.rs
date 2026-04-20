use crate::evidence::CanonicalizationMetrics;
use crate::failure::StoreError;

use super::{StateBackedStoreBackend, StatePersistence};

impl<P: StatePersistence> StateBackedStoreBackend<P> {
    pub fn plan_retention_candidates(
        &self,
        policy_class: crate::RetentionPolicyClass,
    ) -> Result<crate::RetentionPlanningReport, StoreError> {
        super::super::retention::planning::plan_retention_candidates(self, policy_class)
    }

    pub fn plan_retention_maintenance_batch(
        &self,
        policy_class: crate::RetentionPolicyClass,
    ) -> Result<crate::MaintenanceBatch, StoreError> {
        let planning = self.plan_retention_candidates(policy_class)?;
        let batch =
            super::super::maintenance::lowering::lower_retention_maintenance_batch(
                planning.lower_to_maintenance_batch(),
            );
        self.counters
            .record_maintenance_declarations(batch.declarations().len() as u64);
        Ok(batch)
    }

    pub fn admit_maintenance_batch(
        &mut self,
        batch: crate::MaintenanceBatch,
    ) -> Result<crate::MaintenanceAdmissionReceipt, StoreError> {
        super::super::maintenance::admission::admit_maintenance_batch(self, batch)
    }

    pub fn publish_compaction_product(
        &mut self,
        plan: crate::CompactionPlan,
    ) -> Result<crate::CompactionPublicationReport, StoreError> {
        super::super::retention::compaction::publish_compaction_product(self, plan)
    }

    pub fn verify_compaction_product(
        &mut self,
        product: crate::PublishedCompactionProduct,
    ) -> Result<crate::RetainedReadCostSurface, StoreError> {
        super::super::retention::compaction::verify_compaction_product(self, product)
    }

    pub fn cutover_compaction_product(
        &mut self,
        product: crate::PublishedCompactionProduct,
    ) -> Result<crate::CompactionCutoverReport, StoreError> {
        super::super::retention::compaction::cutover_compaction_product(self, product)
    }

    pub fn execute_derived_reclaim(
        &mut self,
        witness: crate::ReclaimEligibilityWitness,
    ) -> Result<crate::ReclaimExecutionReport, StoreError> {
        super::super::retention::reclaim::execute_derived_reclaim(self, witness)
    }

    pub fn execute_authoritative_reclaim(
        &mut self,
        range: crate::PolicyExpiredAuthorityRange,
    ) -> Result<crate::AuthoritativeReclaimReport, StoreError> {
        super::super::retention::reclaim::execute_authoritative_reclaim(self, range)
    }

    pub fn rebuild_reclaimed_derived_family(
        &mut self,
        rebuild_unit: crate::RetainedRangeRebuildUnit,
    ) -> Result<crate::RetainedRangeRebuildReport, StoreError> {
        super::super::retention::reclaim::rebuild_reclaimed_derived_family(self, rebuild_unit)
    }

    pub fn start_maintenance_declaration(
        &mut self,
        declaration: &crate::AdmittedMaintenanceDeclaration,
    ) -> Result<crate::CompletedMaintenance, crate::FailedMaintenance> {
        super::super::maintenance::execution::start_maintenance_declaration(
            self,
            declaration.declaration().id(),
        )
    }

    pub fn resume_maintenance_declaration(
        &mut self,
        declaration_id: &crate::MaintenanceDeclarationId,
    ) -> Result<crate::CompletedMaintenance, crate::FailedMaintenance> {
        super::super::maintenance::execution::resume_maintenance_declaration(self, declaration_id)
    }

    pub fn maintenance_status(
        &self,
        declaration_id: &crate::MaintenanceDeclarationId,
    ) -> Result<crate::MaintenanceStatusReport, StoreError> {
        super::super::maintenance::lifecycle::maintenance_status(self, declaration_id)
    }

    pub fn milestone_10_counter_contract(&self) -> crate::Milestone10CounterContract {
        super::super::retention::evidence::milestone_10_counter_contract(self)
    }

    pub fn milestone_10_complexity_surface(&self) -> crate::Milestone10ComplexitySurface {
        super::super::retention::evidence::milestone_10_complexity_surface(self)
    }

    pub fn milestone_11_counter_contract(&self) -> crate::Milestone11CounterContract {
        super::super::maintenance::evidence::milestone_11_counter_contract(self)
    }

    pub fn milestone_11_complexity_surface(&self) -> crate::Milestone11ComplexitySurface {
        super::super::maintenance::evidence::milestone_11_complexity_surface(self)
    }

    pub fn milestone_13_counter_contract(&self) -> crate::Milestone13CounterContract {
        super::super::tiering::milestone_13_counter_contract(self)
    }

    pub fn milestone_13_complexity_surface(&self) -> crate::Milestone13ComplexitySurface {
        super::super::tiering::milestone_13_complexity_surface(self)
    }

    pub fn milestone_11_maintenance_report(&self) -> crate::Milestone11MaintenanceReport {
        super::super::maintenance::evidence::milestone_11_maintenance_report(self)
    }

    pub fn maintenance_evidence(&self) -> crate::Milestone11MaintenanceReport {
        self.milestone_11_maintenance_report()
    }

    pub fn milestone_10_artifact_report(
        &self,
    ) -> Result<crate::Milestone10ArtifactReport, StoreError> {
        super::super::retention::evidence::milestone_10_artifact_report(self)
    }

    pub fn record_canonicalization(&self, metrics: CanonicalizationMetrics) {
        self.counters.record_canonicalization(metrics);
    }

    pub fn record_stable_basis_lookup(&self) { self.counters.record_stable_basis_lookup(); }
    pub fn record_stable_basis_read(
        &self,
        support_rows_read: u64,
        scope_lookup_count: u64,
        used_fallback: bool,
    ) {
        self.counters.record_stable_basis_read(
            support_rows_read,
            scope_lookup_count,
            used_fallback,
        );
    }
    pub fn record_stable_basis_broadening(&self) { self.counters.record_stable_basis_broadening(); }
    pub fn record_continuation_plan(&self) { self.counters.record_continuation_plan(); }
    pub fn record_continuation_identity_lookup(&self) { self.counters.record_continuation_identity_lookup(); }
    pub fn record_continuation_checkpoint_lookup(&self) { self.counters.record_continuation_checkpoint_lookup(); }
    pub fn record_continuation_broadening(&self) { self.counters.record_continuation_broadening(); }
    pub fn record_continuation_parity(&self) { self.counters.record_continuation_parity(); }
    pub fn record_continuation_illegal_acknowledgment(&self) { self.counters.record_continuation_illegal_acknowledgment(); }
    pub fn record_continuation_batch_gap(&self) { self.counters.record_continuation_batch_gap(); }
    pub fn record_continuation_batch_duplicate(&self) { self.counters.record_continuation_batch_duplicate(); }
    pub fn record_continuation_schema_mismatch(&self) { self.counters.record_continuation_schema_mismatch(); }
    pub fn record_continuation_scope_mismatch(&self) { self.counters.record_continuation_scope_mismatch(); }
    pub fn record_continuation_degraded_basis(&self) { self.counters.record_continuation_degraded_basis(); }
    pub fn record_continuation_rejected_basis(&self) { self.counters.record_continuation_rejected_basis(); }

    pub fn milestone_7_access_structure_verification(
        &self,
    ) -> crate::evidence::Milestone7AccessStructureVerification {
        self.milestone_7_access_structure_verification.clone()
    }

    pub fn milestone_6_access_structure_verification(
        &self,
    ) -> crate::evidence::Milestone6AccessStructureVerification {
        self.milestone_6_access_structure_verification.clone()
    }
}
