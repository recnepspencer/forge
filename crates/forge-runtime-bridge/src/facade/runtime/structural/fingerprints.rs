use super::*;

impl RuntimeBridge {
    pub fn materialize_structural_fingerprint(
        &self,
        contract: &AdmittedStructuralComparisonContract,
        read_packet: SnapshotReadPacket,
    ) -> Result<StructuralFingerprint, BridgeDeliveryError> {
        let declaration = contract.validated_declaration().declaration();
        let selector = match declaration.truth_view_basis() {
            StructuralTruthViewBasis::Single { selector, .. } => selector.clone(),
            StructuralTruthViewBasis::BranchPair { .. } => {
                return Err(BridgeDeliveryError::new(
                    BridgeDeliveryErrorKind::StructuralPlanRejected,
                    format!(
                        "Structural contract `{}` requires a branch-pair basis and cannot materialize a single structural fingerprint.",
                        contract.contract_identity().as_str()
                    ),
                ))
            }
        };

        let observation = self.materialize_truth_view_observation(self.plan_truth_view_packet(
            HistoricalEvaluationDeclaration::new(
                selector,
                BridgeReplayMode::Enabled,
                BridgeDiagnosticsTier::Standard,
                BridgeDeliveryIntent::PrepareSignalEvaluation,
            ),
            read_packet,
        )?)?;

        StructuralFingerprint::from_observation(contract, &observation).map_err(|error| {
            BridgeDeliveryError::new(
                BridgeDeliveryErrorKind::SnapshotReadContractViolation,
                format!("Structural fingerprint materialization could not validate reads: {error}"),
            )
        })
    }

    pub fn materialize_structural_branch_fingerprints(
        &self,
        contract: &AdmittedStructuralComparisonContract,
        read_packet: SnapshotReadPacket,
    ) -> Result<(StructuralFingerprint, StructuralFingerprint), BridgeDeliveryError> {
        let declaration = contract.validated_declaration().declaration();
        let (left_selector, right_selector) = match declaration.truth_view_basis() {
            StructuralTruthViewBasis::BranchPair {
                left_selector,
                right_selector,
                ..
            } => (left_selector.clone(), right_selector.clone()),
            StructuralTruthViewBasis::Single { .. } => {
                return Err(BridgeDeliveryError::new(
                    BridgeDeliveryErrorKind::StructuralPlanRejected,
                    format!(
                    "Structural contract `{}` does not admit branch-pair structural comparison.",
                    contract.contract_identity().as_str()
                ),
                ))
            }
        };

        let left = self.materialize_truth_view_observation(self.plan_truth_view_packet(
            HistoricalEvaluationDeclaration::new(
                left_selector,
                BridgeReplayMode::Enabled,
                BridgeDiagnosticsTier::Standard,
                BridgeDeliveryIntent::PrepareSignalEvaluation,
            ),
            read_packet.clone(),
        )?)?;
        let right = self.materialize_truth_view_observation(self.plan_truth_view_packet(
            HistoricalEvaluationDeclaration::new(
                right_selector,
                BridgeReplayMode::Enabled,
                BridgeDiagnosticsTier::Standard,
                BridgeDeliveryIntent::PrepareSignalEvaluation,
            ),
            read_packet,
        )?)?;

        let left = StructuralFingerprint::from_observation(contract, &left).map_err(|error| {
            BridgeDeliveryError::new(
                BridgeDeliveryErrorKind::SnapshotReadContractViolation,
                format!("Structural branch comparison could not validate left-side reads: {error}"),
            )
        })?;
        let right = StructuralFingerprint::from_observation(contract, &right).map_err(|error| {
            BridgeDeliveryError::new(
                BridgeDeliveryErrorKind::SnapshotReadContractViolation,
                format!(
                    "Structural branch comparison could not validate right-side reads: {error}"
                ),
            )
        })?;

        Ok((left, right))
    }
}
