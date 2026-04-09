use super::*;
use crate::snapshot::{MaterializedTruthViewObservation, PlannedTruthViewPacket};
use crate::source::{
    MaterializedTruthViewPacketSet, PlannedSourceReadPacketSet, SourceFailureClass,
    SourceFailureRecord, SourceMaterializationRecord, ValidatedSourceDeclaration,
};

impl RuntimeBridge {
    pub fn validate_source_declaration(
        &self,
        declaration: SourceDeclaration,
    ) -> Result<ValidatedSourceDeclaration, BridgeDeliveryError> {
        let contract = self.admit_source(declaration)?;
        Ok(ValidatedSourceDeclaration::from_contract(&contract))
    }

    pub fn admit_source(
        &self,
        declaration: SourceDeclaration,
    ) -> Result<AdmittedSourceContract, BridgeDeliveryError> {
        self.source_registry
            .contract_for_declaration(&declaration)
            .cloned()
            .ok_or_else(|| {
                let error = BridgeDeliveryError::new(
                    BridgeDeliveryErrorKind::SourceContractMismatch,
                    format!(
                        "Source declaration `{}` was not admitted by the runtime source registry.",
                        declaration.declaration_identity().as_str()
                    ),
                );
                self.record_source_failure(
                    &declaration,
                    SourceFailureClass::SourceContractMismatch,
                    error.kind(),
                    error.to_string(),
                );
                error
            })
    }

    pub fn plan_source_packet(
        &self,
        contract: &AdmittedSourceContract,
        read_packet: SnapshotReadPacket,
    ) -> Result<PlannedTruthViewPacket, BridgeDeliveryError> {
        self.plan_source_packet_set(contract, read_packet)
            .map(|planned| planned.first().clone())
    }

    pub fn plan_source_packet_set(
        &self,
        contract: &AdmittedSourceContract,
        read_packet: SnapshotReadPacket,
    ) -> Result<PlannedSourceReadPacketSet, BridgeDeliveryError> {
        self.plan_source_packet_set_from_packets(contract, vec![read_packet])
    }

    pub fn plan_source_packet_batch(
        &self,
        contract: &AdmittedSourceContract,
        read_packets: Vec<SnapshotReadPacket>,
    ) -> Result<PlannedSourceReadPacketSet, BridgeDeliveryError> {
        self.plan_source_packet_set_from_packets(contract, read_packets)
    }

    fn plan_source_packet_set_from_packets(
        &self,
        contract: &AdmittedSourceContract,
        read_packets: Vec<SnapshotReadPacket>,
    ) -> Result<PlannedSourceReadPacketSet, BridgeDeliveryError> {
        let validated_declaration = ValidatedSourceDeclaration::from_contract(contract);
        let declaration = HistoricalEvaluationDeclaration::new(
            validated_declaration.declaration().selector().clone(),
            if contract
                .required_capabilities()
                .contains(BridgeSourceCapability::ReplayCompatibleRead)
            {
                BridgeReplayMode::Required
            } else {
                BridgeReplayMode::Disabled
            },
            BridgeDiagnosticsTier::Standard,
            BridgeDeliveryIntent::PrepareSignalEvaluation,
        );
        let packets = read_packets
            .into_iter()
            .map(|read_packet| self.plan_truth_view_packet(declaration.clone(), read_packet))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(PlannedSourceReadPacketSet::new(
            contract.clone(),
            validated_declaration,
            packets,
        ))
    }

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
                    source_failure_class_for_materialization_error(delivery_error.kind()),
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
                    source_failure_class_for_materialization_error(delivery_error.kind()),
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

