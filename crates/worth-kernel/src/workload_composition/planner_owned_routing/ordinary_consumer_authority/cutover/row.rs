use super::{
    PendingWorthWorkloadOrdinaryConsumerCutoverRow, WorthWorkloadOrdinaryConsumerCutoverPosture,
    WorthWorkloadOrdinaryConsumerCutoverRow, WorthWorkloadOrdinaryConsumerSelectedPlanWitness,
};
use crate::workload_composition::BatchAdmissionExecutionReceipt;

impl WorthWorkloadOrdinaryConsumerCutoverRow {
    pub fn surface_name(&self) -> &str {
        &self.surface_name
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn blocker(&self) -> &str {
        &self.blocker
    }

    pub fn removal_trigger(&self) -> &str {
        &self.removal_trigger
    }

    pub const fn posture(&self) -> WorthWorkloadOrdinaryConsumerCutoverPosture {
        self.posture
    }

    pub(crate) fn selected_plan_witness(
        &self,
    ) -> Option<&WorthWorkloadOrdinaryConsumerSelectedPlanWitness> {
        self.selected_plan_witness.as_ref()
    }

    #[cfg(test)]
    pub(crate) fn test_posture_from_phase_eleven_inventory_row(
        row: crate::workload_composition::ConflictBatchAdmissionInventoryRow,
    ) -> WorthWorkloadOrdinaryConsumerCutoverPosture {
        PendingWorthWorkloadOrdinaryConsumerCutoverRow::from_phase_eleven_inventory_row(row)
            .expect("test row should lower")
            .posture
    }
}

impl PendingWorthWorkloadOrdinaryConsumerCutoverRow {
    pub(super) fn route_witness(
        &self,
    ) -> Option<super::WorthWorkloadOrdinaryConsumerCurrentRouteWitness> {
        self.route_witness.clone()
    }

    pub(super) fn bind_receipt(
        self,
        batch_execution_receipt: &BatchAdmissionExecutionReceipt,
    ) -> WorthWorkloadOrdinaryConsumerCutoverRow {
        WorthWorkloadOrdinaryConsumerCutoverRow {
            surface_name: self.surface_name,
            owner: self.owner,
            blocker: self.blocker,
            removal_trigger: self.removal_trigger,
            posture: self.posture,
            selected_plan_witness: self.route_witness.as_ref().map(|route_witness| {
                WorthWorkloadOrdinaryConsumerSelectedPlanWitness::new(
                    route_witness,
                    batch_execution_receipt.execution_receipt_digest(),
                )
            }),
        }
    }
}
