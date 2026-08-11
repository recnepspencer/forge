use crate::basis_lifecycle::NormalizedBasisIntent;
use crate::projection_consumption::ProjectionConsumptionDeclaration;
use crate::runtime::WorthQueryIntentDeclaration;

use super::super::super::seeds::{
    WorthQueryAuthoritativeMutationBatchIntentSeed, WorthQueryAuthoritativeMutationIntentSeed,
    WorthQueryDerivedViewIntentSeed, WorthQueryExistingTruthProbeIntentSeed,
    WorthQueryGenericInspectionIntentSeed, WorthQueryLiveReadIntentSeed,
    WorthQueryReadExecutionIntentSeed,
};
use crate::intent_admission::{
    WorthQueryIntentAdmissionCoveredEntrypoint, WorthQueryIntentAdmissionFamily,
};

use super::{WorthQueryIntentAdmissionRequestPayload, WorthQueryRawIntentAdmissionRequest};

impl WorthQueryRawIntentAdmissionRequest {
    pub fn family(&self) -> WorthQueryIntentAdmissionFamily {
        self.family
    }

    pub fn entrypoint(&self) -> WorthQueryIntentAdmissionCoveredEntrypoint {
        self.entrypoint
    }

    pub fn intent_name(&self) -> &str {
        &self.intent_name
    }

    pub fn input_digest(&self) -> &str {
        &self.input_digest
    }

    pub fn runtime_declaration(&self) -> Option<&WorthQueryIntentDeclaration> {
        match &self.payload {
            WorthQueryIntentAdmissionRequestPayload::RuntimeIntent(declaration) => {
                Some(declaration)
            }
            WorthQueryIntentAdmissionRequestPayload::AuthoritativeMutation(_)
            | WorthQueryIntentAdmissionRequestPayload::AuthoritativeMutationBatch(_)
            | WorthQueryIntentAdmissionRequestPayload::ReadExecution(_)
            | WorthQueryIntentAdmissionRequestPayload::LiveReadExecution(_)
            | WorthQueryIntentAdmissionRequestPayload::DerivedViewExecution(_)
            | WorthQueryIntentAdmissionRequestPayload::GenericInspection(_)
            | WorthQueryIntentAdmissionRequestPayload::ExistingTruthProbeRouting(_)
            | WorthQueryIntentAdmissionRequestPayload::BasisObservation(_)
            | WorthQueryIntentAdmissionRequestPayload::ProjectionConsumption(_) => None,
        }
    }

    pub fn authoritative_mutation_seed(
        &self,
    ) -> Option<&WorthQueryAuthoritativeMutationIntentSeed> {
        match &self.payload {
            WorthQueryIntentAdmissionRequestPayload::AuthoritativeMutation(seed) => Some(seed),
            WorthQueryIntentAdmissionRequestPayload::RuntimeIntent(_)
            | WorthQueryIntentAdmissionRequestPayload::AuthoritativeMutationBatch(_)
            | WorthQueryIntentAdmissionRequestPayload::ReadExecution(_)
            | WorthQueryIntentAdmissionRequestPayload::LiveReadExecution(_)
            | WorthQueryIntentAdmissionRequestPayload::DerivedViewExecution(_)
            | WorthQueryIntentAdmissionRequestPayload::GenericInspection(_)
            | WorthQueryIntentAdmissionRequestPayload::ExistingTruthProbeRouting(_)
            | WorthQueryIntentAdmissionRequestPayload::BasisObservation(_)
            | WorthQueryIntentAdmissionRequestPayload::ProjectionConsumption(_) => None,
        }
    }

