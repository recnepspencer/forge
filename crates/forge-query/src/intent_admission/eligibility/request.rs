use crate::basis_lifecycle::{normalize_raw_basis_intent, NormalizedBasisIntent, RawBasisIntent};
use crate::identity::hash_parts;
use crate::projection_consumption::ProjectionConsumptionDeclaration;
use crate::runtime::ForgeQueryIntentDeclaration;

use super::seeds::{
    ForgeQueryAuthoritativeMutationBatchIntentSeed, ForgeQueryAuthoritativeMutationIntentSeed,
    ForgeQueryDerivedViewIntentSeed,
};
use super::seeds::{
    ForgeQueryExistingTruthProbeIntentSeed, ForgeQueryGenericInspectionIntentSeed,
    ForgeQueryLiveReadIntentSeed, ForgeQueryReadExecutionIntentSeed,
};
use crate::intent_admission::{
    intent_family_for_entrypoint, ForgeQueryIntentAdmissionCoveredEntrypoint,
    ForgeQueryIntentAdmissionFamily, ForgeQueryIntentViolationDecision,
};

#[derive(Clone, Debug, PartialEq)]
pub enum ForgeQueryIntentAdmissionRequestPayload {
    RuntimeIntent(ForgeQueryIntentDeclaration),
    AuthoritativeMutation(ForgeQueryAuthoritativeMutationIntentSeed),
    AuthoritativeMutationBatch(ForgeQueryAuthoritativeMutationBatchIntentSeed),
    ReadExecution(ForgeQueryReadExecutionIntentSeed),
    LiveReadExecution(ForgeQueryLiveReadIntentSeed),
    DerivedViewExecution(ForgeQueryDerivedViewIntentSeed),
    GenericInspection(ForgeQueryGenericInspectionIntentSeed),
    ExistingTruthProbeRouting(ForgeQueryExistingTruthProbeIntentSeed),
    BasisObservation(NormalizedBasisIntent),
    ProjectionConsumption(ProjectionConsumptionDeclaration),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryRawIntentAdmissionRequest {
    family: ForgeQueryIntentAdmissionFamily,
    entrypoint: ForgeQueryIntentAdmissionCoveredEntrypoint,
    intent_name: String,
    input_digest: String,
    payload: ForgeQueryIntentAdmissionRequestPayload,
    request_digest: String,
}

impl ForgeQueryRawIntentAdmissionRequest {
    pub fn authoritative_runtime_entrypoint(
        declaration: ForgeQueryIntentDeclaration,
    ) -> Result<Self, ForgeQueryIntentViolationDecision> {
        Self::new(
            ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteIntent,
            declaration.name().to_string(),
            declaration.input_digest().to_string(),
            ForgeQueryIntentAdmissionRequestPayload::RuntimeIntent(declaration),
        )
    }

    pub fn effect_runtime_entrypoint(
        declaration: ForgeQueryIntentDeclaration,
    ) -> Result<Self, ForgeQueryIntentViolationDecision> {
        Self::new(
            ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteNextEffectWriteIntent,
            declaration.name().to_string(),
            declaration.input_digest().to_string(),
            ForgeQueryIntentAdmissionRequestPayload::RuntimeIntent(declaration),
        )
    }

    pub fn authoritative_write_entrypoint(
        seed: ForgeQueryAuthoritativeMutationIntentSeed,
    ) -> Result<Self, ForgeQueryIntentViolationDecision> {
        let intent_name = seed.command_label();
        let input_digest = seed.command_input_digest();
        Self::new(
            ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
            intent_name,
            input_digest,
            ForgeQueryIntentAdmissionRequestPayload::AuthoritativeMutation(seed),
        )
    }

