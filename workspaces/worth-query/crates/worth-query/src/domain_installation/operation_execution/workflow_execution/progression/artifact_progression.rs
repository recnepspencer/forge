use crate::basis_lifecycle::BasisOperationLane;
use crate::runtime::WorthQueryWorkspace;

use super::workflow_progression::WorthQueryWorkflowAdvanceOutcome;
use super::{
    WorthQueryArtifactDenial, WorthQueryArtifactTransferAdmission, WorthQueryWorkflowAdvanceDenial,
    WorthQueryWorkflowAdvanceDenialKind, WorthQueryWorkflowRun, WorthQueryWorkflowValue,
};

impl<D: 'static, O: 'static, F: 'static, L: BasisOperationLane> WorthQueryWorkflowRun<D, O, F, L> {
    pub fn advance_with_artifact(
        mut self,
        stage_identity: &str,
        predecessor_stage: &str,
        workspace: &mut WorthQueryWorkspace,
    ) -> WorthQueryWorkflowAdvanceOutcome<D, O, F, L> {
        let runtime_admission = match self.admit_stage_runtime_authority(workspace) {
            Ok(admission) => admission,
            Err(denial) => return self.outcome_from_denial(denial),
        };
        let admission = match self.artifact_transfer_admission(stage_identity, predecessor_stage) {
            Ok(admission) => admission,
            Err(denial) => return self.outcome_from_denial(denial),
        };
        let input = match self.take_predecessor_artifact(predecessor_stage) {
            Ok(handle) => handle
                .transfer(&admission)
                .map(WorthQueryWorkflowValue::TransferredArtifact),
            Err(denial) => return self.outcome_from_denial(denial),
        };
        match input {
            Ok(input) => self.advance_with_runtime_admission(
                stage_identity,
                input,
                workspace,
                runtime_admission,
            ),
            Err(denial) => {
                let stop = self.artifact_carriage_denial(denial);
                self.outcome_from_denial(stop)
            }
        }
    }

    pub fn advance_with_artifact_lease(
        mut self,
        stage_identity: &str,
        predecessor_stage: &str,
        lease_role: impl Into<String>,
        workspace: &mut WorthQueryWorkspace,
    ) -> WorthQueryWorkflowAdvanceOutcome<D, O, F, L> {
        let runtime_admission = match self.admit_stage_runtime_authority(workspace) {
            Ok(admission) => admission,
            Err(denial) => return self.outcome_from_denial(denial),
        };
        let admission = match self.artifact_transfer_admission(stage_identity, predecessor_stage) {
            Ok(admission) => admission,
            Err(denial) => return self.outcome_from_denial(denial),
        };
        let input = self
            .predecessor_artifact(predecessor_stage)
            .and_then(|handle| handle.lease_for_transfer(&admission, lease_role))
            .map(WorthQueryWorkflowValue::TransferredArtifact);
        if input.is_ok() {
            if let Some(index) = self.receipt_index.get(predecessor_stage).copied() {
                self.receipts[index].set_artifact_disposition(
                    crate::domain_installation::WorthQueryArtifactDisposition::Leased,
                );
            }
        }
        match input {
            Ok(input) => self.advance_with_runtime_admission(
                stage_identity,
                input,
                workspace,
                runtime_admission,
            ),
            Err(denial) => {
                let stop = self.artifact_carriage_denial(denial);
                self.outcome_from_denial(stop)
            }
        }
    }

    fn artifact_transfer_admission(
        &self,
        stage_identity: &str,
        predecessor_stage: &str,
    ) -> Result<WorthQueryArtifactTransferAdmission, WorthQueryWorkflowAdvanceDenial> {
        let stage = self.graph.stage(stage_identity).ok_or_else(|| {
            WorthQueryWorkflowAdvanceDenial::new(
                WorthQueryWorkflowAdvanceDenialKind::UnknownStage,
                self.counters,
            )
        })?;
        if !stage
            .predecessors()
            .iter()
            .any(|predecessor| predecessor == predecessor_stage)
        {
            return Err(WorthQueryWorkflowAdvanceDenial::new(
                WorthQueryWorkflowAdvanceDenialKind::PredecessorAuthorityMissing(
                    predecessor_stage.to_owned(),
                ),
                self.counters,
            ));
        }
        let expected_contract = self
            .graph
            .artifact_contracts(stage_identity)
            .and_then(super::WorthQueryInstalledWorkflowArtifactContracts::input)
            .cloned()
            .ok_or_else(|| {
                WorthQueryWorkflowAdvanceDenial::new(
                    WorthQueryWorkflowAdvanceDenialKind::InputContract,
                    self.counters,
                )
            })?;
        Ok(WorthQueryArtifactTransferAdmission::mint(
            super::WorthQueryArtifactTransferAdmissionParts {
                expected_contract,
                domain_authority: std::sync::Arc::clone(self.bound.operation().domain_authority()),
                operation_identity: self.bound.definition().canonical_identity().to_owned(),
                binding_identity: self.bound.binding_identity().to_owned(),
                run_identity: self.identity.clone(),
                predecessor_stage: predecessor_stage.to_owned(),
                consumer_stage: stage_identity.to_owned(),
                basis_identity: self.bound.basis().capability_digest().to_owned(),
            },
        ))
    }

    fn predecessor_artifact(
        &self,
        predecessor_stage: &str,
    ) -> Result<
        &crate::domain_installation::WorthQueryMoveOnlyArtifactHandle,
        WorthQueryArtifactDenial,
    > {
        self.receipt_index
            .get(predecessor_stage)
            .and_then(|index| self.receipts.get(*index))
            .and_then(|receipt| match receipt.output() {
                WorthQueryWorkflowValue::InstalledArtifact(handle) => Some(handle),
                _ => None,
            })
            .ok_or_else(|| {
                WorthQueryArtifactDenial::new(
                    crate::domain_installation::WorthQueryArtifactDenialKind::StageMismatch,
                    None,
                    "predecessor stage does not retain an owned artifact output",
                )
            })
    }

    fn take_predecessor_artifact(
        &mut self,
        predecessor_stage: &str,
    ) -> Result<
        crate::domain_installation::WorthQueryMoveOnlyArtifactHandle,
        WorthQueryWorkflowAdvanceDenial,
    > {
        let Some(index) = self.receipt_index.get(predecessor_stage).copied() else {
            return Err(WorthQueryWorkflowAdvanceDenial::new(
                WorthQueryWorkflowAdvanceDenialKind::PredecessorAuthorityMissing(
                    predecessor_stage.to_owned(),
                ),
                self.counters,
            ));
        };
        let output = self.receipts[index].take_output();
        output
            .into_move_only_artifact()
            .inspect(|_| {
                self.receipts[index].set_artifact_disposition(
                    crate::domain_installation::WorthQueryArtifactDisposition::Transferred,
                );
            })
            .map_err(|output| {
                self.receipts[index].restore_output(output);
                WorthQueryWorkflowAdvanceDenial::new(
                    WorthQueryWorkflowAdvanceDenialKind::InputContract,
                    self.counters,
                )
            })
    }

    fn artifact_carriage_denial(
        &self,
        denial: WorthQueryArtifactDenial,
    ) -> WorthQueryWorkflowAdvanceDenial {
        WorthQueryWorkflowAdvanceDenial::new(
            WorthQueryWorkflowAdvanceDenialKind::ArtifactCarriage(denial),
            self.counters,
        )
    }
}