    pub fn replay_source_materialization_record(
        &self,
        record: &SourceMaterializationRecord,
    ) -> Result<SourceMaterializationRecord, BridgeReplayError> {
        let contract = self
            .source_registry
            .contract_for_identity(record.source_contract_identity())
            .ok_or_else(|| {
                BridgeReplayError::new(
                    BridgeReplayErrorKind::PlanningContractMismatch,
                    format!(
                        "Bridge source replay could not find source contract `{}` in the runtime source registry.",
                        record.source_contract_identity()
                    ),
                )
                .with_context(BridgeErrorContext::default())
            })?;

        let adapter_capability_digest = self
            .source_adapter
            .as_ref()
            .map(|adapter| adapter.declared_capabilities().digest().to_owned())
            .ok_or_else(|| {
                BridgeReplayError::new(
                    BridgeReplayErrorKind::PlanningContractMismatch,
                    "Bridge source replay requires a configured source adapter.",
                )
                .with_context(BridgeErrorContext::default())
            })?;

        if adapter_capability_digest != record.adapter_capability_digest() {
            return Err(BridgeReplayError::new(
                BridgeReplayErrorKind::PlanningContractMismatch,
                format!(
                    "Bridge source replay reconstructed adapter capabilities `{}` but original source record expected `{}`.",
                    adapter_capability_digest,
                    record.adapter_capability_digest()
                ),
            )
            .with_context(BridgeErrorContext::default()));
        }

        let observation = self
            .plan_source_packet_set_from_packets(contract, record.read_packets().to_vec())
            .and_then(|planned_packet_set| self.materialize_source(&planned_packet_set))
            .map_err(|error| {
                BridgeReplayError::new(
                    BridgeReplayErrorKind::HistoricalEvaluationAuthorityMismatch,
                    format!(
                        "Bridge source replay could not materialize the planned source packet set: {error}"
                    ),
                )
                .with_context(BridgeErrorContext::default())
            })?;

        let replayed = SourceMaterializationRecord::from_packet_set(
            contract,
            &observation,
            Arc::<str>::from(adapter_capability_digest),
        );

        if replayed != *record {
            return Err(BridgeReplayError::new(
                BridgeReplayErrorKind::HistoricalEvaluationAuthorityMismatch,
                format!(
                    "Bridge source replay reconstructed record `{}` but original source record was `{}`.",
                    replayed.record_identity().as_str(),
                    record.record_identity().as_str()
                ),
            )
            .with_context(BridgeErrorContext::default()));
        }

        Ok(replayed)
    }
}

impl RuntimeBridge {
    fn record_source_failure(
        &self,
        declaration: &SourceDeclaration,
        failure_class: SourceFailureClass,
        delivery_error_kind: BridgeDeliveryErrorKind,
        detail: impl Into<Arc<str>>,
    ) {
        self.diagnostics
            .record_source_failure(SourceFailureRecord::new(
                declaration.declaration_identity().clone(),
                declaration.selector(),
                declaration.required_capabilities(),
                failure_class,
                delivery_error_kind,
                detail,
            ));
    }
}

impl RuntimeBridge {
    fn validate_materialized_source_observation(
        &self,
        planned: &PlannedTruthViewPacket,
        materialized: MaterializedTruthViewObservation,
    ) -> Result<MaterializedTruthViewObservation, BridgeDeliveryError> {
        if materialized.planned().digest() != planned.digest() {
            return Err(BridgeDeliveryError::new(
                BridgeDeliveryErrorKind::SourceContractMismatch,
                format!(
                    "Source adapter materialized planned packet `{}` but bridge required `{}`.",
                    materialized.planned().digest(),
                    planned.digest()
                ),
            ));
        }

        Ok(materialized)
    }

    fn validate_materialized_source_packet_set(
        &self,
        planned_packet_set: &PlannedSourceReadPacketSet,
        materialized_packet_set: &MaterializedTruthViewPacketSet,
    ) -> Result<(), BridgeDeliveryError> {
        for (index, (planned, materialized)) in planned_packet_set
            .packets()
            .iter()
            .zip(materialized_packet_set.observations().iter())
            .enumerate()
        {
            if materialized.planned().digest() != planned.digest() {
                return Err(BridgeDeliveryError::new(
                    BridgeDeliveryErrorKind::SourceContractMismatch,
                    format!(
                        "Source adapter changed packet identity at position {index}: materialized `{}` but bridge planned `{}`.",
                        materialized.planned().digest(),
                        planned.digest()
                    ),
                ));
            }

            if materialized.planned().declaration().selector()
                != planned_packet_set.contract().declaration().selector()
            {
                return Err(BridgeDeliveryError::new(
                    BridgeDeliveryErrorKind::SourceContractMismatch,
                    format!(
                        "Source adapter materialized selector `{}` at position {index} but contract `{}` admits only `{}`.",
                        materialized
                            .planned()
                            .declaration()
                            .selector()
                            .selector_identity()
                            .as_str(),
                        planned_packet_set.contract().contract_identity().as_str(),
                        planned_packet_set
                            .contract()
                            .declaration()
                            .selector()
                            .selector_identity()
                            .as_str()
                    ),
                ));
            }
        }

        Ok(())
    }
}

fn source_failure_class_for_materialization_error(
    error_kind: BridgeDeliveryErrorKind,
) -> SourceFailureClass {
    match error_kind {
        BridgeDeliveryErrorKind::SourceContractMismatch => {
            SourceFailureClass::AdapterCapabilityDrift
        }
        BridgeDeliveryErrorKind::SnapshotIdentityMismatch => {
            SourceFailureClass::AdapterCapabilityDrift
        }
        BridgeDeliveryErrorKind::HistoricalPolicyRejected => {
            SourceFailureClass::TruthViewSelectionMismatch
        }
        _ => SourceFailureClass::SourceMaterializationRejected,
    }
}
