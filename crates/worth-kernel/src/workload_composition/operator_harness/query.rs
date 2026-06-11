use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily,
    ForgeQueryDeclarationAspectContract, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationRouteContract,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext, ForgeQueryOrdinaryOutcome,
    ForgeQueryRelationalTruthAuthority, ForgeQuerySignalNotCompatiblePosture,
    ForgeQuerySingleOnlyGrouping,
};

use super::declaration::WorkloadOperatorFamily;
use crate::workload_composition::WorkloadStageRequirement;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorkloadOperatorQueryDomain;

impl ForgeQueryDomainEntryMarker for WorkloadOperatorQueryDomain {
    fn domain_key(&self) -> &'static str {
        "worth.kernel.workload_operator"
    }

    fn display_name(&self) -> &'static str {
        "WorthKernelWorkloadOperatorDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::QueryContext,
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkloadOperatorQueryWorld {
    identity: String,
}

impl WorkloadOperatorQueryWorld {
    fn new(identity: impl Into<String>) -> Self {
        Self {
            identity: identity.into(),
        }
    }
}

impl ForgeQueryDomainOperatingContext<WorkloadOperatorQueryDomain> for WorkloadOperatorQueryWorld {
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
        format!("worth.kernel.workload_operator.{}", self.identity)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorkloadOperatorDeclarationFamily;

impl ForgeQueryDeclarationFamilyMarker<WorkloadOperatorQueryDomain>
    for WorkloadOperatorDeclarationFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "WorkloadOperator"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &[
                "workload_operator.family",
                "workload_operator.required_stage",
                "workload_operator.intent",
            ],
            &[
                "workload_operator.declaration",
                "workload_operator.support",
                "workload_operator.receipt_set",
                "workload_operator.outcome",
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
pub struct WorkloadOperatorSupportFamily;

impl ForgeQueryDeclarationFamilyMarker<WorkloadOperatorQueryDomain>
    for WorkloadOperatorSupportFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "WorkloadOperatorSupport"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &[
                "workload_operator.family",
                "workload_operator.required_stage",
                "workload_operator.support_posture",
                "workload_operator.declaration_digest",
            ],
            &[
                "workload_operator.support",
                "workload_operator.admission_row",
                "workload_operator.declaration_link",
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
pub struct WorkloadOperatorQueryEntry {
    family: WorkloadOperatorFamily,
    requirement: WorkloadStageRequirement,
    query_intent: String,
}

impl WorkloadOperatorQueryEntry {
    pub fn new(
        family: WorkloadOperatorFamily,
        requirement: WorkloadStageRequirement,
        query_intent: impl Into<String>,
    ) -> Self {
        Self {
            family,
            requirement,
            query_intent: query_intent.into(),
        }
    }
}

impl ForgeQueryDeclarationInput<WorkloadOperatorQueryDomain> for WorkloadOperatorQueryEntry {
    type Family = WorkloadOperatorDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            ForgeQueryDeclarationCanonicalEntry::text(
                "workload_operator.family",
                self.family.query_key(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "workload_operator.required_stage",
                self.requirement.query_key(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "workload_operator.intent",
                self.query_intent.clone(),
            ),
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkloadOperatorSupportQueryEntry {
    family: WorkloadOperatorFamily,
    requirement: WorkloadStageRequirement,
    support_posture: String,
    declaration_digest: String,
    envelope_digest: String,
}

impl WorkloadOperatorSupportQueryEntry {
    pub fn new(
        family: WorkloadOperatorFamily,
        requirement: WorkloadStageRequirement,
        support_posture: impl Into<String>,
        declaration_digest: impl Into<String>,
        envelope_digest: impl Into<String>,
    ) -> Self {
        Self {
            family,
            requirement,
            support_posture: support_posture.into(),
            declaration_digest: declaration_digest.into(),
            envelope_digest: envelope_digest.into(),
        }
    }
}

impl ForgeQueryDeclarationInput<WorkloadOperatorQueryDomain> for WorkloadOperatorSupportQueryEntry {
    type Family = WorkloadOperatorSupportFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            ForgeQueryDeclarationCanonicalEntry::text(
                "workload_operator.family",
                self.family.query_key(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "workload_operator.required_stage",
                self.requirement.query_key(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "workload_operator.support_posture",
                self.support_posture.clone(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "workload_operator.declaration_digest",
                self.declaration_digest.clone(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "workload_operator.envelope_digest",
                self.envelope_digest.clone(),
            ),
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkloadOperatorQueryReceipt {
    declaration_digest: String,
    envelope_digest: String,
    handle_digest: String,
}

impl WorkloadOperatorQueryReceipt {
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

pub fn query_backed_operator_declaration(
    family: WorkloadOperatorFamily,
    requirement: WorkloadStageRequirement,
    query_intent: &str,
) -> Result<WorkloadOperatorQueryReceipt, String> {
    let handle = ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(WorkloadOperatorQueryDomain)
        .with_operating_context(WorkloadOperatorQueryWorld::new(query_intent))
        .validate()
        .map_err(|error| format!("{error:?}"))?
        .admit()
        .map_err(|error| format!("{error:?}"))?;
    let entry = WorkloadOperatorQueryEntry::new(family, requirement, query_intent);
    match handle.orchestrate_declaration_entry_outcome(entry) {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => Ok(WorkloadOperatorQueryReceipt {
            declaration_digest: envelope.declaration_digest().to_string(),
            envelope_digest: format!("{:?}", envelope.envelope_digest()),
            handle_digest: handle.handle_identity_digest().to_string(),
        }),
        _ => Err("operator declaration was not bound by Forge Query".to_string()),
    }
}

pub fn query_backed_operator_support(
    family: WorkloadOperatorFamily,
    requirement: WorkloadStageRequirement,
    query_intent: &str,
    support_posture: &str,
    declaration_digest: &str,
    envelope_digest: &str,
) -> Result<WorkloadOperatorQueryReceipt, String> {
    let handle = ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(WorkloadOperatorQueryDomain)
        .with_operating_context(WorkloadOperatorQueryWorld::new(query_intent))
        .validate()
        .map_err(|error| format!("{error:?}"))?
        .admit()
        .map_err(|error| format!("{error:?}"))?;
    let entry = WorkloadOperatorSupportQueryEntry::new(
        family,
        requirement,
        support_posture,
        declaration_digest,
        envelope_digest,
    );
    match handle.orchestrate_declaration_entry_outcome(entry) {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => Ok(WorkloadOperatorQueryReceipt {
            declaration_digest: envelope.declaration_digest().to_string(),
            envelope_digest: format!("{:?}", envelope.envelope_digest()),
            handle_digest: handle.handle_identity_digest().to_string(),
        }),
        _ => Err("operator support row was not bound by Forge Query".to_string()),
    }
}