    pub fn authoritative_write_batch_entrypoint(
        seed: ForgeQueryAuthoritativeMutationBatchIntentSeed,
    ) -> Result<Self, ForgeQueryIntentViolationDecision> {
        let intent_name = seed.batch_label();
        let input_digest = seed.batch_input_digest();
        Self::new(
            ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteBatchWrite,
            intent_name,
            input_digest,
            ForgeQueryIntentAdmissionRequestPayload::AuthoritativeMutationBatch(seed),
        )
    }

    pub fn basis_observation_lane(
        raw: RawBasisIntent,
    ) -> Result<Self, ForgeQueryIntentViolationDecision> {
        let normalized = normalize_raw_basis_intent(raw, "observation").map_err(|denial| {
            ForgeQueryIntentViolationDecision::new(
                ForgeQueryIntentAdmissionFamily::BasisUseIntent,
                ForgeQueryIntentAdmissionCoveredEntrypoint::BasisObservation,
                "raw-basis-intent",
                denial.message(),
                "basis-observation-raw-intent-rejected",
                "basis-observation-raw-intent-rejected",
            )
        })?;
        Self::new(
            ForgeQueryIntentAdmissionCoveredEntrypoint::BasisObservation,
            format!("basis.observation.{}", normalized.family().as_str()),
            normalized.normalized_digest().to_string(),
            ForgeQueryIntentAdmissionRequestPayload::BasisObservation(normalized),
        )
    }

    pub fn read_family_entrypoint(
        seed: ForgeQueryReadExecutionIntentSeed,
    ) -> Result<Self, ForgeQueryIntentViolationDecision> {
        Self::new(
            ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteReadFamily,
            seed.request_label().to_string(),
            seed.request_input_digest().to_string(),
            ForgeQueryIntentAdmissionRequestPayload::ReadExecution(seed),
        )
    }

    pub fn read_family_in_basis_context_entrypoint(
        seed: ForgeQueryReadExecutionIntentSeed,
    ) -> Result<Self, ForgeQueryIntentViolationDecision> {
        Self::new(
            ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteReadFamilyInBasisContext,
            seed.request_label().to_string(),
            seed.request_input_digest().to_string(),
            ForgeQueryIntentAdmissionRequestPayload::ReadExecution(seed),
        )
    }

    pub fn live_read_entrypoint(
        seed: ForgeQueryLiveReadIntentSeed,
    ) -> Result<Self, ForgeQueryIntentViolationDecision> {
        Self::new(
            ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteLiveRead,
            seed.request_label().to_string(),
            seed.request_input_digest().to_string(),
            ForgeQueryIntentAdmissionRequestPayload::LiveReadExecution(seed),
        )
    }

    pub fn derived_materialization_entrypoint(
        seed: ForgeQueryDerivedViewIntentSeed,
    ) -> Result<Self, ForgeQueryIntentViolationDecision> {
        let intent_name = seed.request_label("materialize");
        let input_digest = seed.request_input_digest("materialize");
        Self::new(
            ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteDerivedMaterialization,
            intent_name,
            input_digest,
            ForgeQueryIntentAdmissionRequestPayload::DerivedViewExecution(seed),
        )
    }

    pub fn derived_inspection_entrypoint(
        seed: ForgeQueryDerivedViewIntentSeed,
    ) -> Result<Self, ForgeQueryIntentViolationDecision> {
        let intent_name = seed.request_label("inspect");
        let input_digest = seed.request_input_digest("inspect");
        Self::new(
            ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteDerivedInspection,
            intent_name,
            input_digest,
            ForgeQueryIntentAdmissionRequestPayload::DerivedViewExecution(seed),
        )
    }

    pub fn generic_inspection_entrypoint(
        seed: ForgeQueryGenericInspectionIntentSeed,
    ) -> Result<Self, ForgeQueryIntentViolationDecision> {
        Self::new(
            ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteUnifiedInspection,
            seed.request_label().as_str().to_string(),
            seed.request_input_digest().to_string(),
            ForgeQueryIntentAdmissionRequestPayload::GenericInspection(seed),
        )
    }

