use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily,
    ForgeQueryDeclarationAspectContract, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationRouteContract,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext, ForgeQueryOrdinaryOutcome,
    ForgeQueryRelationalTruthAuthority, ForgeQuerySignalNotCompatiblePosture,
    ForgeQuerySingleOnlyGrouping,
};

use super::error::WorkloadCatalogError;
use super::recipe_kind::{WorkloadCatalogRecipeKind, WorkloadCatalogSupportPosture};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorkloadCatalogQueryDomain;

impl ForgeQueryDomainEntryMarker for WorkloadCatalogQueryDomain {
    fn domain_key(&self) -> &'static str {
        "worth.kernel.workload_catalog"
    }

    fn display_name(&self) -> &'static str {
        "WorthKernelWorkloadCatalogDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::QueryContext,
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkloadCatalogQueryWorld {
    identity: String,
}

impl WorkloadCatalogQueryWorld {
    fn new(identity: impl Into<String>) -> Self {
        Self {
            identity: identity.into(),
        }
    }
}

impl ForgeQueryDomainOperatingContext<WorkloadCatalogQueryDomain> for WorkloadCatalogQueryWorld {
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
        format!("worth.kernel.workload_catalog.{}", self.identity)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorkloadCatalogDeclarationFamily;

impl ForgeQueryDeclarationFamilyMarker<WorkloadCatalogQueryDomain>
    for WorkloadCatalogDeclarationFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "WorkloadCatalogRecipe"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        crate::query_authoring_helpers::declaration_aspect_contract_from_slices(
            &["workload_catalog.recipe", "workload_catalog.declaration"],
            &[
                "workload_catalog.recipe_declaration",
                "workload_catalog.support",
                "workload_catalog.evidence_ledger",
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorkloadCatalogSupportFamily;

impl ForgeQueryDeclarationFamilyMarker<WorkloadCatalogQueryDomain>
    for WorkloadCatalogSupportFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "WorkloadCatalogSupport"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        crate::query_authoring_helpers::declaration_aspect_contract_from_slices(
            &[
                "workload_catalog.recipe",
                "workload_catalog.support_posture",
                "workload_catalog.declaration_digest",
            ],
            &[
                "workload_catalog.support",
                "workload_catalog.recipe_admission",
                "workload_catalog.declaration_link",
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
struct WorkloadCatalogDeclarationEntry {
    recipe: WorkloadCatalogRecipeKind,
    declaration: String,
}

impl ForgeQueryDeclarationInput<WorkloadCatalogQueryDomain> for WorkloadCatalogDeclarationEntry {
    type Family = WorkloadCatalogDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            ForgeQueryDeclarationCanonicalEntry::text(
                "workload_catalog.recipe",
                self.recipe.query_key(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "workload_catalog.declaration",
                self.declaration.clone(),
            ),
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct WorkloadCatalogSupportEntry {
    recipe: WorkloadCatalogRecipeKind,
    posture: WorkloadCatalogSupportPosture,
    declaration_digest: String,
}

impl ForgeQueryDeclarationInput<WorkloadCatalogQueryDomain> for WorkloadCatalogSupportEntry {
    type Family = WorkloadCatalogSupportFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            ForgeQueryDeclarationCanonicalEntry::text(
                "workload_catalog.recipe",
                self.recipe.query_key(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "workload_catalog.support_posture",
                self.posture.query_key(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "workload_catalog.declaration_digest",
                self.declaration_digest.clone(),
            ),
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkloadCatalogQueryReceipt {
    declaration_digest: String,
    envelope_digest: String,
    handle_digest: String,
}

impl WorkloadCatalogQueryReceipt {
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

pub fn query_backed_catalog_declaration(
    recipe: WorkloadCatalogRecipeKind,
    declaration: &str,
) -> Result<WorkloadCatalogQueryReceipt, WorkloadCatalogError> {
    query_catalog_entry(
        declaration,
        WorkloadCatalogDeclarationEntry {
            recipe,
            declaration: declaration.to_string(),
        },
    )
}

pub fn query_backed_catalog_support(
    recipe: WorkloadCatalogRecipeKind,
    declaration: &str,
    posture: WorkloadCatalogSupportPosture,
    declaration_digest: &str,
) -> Result<WorkloadCatalogQueryReceipt, WorkloadCatalogError> {
    query_catalog_entry(
        declaration,
        WorkloadCatalogSupportEntry {
            recipe,
            posture,
            declaration_digest: declaration_digest.to_string(),
        },
    )
}

fn query_catalog_entry<I>(
    world: &str,
    entry: I,
) -> Result<WorkloadCatalogQueryReceipt, WorkloadCatalogError>
where
    I: ForgeQueryDeclarationInput<WorkloadCatalogQueryDomain>,
{
    let handle = ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(WorkloadCatalogQueryDomain)
        .with_operating_context(WorkloadCatalogQueryWorld::new(world))
        .validate()
        .map_err(|error| WorkloadCatalogError::QueryAdmissionFailed(format!("{error:?}")))?
        .admit()
        .map_err(|error| WorkloadCatalogError::QueryAdmissionFailed(format!("{error:?}")))?;
    match handle.orchestrate_declaration_entry_outcome(entry) {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => Ok(WorkloadCatalogQueryReceipt {
            declaration_digest: envelope.declaration_digest().to_string(),
            envelope_digest: format!("{:?}", envelope.envelope_digest()),
            handle_digest: handle.handle_identity_digest().to_string(),
        }),
        _ => Err(WorkloadCatalogError::QueryAdmissionFailed(
            "workload catalog Query entry was not bound".to_string(),
        )),
    }
}
