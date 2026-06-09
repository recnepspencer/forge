use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationEnvelope,
    ForgeQueryDomainOperatingContext, ForgeQueryOrdinaryNextStep, ForgeQueryOrdinaryOutcome,
    ForgeQueryOrdinaryPosture, ForgeQueryOrdinaryPostureKind,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::bindings::authority::SpatialBindingKind;
use crate::bindings::query_native_rebinding::PrimitiveRebindingQueryDomain;
use crate::bindings::query_native_rebinding_authoring::{
    PrimitiveRebindingAuthoringError, PrimitiveRebindingDeclarationEntry,
};
use crate::bindings::rebinding::{
    BindingContinuityClass, MotionAwareBindingPosture, NeighborhoodBindingFamily,
    PrimitiveRebindingFactReceipt, PrimitiveRebindingRetainedFactSource, RebindingOutcomeClass,
    UnsupportedRebindingReason,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveRebindingFactReadSurface {
    ProjectionConsumptionFromDeclarationEnvelope,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveRebindingFactProvenance {
    DeclarationEnvelopeBackedProjectionConsumption,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveRebindingProjectionFactReceipt {
    prior_binding_identity: String,
    prior_site_identity: String,
    selected_candidate_identity: Option<String>,
    selected_candidate_label: Option<String>,
    candidate_identities: Vec<String>,
    candidate_labels: Vec<String>,
    candidate_site_identities: Vec<String>,
    continuity_class: BindingContinuityClass,
    motion_posture: MotionAwareBindingPosture,
    neighborhood_family: NeighborhoodBindingFamily,
    outcome_class: RebindingOutcomeClass,
    unsupported_reason: Option<UnsupportedRebindingReason>,
    read_surface: PrimitiveRebindingFactReadSurface,
    fact_provenance: PrimitiveRebindingFactProvenance,
    declaration_digest: String,
    progression_digest: Option<String>,
    route_plan_digest: Option<String>,
    receipt_digest: String,
    envelope_digest: String,
    fact_digest: String,
}

impl PrimitiveRebindingProjectionFactReceipt {
    fn from_bound_envelope(
        receipt: &PrimitiveRebindingFactReceipt,
        envelope: &ForgeQueryDeclarationEnvelope<
            PrimitiveRebindingQueryDomain,
            PrimitiveRebindingDeclarationEntry,
        >,
    ) -> Self {
        let read_surface =
            PrimitiveRebindingFactReadSurface::ProjectionConsumptionFromDeclarationEnvelope;
        let fact_provenance =
            PrimitiveRebindingFactProvenance::DeclarationEnvelopeBackedProjectionConsumption;
        let declaration_digest = envelope.declaration_digest().to_string();
        let progression_digest = envelope.progression_digest().map(ToOwned::to_owned);
        let route_plan_digest = envelope.route_plan_digest().map(ToOwned::to_owned);
        let receipt_digest = format!("{:?}", envelope.receipt_digest());
        let envelope_digest = format!("{:?}", envelope.envelope_digest());
        let fact_digest = projection_fact_digest(&[
            receipt.prior_binding_identity().to_string(),
            receipt.prior_site_identity().to_string(),
            receipt
                .selected_candidate_identity()
                .unwrap_or("none")
                .to_string(),
            receipt
                .selected_candidate_label()
                .unwrap_or("none")
                .to_string(),
            format!("{:?}", receipt.candidate_identities()),
            format!("{:?}", receipt.candidate_labels()),
            format!("{:?}", receipt.candidate_site_identities()),
            format!("{:?}", receipt.continuity_class()),
            format!("{:?}", receipt.motion_posture()),
            receipt
                .neighborhood_family()
                .rebinding_kind_label()
                .to_string(),
            format!("{:?}", receipt.outcome_class()),
            format!("{:?}", receipt.unsupported_reason()),
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
            prior_binding_identity: receipt.prior_binding_identity().to_string(),
            prior_site_identity: receipt.prior_site_identity().to_string(),
            selected_candidate_identity: receipt.selected_candidate_identity().map(str::to_string),
            selected_candidate_label: receipt.selected_candidate_label().map(str::to_string),
            candidate_identities: receipt.candidate_identities().to_vec(),
            candidate_labels: receipt.candidate_labels().to_vec(),
            candidate_site_identities: receipt.candidate_site_identities().to_vec(),
            continuity_class: receipt.continuity_class(),
            motion_posture: receipt.motion_posture(),
            neighborhood_family: receipt.neighborhood_family(),
            outcome_class: receipt.outcome_class(),
            unsupported_reason: receipt.unsupported_reason(),
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

    pub fn prior_binding_identity(&self) -> &str {
        &self.prior_binding_identity
    }

    pub fn prior_site_identity(&self) -> &str {
        &self.prior_site_identity
    }

    pub fn selected_candidate_identity(&self) -> Option<&str> {
        self.selected_candidate_identity.as_deref()
    }

    pub fn selected_candidate_label(&self) -> Option<&str> {
        self.selected_candidate_label.as_deref()
    }

    pub fn candidate_identities(&self) -> &[String] {
        &self.candidate_identities
    }

    pub fn candidate_labels(&self) -> &[String] {
        &self.candidate_labels
    }

    pub fn candidate_site_identities(&self) -> &[String] {
        &self.candidate_site_identities
    }

    pub fn continuity_class(&self) -> BindingContinuityClass {
        self.continuity_class
    }

    pub fn motion_posture(&self) -> MotionAwareBindingPosture {
        self.motion_posture.clone()
    }

    pub fn neighborhood_family(&self) -> NeighborhoodBindingFamily {
        self.neighborhood_family
    }

    pub fn binding_kind(&self) -> SpatialBindingKind {
        binding_kind_from_neighborhood_family(self.neighborhood_family)
    }

    pub fn outcome_class(&self) -> RebindingOutcomeClass {
        self.outcome_class
    }

    pub fn unsupported_reason(&self) -> Option<UnsupportedRebindingReason> {
        self.unsupported_reason
    }

    pub fn read_surface(&self) -> PrimitiveRebindingFactReadSurface {
        self.read_surface
    }

    pub fn fact_provenance(&self) -> PrimitiveRebindingFactProvenance {
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
pub enum PrimitiveRebindingProjectionFactError {
    DeclarationDenied(PrimitiveRebindingAuthoringError),
    OutcomeNotBound {
        kind: ForgeQueryOrdinaryPostureKind,
        reason: String,
        next_step: ForgeQueryOrdinaryNextStep,
    },
}

impl PrimitiveRebindingProjectionFactError {
    fn outcome_not_bound(posture: &ForgeQueryOrdinaryPosture) -> Self {
        Self::OutcomeNotBound {
            kind: posture.kind(),
            reason: posture.reason().to_string(),
            next_step: posture.next_step(),
        }
    }
}

pub fn primitive_rebinding_projection_facts<C>(
    declaration: &PrimitiveRebindingDeclarationEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveRebindingQueryDomain, C>,
) -> Result<PrimitiveRebindingProjectionFactReceipt, PrimitiveRebindingProjectionFactError>
where
    C: ForgeQueryDomainOperatingContext<PrimitiveRebindingQueryDomain>,
{
    match handle.orchestrate_declaration_entry_outcome(declaration.clone()) {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => {
            let receipt = declaration
                .projection_receipt()
                .map_err(PrimitiveRebindingProjectionFactError::DeclarationDenied)?;
            Ok(PrimitiveRebindingProjectionFactReceipt::from_bound_envelope(&receipt, &envelope))
        }
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
            PrimitiveRebindingProjectionFactError::outcome_not_bound(&posture),
        ),
    }
}

pub fn primitive_rebinding_retained_fact_source<C>(
    declaration: &PrimitiveRebindingDeclarationEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveRebindingQueryDomain, C>,
) -> Result<PrimitiveRebindingRetainedFactSource, PrimitiveRebindingProjectionFactError>
where
    C: ForgeQueryDomainOperatingContext<PrimitiveRebindingQueryDomain>,
{
    primitive_rebinding_projection_facts(declaration, handle)
        .map(PrimitiveRebindingRetainedFactSource::from)
}

fn projection_fact_digest(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, parts)
}

fn binding_kind_from_neighborhood_family(family: NeighborhoodBindingFamily) -> SpatialBindingKind {
    match family {
        NeighborhoodBindingFamily::FaceSurface
        | NeighborhoodBindingFamily::FaceSurfacePointAnchor
        | NeighborhoodBindingFamily::FaceSurfaceDirectionAnchor => SpatialBindingKind::FaceSurface,
        NeighborhoodBindingFamily::EdgeCurve
        | NeighborhoodBindingFamily::EdgeCurvePointAnchor
        | NeighborhoodBindingFamily::EdgeCurveDirectionAnchor => SpatialBindingKind::EdgeCurve,
        NeighborhoodBindingFamily::CoedgePCurve
        | NeighborhoodBindingFamily::CoedgePCurvePointAnchor
        | NeighborhoodBindingFamily::CoedgePCurveDirectionAnchor => {
            SpatialBindingKind::CoedgePCurve
        }
        NeighborhoodBindingFamily::VertexGeometry => SpatialBindingKind::VertexGeometry,
    }
}