    pub fn authoritative_mutation_batch_seed(
        &self,
    ) -> Option<&WorthQueryAuthoritativeMutationBatchIntentSeed> {
        match &self.payload {
            WorthQueryIntentAdmissionRequestPayload::AuthoritativeMutationBatch(seed) => Some(seed),
            WorthQueryIntentAdmissionRequestPayload::RuntimeIntent(_)
            | WorthQueryIntentAdmissionRequestPayload::AuthoritativeMutation(_)
            | WorthQueryIntentAdmissionRequestPayload::ReadExecution(_)
            | WorthQueryIntentAdmissionRequestPayload::LiveReadExecution(_)
            | WorthQueryIntentAdmissionRequestPayload::DerivedViewExecution(_)
            | WorthQueryIntentAdmissionRequestPayload::GenericInspection(_)
            | WorthQueryIntentAdmissionRequestPayload::ExistingTruthProbeRouting(_)
            | WorthQueryIntentAdmissionRequestPayload::BasisObservation(_)
            | WorthQueryIntentAdmissionRequestPayload::ProjectionConsumption(_) => None,
        }
    }

    pub fn read_execution_seed(&self) -> Option<&WorthQueryReadExecutionIntentSeed> {
        match &self.payload {
            WorthQueryIntentAdmissionRequestPayload::ReadExecution(seed) => Some(seed),
            WorthQueryIntentAdmissionRequestPayload::RuntimeIntent(_)
            | WorthQueryIntentAdmissionRequestPayload::AuthoritativeMutation(_)
            | WorthQueryIntentAdmissionRequestPayload::AuthoritativeMutationBatch(_)
            | WorthQueryIntentAdmissionRequestPayload::LiveReadExecution(_)
            | WorthQueryIntentAdmissionRequestPayload::DerivedViewExecution(_)
            | WorthQueryIntentAdmissionRequestPayload::GenericInspection(_)
            | WorthQueryIntentAdmissionRequestPayload::ExistingTruthProbeRouting(_)
            | WorthQueryIntentAdmissionRequestPayload::BasisObservation(_)
            | WorthQueryIntentAdmissionRequestPayload::ProjectionConsumption(_) => None,
        }
    }

    pub fn live_read_execution_seed(&self) -> Option<&WorthQueryLiveReadIntentSeed> {
        match &self.payload {
            WorthQueryIntentAdmissionRequestPayload::LiveReadExecution(seed) => Some(seed),
            WorthQueryIntentAdmissionRequestPayload::RuntimeIntent(_)
            | WorthQueryIntentAdmissionRequestPayload::AuthoritativeMutation(_)
            | WorthQueryIntentAdmissionRequestPayload::AuthoritativeMutationBatch(_)
            | WorthQueryIntentAdmissionRequestPayload::ReadExecution(_)
            | WorthQueryIntentAdmissionRequestPayload::DerivedViewExecution(_)
            | WorthQueryIntentAdmissionRequestPayload::GenericInspection(_)
            | WorthQueryIntentAdmissionRequestPayload::ExistingTruthProbeRouting(_)
            | WorthQueryIntentAdmissionRequestPayload::BasisObservation(_)
            | WorthQueryIntentAdmissionRequestPayload::ProjectionConsumption(_) => None,
        }
    }

    pub fn derived_view_seed(&self) -> Option<&WorthQueryDerivedViewIntentSeed> {
        match &self.payload {
            WorthQueryIntentAdmissionRequestPayload::DerivedViewExecution(seed) => Some(seed),
            WorthQueryIntentAdmissionRequestPayload::RuntimeIntent(_)
            | WorthQueryIntentAdmissionRequestPayload::AuthoritativeMutation(_)
            | WorthQueryIntentAdmissionRequestPayload::AuthoritativeMutationBatch(_)
            | WorthQueryIntentAdmissionRequestPayload::ReadExecution(_)
            | WorthQueryIntentAdmissionRequestPayload::LiveReadExecution(_)
            | WorthQueryIntentAdmissionRequestPayload::GenericInspection(_)
            | WorthQueryIntentAdmissionRequestPayload::ExistingTruthProbeRouting(_)
            | WorthQueryIntentAdmissionRequestPayload::BasisObservation(_)
            | WorthQueryIntentAdmissionRequestPayload::ProjectionConsumption(_) => None,
        }
    }

