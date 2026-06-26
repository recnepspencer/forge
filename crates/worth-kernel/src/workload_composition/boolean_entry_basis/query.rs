use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily,
    ForgeQueryDeclarationAspectContract, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationRouteContract,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext, ForgeQueryOrdinaryOutcome,
    ForgeQueryRelationalTruthAuthority, ForgeQuerySignalNotCompatiblePosture,
    ForgeQuerySingleOnlyGrouping,
};
use worth_spatial::facade::boolean_readiness_workload::PlanarBooleanReadinessWorkloadReceipt;

use super::error::PlanarBooleanEntryBasisError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanarBooleanEntryBasisQueryDomain;

impl ForgeQueryDomainEntryMarker for PlanarBooleanEntryBasisQueryDomain {
    fn domain_key(&self) -> &'static str {
        "worth.kernel.planar_boolean_entry_basis"
    }

    fn display_name(&self) -> &'static str {
        "WorthKernelPlanarBooleanEntryBasisDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::QueryContext,
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PlanarBooleanEntryBasisQueryWorld {
    identity: String,
}

impl PlanarBooleanEntryBasisQueryWorld {
    fn new(identity: impl Into<String>) -> Self {
        Self {
            identity: identity.into(),
        }
    }
}

impl ForgeQueryDomainOperatingContext<PlanarBooleanEntryBasisQueryDomain>
    for PlanarBooleanEntryBasisQueryWorld
{
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::QueryContext,
        ]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::Relational,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!("worth.kernel.planar_boolean_entry_basis.{}", self.identity)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanarBooleanEntryBasisDeclarationFamily;

impl ForgeQueryDeclarationFamilyMarker<PlanarBooleanEntryBasisQueryDomain>
    for PlanarBooleanEntryBasisDeclarationFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "PlanarBooleanEntryBasis"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        crate::query_authoring_helpers::declaration_aspect_contract_from_slices(
            &[
                "planar_boolean_basis.readiness_digest",
                "planar_boolean_basis.readiness_declaration_digest",
                "planar_boolean_basis.readiness_envelope_digest",
                "planar_boolean_basis.workload_digest",
                "planar_boolean_basis.stage_coverage_digest",
            ],
            &[
                "planar_boolean_basis.declaration",
                "planar_boolean_basis.entry_receipt",
            ],
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

#[derive(Clone, Debug, Eq, PartialEq)]
struct PlanarBooleanEntryBasisEntry {
    readiness_digest: String,
    readiness_declaration_digest: String,
    readiness_envelope_digest: String,
    workload_digest: String,
    stage_coverage_digest: String,
}

impl ForgeQueryDeclarationInput<PlanarBooleanEntryBasisQueryDomain>
    for PlanarBooleanEntryBasisEntry
{
    type Family = PlanarBooleanEntryBasisDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            ForgeQueryDeclarationCanonicalEntry::text(
                "planar_boolean_basis.readiness_digest",
                self.readiness_digest.clone(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "planar_boolean_basis.readiness_declaration_digest",
                self.readiness_declaration_digest.clone(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "planar_boolean_basis.readiness_envelope_digest",
                self.readiness_envelope_digest.clone(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "planar_boolean_basis.workload_digest",
                self.workload_digest.clone(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "planar_boolean_basis.stage_coverage_digest",
                self.stage_coverage_digest.clone(),
            ),
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanEntryBasisQueryReceipt {
    declaration_digest: String,
    envelope_digest: String,
    handle_digest: String,
}

impl PlanarBooleanEntryBasisQueryReceipt {
    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn envelope_digest(&self) -> &str {
        &self.envelope_digest
    }

    pub fn handle_digest(&self) -> &str {
        &self.handle_digest
    }
}

pub fn query_backed_planar_boolean_entry_basis(
    readiness: &PlanarBooleanReadinessWorkloadReceipt,
    query_intent: &str,
) -> Result<PlanarBooleanEntryBasisQueryReceipt, PlanarBooleanEntryBasisError> {
    query_planar_boolean_entry_basis(
        query_intent,
        PlanarBooleanEntryBasisEntry {
            readiness_digest: readiness
                .m7_readiness_receipt()
                .readiness_digest()
                .to_string(),
            readiness_declaration_digest: readiness
                .m7_readiness_receipt()
                .declaration_digest()
                .to_string(),
            readiness_envelope_digest: readiness
                .m7_readiness_receipt()
                .envelope_digest()
                .to_string(),
            workload_digest: readiness.workload_digest().to_string(),
            stage_coverage_digest: readiness.stage_coverage().coverage_digest().to_string(),
        },
    )
}

fn query_planar_boolean_entry_basis<I>(
    world: &str,
    entry: I,
) -> Result<PlanarBooleanEntryBasisQueryReceipt, PlanarBooleanEntryBasisError>
where
    I: ForgeQueryDeclarationInput<PlanarBooleanEntryBasisQueryDomain>,
{
    let handle = ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarBooleanEntryBasisQueryDomain)
        .with_operating_context(PlanarBooleanEntryBasisQueryWorld::new(world))
        .validate()
        .map_err(|error| PlanarBooleanEntryBasisError::QueryAdmissionFailed(format!("{error:?}")))?
        .admit()
        .map_err(|error| {
            PlanarBooleanEntryBasisError::QueryAdmissionFailed(format!("{error:?}"))
        })?;
    match handle.orchestrate_declaration_entry_outcome(entry) {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => Ok(PlanarBooleanEntryBasisQueryReceipt {
            declaration_digest: envelope.declaration_digest().to_string(),
            envelope_digest: format!("{:?}", envelope.envelope_digest()),
            handle_digest: handle.handle_identity_digest().to_string(),
        }),
        _ => Err(PlanarBooleanEntryBasisError::QueryAdmissionFailed(
            "planar boolean entry basis Query entry was not bound".to_string(),
        )),
    }
}
