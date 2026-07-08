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
    pub(crate) query_host_bands: Vec<String>,
    pub(crate) replay_surfaces: Vec<ReplaySurfaceConfig>,
    pub(crate) band_rules: Vec<BandRuleConfig>,
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
