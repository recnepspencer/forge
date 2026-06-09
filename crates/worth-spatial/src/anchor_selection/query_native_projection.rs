use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationEnvelope,
    ForgeQueryDomainOperatingContext, ForgeQueryOrdinaryNextStep, ForgeQueryOrdinaryOutcome,
    ForgeQueryOrdinaryPosture, ForgeQueryOrdinaryPostureKind,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::anchor_selection::query_native::SpatialAnchorSelectionQueryDomain;
use crate::anchor_selection::query_native_authoring::{
    SpatialAnchorSelectionDeclarationEntry, SpatialAnchorSelectionFailureKind,
    SpatialAnchorSelectionKind, SpatialAnchorSelectionProjectionSeed,
    SpatialAnchorSelectionRequestedInput, SpatialAnchorSelectionStatus,
    SpatialResolvedAnchorWitness,
};
use crate::witness_resolution::SpatialWitnessResolutionClass;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialAnchorSelectionFactReadSurface {
    ProjectionConsumptionFromDeclarationEnvelope,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialAnchorSelectionFactProvenance {
    DeclarationEnvelopeBackedProjectionConsumption,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SpatialAnchorSelectionProjectionFactReceipt {
    kind: SpatialAnchorSelectionKind,
    anchor: String,
    requested_input: SpatialAnchorSelectionRequestedInput,
    status: SpatialAnchorSelectionStatus,
    resolved_witness: Option<SpatialResolvedAnchorWitness>,
    resolution_class: Option<SpatialWitnessResolutionClass>,
    failure_kind: Option<SpatialAnchorSelectionFailureKind>,
    read_surface: SpatialAnchorSelectionFactReadSurface,
    fact_provenance: SpatialAnchorSelectionFactProvenance,
    declaration_digest: String,
    progression_digest: Option<String>,
    route_plan_digest: Option<String>,
    receipt_digest: String,
    envelope_digest: String,
    fact_digest: String,
}

impl SpatialAnchorSelectionProjectionFactReceipt {
    fn from_bound_envelope(
        projection_seed: &SpatialAnchorSelectionProjectionSeed,
        envelope: &ForgeQueryDeclarationEnvelope<
            SpatialAnchorSelectionQueryDomain,
            SpatialAnchorSelectionDeclarationEntry,
        >,
    ) -> Self {
        let read_surface =
            SpatialAnchorSelectionFactReadSurface::ProjectionConsumptionFromDeclarationEnvelope;
        let fact_provenance =
            SpatialAnchorSelectionFactProvenance::DeclarationEnvelopeBackedProjectionConsumption;
        let declaration_digest = envelope.declaration_digest().to_string();
        let progression_digest = envelope.progression_digest().map(ToOwned::to_owned);
        let route_plan_digest = envelope.route_plan_digest().map(ToOwned::to_owned);
        let receipt_digest = format!("{:?}", envelope.receipt_digest());
        let envelope_digest = format!("{:?}", envelope.envelope_digest());
        let fact_digest = projection_fact_digest(&[
            format!("{:?}", projection_seed.kind()),
            projection_seed.anchor().to_string(),
            format!("{:?}", projection_seed.requested_input()),
            format!("{:?}", projection_seed.status()),
            format!("{:?}", projection_seed.resolved_witness()),
            format!("{:?}", projection_seed.resolution_class()),
            format!("{:?}", projection_seed.failure_kind()),
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
        Self {
            kind: projection_seed.kind(),
            anchor: projection_seed.anchor().to_string(),
            requested_input: projection_seed.requested_input().clone(),
            status: projection_seed.status(),
            resolved_witness: projection_seed.resolved_witness(),
            resolution_class: projection_seed.resolution_class(),
            failure_kind: projection_seed.failure_kind(),
            read_surface,
            fact_provenance,
            declaration_digest,
            progression_digest,
            route_plan_digest,
            receipt_digest,
            envelope_digest,
            fact_digest,
        }
    }

    pub fn kind(&self) -> SpatialAnchorSelectionKind {
        self.kind
    }

    pub fn anchor(&self) -> &str {
        &self.anchor
    }

    pub fn requested_input(&self) -> &SpatialAnchorSelectionRequestedInput {
        &self.requested_input
    }

    pub fn status(&self) -> SpatialAnchorSelectionStatus {
        self.status
    }

    pub fn resolved_witness(&self) -> Option<SpatialResolvedAnchorWitness> {
        self.resolved_witness
    }

    pub fn resolution_class(&self) -> Option<SpatialWitnessResolutionClass> {
        self.resolution_class
    }

    pub fn failure_kind(&self) -> Option<SpatialAnchorSelectionFailureKind> {
        self.failure_kind
    }

    pub fn read_surface(&self) -> SpatialAnchorSelectionFactReadSurface {
        self.read_surface
    }

    pub fn fact_provenance(&self) -> SpatialAnchorSelectionFactProvenance {
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
pub enum SpatialAnchorSelectionProjectionFactError {
    OutcomeNotBound {
        kind: ForgeQueryOrdinaryPostureKind,
        reason: String,
        next_step: ForgeQueryOrdinaryNextStep,
    },
}

impl SpatialAnchorSelectionProjectionFactError {
    fn outcome_not_bound(posture: &ForgeQueryOrdinaryPosture) -> Self {
        Self::OutcomeNotBound {
            kind: posture.kind(),
            reason: posture.reason().to_string(),
            next_step: posture.next_step(),
        }
    }
}

pub fn spatial_anchor_selection_projection_facts<C>(
    declaration: &SpatialAnchorSelectionDeclarationEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<SpatialAnchorSelectionQueryDomain, C>,
) -> Result<SpatialAnchorSelectionProjectionFactReceipt, SpatialAnchorSelectionProjectionFactError>
where
    C: ForgeQueryDomainOperatingContext<SpatialAnchorSelectionQueryDomain>,
{
    match handle.orchestrate_declaration_entry_outcome(declaration.clone()) {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => Ok(
            SpatialAnchorSelectionProjectionFactReceipt::from_bound_envelope(
                declaration.projection_seed(),
                &envelope,
            ),
        ),
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
            Err(SpatialAnchorSelectionProjectionFactError::outcome_not_bound(&posture))
        }
    }
}

fn projection_fact_digest(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, parts)
}
