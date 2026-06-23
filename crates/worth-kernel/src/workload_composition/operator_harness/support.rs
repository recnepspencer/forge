use worth_spatial::facade::workload_vocabulary::{
    WorkloadEvidenceGuardError, WorkloadEvidenceLedgerError, WorkloadEvidenceStage,
};

use super::declaration::{
    OperatorDeclarationReceipt, UnsupportedOperatorFamily, WorkloadOperatorFamily,
};
use super::query::query_backed_operator_support;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperatorSupportReceipt {
    family: WorkloadOperatorFamily,
    posture: OperatorSupportPosture,
    query_support_digest: String,
    human_reason: String,
}

impl OperatorSupportReceipt {
    pub(super) fn for_declaration(
        declaration: &OperatorDeclarationReceipt,
    ) -> Result<Self, OperatorWorkloadError> {
        let posture = if declaration.family().is_supported() {
            OperatorSupportPosture::Admitted
        } else {
            OperatorSupportPosture::Unsupported
        };
        let human_reason = match posture {
            OperatorSupportPosture::Admitted => format!(
                "{} is admitted for {}",
                declaration.family().human_name(),
                declaration.requirement().human_name()
            ),
            OperatorSupportPosture::Unsupported => format!(
                "{} is not supported by the workload operator harness yet",
                declaration.family().human_name()
            ),
        };
        let query_receipt = query_backed_operator_support(
            declaration.family(),
            declaration.requirement(),
            declaration.query_intent(),
            posture.query_key(),
            declaration.query_declaration_digest(),
            declaration.query_envelope_digest(),
        )
        .map_err(OperatorWorkloadError::QueryAdmissionFailed)?;
        let query_support_digest = query_receipt.declaration_digest().to_string();
        Ok(Self {
            family: declaration.family(),
            posture,
            query_support_digest,
            human_reason,
        })
    }

    pub fn family(&self) -> WorkloadOperatorFamily {
        self.family
    }

    pub fn posture(&self) -> OperatorSupportPosture {
        self.posture
    }

    pub fn query_support_digest(&self) -> &str {
        &self.query_support_digest
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OperatorSupportPosture {
    Admitted,
    Unsupported,
}

impl OperatorSupportPosture {
    fn query_key(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Unsupported => "unsupported",
        }
    }
}

pub type OperatorWorkloadReceipt = super::receipt_set::OperatorReceiptSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OperatorWorkloadError {
    MissingQueryDeclaration,
    QueryAdmissionFailed(String),
    MissingRequiredStage(WorkloadEvidenceStage),
    UnsupportedRequirement(crate::workload_composition::WorkloadStageRequirement),
    EvidenceStageBindingFailed(WorkloadEvidenceLedgerError),
    EvidenceGuard(WorkloadEvidenceGuardError),
    SyntheticProjection,
    UnsupportedOperatorFamily {
        family: UnsupportedOperatorFamily,
        support: OperatorSupportReceipt,
    },
    WrongOperatorFamily {
        expected: WorkloadOperatorFamily,
        actual: WorkloadOperatorFamily,
    },
    SpatialOperatorDenied(String),
}

impl OperatorWorkloadError {
    pub fn human_reason(&self) -> String {
        match self {
            Self::MissingQueryDeclaration => {
                "operator declaration requires a human-readable Query intent".to_string()
            }
            Self::QueryAdmissionFailed(reason) => {
                format!("operator declaration could not be admitted by Forge Query: {reason}")
            }
            Self::MissingRequiredStage(stage) => {
                format!("operator workload is missing {}", stage.human_name())
            }
            Self::UnsupportedRequirement(requirement) => format!(
                "{} is not a valid operator workload requirement",
                requirement.human_name()
            ),
            Self::EvidenceStageBindingFailed(error) => error.human_reason(),
            Self::EvidenceGuard(error) => error.human_reason().to_string(),
            Self::SyntheticProjection => {
                "operator workload requires projected entities and local-basis evidence".to_string()
            }
            Self::UnsupportedOperatorFamily { support, .. } => support.human_reason().to_string(),
            Self::WrongOperatorFamily { expected, actual } => format!(
                "operator run for {} cannot execute as {}",
                actual.human_name(),
                expected.human_name()
            ),
            Self::SpatialOperatorDenied(reason) => reason.clone(),
        }
    }
}
