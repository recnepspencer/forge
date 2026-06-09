use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDomainOperatingContext,
    ForgeQueryNeighborhoodCapableGrouping, ForgeQueryOrdinaryNextStep, ForgeQueryOrdinaryOutcome,
    ForgeQueryOrdinaryPostureKind, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalNotCompatiblePosture,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::bindings::query_native_rebinding::PrimitiveRebindingQueryDomain;
use crate::bindings::query_native_rebinding_authoring::{
    PrimitiveRebindingAuthoringError, PrimitiveRebindingDeclarationEntry,
};
use crate::bindings::query_native_rebinding_projection::{
    primitive_rebinding_retained_fact_source, PrimitiveRebindingProjectionFactError,
};
use crate::bindings::query_native_retained_geometry::retained_source_digest;
use crate::bindings::rebinding::PrimitiveRebindingRetainedFactSource;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveRebindingNeighborhoodReplacementSource {
    retained_source: PrimitiveRebindingRetainedFactSource,
    neighborhood_family: &'static str,
    prior_binding_identity: String,
    prior_site_identity: String,
    affected_target_identities: Vec<String>,
    candidate_frontier: Vec<String>,
    candidate_labels: Vec<String>,
}

impl PrimitiveRebindingNeighborhoodReplacementSource {
    pub fn retained_source(&self) -> &PrimitiveRebindingRetainedFactSource {
        &self.retained_source
    }

    pub fn neighborhood_family(&self) -> &'static str {
        self.neighborhood_family
    }

    pub fn prior_binding_identity(&self) -> &str {
        &self.prior_binding_identity
    }

    pub fn prior_site_identity(&self) -> &str {
        &self.prior_site_identity
    }

    pub fn affected_target_identities(&self) -> &[String] {
        &self.affected_target_identities
    }

    pub fn candidate_frontier(&self) -> &[String] {
        &self.candidate_frontier
    }

    pub fn candidate_labels(&self) -> &[String] {
        &self.candidate_labels
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TopologyNeighborhoodReplacementDeclarationFamily;

impl ForgeQueryDeclarationFamilyMarker<PrimitiveRebindingQueryDomain>
    for TopologyNeighborhoodReplacementDeclarationFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "TopologyNeighborhoodReplacement"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &[
                "geometry.neighborhood.source_family",
                "geometry.neighborhood.prior_binding_identity",
                "geometry.neighborhood.candidate_frontier",
            ],
            &["geometry.neighborhood.replacement"],
            &[],
            &[],
            &[],
        )
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::relational_only()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TopologyNeighborhoodReplacementEntry {
    source: PrimitiveRebindingNeighborhoodReplacementSource,
    receipt: TopologyNeighborhoodReplacementFactReceipt,
}

impl TopologyNeighborhoodReplacementEntry {
    pub fn source(&self) -> &PrimitiveRebindingNeighborhoodReplacementSource {
        &self.source
    }

    fn receipt(&self) -> &TopologyNeighborhoodReplacementFactReceipt {
        &self.receipt
    }
}