    pub fn existing_truth_probe_entrypoint(
        seed: ForgeQueryExistingTruthProbeIntentSeed,
    ) -> Result<Self, ForgeQueryIntentViolationDecision> {
        Self::new(
            ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteExistingTruthProbeRouting,
            seed.request_label(),
            seed.request_input_digest(),
            ForgeQueryIntentAdmissionRequestPayload::ExistingTruthProbeRouting(seed),
        )
    }

    pub fn projection_consumption(
        declaration: ProjectionConsumptionDeclaration,
    ) -> Result<Self, ForgeQueryIntentViolationDecision> {
        let source_family = declaration.source().family().as_str().to_string();
        let declaration_digest = declaration.declaration_digest().to_string();
        Self::new(
            ForgeQueryIntentAdmissionCoveredEntrypoint::ProjectionConsumption,
            format!("projection.consume.{source_family}"),
            declaration_digest,
            ForgeQueryIntentAdmissionRequestPayload::ProjectionConsumption(declaration),
        )
    }

    pub(crate) fn deferred_neighbor(
        entrypoint: ForgeQueryIntentAdmissionCoveredEntrypoint,
        declaration: ForgeQueryIntentDeclaration,
    ) -> Result<Self, ForgeQueryIntentViolationDecision> {
        Self::new(
            entrypoint,
            declaration.name().to_string(),
            declaration.input_digest().to_string(),
            ForgeQueryIntentAdmissionRequestPayload::RuntimeIntent(declaration),
        )
    }

    fn new(
        entrypoint: ForgeQueryIntentAdmissionCoveredEntrypoint,
        intent_name: String,
        input_digest: String,
        payload: ForgeQueryIntentAdmissionRequestPayload,
    ) -> Result<Self, ForgeQueryIntentViolationDecision> {
        let family = intent_family_for_entrypoint(entrypoint);
        let source_label = match &payload {
            ForgeQueryIntentAdmissionRequestPayload::RuntimeIntent(declaration) => {
                declaration.source_lane().as_str().to_string()
            }
            ForgeQueryIntentAdmissionRequestPayload::AuthoritativeMutation(seed) => {
                format!(
                    "authoritative-mutation:{}",
                    seed.command().mutation_family().as_str()
                )
            }
            ForgeQueryIntentAdmissionRequestPayload::AuthoritativeMutationBatch(seed) => {
                format!("authoritative-mutation-batch:{}", seed.commands().len())
            }
            ForgeQueryIntentAdmissionRequestPayload::ReadExecution(seed) => format!(
                "read-execution:{}:{}",
                seed.read_family().family_digest(),
                seed.basis_context()
                    .map(|context| context.family().as_str())
                    .unwrap_or("runtime-current")
            ),
            ForgeQueryIntentAdmissionRequestPayload::LiveReadExecution(seed) => {
                format!("live-read-execution:{}", seed.live_view_digest())
            }
            ForgeQueryIntentAdmissionRequestPayload::DerivedViewExecution(seed) => {
                format!("derived-view-execution:{}", seed.view_name())
            }
            ForgeQueryIntentAdmissionRequestPayload::GenericInspection(seed) => {
                format!("generic-inspection:{}", seed.request_input_digest())
            }
            ForgeQueryIntentAdmissionRequestPayload::ExistingTruthProbeRouting(seed) => {
                format!(
                    "existing-truth-probe-routing:{}",
                    seed.request().binding().binding_digest()
                )
            }
            ForgeQueryIntentAdmissionRequestPayload::BasisObservation(_) => {
                "basis-observation".to_string()
            }
            ForgeQueryIntentAdmissionRequestPayload::ProjectionConsumption(_) => {
                "projection-consumption".to_string()
            }
        };
        let request_digest = hash_parts(&[
            "forge_query_raw_intent_admission_request_v1".to_string(),
            format!("family:{}", family.as_str()),
            format!("entrypoint:{}", entrypoint.as_str()),
            format!("intent:{intent_name}"),
            format!("input:{input_digest}"),
            format!("source:{source_label}"),
        ]);
        Ok(Self {
            family,
            entrypoint,
            intent_name,
            input_digest,
            payload,
            request_digest,
        })
    }

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
        self.family
    }

