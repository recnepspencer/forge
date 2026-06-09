use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationEnvelope,
    ForgeQueryDomainOperatingContext, ForgeQueryOrdinaryNextStep, ForgeQueryOrdinaryOutcome,
    ForgeQueryOrdinaryPosture, ForgeQueryOrdinaryPostureKind,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::bindings::authority::{SpatialBindingCompleteness, SpatialBindingKind};
use crate::bindings::query_native::PrimitiveBindingQueryDomain;
use crate::bindings::query_native_binding_authoring::{
    PrimitiveBindingAuthoringError, PrimitiveBindingDeclarationEntry,
};
use crate::bindings::query_native_binding_projection_payload::PrimitiveBindingProjectionPayload;
use crate::bindings::query_native_target_identity::{
    binding_target_identity_from_envelope, GeometryTargetIdentityFactError,
    GeometryTargetIdentityFactReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveBindingMutationEvidence {
    binding_kind: SpatialBindingKind,
    binding_identity: String,
    site_identity: String,
    target_identity: GeometryTargetIdentityFactReceipt,
    completeness: SpatialBindingCompleteness,
    declaration_digest: String,
    progression_digest: Option<String>,
    route_plan_digest: Option<String>,
    receipt_digest: String,
    envelope_digest: String,
    evidence_digest: String,
}

impl PrimitiveBindingMutationEvidence {
    fn from_bound_envelope(
        projection_payload: &PrimitiveBindingProjectionPayload,
        target_identity: GeometryTargetIdentityFactReceipt,
        envelope: &ForgeQueryDeclarationEnvelope<
            PrimitiveBindingQueryDomain,
            PrimitiveBindingDeclarationEntry,
        >,
    ) -> Result<Self, PrimitiveBindingMutationEvidenceError> {
        let binding_kind = projection_payload.binding_kind();
        let binding_identity = projection_payload.binding_identity().to_string();
        let site_identity = projection_payload.site_identity().to_string();
        let completeness = projection_payload.completeness();
        let declaration_digest = envelope.declaration_digest().to_string();
        let progression_digest = envelope.progression_digest().map(ToOwned::to_owned);
        let route_plan_digest = envelope.route_plan_digest().map(ToOwned::to_owned);
        let receipt_digest = format!("{:?}", envelope.receipt_digest());
        let envelope_digest = format!("{:?}", envelope.envelope_digest());
        let evidence_digest = mutation_evidence_digest(&[
            format!("{binding_kind:?}"),
            binding_identity.clone(),
            site_identity.clone(),
            target_identity.fact_digest().to_string(),
            format!("{completeness:?}"),
            declaration_digest.clone(),
            progression_digest
                .clone()
                .unwrap_or_else(|| "none".to_string()),
            route_plan_digest
                .clone()
                .unwrap_or_else(|| "none".to_string()),
            receipt_digest.clone(),
            envelope_digest.clone(),
        ]);
        Ok(Self {
            binding_kind,
            binding_identity,
            site_identity,
            target_identity,
            completeness,
            declaration_digest,
            progression_digest,
            route_plan_digest,
            receipt_digest,
            envelope_digest,
            evidence_digest,
        })
    }

    pub fn binding_kind(&self) -> SpatialBindingKind {
        self.binding_kind
    }

    pub fn binding_identity(&self) -> &str {
        &self.binding_identity
    }

    pub fn site_identity(&self) -> &str {
        &self.site_identity
    }

    pub fn target_identity(&self) -> &GeometryTargetIdentityFactReceipt {
        &self.target_identity
    }

    pub fn completeness(&self) -> SpatialBindingCompleteness {
        self.completeness
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn progression_digest(&self) -> Option<&str> {
        self.progression_digest.as_deref()
    }

    pub fn route_plan_digest(&self) -> Option<&str> {
        self.route_plan_digest.as_deref()
    }

    pub fn receipt_digest(&self) -> &str {
        &self.receipt_digest
    }

    pub fn envelope_digest(&self) -> &str {
        &self.envelope_digest
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PrimitiveBindingMutationEvidenceError {
    DeclarationDenied(PrimitiveBindingAuthoringError),
    TargetIdentity(GeometryTargetIdentityFactError),
    OutcomeNotBound {
        kind: ForgeQueryOrdinaryPostureKind,
        reason: String,
        next_step: ForgeQueryOrdinaryNextStep,
    },
}

impl PrimitiveBindingMutationEvidenceError {
    fn outcome_not_bound(posture: &ForgeQueryOrdinaryPosture) -> Self {
        Self::OutcomeNotBound {
            kind: posture.kind(),
            reason: posture.reason().to_string(),
            next_step: posture.next_step(),
        }
    }
}

pub fn primitive_binding_mutation_evidence<C>(
    declaration: &PrimitiveBindingDeclarationEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveBindingQueryDomain, C>,
) -> Result<PrimitiveBindingMutationEvidence, PrimitiveBindingMutationEvidenceError>
where
    C: ForgeQueryDomainOperatingContext<PrimitiveBindingQueryDomain>,
{
    match handle.orchestrate_declaration_entry_outcome(declaration.clone()) {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => declaration
            .projection_payload()
            .map_err(PrimitiveBindingMutationEvidenceError::DeclarationDenied)
            .and_then(|projection_payload| {
                declaration
                    .target_identity_payload()
                    .map_err(PrimitiveBindingMutationEvidenceError::DeclarationDenied)
                    .and_then(|target_payload| {
                        binding_target_identity_from_envelope(target_payload, &envelope)
                            .map_err(PrimitiveBindingMutationEvidenceError::TargetIdentity)
                            .and_then(|target_identity| {
                                PrimitiveBindingMutationEvidence::from_bound_envelope(
                                    projection_payload,
                                    target_identity,
                                    &envelope,
                                )
                            })
                    })
            }),
        ForgeQueryOrdinaryOutcome::Ambiguous(posture)
        | ForgeQueryOrdinaryOutcome::AspectConflict(posture)
        | ForgeQueryOrdinaryOutcome::AuthorityMismatch(posture)
        | ForgeQueryOrdinaryOutcome::BasisMismatch(posture)
        | ForgeQueryOrdinaryOutcome::Deferred(posture)
        | ForgeQueryOrdinaryOutcome::Denied(posture)
        | ForgeQueryOrdinaryOutcome::ExplicitNarrowingRequired(posture)
        | ForgeQueryOrdinaryOutcome::Failed(posture)
        | ForgeQueryOrdinaryOutcome::MissingRequiredAspect(posture)
        | ForgeQueryOrdinaryOutcome::RebindRequired(posture)
        | ForgeQueryOrdinaryOutcome::Refused(posture)
        | ForgeQueryOrdinaryOutcome::Stale(posture)
        | ForgeQueryOrdinaryOutcome::Unavailable(posture)
        | ForgeQueryOrdinaryOutcome::Unsupported(posture)
        | ForgeQueryOrdinaryOutcome::WrongHandle(posture)
        | ForgeQueryOrdinaryOutcome::WrongWorld(posture) => Err(
            PrimitiveBindingMutationEvidenceError::outcome_not_bound(&posture),
        ),
    }
}

fn mutation_evidence_digest(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, parts)
}
