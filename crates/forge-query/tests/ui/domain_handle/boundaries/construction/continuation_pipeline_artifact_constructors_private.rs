#![allow(invalid_value)]

use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryContinuationExecution,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
    ForgeQueryNeighborhoodCapableGrouping, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture,
    ForgeQueryPreparedContinuation,
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
    let prepared = ForgeQueryPreparedContinuation::<GeometryDomainEntry, GeometryInput> {
        family: unsafe { std::mem::zeroed() },
        truth_context: unsafe { std::mem::zeroed() },
        basis_posture: unsafe { std::mem::zeroed() },
        workspace_contract: unsafe { std::mem::zeroed() },
        runtime_contract: unsafe { std::mem::zeroed() },
        execution_mode: unsafe { std::mem::zeroed() },
        required_basis_families: vec![],
        bridge_routing: unsafe { std::mem::zeroed() },
        signal_posture: unsafe { std::mem::zeroed() },
        signal_execution_family: None,
        signal_compatibility_digest: None,
        prepared_digest: String::new(),
    };
    let _ = ForgeQueryContinuationExecution::<GeometryDomainEntry, GeometryInput> {
        prepared,
        signal_execution_family: None,
        bridge_binding_surface: String::new(),
        execution_digest: String::new(),
    };
}
