use crate::basis_lifecycle::{normalize_raw_basis_intent, NormalizedBasisIntent, RawBasisIntent};
use crate::identity::hash_parts;
use crate::projection_consumption::ProjectionConsumptionDeclaration;
use crate::runtime::ForgeQueryIntentDeclaration;

use crate::intent_admission::{
    intent_family_for_entrypoint, ForgeQueryIntentAdmissionCoveredEntrypoint,
    ForgeQueryIntentAdmissionFamily, ForgeQueryIntentViolationDecision,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryIntentAdmissionRequestPayload {
    RuntimeIntent(ForgeQueryIntentDeclaration),
    BasisObservation(NormalizedBasisIntent),
    ProjectionConsumption(ProjectionConsumptionDeclaration),
}

#[derive(Clone, Debug, Eq, PartialEq)]
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
            ForgeQueryIntentAdmissionRequestPayload::BasisObservation(_)
            | ForgeQueryIntentAdmissionRequestPayload::ProjectionConsumption(_) => None,
        }
    }

    pub fn basis_observation(&self) -> Option<&NormalizedBasisIntent> {
        match &self.payload {
            ForgeQueryIntentAdmissionRequestPayload::BasisObservation(normalized) => {
                Some(normalized)
            }
            ForgeQueryIntentAdmissionRequestPayload::RuntimeIntent(_)
            | ForgeQueryIntentAdmissionRequestPayload::ProjectionConsumption(_) => None,
        }
    }

    pub fn projection_consumption_declaration(&self) -> Option<&ProjectionConsumptionDeclaration> {
        match &self.payload {
            ForgeQueryIntentAdmissionRequestPayload::ProjectionConsumption(declaration) => {
                Some(declaration)
            }
            ForgeQueryIntentAdmissionRequestPayload::RuntimeIntent(_)
            | ForgeQueryIntentAdmissionRequestPayload::BasisObservation(_) => None,
        }
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }
}
