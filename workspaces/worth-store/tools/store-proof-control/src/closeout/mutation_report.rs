use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::evidence::sha256_serialized;

use super::ControlledDefectKind;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ControlledDefectObservation {
    pub defect: ControlledDefectKind,
    pub failed_product: String,
    pub failed_predicate: String,
    pub failure_code: String,
    pub execution: MutationExecutionEvidence,
    pub unrelated_products: Vec<InterpretableProofProduct>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum MutationExecutionEvidence {
    ProductionValidator {
        validator: String,
        mutated_subject_sha256: String,
        denial_sha256: String,
    },
    IsolatedCargoFixture {
        command: Vec<String>,
        exit_code: i32,
        transcript_sha256: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InterpretableProofProduct {
    pub product: String,
    pub evidence_identity: String,
    pub posture: InterpretableProductPosture,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum InterpretableProductPosture {
    PassedUnchangedControl,
    IndependentlyFailedWithNamedPredicate,
    ExplicitlyNotSelected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofMutationSensitivityReport {
    schema_version: u32,
    evidence_identity: String,
    observations: Vec<ControlledDefectObservation>,
}

impl ControlledDefectObservation {
    pub fn localized(
        defect: ControlledDefectKind,
        execution: MutationExecutionEvidence,
        unrelated_products: Vec<InterpretableProofProduct>,
    ) -> Result<Self, String> {
        let observation = Self {
            defect,
            failed_product: defect.expected_product().to_owned(),
            failed_predicate: defect.expected_predicate().to_owned(),
            failure_code: defect.expected_failure_code().to_owned(),
            execution,
            unrelated_products,
        };
        observation.validate()?;
        Ok(observation)
    }

    fn validate(&self) -> Result<(), String> {
        if self.failed_product != self.defect.expected_product()
            || self.failed_predicate != self.defect.expected_predicate()
            || self.failure_code != self.defect.expected_failure_code()
        {
            return Err(format!(
                "controlled defect {:?} failed an unrelated product or predicate",
                self.defect
            ));
        }
        validate_execution(&self.execution)?;
        if self.unrelated_products.is_empty() {
            return Err(format!(
                "controlled defect {:?} has no interpretable unrelated control",
                self.defect
            ));
        }
        let mut products = BTreeSet::new();
        for product in &self.unrelated_products {
            if product.product == self.failed_product
                || product.product.trim().is_empty()
                || !is_sha256(&product.evidence_identity)
                || !products.insert(product.product.as_str())
            {
                return Err(format!(
                    "controlled defect {:?} has an invalid unrelated control",
                    self.defect
                ));
            }
        }
        Ok(())
    }
}

impl ProofMutationSensitivityReport {
    pub fn certify(mut observations: Vec<ControlledDefectObservation>) -> Result<Self, String> {
        observations.sort_by_key(|observation| observation.defect);
        let mut report = Self {
            schema_version: 1,
            evidence_identity: String::new(),
            observations,
        };
        report.validate_surface()?;
        report.evidence_identity = report.expected_identity()?;
        Ok(report)
    }

    pub fn validate(&self) -> Result<(), String> {
        self.validate_surface()?;
        if self.expected_identity()? != self.evidence_identity {
            return Err(
                "mutation sensitivity report identity does not match its contents".to_owned(),
            );
        }
        Ok(())
    }

    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }

    pub fn observations(&self) -> &[ControlledDefectObservation] {
        &self.observations
    }

    fn validate_surface(&self) -> Result<(), String> {
        if self.schema_version != 1 {
            return Err(format!(
                "unsupported mutation sensitivity schema: {}",
                self.schema_version
            ));
        }
        let observed: BTreeSet<_> = self
            .observations
            .iter()
            .map(|observation| observation.defect)
            .collect();
        let expected: BTreeSet<_> = ControlledDefectKind::ALL.into_iter().collect();
        if observed != expected || observed.len() != self.observations.len() {
            return Err(format!(
                "mutation sensitivity matrix is incomplete: expected {expected:?}, observed {observed:?}"
            ));
        }
        for observation in &self.observations {
            observation.validate()?;
        }
        Ok(())
    }

    fn expected_identity(&self) -> Result<String, String> {
        let mut basis = self.clone();
        basis.evidence_identity.clear();
        sha256_serialized(&basis)
    }
}

fn validate_execution(execution: &MutationExecutionEvidence) -> Result<(), String> {
    match execution {
        MutationExecutionEvidence::ProductionValidator {
            validator,
            mutated_subject_sha256,
            denial_sha256,
        } => {
            if validator.trim().is_empty()
                || !is_sha256(mutated_subject_sha256)
                || !is_sha256(denial_sha256)
            {
                return Err("production mutation validator evidence is incomplete".to_owned());
            }
        }
        MutationExecutionEvidence::IsolatedCargoFixture {
            command,
            exit_code,
            transcript_sha256,
        } => {
            if command.is_empty() || *exit_code == 0 || !is_sha256(transcript_sha256) {
                return Err(
                    "isolated Cargo mutation evidence did not carry a failed execution".to_owned(),
                );
            }
        }
    }
    Ok(())
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}
