use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDomainOperatingContext,
    ForgeQueryOrdinaryOutcome, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture, ForgeQuerySingleOnlyGrouping,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::bindings::query_native_rebinding::PrimitiveRebindingQueryDomain;
use crate::bindings::query_native_rebinding_projection::PrimitiveRebindingProjectionFactError;
use crate::bindings::query_native_retained_geometry::retained_source_digest;
use crate::bindings::rebinding::{NeighborhoodBindingFamily, PrimitiveRebindingRetainedFactSource};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GeometryProjectionConsumptionDeclarationFamily;

impl ForgeQueryDeclarationFamilyMarker<PrimitiveRebindingQueryDomain>
    for GeometryProjectionConsumptionDeclarationFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "GeometryProjectionConsumption"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &[
                "geometry.projection.source_family",
                "geometry.projection.source_receipt",
            ],
            &["geometry.projection.consumption"],
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
pub struct GeometryProjectionConsumptionEntry {
    source: PrimitiveRebindingRetainedFactSource,
    receipt: GeometryProjectionConsumptionReceipt,
}

impl GeometryProjectionConsumptionEntry {
    pub fn source(&self) -> &PrimitiveRebindingRetainedFactSource {
        &self.source
    }

    fn receipt(&self) -> &GeometryProjectionConsumptionReceipt {
        &self.receipt
    }
}

impl ForgeQueryDeclarationInput<PrimitiveRebindingQueryDomain>
    for GeometryProjectionConsumptionEntry
{
    type Family = GeometryProjectionConsumptionDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        let entries = vec![
            ForgeQueryDeclarationCanonicalEntry::text(
                "geometry.projection.kind",
                "geometry_projection_consumption",
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "geometry.projection.source_family",
                self.source
                    .receipt()
                    .neighborhood_family()
                    .rebinding_kind_label(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "geometry.projection.source_receipt",
                retained_source_digest(&self.source),
            ),
        ];
        entries
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GeometryProjectedFactKind {
    PrimitiveRebindingProjectionFact,
}

impl GeometryProjectedFactKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PrimitiveRebindingProjectionFact => "primitive_rebinding_projection_fact",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeometryProjectionConsumptionReceipt {
    projected_fact_kind: GeometryProjectedFactKind,
    source_family: NeighborhoodBindingFamily,
    source_receipt_digest: String,
    projection_contract_identity: String,
    projection_digest: String,
    materialization_basis_digest: Option<String>,
}

impl GeometryProjectionConsumptionReceipt {
    fn from_retained_source(source: &PrimitiveRebindingRetainedFactSource) -> Self {
        let facts = source.receipt();
        let projected_fact_kind = GeometryProjectedFactKind::PrimitiveRebindingProjectionFact;
        let source_family = facts.neighborhood_family();
        let source_receipt_digest = retained_source_digest(source);
        let projection_contract_identity =
            "worth.spatial.rebinding.geometry_projection_consumption".to_string();
        let materialization_basis_digest = None;
        let projection_digest = digest_parts(&[
            projected_fact_kind.as_str().to_string(),
            source_family.rebinding_kind_label().to_string(),
            source_receipt_digest.clone(),
            projection_contract_identity.clone(),
            materialization_basis_digest
                .clone()
                .unwrap_or_else(|| "none".to_string()),
        ]);
        Self {
            projected_fact_kind,
            source_family,
            source_receipt_digest,
            projection_contract_identity,
            projection_digest,
            materialization_basis_digest,
        }
    }

    pub fn projected_fact_kind(&self) -> GeometryProjectedFactKind {
        self.projected_fact_kind
    }

    pub fn source_family(&self) -> NeighborhoodBindingFamily {
        self.source_family
    }

    pub fn source_receipt_digest(&self) -> &str {
        &self.source_receipt_digest
    }

    pub fn projection_contract_identity(&self) -> &str {
        &self.projection_contract_identity
    }

    pub fn projection_digest(&self) -> &str {
        &self.projection_digest
    }

    pub fn materialization_basis_digest(&self) -> Option<&str> {
        self.materialization_basis_digest.as_deref()
    }
}

pub fn geometry_projection_consumption_entry(
    source: PrimitiveRebindingRetainedFactSource,
) -> GeometryProjectionConsumptionEntry {
    let receipt = GeometryProjectionConsumptionReceipt::from_retained_source(&source);
    GeometryProjectionConsumptionEntry { source, receipt }
}

pub fn primitive_rebinding_geometry_projection_consumption<C>(
    entry: &GeometryProjectionConsumptionEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveRebindingQueryDomain, C>,
) -> Result<GeometryProjectionConsumptionReceipt, PrimitiveRebindingProjectionFactError>
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
            return Err(PrimitiveRebindingProjectionFactError::OutcomeNotBound {
                kind: posture.kind(),
                reason: posture.reason().to_string(),
                next_step: posture.next_step(),
            })
        }
    }
    Ok(entry.receipt().clone())
}

fn digest_parts(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, parts)
}
