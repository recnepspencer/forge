use topology::facade::{
    PlanarBooleanLoopBlueprintRegistryIdentity, PlanarBooleanLoopOperatorClassification as Class,
    PlanarBooleanLoopOperatorClassificationMatrix, PlanarBooleanLoopValidatorRegistrationPlan,
    PlanarBooleanLoopValidatorRuntimeLane as Lane,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};
use worth_spatial::facade::planar_boolean_loop_reconstruction::{
    PlanarBooleanLoopReconstructionEvidenceReceipt, PlanarBooleanLoopReconstructionLedgerReceipt,
};
use worth_spatial::facade::workload_vocabulary::WorkloadEvidenceBooleanReceiptLookupProduct;

use super::{CompletedBooleanLoopReconstructionProducts, WorkloadCompositionError, WorthWorkload};

const REQUIRED_PHASE_15_OPERATOR_NAMES: &[&str] = &[
    "RequireBooleanLoopReconstructionEvidence",
    "RegisterBooleanLoopReconstructionStageRequirement",
    "ReplayPlanarBooleanLoopReconstruction",
    "CompareLoopReconstructionReplayParity",
    "CompareLoopReconstructionCheckpointParity",
];

const REQUIRED_PHASE_15_VALIDATOR_NAMES: &[&str] = &[
    "ValidatePlanarBooleanLoopReplayParity",
    "ValidatePlanarBooleanLoopCheckpointParity",
    "ValidateLoopValidatorRuntimeRegistration",
    "ValidateLoopGraphInvariantPackRegistration",
];

#[derive(Clone, Debug, PartialEq)]
pub struct CompletedBooleanLoopReconstructionHandoff {
    completed_workload: WorthWorkload,
    products: Option<CompletedBooleanLoopReconstructionProducts>,
    loop_ledger_receipt: PlanarBooleanLoopReconstructionLedgerReceipt,
    evidence_receipt: PlanarBooleanLoopReconstructionEvidenceReceipt,
    runtime_registration_proof: PlanarBooleanLoopRuntimeRegistrationProof,
}

impl CompletedBooleanLoopReconstructionHandoff {
    pub(crate) fn new(
        completed_workload: WorthWorkload,
        products: Option<CompletedBooleanLoopReconstructionProducts>,
        loop_ledger_receipt: PlanarBooleanLoopReconstructionLedgerReceipt,
        evidence_receipt: PlanarBooleanLoopReconstructionEvidenceReceipt,
        runtime_registration_proof: PlanarBooleanLoopRuntimeRegistrationProof,
    ) -> Self {
        Self {
            completed_workload,
            products,
            loop_ledger_receipt,
            evidence_receipt,
            runtime_registration_proof,
        }
    }

    pub fn completed_workload(&self) -> &WorthWorkload {
        &self.completed_workload
    }

    pub fn products(&self) -> Option<&CompletedBooleanLoopReconstructionProducts> {
        self.products.as_ref()
    }

    pub fn loop_ledger_receipt(&self) -> &PlanarBooleanLoopReconstructionLedgerReceipt {
        &self.loop_ledger_receipt
    }

    pub fn evidence_receipt(&self) -> &PlanarBooleanLoopReconstructionEvidenceReceipt {
        &self.evidence_receipt
    }

    pub fn runtime_registration_proof(&self) -> &PlanarBooleanLoopRuntimeRegistrationProof {
        &self.runtime_registration_proof
    }

    pub fn workload_stage_index_identity(&self) -> &str {
        self.completed_workload
            .evidence_ledger()
            .stage_index()
            .index_identity()
    }

    pub fn require_boolean_loop_reconstruction(&self) -> Result<(), WorkloadCompositionError> {
        self.require_boolean_loop_reconstruction_lookup()
            .map(|_| ())
    }

