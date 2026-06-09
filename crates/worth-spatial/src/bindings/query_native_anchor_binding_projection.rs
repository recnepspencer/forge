use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationEnvelope,
    ForgeQueryDomainOperatingContext, ForgeQueryOrdinaryNextStep, ForgeQueryOrdinaryOutcome,
    ForgeQueryOrdinaryPosture, ForgeQueryOrdinaryPostureKind,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::bindings::authority::{SpatialBindingCompleteness, SpatialBindingKind};
use crate::bindings::query_native::PrimitiveAnchorBindingQueryDomain;
use crate::bindings::query_native_anchor_binding_authoring::{
    PrimitiveAnchorBindingAuthoringError, PrimitiveAnchorBindingDeclarationEntry,
};
use crate::bindings::query_native_binding_projection_payload::PrimitiveAnchorBindingProjectionPayload;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveAnchorBindingFactReadSurface {
    ProjectionConsumptionFromDeclarationEnvelope,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveAnchorBindingFactProvenance {
    DeclarationEnvelopeBackedProjectionConsumption,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveAnchorBindingProjectionFactReceipt {
    binding_kind: SpatialBindingKind,
    binding_identity: String,
    site_identity: String,
    completeness: SpatialBindingCompleteness,
    read_surface: PrimitiveAnchorBindingFactReadSurface,
    fact_provenance: PrimitiveAnchorBindingFactProvenance,
    declaration_digest: String,
    progression_digest: Option<String>,
    route_plan_digest: Option<String>,
    receipt_digest: String,
    envelope_digest: String,
    fact_digest: String,
}

impl PrimitiveAnchorBindingProjectionFactReceipt {
    fn from_bound_envelope(
        payload: &PrimitiveAnchorBindingProjectionPayload,
        envelope: &ForgeQueryDeclarationEnvelope<
            PrimitiveAnchorBindingQueryDomain,
            PrimitiveAnchorBindingDeclarationEntry,
        >,
    ) -> Result<Self, PrimitiveAnchorBindingProjectionFactError> {
        let read_surface =
            PrimitiveAnchorBindingFactReadSurface::ProjectionConsumptionFromDeclarationEnvelope;
        let fact_provenance =
            PrimitiveAnchorBindingFactProvenance::DeclarationEnvelopeBackedProjectionConsumption;
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

    pub fn read_surface(&self) -> PrimitiveAnchorBindingFactReadSurface {
        self.read_surface
    }

    pub fn fact_provenance(&self) -> PrimitiveAnchorBindingFactProvenance {
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
pub enum PrimitiveAnchorBindingProjectionFactError {
    DeclarationDenied(PrimitiveAnchorBindingAuthoringError),
    OutcomeNotBound {
        kind: ForgeQueryOrdinaryPostureKind,
        reason: String,
        next_step: ForgeQueryOrdinaryNextStep,
    },
}

impl PrimitiveAnchorBindingProjectionFactError {
    fn outcome_not_bound(posture: &ForgeQueryOrdinaryPosture) -> Self {
        Self::OutcomeNotBound {
            kind: posture.kind(),
            reason: posture.reason().to_string(),
            next_step: posture.next_step(),
        }
    }
}

pub fn primitive_anchor_binding_projection_facts<C>(
    declaration: &PrimitiveAnchorBindingDeclarationEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveAnchorBindingQueryDomain, C>,
) -> Result<PrimitiveAnchorBindingProjectionFactReceipt, PrimitiveAnchorBindingProjectionFactError>
where
    C: ForgeQueryDomainOperatingContext<PrimitiveAnchorBindingQueryDomain>,
{
    match handle.orchestrate_declaration_entry_outcome(declaration.clone()) {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => declaration
            .projection_payload()
            .map_err(PrimitiveAnchorBindingProjectionFactError::DeclarationDenied)
            .and_then(|payload| {
                PrimitiveAnchorBindingProjectionFactReceipt::from_bound_envelope(payload, &envelope)
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
        | ForgeQueryOrdinaryOutcome::WrongWorld(posture) => {
            Err(PrimitiveAnchorBindingProjectionFactError::outcome_not_bound(&posture))
        }
    }
}

fn projection_fact_digest(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, parts)
}
