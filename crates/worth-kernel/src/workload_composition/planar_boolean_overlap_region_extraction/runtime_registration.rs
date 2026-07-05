use topology::facade::{
    PlanarBooleanOverlapOperatorClassification as Class,
    PlanarBooleanOverlapOperatorClassificationMatrix,
    PlanarBooleanOverlapValidatorRegistrationPlan, PlanarBooleanOverlapValidatorRuntimeLane as Lane,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};
use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapRegionEvidenceReceipt, PlanarBooleanOverlapRegionLedgerReceipt,
};

use crate::workload_composition::{WorkloadCompositionError, WorthWorkload};

const REQUIRED_PHASE_15_OPERATOR_NAMES: &[&str] = &[
    "RequireBooleanOverlapRegionEvidence",
    "RegisterBooleanOverlapRegionStageRequirement",
    "ReplayPlanarBooleanOverlapRegionExtraction",
    "CompareOverlapRegionReplayParity",
    "CompareOverlapRegionCheckpointParity",
];

const REQUIRED_PHASE_15_VALIDATOR_NAMES: &[&str] = &[
    "ValidatePlanarBooleanOverlapRegionReplayParity",
    "ValidatePlanarBooleanOverlapRegionCheckpointParity",
    "ValidateOverlapRegionValidatorRuntimeRegistration",
    "ValidateOverlapRegionGraphInvariantPackRegistration",
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRuntimeRegistrationProof {
    proof_identity: String,
    evidence_receipt_identity: String,
    overlap_ledger_receipt_identity: String,
    request_identity: String,
    stage_index_identity: String,
    operator_names: Vec<String>,
    validator_names: Vec<String>,
}

impl PlanarBooleanOverlapRuntimeRegistrationProof {
    pub(crate) fn certify(
        ledger_receipt: &PlanarBooleanOverlapRegionLedgerReceipt,
        evidence_receipt: &PlanarBooleanOverlapRegionEvidenceReceipt,
        completed_workload: &WorthWorkload,
        matrix: &PlanarBooleanOverlapOperatorClassificationMatrix,
        plan: &PlanarBooleanOverlapValidatorRegistrationPlan,
    ) -> Result<Self, WorkloadCompositionError> {
        if matrix.registry_identity() != plan.registry_identity() {
            return Err(WorkloadCompositionError::OverlapRuntimeRegistration(
                "overlap runtime registration requires one matching overlap blueprint registry identity".to_string(),
            ));
        }
        let operator_names = certify_required_phase_15_operators(matrix)?;
        let validator_names = certify_required_phase_15_validators(plan)?;
        Ok(Self {
            proof_identity: truth_digest_parts(
                TruthDigestScope::ArtifactIdentity,
                &[
                    "planar-boolean-overlap-runtime-registration-proof".to_string(),
                    format!("evidence:{}", evidence_receipt.receipt_identity()),
                    format!("ledger:{}", ledger_receipt.receipt_identity()),
                    format!("request:{}", evidence_receipt.request_identity()),
                    format!(
                        "stage-index:{}",
                        completed_workload.evidence_ledger().stage_index().index_identity()
                    ),
                    format!("operators:{}", operator_names.join("|")),
                    format!("validators:{}", validator_names.join("|")),
                ],
            ),
            evidence_receipt_identity: evidence_receipt.receipt_identity().to_string(),
            overlap_ledger_receipt_identity: ledger_receipt.receipt_identity().to_string(),
            request_identity: evidence_receipt.request_identity().to_string(),
            stage_index_identity: completed_workload
                .evidence_ledger()
                .stage_index()
                .index_identity()
                .to_string(),
            operator_names,
            validator_names,
        })
    }

    pub fn proof_identity(&self) -> &str { &self.proof_identity }
    pub fn evidence_receipt_identity(&self) -> &str { &self.evidence_receipt_identity }
    pub fn overlap_ledger_receipt_identity(&self) -> &str { &self.overlap_ledger_receipt_identity }
    pub fn request_identity(&self) -> &str { &self.request_identity }
    pub fn stage_index_identity(&self) -> &str { &self.stage_index_identity }
}

fn certify_required_phase_15_operators(
    matrix: &PlanarBooleanOverlapOperatorClassificationMatrix,
) -> Result<Vec<String>, WorkloadCompositionError> {
    let mut operator_names = Vec::new();
    for operator_name in REQUIRED_PHASE_15_OPERATOR_NAMES {
        let operator = matrix.operator(operator_name).ok_or_else(|| {
            WorkloadCompositionError::OverlapRuntimeRegistration(format!(
                "overlap runtime registration is missing required phase 15 operator `{operator_name}`"
            ))
        })?;
        if operator.classification() != Class::QueryGraphCompositionProgram {
            return Err(WorkloadCompositionError::OverlapRuntimeRegistration(format!(
                "overlap runtime registration requires `{operator_name}` to stay classified as a Query graph-composition program"
            )));
        }
        operator_names.push(operator.operator_name().to_string());
    }
    Ok(operator_names)
}

fn certify_required_phase_15_validators(
    plan: &PlanarBooleanOverlapValidatorRegistrationPlan,
) -> Result<Vec<String>, WorkloadCompositionError> {
    let mut validator_names = Vec::new();
    for validator_name in REQUIRED_PHASE_15_VALIDATOR_NAMES {
        let validator = plan.validator(validator_name).ok_or_else(|| {
            WorkloadCompositionError::OverlapRuntimeRegistration(format!(
                "overlap runtime registration is missing required phase 15 validator `{validator_name}`"
            ))
        })?;
        if validator.runtime_lane() != Lane::QueryGraphInvariantPack {
            return Err(WorkloadCompositionError::OverlapRuntimeRegistration(format!(
                "overlap runtime registration requires `{validator_name}` to stay on the Query graph-invariant runtime lane"
            )));
        }
        if !validator.governs_topology_legality() {
            return Err(WorkloadCompositionError::OverlapRuntimeRegistration(format!(
                "overlap runtime registration requires `{validator_name}` to remain a topology-legality validator"
            )));
        }
        validator_names.push(validator.validator_name().to_string());
    }
    Ok(validator_names)
}
