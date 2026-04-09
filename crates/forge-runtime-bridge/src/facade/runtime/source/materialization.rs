use super::*;

impl RuntimeBridge {
    pub fn materialize_source(
        &self,
        planned_packet_set: &PlannedSourceReadPacketSet,
    ) -> Result<MaterializedTruthViewPacketSet, BridgeDeliveryError> {
        let adapter = self.source_adapter.as_ref().ok_or_else(|| {
            let error = BridgeDeliveryError::new(
                BridgeDeliveryErrorKind::SourceContractMismatch,
                "Runtime cannot materialize a source packet set without a configured source adapter.",
            );
            self.record_source_failure(
                planned_packet_set.contract().declaration(),
                SourceFailureClass::SourceMaterializationRejected,
                error.kind(),
                error.to_string(),
            );
            error
        })?;

        adapter
            .materialize_packets(planned_packet_set)
            .and_then(|materialized| {
                self.validate_materialized_source_packet_set(planned_packet_set, &materialized)?;
                Ok(materialized)
            })
            .map_err(|delivery_error| {
                for planned in planned_packet_set.packets() {
                    self.record_historical_evaluation_failure(
                        planned.declaration(),
                        crate::historical::failures::historical_failure_class_for_delivery_error(
                            &delivery_error,
                        ),
                        delivery_error.to_string(),
                        crate::historical::failures::historical_failure_counters_for_delivery_error(
                            planned.declaration(),
                            &delivery_error,
                        ),
                    );
                }
                self.record_source_failure(
                    planned_packet_set.contract().declaration(),
                    validation::source_failure_class_for_materialization_error(
                        delivery_error.kind(),
                    ),
                    delivery_error.kind(),
                    delivery_error.to_string(),
                );
                delivery_error
            })
    }

    pub fn materialize_source_packet(
        &self,
        contract: &AdmittedSourceContract,
        read_packet: SnapshotReadPacket,
    ) -> Result<MaterializedTruthViewObservation, BridgeDeliveryError> {
        let planned_packet_set = self.plan_source_packet_set(contract, read_packet)?;
        let adapter = self.source_adapter.as_ref().ok_or_else(|| {
            let error = BridgeDeliveryError::new(
                BridgeDeliveryErrorKind::SourceContractMismatch,
                "Runtime cannot materialize a source packet without a configured source adapter.",
            );
            self.record_source_failure(
                contract.declaration(),
                SourceFailureClass::SourceMaterializationRejected,
                error.kind(),
                error.to_string(),
            );
            error
        })?;
        let planned = planned_packet_set.first().clone();
        adapter
            .materialize_packet(planned.clone())
            .and_then(|materialized| {
                self.validate_materialized_source_observation(&planned, materialized)
            })
            .map_err(|delivery_error| {
                self.record_historical_evaluation_failure(
                    planned.declaration(),
                    crate::historical::failures::historical_failure_class_for_delivery_error(
                        &delivery_error,
                    ),
                    delivery_error.to_string(),
                    crate::historical::failures::historical_failure_counters_for_delivery_error(
                        planned.declaration(),
                        &delivery_error,
                    ),
                );
                self.record_source_failure(
                    contract.declaration(),
                    validation::source_failure_class_for_materialization_error(
                        delivery_error.kind(),
                    ),
                    delivery_error.kind(),
                    delivery_error.to_string(),
                );
                delivery_error
            })
    }

    pub fn materialize_source_packet_batch(
        &self,
        contract: &AdmittedSourceContract,
        read_packets: Vec<SnapshotReadPacket>,
    ) -> Result<MaterializedTruthViewPacketSet, BridgeDeliveryError> {
        let planned_packet_set = self.plan_source_packet_batch(contract, read_packets)?;
        self.materialize_source(&planned_packet_set)
    }
}
