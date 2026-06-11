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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveBindingFactReadSurface {
    ProjectionConsumptionFromDeclarationEnvelope,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveBindingFactProvenance {
    DeclarationEnvelopeBackedProjectionConsumption,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveBindingProjectionFactReceipt {
    binding_kind: SpatialBindingKind,
    binding_identity: String,
    site_identity: String,
    completeness: SpatialBindingCompleteness,
    read_surface: PrimitiveBindingFactReadSurface,
    fact_provenance: PrimitiveBindingFactProvenance,
    declaration_digest: String,
    progression_digest: Option<String>,
    route_plan_digest: Option<String>,
    receipt_digest: String,
    envelope_digest: String,
    fact_digest: String,
}

impl PrimitiveBindingProjectionFactReceipt {
    fn from_bound_envelope(
        payload: &PrimitiveBindingProjectionPayload,
        envelope: &ForgeQueryDeclarationEnvelope<
            PrimitiveBindingQueryDomain,
            PrimitiveBindingDeclarationEntry,
        >,
    ) -> Result<Self, PrimitiveBindingProjectionFactError> {
        let read_surface =
            PrimitiveBindingFactReadSurface::ProjectionConsumptionFromDeclarationEnvelope;
        let fact_provenance =
            PrimitiveBindingFactProvenance::DeclarationEnvelopeBackedProjectionConsumption;
        let binding_kind = payload.binding_kind();
        let binding_identity = payload.binding_identity().to_string();
        let site_identity = payload.site_identity().to_string();
        let completeness = payload.completeness();
        let declaration_digest = envelope.declaration_digest().to_string();
        let progression_digest = envelope.progression_digest().map(ToOwned::to_owned);
        let route_plan_digest = envelope.route_plan_digest().map(ToOwned::to_owned);
        let receipt_digest = format!("{:?}", envelope.receipt_digest());
        let envelope_digest = format!("{:?}", envelope.envelope_digest());
        let fact_digest = projection_fact_digest(&[
            format!("{binding_kind:?}"),
            binding_identity.clone(),
            site_identity.clone(),
            format!("{completeness:?}"),
            format!("{read_surface:?}"),
            format!("{fact_provenance:?}"),
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
            completeness,
            read_surface,
            fact_provenance,
            declaration_digest,
            progression_digest,
            route_plan_digest,
            receipt_digest,
            envelope_digest,
            fact_digest,
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

    pub fn completeness(&self) -> SpatialBindingCompleteness {
        self.completeness
    }

    pub fn read_surface(&self) -> PrimitiveBindingFactReadSurface {
        self.read_surface
    }

    pub fn fact_provenance(&self) -> PrimitiveBindingFactProvenance {
        self.fact_provenance
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

    pub fn fact_digest(&self) -> &str {
        &self.fact_digest
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PrimitiveBindingProjectionFactError {
    DeclarationDenied(PrimitiveBindingAuthoringError),
    OutcomeNotBound {
        kind: ForgeQueryOrdinaryPostureKind,
        reason: String,
        next_step: ForgeQueryOrdinaryNextStep,
    },
}

impl PrimitiveBindingProjectionFactError {
    fn outcome_not_bound(posture: &ForgeQueryOrdinaryPosture) -> Self {
        Self::OutcomeNotBound {
            kind: posture.kind(),
            reason: posture.reason().to_string(),
            next_step: posture.next_step(),
        }
    }
}

pub fn primitive_binding_projection_facts<C>(
    declaration: &PrimitiveBindingDeclarationEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveBindingQueryDomain, C>,
) -> Result<PrimitiveBindingProjectionFactReceipt, PrimitiveBindingProjectionFactError>
where
    C: ForgeQueryDomainOperatingContext<PrimitiveBindingQueryDomain>,
{
    match handle.orchestrate_declaration_entry_outcome(declaration.clone()) {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => declaration
            .projection_payload()
            .map_err(PrimitiveBindingProjectionFactError::DeclarationDenied)
            .and_then(|payload| {
                PrimitiveBindingProjectionFactReceipt::from_bound_envelope(payload, &envelope)
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
            PrimitiveBindingProjectionFactError::outcome_not_bound(&posture),
        ),
    }
}

fn projection_fact_digest(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, parts)
}
