use crate::historical::{
    AdmittedHistoricalPathClass, HistoricalEvaluationAdmission,
    HistoricalMaterializationPathMetadata,
};

use super::basis::{HistoricalAdmissionClass, QueryContextDriftOutcome};
use super::performance::HistoricalMaterializationCostClass;

pub(crate) fn historical_admission_class(
    admission: &HistoricalEvaluationAdmission,
) -> HistoricalAdmissionClass {
    match admission.admitted_path().admitted_path_class() {
        AdmittedHistoricalPathClass::AdmittedRetainedSnapshotPath => {
            HistoricalAdmissionClass::RuntimeRetained
        }
        AdmittedHistoricalPathClass::AdmittedDeltaReplayPath => {
            HistoricalAdmissionClass::RuntimeReplay
        }
        AdmittedHistoricalPathClass::AdmittedFullReconstructionPath => {
            HistoricalAdmissionClass::RuntimeReconstruction
        }
    }
}

pub(crate) fn materialization_path_identity(
    metadata: &HistoricalMaterializationPathMetadata,
) -> String {
    format!(
        "{}:{}:{}",
        metadata.requested_path_class().as_str(),
        metadata.admitted_path_class().as_str(),
        metadata.resolved_path_class().as_str()
    )
}

pub(crate) fn requested_path_identity(metadata: &HistoricalMaterializationPathMetadata) -> String {
    metadata.requested_path_class().as_str().to_string()
}

pub(crate) fn admitted_path_identity(metadata: &HistoricalMaterializationPathMetadata) -> String {
    metadata.admitted_path_class().as_str().to_string()
}

pub(crate) fn resolved_path_identity(metadata: &HistoricalMaterializationPathMetadata) -> String {
    metadata.resolved_path_class().as_str().to_string()
}

pub(crate) fn drift_outcome_for_historical(
    admission: &HistoricalEvaluationAdmission,
) -> QueryContextDriftOutcome {
    match historical_admission_class(admission) {
        HistoricalAdmissionClass::RuntimeRetained
        | HistoricalAdmissionClass::RuntimeReplay
        | HistoricalAdmissionClass::RuntimeReconstruction => QueryContextDriftOutcome::BasisExact,
        HistoricalAdmissionClass::StoreDeferredDebt => {
            QueryContextDriftOutcome::ExplicitHistoricalDenial
        }
    }
}

pub(crate) fn materialization_path_cost_class(
    admission: &HistoricalEvaluationAdmission,
) -> HistoricalMaterializationCostClass {
    match historical_admission_class(admission) {
        HistoricalAdmissionClass::RuntimeRetained => {
            HistoricalMaterializationCostClass::RetainedBounded
        }
        HistoricalAdmissionClass::RuntimeReplay => {
            HistoricalMaterializationCostClass::ReplayBounded
        }
        HistoricalAdmissionClass::RuntimeReconstruction => {
            HistoricalMaterializationCostClass::ReconstructionBounded
        }
        HistoricalAdmissionClass::StoreDeferredDebt => {
            HistoricalMaterializationCostClass::ReconstructionBounded
        }
    }
}
