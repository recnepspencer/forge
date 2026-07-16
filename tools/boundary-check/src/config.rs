use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Road1Config {
    pub(crate) machine_authority: MachineAuthorityConfig,
    pub(crate) root_manifest: String,
    pub(crate) forbidden_root_prefixes: Vec<String>,
    pub(crate) naming: NamingConfig,
    pub(crate) rule_contracts: RuleContracts,
    pub(crate) born_crates: Vec<BornCrateConfig>,
    pub(crate) seed_skeletons: Vec<SeedSkeletonConfig>,
    pub(crate) subworkspaces: Vec<SubworkspaceConfig>,
    pub(crate) legacy_reference_ratchet: LegacyReferenceRatchetConfig,
    /// Compile-time law substrates legal outside the band grammar (e.g. worth-proof).
    #[serde(default)]
    pub(crate) law_substrates: Vec<LawSubstrateConfig>,
    #[serde(default)]
    pub(crate) dependency_denials: Vec<DependencyDenialConfig>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct DependencyDenialConfig {
    pub(crate) workspace_manifest: String,
    pub(crate) sources: Vec<String>,
    pub(crate) forbidden_targets: Vec<String>,
    pub(crate) guidance: String,
}

/// One machine-owned law substrate: package identity plus legal tier/band sets.
#[derive(Clone, Debug, Deserialize)]
pub(crate) struct LawSubstrateConfig {
    pub(crate) package: String,
    pub(crate) tiers: Vec<String>,
    pub(crate) bands: Vec<String>,
}

#[derive(Deserialize)]
pub(crate) struct LegacyReferenceRatchetConfig {
    pub(crate) governed_roots: Vec<String>,
    pub(crate) forbidden_fragments: Vec<String>,
    pub(crate) snapshot: String,
    pub(crate) exclude_paths: Vec<String>,
    pub(crate) replacement_guidance: String,
}

#[derive(Deserialize)]
pub(crate) struct MachineAuthorityConfig {
    pub(crate) canonical_config: String,
    pub(crate) mirrored_docs: Vec<String>,
}

#[derive(Deserialize)]
pub(crate) struct NamingConfig {
    pub(crate) bands: Vec<String>,
    pub(crate) reserved_domains: Vec<ReservedDomainConfig>,
}

#[derive(Deserialize)]
pub(crate) struct ReservedDomainConfig {
    pub(crate) tier: String,
    pub(crate) band: String,
    pub(crate) domains: Vec<String>,
}

#[derive(Deserialize)]
pub(crate) struct RuleContracts {
    pub(crate) query_audience: QueryAudienceContract,
    pub(crate) replay_surfaces: Vec<ReplaySurfaceConfig>,
    pub(crate) band_rules: Vec<BandRuleConfig>,
}

/// Machine-owned Query audience matrix: one engine package plus leaf facade rows.
#[derive(Clone, Debug, Deserialize)]
pub(crate) struct QueryAudienceContract {
    pub(crate) engine_package: String,
    pub(crate) audiences: Vec<QueryAudienceFacadeConfig>,
}

/// One audience facade row: package identity, legal bands, and repair guidance.
#[derive(Clone, Debug, Deserialize)]
pub(crate) struct QueryAudienceFacadeConfig {
    pub(crate) package: String,
    pub(crate) label: String,
    pub(crate) allowed_bands: Vec<String>,
    pub(crate) guidance: String,
}

#[derive(Deserialize)]
pub(crate) struct BandRuleConfig {
    pub(crate) source_band: String,
    pub(crate) allowed_target_bands: Vec<String>,
}

#[derive(Deserialize)]
pub(crate) struct ReplaySurfaceConfig {
    pub(crate) label: String,
    pub(crate) package_prefixes: Vec<String>,
    pub(crate) cert_domains: Vec<String>,
}

#[derive(Deserialize)]
pub(crate) struct BornCrateConfig {
    pub(crate) path: String,
    pub(crate) package: String,
}

#[derive(Deserialize)]
pub(crate) struct SeedSkeletonConfig {
    pub(crate) path: String,
    pub(crate) package: String,
    pub(crate) lib_rs: String,
    pub(crate) facade_rs: String,
    pub(crate) allowed_entries: Vec<String>,
}

#[derive(Deserialize)]
pub(crate) struct SubworkspaceConfig {
    pub(crate) path: String,
    pub(crate) allowed_crate_prefixes: Vec<String>,
    pub(crate) member_lane: String,
}