    pub fn entrypoint(&self) -> ForgeQueryIntentAdmissionCoveredEntrypoint {
        self.entrypoint
    }

    pub fn intent_name(&self) -> &str {
        &self.intent_name
    }

    pub fn input_digest(&self) -> &str {
        &self.input_digest
    }

    pub fn runtime_declaration(&self) -> Option<&ForgeQueryIntentDeclaration> {
        match &self.payload {
            ForgeQueryIntentAdmissionRequestPayload::RuntimeIntent(declaration) => {
                Some(declaration)
            }
            ForgeQueryIntentAdmissionRequestPayload::AuthoritativeMutation(_)
            | ForgeQueryIntentAdmissionRequestPayload::AuthoritativeMutationBatch(_)
            | ForgeQueryIntentAdmissionRequestPayload::ReadExecution(_)
            | ForgeQueryIntentAdmissionRequestPayload::LiveReadExecution(_)
            | ForgeQueryIntentAdmissionRequestPayload::DerivedViewExecution(_)
            | ForgeQueryIntentAdmissionRequestPayload::GenericInspection(_)
            | ForgeQueryIntentAdmissionRequestPayload::ExistingTruthProbeRouting(_)
            | ForgeQueryIntentAdmissionRequestPayload::BasisObservation(_)
            | ForgeQueryIntentAdmissionRequestPayload::ProjectionConsumption(_) => None,
        }
    }

    pub fn authoritative_mutation_seed(
        &self,
    ) -> Option<&ForgeQueryAuthoritativeMutationIntentSeed> {
        match &self.payload {
            ForgeQueryIntentAdmissionRequestPayload::AuthoritativeMutation(seed) => Some(seed),
            ForgeQueryIntentAdmissionRequestPayload::RuntimeIntent(_)
            | ForgeQueryIntentAdmissionRequestPayload::AuthoritativeMutationBatch(_)
            | ForgeQueryIntentAdmissionRequestPayload::ReadExecution(_)
            | ForgeQueryIntentAdmissionRequestPayload::LiveReadExecution(_)
            | ForgeQueryIntentAdmissionRequestPayload::DerivedViewExecution(_)
            | ForgeQueryIntentAdmissionRequestPayload::GenericInspection(_)
            | ForgeQueryIntentAdmissionRequestPayload::ExistingTruthProbeRouting(_)
            | ForgeQueryIntentAdmissionRequestPayload::BasisObservation(_)
            | ForgeQueryIntentAdmissionRequestPayload::ProjectionConsumption(_) => None,
        }
    }

    pub fn authoritative_mutation_batch_seed(
        &self,
    ) -> Option<&ForgeQueryAuthoritativeMutationBatchIntentSeed> {
        match &self.payload {
            ForgeQueryIntentAdmissionRequestPayload::AuthoritativeMutationBatch(seed) => Some(seed),
            ForgeQueryIntentAdmissionRequestPayload::RuntimeIntent(_)
            | ForgeQueryIntentAdmissionRequestPayload::AuthoritativeMutation(_)
            | ForgeQueryIntentAdmissionRequestPayload::ReadExecution(_)
            | ForgeQueryIntentAdmissionRequestPayload::LiveReadExecution(_)
            | ForgeQueryIntentAdmissionRequestPayload::DerivedViewExecution(_)
            | ForgeQueryIntentAdmissionRequestPayload::GenericInspection(_)
            | ForgeQueryIntentAdmissionRequestPayload::ExistingTruthProbeRouting(_)
            | ForgeQueryIntentAdmissionRequestPayload::BasisObservation(_)
            | ForgeQueryIntentAdmissionRequestPayload::ProjectionConsumption(_) => None,
        }
    }

