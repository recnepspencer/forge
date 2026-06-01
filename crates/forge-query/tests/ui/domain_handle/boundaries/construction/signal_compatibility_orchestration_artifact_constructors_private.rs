#![allow(invalid_value)]

use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityContract, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryRelationalTruthAuthority, ForgeQuerySignalCompatibilityOrchestrationChecked,
    ForgeQuerySignalCompatibilityOrchestrationOutcome,
    ForgeQuerySignalCompatibilityOrchestrationTranscript, ForgeQuerySignalCompatiblePosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomainEntry;

impl ForgeQueryDomainEntryMarker for GeometryDomainEntry {
    fn domain_key(&self) -> &'static str {
        "example.geometry"
    }

    fn display_name(&self) -> &'static str {
        "GeometryDomainEntry"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryOperatingContext;

impl ForgeQueryDomainOperatingContext<GeometryDomainEntry> for GeometryOperatingContext {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[]
    }

    fn context_identity_digest(&self) -> String {
        "geometry".to_string()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct GeometryFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomainEntry> for GeometryFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "geometry-family"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct GeometryInput;

impl ForgeQueryDeclarationInput<GeometryDomainEntry> for GeometryInput {
    type Family = GeometryFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text("edge_ref", "edge-a")]
    }
}

fn main() {
    let outcome = ForgeQuerySignalCompatibilityOrchestrationOutcome::<
        GeometryDomainEntry,
        GeometryInput,
    >::Unavailable(String::new());
    let _ = ForgeQuerySignalCompatibilityOrchestrationChecked::<
        GeometryDomainEntry,
        GeometryInput,
    > {
        outcome,
        orchestration_digest: String::new(),
        linked_artifacts: unsafe { std::mem::zeroed() },
    };
    let outcome = ForgeQuerySignalCompatibilityOrchestrationOutcome::<
        GeometryDomainEntry,
        GeometryInput,
    >::Unavailable(String::new());
    let _ = ForgeQuerySignalCompatibilityOrchestrationTranscript::<
        GeometryDomainEntry,
        GeometryInput,
    > {
        request: unsafe { std::mem::zeroed() },
        outcome,
        witness_checks: vec![],
        narrowing_decisions: vec![],
        orchestration_digest: String::new(),
        linked_artifacts: unsafe { std::mem::zeroed() },
    };
}
