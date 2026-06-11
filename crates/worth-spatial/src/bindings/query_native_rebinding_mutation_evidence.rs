use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext,
    ForgeQueryOrdinaryNextStep, ForgeQueryOrdinaryPostureKind,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::bindings::query_native_rebinding::PrimitiveRebindingQueryDomain;
use crate::bindings::query_native_rebinding_authoring::{
    PrimitiveRebindingAuthoringError, PrimitiveRebindingDeclarationEntry,
};
use crate::bindings::query_native_rebinding_neighborhood_replacement::{
    primitive_rebinding_neighborhood_replacement_facts,
    primitive_rebinding_neighborhood_replacement_source, topology_neighborhood_replacement_entry,
    TopologyNeighborhoodReplacementFactReceipt,
};
use crate::bindings::query_native_rebinding_projection::{
    primitive_rebinding_projection_facts, PrimitiveRebindingProjectionFactError,
    PrimitiveRebindingProjectionFactReceipt,
};
use crate::bindings::rebinding::{
    BindingContinuityClass, MotionAwareBindingPosture, NeighborhoodBindingFamily,
    RebindingOutcomeClass,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveRebindingMutationEvidence {
    prior_binding_identity: String,
    prior_site_identity: String,
    neighborhood_family: NeighborhoodBindingFamily,
    outcome_class: RebindingOutcomeClass,
    continuity_class: BindingContinuityClass,
    motion_posture: MotionAwareBindingPosture,
    selected_candidate_identity: Option<String>,
    selected_candidate_label: Option<String>,
    selected_candidate_site_identity: Option<String>,
    neighborhood_replacement: TopologyNeighborhoodReplacementFactReceipt,
    declaration_digest: String,
    progression_digest: Option<String>,
    route_plan_digest: Option<String>,
    receipt_digest: String,
    envelope_digest: String,
    evidence_digest: String,
}

impl PrimitiveRebindingMutationEvidence {
    fn from_source(source: PrimitiveRebindingMutationEvidenceSource) -> Self {
        let PrimitiveRebindingMutationEvidenceSource {
            projection,
            neighborhood_replacement,
        } = source;
        let prior_binding_identity = projection.prior_binding_identity().to_string();
        let prior_site_identity = projection.prior_site_identity().to_string();
        let neighborhood_family = projection.neighborhood_family();
        let outcome_class = projection.outcome_class();
        let continuity_class = projection.continuity_class();
        let motion_posture = projection.motion_posture();
        let selected_candidate_identity =
            projection.selected_candidate_identity().map(str::to_string);
        let selected_candidate_label = projection.selected_candidate_label().map(str::to_string);
        let selected_candidate_site_identity =
            selected_candidate_site_identity(&projection).map(str::to_string);
        let declaration_digest = projection.declaration_digest().to_string();
        let progression_digest = projection.progression_digest().map(ToOwned::to_owned);
        let route_plan_digest = projection.route_plan_digest().map(ToOwned::to_owned);
        let receipt_digest = projection.receipt_digest().to_string();
        let envelope_digest = projection.envelope_digest().to_string();
        let evidence_digest = rebinding_mutation_evidence_digest(&[
            prior_binding_identity.clone(),
            prior_site_identity.clone(),
            neighborhood_family.rebinding_kind_label().to_string(),
            format!("{outcome_class:?}"),
            format!("{continuity_class:?}"),
            format!("{motion_posture:?}"),
            selected_candidate_identity
                .clone()
                .unwrap_or_else(|| "none".to_string()),
            selected_candidate_label
                .clone()
                .unwrap_or_else(|| "none".to_string()),
            selected_candidate_site_identity
                .clone()
                .unwrap_or_else(|| "none".to_string()),
            neighborhood_replacement.fact_digest().to_string(),
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
            prior_binding_identity,
            prior_site_identity,
            neighborhood_family,
            outcome_class,
            continuity_class,
            motion_posture,
            selected_candidate_identity,
            selected_candidate_label,
            selected_candidate_site_identity,
            neighborhood_replacement,
            declaration_digest,
            progression_digest,
            route_plan_digest,
            receipt_digest,
            envelope_digest,
            evidence_digest,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PrimitiveRebindingMutationEvidenceSource {
    projection: PrimitiveRebindingProjectionFactReceipt,
    neighborhood_replacement: TopologyNeighborhoodReplacementFactReceipt,
}

impl PrimitiveRebindingMutationEvidence {
    fn from_projection_facts(
        projection: &PrimitiveRebindingProjectionFactReceipt,
        neighborhood_replacement: TopologyNeighborhoodReplacementFactReceipt,
    ) -> Self {
        Self::from_source(PrimitiveRebindingMutationEvidenceSource {
            projection: projection.clone(),
            neighborhood_replacement,
        })
    }

    pub fn prior_binding_identity(&self) -> &str {
        &self.prior_binding_identity
    }

    pub fn prior_site_identity(&self) -> &str {
        &self.prior_site_identity
    }

    pub fn neighborhood_family(&self) -> NeighborhoodBindingFamily {
        self.neighborhood_family
    }

    pub fn outcome_class(&self) -> RebindingOutcomeClass {
        self.outcome_class
    }

    pub fn continuity_class(&self) -> BindingContinuityClass {
        self.continuity_class
    }

    pub fn motion_posture(&self) -> MotionAwareBindingPosture {
        self.motion_posture.clone()
    }

    pub fn selected_candidate_identity(&self) -> Option<&str> {
        self.selected_candidate_identity.as_deref()
    }

    pub fn selected_candidate_label(&self) -> Option<&str> {
        self.selected_candidate_label.as_deref()
    }

    pub fn selected_candidate_site_identity(&self) -> Option<&str> {
        self.selected_candidate_site_identity.as_deref()
    }

    pub fn neighborhood_replacement(&self) -> &TopologyNeighborhoodReplacementFactReceipt {
        &self.neighborhood_replacement
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
pub enum PrimitiveRebindingMutationEvidenceError {
    DeclarationDenied(PrimitiveRebindingAuthoringError),
    OutcomeNotBound {
        kind: ForgeQueryOrdinaryPostureKind,
        reason: String,
        next_step: ForgeQueryOrdinaryNextStep,
    },
}

impl PrimitiveRebindingMutationEvidenceError {
    fn from_projection_error(error: PrimitiveRebindingProjectionFactError) -> Self {
        match error {
            PrimitiveRebindingProjectionFactError::DeclarationDenied(inner) => {
                Self::DeclarationDenied(inner)
            }
            PrimitiveRebindingProjectionFactError::OutcomeNotBound {
                kind,
                reason,
                next_step,
            } => Self::OutcomeNotBound {
                kind,
                reason,
                next_step,
            },
        }
    }
}

pub fn primitive_rebinding_mutation_evidence<C>(
    declaration: &PrimitiveRebindingDeclarationEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveRebindingQueryDomain, C>,
) -> Result<PrimitiveRebindingMutationEvidence, PrimitiveRebindingMutationEvidenceError>
where
    C: ForgeQueryDomainOperatingContext<PrimitiveRebindingQueryDomain>,
{
    let projection = primitive_rebinding_projection_facts(declaration, handle)
        .map_err(PrimitiveRebindingMutationEvidenceError::from_projection_error)?;
    let replacement_source =
        primitive_rebinding_neighborhood_replacement_source(declaration, handle)
            .map_err(PrimitiveRebindingMutationEvidenceError::from_projection_error)?;
    let replacement_entry = topology_neighborhood_replacement_entry(replacement_source.clone());
    let neighborhood_replacement = primitive_rebinding_neighborhood_replacement_facts(
        &replacement_entry,
        handle,
    )
    .map_err(|error| match error {
        crate::bindings::query_native_rebinding_neighborhood_replacement::TopologyNeighborhoodReplacementFactError::DeclarationDenied(
            inner,
        ) => PrimitiveRebindingMutationEvidenceError::DeclarationDenied(inner),
        crate::bindings::query_native_rebinding_neighborhood_replacement::TopologyNeighborhoodReplacementFactError::OutcomeNotBound {
            kind,
            reason,
            next_step,
        } => PrimitiveRebindingMutationEvidenceError::OutcomeNotBound {
            kind,
            reason,
            next_step,
        },
    })?;
    Ok(PrimitiveRebindingMutationEvidence::from_projection_facts(
        &projection,
        neighborhood_replacement,
    ))
}

fn selected_candidate_site_identity(
    projection: &PrimitiveRebindingProjectionFactReceipt,
) -> Option<&str> {
    let selected_identity = projection.selected_candidate_identity()?;
    projection
        .candidate_identities()
        .iter()
        .zip(projection.candidate_site_identities().iter())
        .find(|(candidate_identity, _)| candidate_identity.as_str() == selected_identity)
        .map(|(_, site_identity)| site_identity.as_str())
}

fn rebinding_mutation_evidence_digest(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, parts)
}