    pub fn read_execution_seed(&self) -> Option<&ForgeQueryReadExecutionIntentSeed> {
        match &self.payload {
            ForgeQueryIntentAdmissionRequestPayload::ReadExecution(seed) => Some(seed),
            ForgeQueryIntentAdmissionRequestPayload::RuntimeIntent(_)
            | ForgeQueryIntentAdmissionRequestPayload::AuthoritativeMutation(_)
            | ForgeQueryIntentAdmissionRequestPayload::AuthoritativeMutationBatch(_)
            | ForgeQueryIntentAdmissionRequestPayload::LiveReadExecution(_)
            | ForgeQueryIntentAdmissionRequestPayload::DerivedViewExecution(_)
            | ForgeQueryIntentAdmissionRequestPayload::GenericInspection(_)
            | ForgeQueryIntentAdmissionRequestPayload::ExistingTruthProbeRouting(_)
            | ForgeQueryIntentAdmissionRequestPayload::BasisObservation(_)
            | ForgeQueryIntentAdmissionRequestPayload::ProjectionConsumption(_) => None,
        }
    }

    pub fn live_read_execution_seed(&self) -> Option<&ForgeQueryLiveReadIntentSeed> {
        match &self.payload {
            ForgeQueryIntentAdmissionRequestPayload::LiveReadExecution(seed) => Some(seed),
            ForgeQueryIntentAdmissionRequestPayload::RuntimeIntent(_)
            | ForgeQueryIntentAdmissionRequestPayload::AuthoritativeMutation(_)
            | ForgeQueryIntentAdmissionRequestPayload::AuthoritativeMutationBatch(_)
            | ForgeQueryIntentAdmissionRequestPayload::ReadExecution(_)
            | ForgeQueryIntentAdmissionRequestPayload::DerivedViewExecution(_)
            | ForgeQueryIntentAdmissionRequestPayload::GenericInspection(_)
            | ForgeQueryIntentAdmissionRequestPayload::ExistingTruthProbeRouting(_)
            | ForgeQueryIntentAdmissionRequestPayload::BasisObservation(_)
            | ForgeQueryIntentAdmissionRequestPayload::ProjectionConsumption(_) => None,
        }
    }

    pub fn derived_view_seed(&self) -> Option<&ForgeQueryDerivedViewIntentSeed> {
        match &self.payload {
            ForgeQueryIntentAdmissionRequestPayload::DerivedViewExecution(seed) => Some(seed),
            ForgeQueryIntentAdmissionRequestPayload::RuntimeIntent(_)
            | ForgeQueryIntentAdmissionRequestPayload::AuthoritativeMutation(_)
            | ForgeQueryIntentAdmissionRequestPayload::AuthoritativeMutationBatch(_)
            | ForgeQueryIntentAdmissionRequestPayload::ReadExecution(_)
            | ForgeQueryIntentAdmissionRequestPayload::LiveReadExecution(_)
            | ForgeQueryIntentAdmissionRequestPayload::GenericInspection(_)
            | ForgeQueryIntentAdmissionRequestPayload::ExistingTruthProbeRouting(_)
            | ForgeQueryIntentAdmissionRequestPayload::BasisObservation(_)
            | ForgeQueryIntentAdmissionRequestPayload::ProjectionConsumption(_) => None,
        }
    }

