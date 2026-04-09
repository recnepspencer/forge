use super::*;

impl RuntimeBridge {
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

    pub(super) fn plan_source_packet_set_from_packets(
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
}
