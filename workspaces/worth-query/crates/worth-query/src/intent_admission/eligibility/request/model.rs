mod authoring;
mod payload_observation;

use crate::basis_lifecycle::NormalizedBasisIntent;
use crate::projection_consumption::ProjectionConsumptionDeclaration;
use crate::runtime::WorthQueryIntentDeclaration;

use super::super::seeds::{
    WorthQueryAuthoritativeMutationBatchIntentSeed, WorthQueryAuthoritativeMutationIntentSeed,
    WorthQueryDerivedViewIntentSeed, WorthQueryExistingTruthProbeIntentSeed,
    WorthQueryGenericInspectionIntentSeed, WorthQueryLiveReadIntentSeed,
    WorthQueryReadExecutionIntentSeed,
};
use crate::intent_admission::{
    WorthQueryIntentAdmissionCoveredEntrypoint, WorthQueryIntentAdmissionFamily,
};

#[derive(Clone, Debug, PartialEq)]
pub enum WorthQueryIntentAdmissionRequestPayload {
    RuntimeIntent(WorthQueryIntentDeclaration),
    AuthoritativeMutation(WorthQueryAuthoritativeMutationIntentSeed),
    AuthoritativeMutationBatch(WorthQueryAuthoritativeMutationBatchIntentSeed),
    ReadExecution(WorthQueryReadExecutionIntentSeed),
    LiveReadExecution(WorthQueryLiveReadIntentSeed),
    DerivedViewExecution(WorthQueryDerivedViewIntentSeed),
    GenericInspection(WorthQueryGenericInspectionIntentSeed),
    ExistingTruthProbeRouting(WorthQueryExistingTruthProbeIntentSeed),
    BasisObservation(NormalizedBasisIntent),
    ProjectionConsumption(ProjectionConsumptionDeclaration),
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryRawIntentAdmissionRequest {
    family: WorthQueryIntentAdmissionFamily,
    entrypoint: WorthQueryIntentAdmissionCoveredEntrypoint,
    intent_name: String,
    input_digest: String,
    payload: WorthQueryIntentAdmissionRequestPayload,
    request_digest: String,
}
