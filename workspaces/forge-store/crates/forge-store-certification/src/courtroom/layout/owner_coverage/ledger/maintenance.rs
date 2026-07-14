use super::{record_layout_observation, LayoutOwnerObservationLedger};
use crate::courtroom::layout::executed_evidence::LayoutExecutedEvidenceKind as Evidence;

impl LayoutOwnerObservationLedger {
    record_layout_observation!(
        record_exact_btree_publication,
        ExactBTreePublication,
        forge_store_layout_indexes::ExactBTreePublicationCaseId,
        as_str
    );
    pub fn record_live_maintenance_posture(
        &mut self,
        observed: forge_store_layout_indexes::OwnerCaseObservation<
            forge_store_layout_indexes::LiveMaintenancePostureCaseId,
        >,
    ) {
        let case = observed.case_id();
        if case.as_str() == "layout.maintenance.posture.deferred" {
            self.record_executed_evidence(Evidence::MaintenanceDeferred);
        }
        self.record(
            super::LayoutOwnerFamily::LiveMaintenancePosture,
            case.as_str(),
        );
    }
    record_layout_observation!(
        record_layout_mutation_admission,
        LayoutMutationAdmission,
        forge_store_layout_indexes::LayoutMutationAdmissionCaseId,
        as_str
    );
    record_layout_observation!(
        record_copy_on_write_mutation_execution,
        CopyOnWriteMutationExecution,
        forge_store_layout_indexes::CopyOnWriteLayoutMutationExecutionCaseId,
        as_str
    );
    record_layout_observation!(
        record_live_exact_maintenance,
        LiveExactMaintenance,
        forge_store_layout_indexes::LiveExactMaintenanceCaseId,
        as_str
    );
    record_layout_observation!(
        record_derived_index_parity,
        DerivedIndexParity,
        forge_store_layout_indexes::DerivedIndexParityCaseId,
        as_str
    );
    record_layout_observation!(
        record_derived_index_rebuild_admission,
        DerivedIndexRebuildAdmission,
        forge_store_layout_indexes::DerivedIndexRebuildAdmissionCaseId,
        as_str
    );
    record_layout_observation!(
        record_derived_index_rebuild_execution,
        DerivedIndexRebuildExecution,
        forge_store_layout_indexes::DerivedIndexRebuildExecutionCaseId,
        as_str
    );

    pub fn record_lsm_maintenance(
        &mut self,
        observed: forge_store_layout_indexes::LsmMaintenanceOwnerCaseObservation,
    ) {
        use forge_store_layout_indexes::LsmMaintenanceOperation;
        let id = observed.id();
        let family = match id.operation() {
            LsmMaintenanceOperation::AdmitRunPublication => {
                super::LayoutOwnerFamily::LsmRunPublicationAdmission
            }
            LsmMaintenanceOperation::AdmitReplay => super::LayoutOwnerFamily::LsmReplayAdmission,
            LsmMaintenanceOperation::AdmitCompaction => {
                super::LayoutOwnerFamily::LsmCompactionAdmission
            }
        };
        self.record(family, id.disposition().as_str());
    }
}