    pub fn generic_inspection_seed(&self) -> Option<&ForgeQueryGenericInspectionIntentSeed> {
        match &self.payload {
            ForgeQueryIntentAdmissionRequestPayload::GenericInspection(seed) => Some(seed),
            ForgeQueryIntentAdmissionRequestPayload::RuntimeIntent(_)
            | ForgeQueryIntentAdmissionRequestPayload::AuthoritativeMutation(_)
            | ForgeQueryIntentAdmissionRequestPayload::AuthoritativeMutationBatch(_)
            | ForgeQueryIntentAdmissionRequestPayload::ReadExecution(_)
            | ForgeQueryIntentAdmissionRequestPayload::LiveReadExecution(_)
            | ForgeQueryIntentAdmissionRequestPayload::DerivedViewExecution(_)
            | ForgeQueryIntentAdmissionRequestPayload::ExistingTruthProbeRouting(_)
            | ForgeQueryIntentAdmissionRequestPayload::BasisObservation(_)
            | ForgeQueryIntentAdmissionRequestPayload::ProjectionConsumption(_) => None,
        }
    }

    pub fn existing_truth_probe_seed(&self) -> Option<&ForgeQueryExistingTruthProbeIntentSeed> {
        match &self.payload {
            ForgeQueryIntentAdmissionRequestPayload::ExistingTruthProbeRouting(seed) => Some(seed),
            ForgeQueryIntentAdmissionRequestPayload::RuntimeIntent(_)
            | ForgeQueryIntentAdmissionRequestPayload::AuthoritativeMutation(_)
            | ForgeQueryIntentAdmissionRequestPayload::AuthoritativeMutationBatch(_)
            | ForgeQueryIntentAdmissionRequestPayload::ReadExecution(_)
            | ForgeQueryIntentAdmissionRequestPayload::LiveReadExecution(_)
            | ForgeQueryIntentAdmissionRequestPayload::DerivedViewExecution(_)
            | ForgeQueryIntentAdmissionRequestPayload::GenericInspection(_)
            | ForgeQueryIntentAdmissionRequestPayload::BasisObservation(_)
            | ForgeQueryIntentAdmissionRequestPayload::ProjectionConsumption(_) => None,
        }
    }

    pub fn basis_observation(&self) -> Option<&NormalizedBasisIntent> {
        match &self.payload {
            ForgeQueryIntentAdmissionRequestPayload::BasisObservation(normalized) => {
                Some(normalized)
            }
            ForgeQueryIntentAdmissionRequestPayload::RuntimeIntent(_)
            | ForgeQueryIntentAdmissionRequestPayload::AuthoritativeMutation(_)
            | ForgeQueryIntentAdmissionRequestPayload::AuthoritativeMutationBatch(_)
            | ForgeQueryIntentAdmissionRequestPayload::ReadExecution(_)
            | ForgeQueryIntentAdmissionRequestPayload::LiveReadExecution(_)
            | ForgeQueryIntentAdmissionRequestPayload::DerivedViewExecution(_)
            | ForgeQueryIntentAdmissionRequestPayload::GenericInspection(_)
            | ForgeQueryIntentAdmissionRequestPayload::ExistingTruthProbeRouting(_)
            | ForgeQueryIntentAdmissionRequestPayload::ProjectionConsumption(_) => None,
        }
    }

    pub fn projection_consumption_declaration(&self) -> Option<&ProjectionConsumptionDeclaration> {
        match &self.payload {
            ForgeQueryIntentAdmissionRequestPayload::ProjectionConsumption(declaration) => {
                Some(declaration)
            }
            ForgeQueryIntentAdmissionRequestPayload::RuntimeIntent(_)
            | ForgeQueryIntentAdmissionRequestPayload::AuthoritativeMutation(_)
            | ForgeQueryIntentAdmissionRequestPayload::AuthoritativeMutationBatch(_)
            | ForgeQueryIntentAdmissionRequestPayload::ReadExecution(_)
            | ForgeQueryIntentAdmissionRequestPayload::LiveReadExecution(_)
            | ForgeQueryIntentAdmissionRequestPayload::DerivedViewExecution(_)
            | ForgeQueryIntentAdmissionRequestPayload::ExistingTruthProbeRouting(_)
            | ForgeQueryIntentAdmissionRequestPayload::GenericInspection(_)
            | ForgeQueryIntentAdmissionRequestPayload::BasisObservation(_) => None,
        }
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }
}
