use super::{record_layout_observation, LayoutOwnerObservationLedger};
use crate::courtroom::layout::executed_evidence::LayoutExecutedEvidenceKind as Evidence;

impl LayoutOwnerObservationLedger {
    record_layout_observation!(
        record_layout_binding_admission,
        LayoutBindingAdmission,
        forge_store_layout_indexes::evolution::migration::LayoutBindingAdmissionCaseId,
        as_str
    );
    record_layout_observation!(
        record_migration_planning,
        MigrationPlanning,
        forge_store_layout_indexes::evolution::migration::MigrationPlanningCaseId,
        as_str
    );
    record_layout_observation!(
        record_migration_execution,
        MigrationExecution,
        forge_store_layout_indexes::evolution::migration::LayoutMigrationExecutionCaseId,
        as_str
    );
    record_layout_observation!(
        record_migration_interruption,
        MigrationInterruption,
        forge_store_layout_indexes::evolution::migration::LayoutMigrationInterruptionCaseId,
        as_str
    );
    pub fn record_rollback_planning(
        &mut self,
        observed: forge_store_layout_indexes::OwnerCaseObservation<
            forge_store_layout_indexes::evolution::migration::RollbackPlanningCaseId,
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
        observed: forge_store_layout_indexes::OwnerCaseObservation<
            forge_store_layout_indexes::evolution::migration::LayoutRollbackExecutionCaseId,
        >,
    ) {
        use forge_store_layout_indexes::evolution::migration::{
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
        forge_store_layout_indexes::evolution::migration::LayoutRollbackInterruptionCaseId,
        as_str
    );
    pub fn record_backward_read_compatibility(
        &mut self,
        observed: forge_store_layout_indexes::OwnerCaseObservation<
            forge_store_layout_indexes::evolution::migration::LayoutBackwardReadCompatibilityCaseId,
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
