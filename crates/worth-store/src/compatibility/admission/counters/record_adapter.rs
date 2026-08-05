use super::*;

impl CompatibilityAdmissionCounters {
    pub(crate) fn record_malformed_frame(&mut self) {
        self.malformed_frame_count += 1;
        self.rejected_count += 1;
    }


    pub(crate) fn record_adapter_cost_class(&mut self, cost_class: CompatibilityAdapterCostClass) {
        self.adapter_cost_class_count += 1;
        match cost_class {
            CompatibilityAdapterCostClass::ZeroCopy
            | CompatibilityAdapterCostClass::BoundedRecordLocal => {
                self.adapter_inline_count += 1;
            }
            CompatibilityAdapterCostClass::BoundedBatchLocal => {
                self.adapter_batch_count += 1;
            }
            CompatibilityAdapterCostClass::MaintenanceOnly => {
                self.adapter_maintenance_scheduled_count += 1;
            }
            CompatibilityAdapterCostClass::OutOfScope => {}
        }
    }

    pub(crate) fn record_adapter_execution(
        &mut self,
        input_record_count: u64,
        output_record_count: u64,
        allocation_scope_count: u64,
    ) {
        self.adapter_input_record_count += input_record_count;
        self.adapter_output_record_count += output_record_count;
        self.adapter_allocation_scope_count += allocation_scope_count;
    }

    pub(crate) fn record_adapter_parity_failure(&mut self) {
        self.adapter_parity_failure_count += 1;
        self.rejected_count += 1;
    }
}
