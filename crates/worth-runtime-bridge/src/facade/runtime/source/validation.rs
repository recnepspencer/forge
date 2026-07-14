use super::*;

impl RuntimeBridge {
    pub(super) fn validate_materialized_source_observation(
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

    pub(super) fn validate_materialized_source_packet_set(
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
                        planned.digest(),
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

pub(super) fn source_failure_class_for_materialization_error(
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