    pub fn require_boolean_loop_reconstruction_lookup(
        &self,
    ) -> Result<WorkloadEvidenceBooleanReceiptLookupProduct, WorkloadCompositionError> {
        self.completed_workload
            .require_boolean_loop_reconstruction_lookup(&self.loop_ledger_receipt)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopRuntimeRegistrationProof {
    proof_identity: String,
    loop_receipt_identity: String,
    loop_ledger_identity: String,
    downstream_consumption_identity: String,
    stage_index_identity: String,
    registry_identity: PlanarBooleanLoopBlueprintRegistryIdentity,
    operator_names: Vec<String>,
    validator_names: Vec<String>,
}

impl PlanarBooleanLoopRuntimeRegistrationProof {
    pub(crate) fn certify(
        receipt: &PlanarBooleanLoopReconstructionLedgerReceipt,
        completed_workload: &WorthWorkload,
        matrix: &PlanarBooleanLoopOperatorClassificationMatrix,
        plan: &PlanarBooleanLoopValidatorRegistrationPlan,
    ) -> Result<Self, WorkloadCompositionError> {
        require_matching_registry_identity(matrix.registry_identity(), plan.registry_identity())?;
        let operator_names = certify_required_phase_15_operators(matrix)?;
        let validator_names = certify_required_phase_15_validators(plan)?;
        let proof_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "planar-boolean-loop-runtime-registration-proof".to_string(),
                format!("receipt:{}", receipt.receipt_identity()),
                format!(
                    "stage-index:{}",
                    completed_workload
                        .evidence_ledger()
                        .stage_index()
                        .index_identity()
                ),
                format!("registry:{}", matrix.registry_identity().digest()),
                format!("operators:{}", operator_names.join("|")),
                format!("validators:{}", validator_names.join("|")),
            ],
        );
        Ok(Self {
            proof_identity,
            loop_receipt_identity: receipt.receipt_identity().to_string(),
            loop_ledger_identity: receipt.ledger_identity().to_string(),
            downstream_consumption_identity: receipt.downstream_consumption_identity().to_string(),
            stage_index_identity: completed_workload
                .evidence_ledger()
                .stage_index()
                .index_identity()
                .to_string(),
            registry_identity: matrix.registry_identity().clone(),
            operator_names,
            validator_names,
        })
    }

    pub fn proof_identity(&self) -> &str {
        &self.proof_identity
    }

    pub fn loop_receipt_identity(&self) -> &str {
        &self.loop_receipt_identity
    }

    pub fn loop_ledger_identity(&self) -> &str {
        &self.loop_ledger_identity
    }

    pub fn downstream_consumption_identity(&self) -> &str {
        &self.downstream_consumption_identity
    }

    pub fn stage_index_identity(&self) -> &str {
        &self.stage_index_identity
    }

    pub fn registry_identity(&self) -> &PlanarBooleanLoopBlueprintRegistryIdentity {
        &self.registry_identity
    }

    pub fn operator_names(&self) -> &[String] {
        &self.operator_names
    }

    pub fn validator_names(&self) -> &[String] {
        &self.validator_names
    }
}

fn require_matching_registry_identity(
    operator_registry: &PlanarBooleanLoopBlueprintRegistryIdentity,
    validator_registry: &PlanarBooleanLoopBlueprintRegistryIdentity,
) -> Result<(), WorkloadCompositionError> {
    if operator_registry == validator_registry {
        Ok(())
    } else {
        Err(WorkloadCompositionError::LoopRuntimeRegistration(
            "loop runtime registration requires one matching loop blueprint registry identity"
                .to_string(),
        ))
    }
}

fn certify_required_phase_15_operators(
    matrix: &PlanarBooleanLoopOperatorClassificationMatrix,
) -> Result<Vec<String>, WorkloadCompositionError> {
    let mut operator_names = Vec::new();
    for operator_name in REQUIRED_PHASE_15_OPERATOR_NAMES {
        let operator = matrix.operator(operator_name).ok_or_else(|| {
            WorkloadCompositionError::LoopRuntimeRegistration(format!(
                "loop runtime registration is missing required phase 15 operator `{operator_name}`"
            ))
        })?;
        if operator.classification() != Class::QueryGraphCompositionProgram {
            return Err(WorkloadCompositionError::LoopRuntimeRegistration(format!(
                "loop runtime registration requires `{operator_name}` to stay classified as a Query graph-composition program"
            )));
        }
        operator_names.push(operator.operator_name().to_string());
    }
    Ok(operator_names)
}

fn certify_required_phase_15_validators(
    plan: &PlanarBooleanLoopValidatorRegistrationPlan,
) -> Result<Vec<String>, WorkloadCompositionError> {
    let mut validator_names = Vec::new();
    for validator_name in REQUIRED_PHASE_15_VALIDATOR_NAMES {
        let validator = plan.validator(validator_name).ok_or_else(|| {
            WorkloadCompositionError::LoopRuntimeRegistration(format!(
                "loop runtime registration is missing required phase 15 validator `{validator_name}`"
            ))
        })?;
        if validator.runtime_lane() != Lane::QueryGraphInvariantPack {
            return Err(WorkloadCompositionError::LoopRuntimeRegistration(format!(
                "loop runtime registration requires `{validator_name}` to stay on the Query graph-invariant runtime lane"
            )));
        }
        if !validator.governs_topology_legality() {
            return Err(WorkloadCompositionError::LoopRuntimeRegistration(format!(
                "loop runtime registration requires `{validator_name}` to remain a topology-legality validator"
            )));
        }
        validator_names.push(validator.validator_name().to_string());
    }
    Ok(validator_names)
}
