use super::{record_layout_observation, LayoutOwnerObservationLedger};
use crate::courtroom::layout::executed_evidence::LayoutExecutedEvidenceKind as Evidence;

impl LayoutOwnerObservationLedger {
    record_layout_observation!(
        record_layout_binding_admission,
        LayoutBindingAdmission,
        worth_store_layout_indexes::evolution::migration::LayoutBindingAdmissionCaseId,
        as_str
    );
    record_layout_observation!(
        record_migration_planning,
        MigrationPlanning,
        worth_store_layout_indexes::evolution::migration::MigrationPlanningCaseId,
        as_str
    );
    record_layout_observation!(
        record_migration_execution,
        MigrationExecution,
        worth_store_layout_indexes::evolution::migration::LayoutMigrationExecutionCaseId,
        as_str
    );
    record_layout_observation!(
        record_migration_interruption,
        MigrationInterruption,
        worth_store_layout_indexes::evolution::migration::LayoutMigrationInterruptionCaseId,
        as_str
    );
    pub fn record_rollback_planning(
        &mut self,
        observed: worth_store_layout_indexes::OwnerCaseObservation<
            worth_store_layout_indexes::evolution::migration::RollbackPlanningCaseId,
        >,
    ) {
        let case = observed.case_id();
        if case.as_str() == "layout.rollback.planning.lowering_rebind_required" {
            self.record_executed_evidence(Evidence::RollbackRebindRequired);
        }
        self.record(super::LayoutOwnerFamily::RollbackPlanning, case.as_str());
    }

    pub fn record_rollback_execution(
        &mut self,
        observed: worth_store_layout_indexes::OwnerCaseObservation<
            worth_store_layout_indexes::evolution::migration::LayoutRollbackExecutionCaseId,
        >,
    ) {
        use worth_store_layout_indexes::evolution::migration::{
            LayoutEvolutionDenialKind as Denial, LayoutRollbackExecutionCaseId as Case,
        };
        let case = observed.case_id();
        if case == Case::Denied(Denial::PhysicalPublicationSourceMismatch) {
            self.record_executed_evidence(Evidence::RollbackPublicationSourceDenied);
        }
        self.record(super::LayoutOwnerFamily::RollbackExecution, case.as_str());
    }
    record_layout_observation!(
        record_rollback_interruption,
        RollbackInterruption,
        worth_store_layout_indexes::evolution::migration::LayoutRollbackInterruptionCaseId,
        as_str
    );
    pub fn record_backward_read_compatibility(
        &mut self,
        observed: worth_store_layout_indexes::OwnerCaseObservation<
            worth_store_layout_indexes::evolution::migration::LayoutBackwardReadCompatibilityCaseId,
        >,
    ) {
        let case = observed.case_id();
        if case.as_str() == "layout.compatibility.backward_read.denied.window_mismatch" {
            self.record_executed_evidence(Evidence::CompatibilityWindowMismatch);
        }
        self.record(
            super::LayoutOwnerFamily::BackwardReadCompatibility,
            case.as_str(),
        );
    }
}
