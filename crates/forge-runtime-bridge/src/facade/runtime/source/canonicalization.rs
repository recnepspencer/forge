use super::*;

impl RuntimeBridge {
    pub fn canonicalize_source_materialization_record(
        &self,
        contract: &AdmittedSourceContract,
        observation: &MaterializedTruthViewObservation,
    ) -> Result<SourceMaterializationRecord, BridgeDeliveryError> {
        let adapter_capability_digest = self
            .source_adapter
            .as_ref()
            .map(|adapter| Arc::<str>::from(adapter.declared_capabilities().digest()))
            .ok_or_else(|| {
                BridgeDeliveryError::new(
                    BridgeDeliveryErrorKind::SourceContractMismatch,
                    "Runtime cannot canonicalize a source materialization record without a configured source adapter.",
                )
            })?;

        if observation.planned().declaration().selector() != contract.declaration().selector() {
            let error = BridgeDeliveryError::new(
                BridgeDeliveryErrorKind::SourceContractMismatch,
                format!(
                    "Source contract `{}` does not match the materialized selector `{}`.",
                    contract.contract_identity().as_str(),
                    observation
                        .planned()
                        .declaration()
                        .selector()
                        .selector_identity()
                        .as_str()
                ),
            );
            self.record_source_failure(
                contract.declaration(),
                SourceFailureClass::TruthViewSelectionMismatch,
                error.kind(),
                error.to_string(),
            );
            return Err(error);
        }

        let record =
            SourceMaterializationRecord::new(contract, observation, adapter_capability_digest);
        self.diagnostics
            .record_source_materialization(record.clone());
        Ok(record)
    }

    pub fn canonicalize_source_materialization_packet_set_record(
        &self,
        materialized_packet_set: &MaterializedTruthViewPacketSet,
    ) -> Result<SourceMaterializationRecord, BridgeDeliveryError> {
        let contract = materialized_packet_set.planned_packet_set().contract();
        let adapter_capability_digest = self
            .source_adapter
            .as_ref()
            .map(|adapter| Arc::<str>::from(adapter.declared_capabilities().digest()))
            .ok_or_else(|| {
                BridgeDeliveryError::new(
                    BridgeDeliveryErrorKind::SourceContractMismatch,
                    "Runtime cannot canonicalize a source materialization record without a configured source adapter.",
                )
            })?;

        self.validate_materialized_source_packet_set(
            materialized_packet_set.planned_packet_set(),
            materialized_packet_set,
        )?;

        let record = SourceMaterializationRecord::from_packet_set(
            contract,
            materialized_packet_set,
            adapter_capability_digest,
        );
        self.diagnostics
            .record_source_materialization(record.clone());
        Ok(record)
    }
}