    pub fn generic_inspection_seed(&self) -> Option<&WorthQueryGenericInspectionIntentSeed> {
        match &self.payload {
            WorthQueryIntentAdmissionRequestPayload::GenericInspection(seed) => Some(seed),
            WorthQueryIntentAdmissionRequestPayload::RuntimeIntent(_)
            | WorthQueryIntentAdmissionRequestPayload::AuthoritativeMutation(_)
            | WorthQueryIntentAdmissionRequestPayload::AuthoritativeMutationBatch(_)
            | WorthQueryIntentAdmissionRequestPayload::ReadExecution(_)
            | WorthQueryIntentAdmissionRequestPayload::LiveReadExecution(_)
            | WorthQueryIntentAdmissionRequestPayload::DerivedViewExecution(_)
            | WorthQueryIntentAdmissionRequestPayload::ExistingTruthProbeRouting(_)
            | WorthQueryIntentAdmissionRequestPayload::BasisObservation(_)
            | WorthQueryIntentAdmissionRequestPayload::ProjectionConsumption(_) => None,
        }
    }

    pub fn existing_truth_probe_seed(&self) -> Option<&WorthQueryExistingTruthProbeIntentSeed> {
        match &self.payload {
            WorthQueryIntentAdmissionRequestPayload::ExistingTruthProbeRouting(seed) => Some(seed),
            WorthQueryIntentAdmissionRequestPayload::RuntimeIntent(_)
            | WorthQueryIntentAdmissionRequestPayload::AuthoritativeMutation(_)
            | WorthQueryIntentAdmissionRequestPayload::AuthoritativeMutationBatch(_)
            | WorthQueryIntentAdmissionRequestPayload::ReadExecution(_)
            | WorthQueryIntentAdmissionRequestPayload::LiveReadExecution(_)
            | WorthQueryIntentAdmissionRequestPayload::DerivedViewExecution(_)
            | WorthQueryIntentAdmissionRequestPayload::GenericInspection(_)
            | WorthQueryIntentAdmissionRequestPayload::BasisObservation(_)
            | WorthQueryIntentAdmissionRequestPayload::ProjectionConsumption(_) => None,
        }
    }

    pub fn basis_observation(&self) -> Option<&NormalizedBasisIntent> {
        match &self.payload {
            WorthQueryIntentAdmissionRequestPayload::BasisObservation(normalized) => {
                Some(normalized)
            }
            WorthQueryIntentAdmissionRequestPayload::RuntimeIntent(_)
            | WorthQueryIntentAdmissionRequestPayload::AuthoritativeMutation(_)
            | WorthQueryIntentAdmissionRequestPayload::AuthoritativeMutationBatch(_)
            | WorthQueryIntentAdmissionRequestPayload::ReadExecution(_)
            | WorthQueryIntentAdmissionRequestPayload::LiveReadExecution(_)
            | WorthQueryIntentAdmissionRequestPayload::DerivedViewExecution(_)
            | WorthQueryIntentAdmissionRequestPayload::GenericInspection(_)
            | WorthQueryIntentAdmissionRequestPayload::ExistingTruthProbeRouting(_)
            | WorthQueryIntentAdmissionRequestPayload::ProjectionConsumption(_) => None,
        }
    }

    pub fn projection_consumption_declaration(&self) -> Option<&ProjectionConsumptionDeclaration> {
        match &self.payload {
            WorthQueryIntentAdmissionRequestPayload::ProjectionConsumption(declaration) => {
                Some(declaration)
            }
            WorthQueryIntentAdmissionRequestPayload::RuntimeIntent(_)
            | WorthQueryIntentAdmissionRequestPayload::AuthoritativeMutation(_)
            | WorthQueryIntentAdmissionRequestPayload::AuthoritativeMutationBatch(_)
            | WorthQueryIntentAdmissionRequestPayload::ReadExecution(_)
            | WorthQueryIntentAdmissionRequestPayload::LiveReadExecution(_)
            | WorthQueryIntentAdmissionRequestPayload::DerivedViewExecution(_)
            | WorthQueryIntentAdmissionRequestPayload::ExistingTruthProbeRouting(_)
            | WorthQueryIntentAdmissionRequestPayload::GenericInspection(_)
            | WorthQueryIntentAdmissionRequestPayload::BasisObservation(_) => None,
        }
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }
}
