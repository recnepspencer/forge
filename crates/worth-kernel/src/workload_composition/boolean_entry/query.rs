use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily,
    ForgeQueryDeclarationAspectContract, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationRouteContract,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext, ForgeQueryOrdinaryOutcome,
    ForgeQueryRelationalTruthAuthority, ForgeQuerySignalNotCompatiblePosture,
    ForgeQuerySingleOnlyGrouping,
};

use super::declaration::{
    PlanarBooleanExecutionLane, PlanarBooleanFamily, PlanarBooleanOperandPairIdentity,
    PlanarBooleanOperation,
};
use super::support::{PlanarBooleanEntryError, PlanarBooleanSupportPosture};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanarBooleanQueryDomain;

impl ForgeQueryDomainEntryMarker for PlanarBooleanQueryDomain {
    fn domain_key(&self) -> &'static str {
        "worth.kernel.planar_boolean_entry"
    }

    fn display_name(&self) -> &'static str {
        "WorthKernelPlanarBooleanEntryDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::QueryContext,
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PlanarBooleanQueryWorld {
    identity: String,
}

impl PlanarBooleanQueryWorld {
    fn new(identity: impl Into<String>) -> Self {
        Self {
            identity: identity.into(),
        }
    }
}

impl ForgeQueryDomainOperatingContext<PlanarBooleanQueryDomain> for PlanarBooleanQueryWorld {
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
        format!("worth.kernel.planar_boolean_entry.{}", self.identity)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanarBooleanDeclarationFamily;

impl ForgeQueryDeclarationFamilyMarker<PlanarBooleanQueryDomain>
    for PlanarBooleanDeclarationFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "PlanarBooleanDeclaration"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &[
                "planar_boolean.family",
                "planar_boolean.operation",
                "planar_boolean.operand_pair_identity",
                "planar_boolean.execution_lane",
                "planar_boolean.readiness_basis",
                "planar_boolean.workload_basis",
            ],
            &[
                "planar_boolean.declaration",
                "planar_boolean.support",
                "planar_boolean.route_plan",
                "planar_boolean.entry_receipt",
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
pub struct PlanarBooleanSupportFamily;

impl ForgeQueryDeclarationFamilyMarker<PlanarBooleanQueryDomain> for PlanarBooleanSupportFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "PlanarBooleanSupport"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &[
                "planar_boolean.family",
                "planar_boolean.operation",
                "planar_boolean.execution_lane",
                "planar_boolean.support_posture",
                "planar_boolean.declaration_digest",
            ],
            &[
                "planar_boolean.support",
                "planar_boolean.admission_row",
                "planar_boolean.declaration_link",
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
struct PlanarBooleanDeclarationEntry {
    family: PlanarBooleanFamily,
    operation: PlanarBooleanOperation,
    operand_pair_identity: String,
    requested_lane: PlanarBooleanExecutionLane,
    readiness_basis_digest: String,
    readiness_workload_digest: String,
}

impl ForgeQueryDeclarationInput<PlanarBooleanQueryDomain> for PlanarBooleanDeclarationEntry {
    type Family = PlanarBooleanDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            ForgeQueryDeclarationCanonicalEntry::text(
                "planar_boolean.family",
                self.family.query_key(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "planar_boolean.operation",
                self.operation.query_key(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "planar_boolean.operand_pair_identity",
                self.operand_pair_identity.clone(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "planar_boolean.execution_lane",
                self.requested_lane.query_key(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "planar_boolean.readiness_basis",
                self.readiness_basis_digest.clone(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "planar_boolean.workload_basis",
                self.readiness_workload_digest.clone(),
            ),
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PlanarBooleanSupportEntry {
    family: PlanarBooleanFamily,
    operation: PlanarBooleanOperation,
    requested_lane: PlanarBooleanExecutionLane,
    support_posture: PlanarBooleanSupportPosture,
    declaration_digest: String,
}

impl ForgeQueryDeclarationInput<PlanarBooleanQueryDomain> for PlanarBooleanSupportEntry {
    type Family = PlanarBooleanSupportFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            ForgeQueryDeclarationCanonicalEntry::text(
                "planar_boolean.family",
                self.family.query_key(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "planar_boolean.operation",
                self.operation.query_key(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "planar_boolean.execution_lane",
                self.requested_lane.query_key(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "planar_boolean.support_posture",
                self.support_posture.query_key(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "planar_boolean.declaration_digest",
                self.declaration_digest.clone(),
            ),
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanQueryReceipt {
    declaration_digest: String,
    envelope_digest: String,
    handle_digest: String,
}

impl PlanarBooleanQueryReceipt {
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

pub fn query_backed_planar_boolean_declaration(
    family: PlanarBooleanFamily,
    operation: PlanarBooleanOperation,
    operand_pair_identity: &PlanarBooleanOperandPairIdentity,
    requested_lane: PlanarBooleanExecutionLane,
    readiness_basis_digest: &str,
    readiness_workload_digest: &str,
    query_intent: &str,
) -> Result<PlanarBooleanQueryReceipt, PlanarBooleanEntryError> {
    query_planar_boolean_entry(
        query_intent,
        PlanarBooleanDeclarationEntry {
            family,
            operation,
            operand_pair_identity: operand_pair_identity.as_str().to_string(),
            requested_lane,
            readiness_basis_digest: readiness_basis_digest.to_string(),
            readiness_workload_digest: readiness_workload_digest.to_string(),
        },
    )
}

pub fn query_backed_planar_boolean_support(
    family: PlanarBooleanFamily,
    operation: PlanarBooleanOperation,
    requested_lane: PlanarBooleanExecutionLane,
    support_posture: PlanarBooleanSupportPosture,
    declaration_digest: &str,
    query_intent: &str,
) -> Result<PlanarBooleanQueryReceipt, PlanarBooleanEntryError> {
    query_planar_boolean_entry(
        query_intent,
        PlanarBooleanSupportEntry {
            family,
            operation,
            requested_lane,
            support_posture,
            declaration_digest: declaration_digest.to_string(),
        },
    )
}

fn query_planar_boolean_entry<I>(
    world: &str,
    entry: I,
) -> Result<PlanarBooleanQueryReceipt, PlanarBooleanEntryError>
where
    I: ForgeQueryDeclarationInput<PlanarBooleanQueryDomain>,
{
    let handle = ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarBooleanQueryDomain)
        .with_operating_context(PlanarBooleanQueryWorld::new(world))
        .validate()
        .map_err(|error| PlanarBooleanEntryError::QueryAdmissionFailed(format!("{error:?}")))?
        .admit()
        .map_err(|error| PlanarBooleanEntryError::QueryAdmissionFailed(format!("{error:?}")))?;
    match handle.orchestrate_declaration_entry_outcome(entry) {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => Ok(PlanarBooleanQueryReceipt {
            declaration_digest: envelope.declaration_digest().to_string(),
            envelope_digest: format!("{:?}", envelope.envelope_digest()),
            handle_digest: handle.handle_identity_digest().to_string(),
        }),
        _ => Err(PlanarBooleanEntryError::QueryAdmissionFailed(
            "planar boolean Query entry was not bound".to_string(),
        )),
    }
}