impl ForgeQueryDeclarationInput<PrimitiveRebindingQueryDomain>
    for TopologyNeighborhoodReplacementEntry
{
    type Family = TopologyNeighborhoodReplacementDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            ForgeQueryDeclarationCanonicalEntry::text(
                "geometry.neighborhood.kind",
                "topology_neighborhood_replacement",
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "geometry.neighborhood.source_family",
                self.source.neighborhood_family(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "geometry.neighborhood.prior_binding_identity",
                self.source.prior_binding_identity(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "geometry.neighborhood.candidate_frontier",
                self.source.candidate_frontier().join("|"),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "geometry.neighborhood.source_receipt_digest",
                retained_source_digest(self.source.retained_source()),
            ),
        ]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyNeighborhoodReplacementScope {
    LocalNeighborhood,
}

impl TopologyNeighborhoodReplacementScope {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LocalNeighborhood => "local_neighborhood",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyNeighborhoodReplacementFactReceipt {
    replacement_neighborhood_identity: String,
    replacement_scope: TopologyNeighborhoodReplacementScope,
    affected_target_identities: Vec<String>,
    existing_target_identity_basis: String,
    structural_correspondence_frontier: Option<Vec<String>>,
    fact_digest: String,
}

impl TopologyNeighborhoodReplacementFactReceipt {
    fn from_source(source: &PrimitiveRebindingNeighborhoodReplacementSource) -> Self {
        let replacement_scope = TopologyNeighborhoodReplacementScope::LocalNeighborhood;
        let affected_target_identities = source.affected_target_identities().to_vec();
        let structural_correspondence_frontier = if source.candidate_frontier().len() > 1 {
            Some(source.candidate_frontier().to_vec())
        } else {
            None
        };
        let existing_target_identity_basis = source.prior_binding_identity().to_string();
        let replacement_neighborhood_identity = digest_parts(&[
            source.neighborhood_family().to_string(),
            source.prior_site_identity().to_string(),
            existing_target_identity_basis.clone(),
            format!("{affected_target_identities:?}"),
            source.candidate_labels().join("|"),
        ]);
        let fact_digest = digest_parts(&[
            replacement_neighborhood_identity.clone(),
            replacement_scope.as_str().to_string(),
            format!("{affected_target_identities:?}"),
            existing_target_identity_basis.clone(),
            structural_correspondence_frontier
                .as_ref()
                .map(|frontier| format!("{frontier:?}"))
                .unwrap_or_else(|| "none".to_string()),
        ]);
        Self {
            replacement_neighborhood_identity,
            replacement_scope,
            affected_target_identities,
            existing_target_identity_basis,
            structural_correspondence_frontier,
            fact_digest,
        }
    }

    pub fn replacement_neighborhood_identity(&self) -> &str {
        &self.replacement_neighborhood_identity
    }

    pub fn replacement_scope(&self) -> TopologyNeighborhoodReplacementScope {
        self.replacement_scope
    }

    pub fn affected_target_identities(&self) -> &[String] {
        &self.affected_target_identities
    }

    pub fn existing_target_identity_basis(&self) -> &str {
        &self.existing_target_identity_basis
    }

    pub fn structural_correspondence_frontier(&self) -> Option<&[String]> {
        self.structural_correspondence_frontier.as_deref()
    }

    pub fn fact_digest(&self) -> &str {
        &self.fact_digest
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TopologyNeighborhoodReplacementFactError {
    DeclarationDenied(PrimitiveRebindingAuthoringError),
    OutcomeNotBound {
        kind: ForgeQueryOrdinaryPostureKind,
        reason: String,
        next_step: ForgeQueryOrdinaryNextStep,
    },
}

impl TopologyNeighborhoodReplacementFactError {
    fn outcome_not_bound(
        kind: ForgeQueryOrdinaryPostureKind,
        reason: String,
        next_step: ForgeQueryOrdinaryNextStep,
    ) -> Self {
        Self::OutcomeNotBound {
            kind,
            reason,
            next_step,
        }
    }
}

pub fn topology_neighborhood_replacement_entry(
    source: PrimitiveRebindingNeighborhoodReplacementSource,
) -> TopologyNeighborhoodReplacementEntry {
    let receipt = TopologyNeighborhoodReplacementFactReceipt::from_source(&source);
    TopologyNeighborhoodReplacementEntry { source, receipt }
}

pub fn primitive_rebinding_neighborhood_replacement_source<C>(
    declaration: &PrimitiveRebindingDeclarationEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveRebindingQueryDomain, C>,
) -> Result<PrimitiveRebindingNeighborhoodReplacementSource, PrimitiveRebindingProjectionFactError>
where
    C: ForgeQueryDomainOperatingContext<PrimitiveRebindingQueryDomain>,
{
    let retained_source = primitive_rebinding_retained_fact_source(declaration, handle)?;
    let seed = declaration.neighborhood_replacement_seed();
    Ok(PrimitiveRebindingNeighborhoodReplacementSource {
        retained_source,
        neighborhood_family: seed.neighborhood_family(),
        prior_binding_identity: seed.prior_binding_identity().to_string(),
        prior_site_identity: seed.prior_site_identity().to_string(),
        affected_target_identities: seed.affected_target_identities().to_vec(),
        candidate_frontier: seed.candidate_frontier().to_vec(),
        candidate_labels: seed.candidate_labels().to_vec(),
    })
}

pub fn primitive_rebinding_neighborhood_replacement_facts<C>(
    entry: &TopologyNeighborhoodReplacementEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveRebindingQueryDomain, C>,
) -> Result<TopologyNeighborhoodReplacementFactReceipt, TopologyNeighborhoodReplacementFactError>
where
    C: ForgeQueryDomainOperatingContext<PrimitiveRebindingQueryDomain>,
{
    match handle.orchestrate_declaration_entry_outcome(entry.clone()) {
        ForgeQueryOrdinaryOutcome::Bound(_) => {}
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
            return Err(TopologyNeighborhoodReplacementFactError::outcome_not_bound(
                posture.kind(),
                posture.reason().to_string(),
                posture.next_step(),
            ))
        }
    }
    Ok(entry.receipt().clone())
}

fn digest_parts(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, parts)
}
