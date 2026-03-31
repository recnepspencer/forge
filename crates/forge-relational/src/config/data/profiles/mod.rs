mod presets;

use std::collections::BTreeMap;

use sha2::{Digest, Sha256};

use crate::config::data::*;

use presets::default_profile_config;

impl RelationalRuntimeConfig {
    pub fn resolved(
        profile: RelationalRuntimeProfile,
        overrides: RelationalConfigOverride,
    ) -> Self {
        let mut config = default_profile_config(profile);
        let mut provenance_entries = BTreeMap::new();

        apply_execution_overrides(&mut config, &overrides, &mut provenance_entries);
        apply_diagnostics_overrides(&mut config, &overrides, &mut provenance_entries);
        apply_history_overrides(&mut config, &overrides, &mut provenance_entries);
        apply_schema_overrides(&mut config, &overrides, &mut provenance_entries);
        apply_commit_strategy_overrides(&mut config, &overrides, &mut provenance_entries);
        apply_identity_overrides(&mut config, &overrides, &mut provenance_entries);
        apply_storage_overrides(&mut config, &overrides, &mut provenance_entries);
        apply_visibility_overrides(&mut config, &overrides, &mut provenance_entries);
        apply_publication_overrides(&mut config, &overrides, &mut provenance_entries);
        apply_durability_overrides(&mut config, &overrides, &mut provenance_entries);

        config.overrides = overrides;
        config.provenance = ConfigProvenance {
            profile,
            entries: provenance_entries,
        };
        config
    }
}

impl Default for RelationalRuntimeConfig {
    fn default() -> Self {
        Self::resolved(
            RelationalRuntimeProfile::CertificationCore,
            RelationalConfigOverride::default(),
        )
    }
}

fn apply_execution_overrides(
    config: &mut RelationalRuntimeConfig,
    override_tree: &RelationalConfigOverride,
    provenance: &mut BTreeMap<String, ConfigProvenanceEntry>,
) {
    let section = &override_tree.execution;
    provenance.insert(
        "execution.runtime_name".to_string(),
        provenance_entry(section.runtime_name.is_some()),
    );
    provenance.insert(
        "execution.execution_model".to_string(),
        provenance_entry(section.execution_model.is_some()),
    );
    provenance.insert(
        "execution.planning".to_string(),
        provenance_entry(section.planning.is_some()),
    );
    provenance.insert(
        "execution.commit_authority".to_string(),
        provenance_entry(section.commit_authority.is_some()),
    );
    provenance.insert(
        "execution.compiled_lane_policy".to_string(),
        provenance_entry(section.compiled_lane_policy.is_some()),
    );
    provenance.insert(
        "execution.relation_integrity_scope_budget".to_string(),
        provenance_entry(section.relation_integrity_scope_budget.is_some()),
    );

    if let Some(runtime_name) = &section.runtime_name {
        config.execution.runtime_name = runtime_name.clone();
    }
    if let Some(execution_model) = section.execution_model {
        config.execution.execution_model = execution_model;
    }
    if let Some(planning) = &section.planning {
        config.execution.planning = planning.clone();
    }
    if let Some(commit_authority) = &section.commit_authority {
        config.execution.commit_authority = commit_authority.clone();
    }
    if let Some(compiled_lane_policy) = section.compiled_lane_policy {
        config.execution.compiled_lane_policy = compiled_lane_policy;
    }
    if let Some(relation_integrity_scope_budget) = &section.relation_integrity_scope_budget {
        config.execution.relation_integrity_scope_budget = relation_integrity_scope_budget.clone();
    }
}

fn apply_diagnostics_overrides(
    config: &mut RelationalRuntimeConfig,
    override_tree: &RelationalConfigOverride,
    provenance: &mut BTreeMap<String, ConfigProvenanceEntry>,
) {
    let section = &override_tree.diagnostics;
    provenance.insert(
        "diagnostics.profile".to_string(),
        provenance_entry(section.profile.is_some()),
    );
    if let Some(profile) = &section.profile {
        config.diagnostics.profile = profile.clone();
    }
}

fn apply_history_overrides(
    config: &mut RelationalRuntimeConfig,
    override_tree: &RelationalConfigOverride,
    provenance: &mut BTreeMap<String, ConfigProvenanceEntry>,
) {
    let section = &override_tree.history;
    provenance.insert(
        "history.version_graph_policy".to_string(),
        provenance_entry(section.version_graph_policy.is_some()),
    );
    provenance.insert(
        "history.retention".to_string(),
        provenance_entry(section.retention.is_some()),
    );
    provenance.insert(
        "history.main_branch".to_string(),
        provenance_entry(section.main_branch.is_some()),
    );
    if let Some(version_graph_policy) = section.version_graph_policy {
        config.history.version_graph_policy = version_graph_policy;
    }
    if let Some(retention) = section.retention {
        config.history.retention = retention;
    }
    if let Some(main_branch) = &section.main_branch {
        config.history.main_branch = main_branch.clone();
    }
}

fn apply_schema_overrides(
    config: &mut RelationalRuntimeConfig,
    override_tree: &RelationalConfigOverride,
    provenance: &mut BTreeMap<String, ConfigProvenanceEntry>,
) {
    let section = &override_tree.schema;
    provenance.insert(
        "schema.registry".to_string(),
        provenance_entry(section.registry.is_some()),
    );
    provenance.insert(
        "schema.invariant_catalog".to_string(),
        provenance_entry(section.invariant_catalog.is_some()),
    );
    if let Some(registry) = &section.registry {
        config.schema.registry = registry.clone();
    }
    if let Some(invariant_catalog) = &section.invariant_catalog {
        config.schema.invariant_catalog = invariant_catalog.clone();
    }
}

fn apply_identity_overrides(
    config: &mut RelationalRuntimeConfig,
    override_tree: &RelationalConfigOverride,
    provenance: &mut BTreeMap<String, ConfigProvenanceEntry>,
) {
    let section = &override_tree.identity;
    provenance.insert(
        "identity.symbol_policy".to_string(),
        provenance_entry(section.symbol_policy.is_some()),
    );
    provenance.insert(
        "identity.symbol_table".to_string(),
        provenance_entry(section.symbol_table.is_some()),
    );
    if let Some(symbol_policy) = section.symbol_policy {
        config.identity.symbol_policy = symbol_policy;
    }
    if let Some(symbol_table) = &section.symbol_table {
        config.identity.symbol_table = symbol_table.clone();
    }
}

fn apply_commit_strategy_overrides(
    config: &mut RelationalRuntimeConfig,
    override_tree: &RelationalConfigOverride,
    provenance: &mut BTreeMap<String, ConfigProvenanceEntry>,
) {
    let section = &override_tree.commit_strategies;
    if let Some(registrations) = &section.registrations {
        config.commit_strategies.registrations = registrations.clone();
        provenance.insert(
            "commit_strategies.registrations".to_string(),
            ConfigProvenanceEntry {
                source: ConfigValueSource::BuilderOverride,
                detail: commit_strategy_override_detail(registrations),
            },
        );
    } else {
        provenance.insert(
            "commit_strategies.registrations".to_string(),
            ConfigProvenanceEntry {
                source: ConfigValueSource::ProfileDefault,
                detail: commit_strategy_override_detail(&config.commit_strategies.registrations),
            },
        );
    }
}

fn commit_strategy_override_detail(
    registrations: &[crate::commit_strategies::data::CommitStrategyRegistration],
) -> String {
    let mut descriptor_digests = registrations
        .iter()
        .map(|registration| registration.descriptor().digest().0)
        .collect::<Vec<_>>();
    descriptor_digests.sort();
    let digest = Sha256::digest(
        serde_json::to_vec(&descriptor_digests)
            .expect("commit strategy override detail serialization"),
    );
    let digest = digest
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    format!(
        "commit strategy registrations resolved; count={}; descriptor_set_digest={digest}",
        registrations.len()
    )
}

fn apply_storage_overrides(
    config: &mut RelationalRuntimeConfig,
    override_tree: &RelationalConfigOverride,
    provenance: &mut BTreeMap<String, ConfigProvenanceEntry>,
) {
    let section = &override_tree.storage;
    provenance.insert(
        "storage.initial_entity_capacity".to_string(),
        provenance_entry(section.initial_entity_capacity.is_some()),
    );
    provenance.insert(
        "storage.initial_relation_capacity".to_string(),
        provenance_entry(section.initial_relation_capacity.is_some()),
    );
    provenance.insert(
        "storage.mvcc".to_string(),
        provenance_entry(section.mvcc.is_some()),
    );
    provenance.insert(
        "storage.retention".to_string(),
        provenance_entry(section.retention.is_some()),
    );
    provenance.insert(
        "storage.layout".to_string(),
        provenance_entry(section.layout.is_some()),
    );
    provenance.insert(
        "storage.payload_policy".to_string(),
        provenance_entry(section.payload_policy.is_some()),
    );
    provenance.insert(
        "storage.adjacency_policy".to_string(),
        provenance_entry(section.adjacency_policy.is_some()),
    );
    provenance.insert(
        "storage.cross_context_policy".to_string(),
        provenance_entry(section.cross_context_policy.is_some()),
    );
    provenance.insert(
        "storage.cascade_delete_policy".to_string(),
        provenance_entry(section.cascade_delete_policy.is_some()),
    );

    if let Some(capacity) = section.initial_entity_capacity {
        config.storage.initial_entity_capacity = capacity;
    }
    if let Some(capacity) = section.initial_relation_capacity {
        config.storage.initial_relation_capacity = capacity;
    }
    if let Some(mvcc) = &section.mvcc {
        config.storage.mvcc = mvcc.clone();
        config.storage.retention.backend = mvcc.retention_backend;
        config.storage.retention.reclaim_batch_size = mvcc.reclaim_batch_size;
    }
    if let Some(retention) = &section.retention {
        config.storage.retention = retention.clone();
    }
    if let Some(layout) = &section.layout {
        config.storage.layout = layout.clone();
    }
    if let Some(payload_policy) = &section.payload_policy {
        config.storage.payload_policy = payload_policy.clone();
    }
    if let Some(adjacency_policy) = &section.adjacency_policy {
        config.storage.adjacency_policy = adjacency_policy.clone();
    }
    if let Some(cross_context_policy) = section.cross_context_policy {
        config.storage.cross_context_policy = cross_context_policy;
    }
    if let Some(cascade_delete_policy) = section.cascade_delete_policy {
        config.storage.cascade_delete_policy = cascade_delete_policy;
    }
}

fn apply_visibility_overrides(
    config: &mut RelationalRuntimeConfig,
    override_tree: &RelationalConfigOverride,
    provenance: &mut BTreeMap<String, ConfigProvenanceEntry>,
) {
    let section = &override_tree.visibility;
    provenance.insert(
        "visibility.cache_policy".to_string(),
        provenance_entry(section.cache_policy.is_some()),
    );
    if let Some(cache_policy) = &section.cache_policy {
        config.visibility.cache_policy = cache_policy.clone();
    }
}

fn apply_publication_overrides(
    config: &mut RelationalRuntimeConfig,
    override_tree: &RelationalConfigOverride,
    provenance: &mut BTreeMap<String, ConfigProvenanceEntry>,
) {
    let section = &override_tree.publication;
    provenance.insert(
        "publication.policy".to_string(),
        provenance_entry(section.policy.is_some()),
    );
    if let Some(policy) = &section.policy {
        config.publication.policy = policy.clone();
    }
}

fn apply_durability_overrides(
    config: &mut RelationalRuntimeConfig,
    override_tree: &RelationalConfigOverride,
    provenance: &mut BTreeMap<String, ConfigProvenanceEntry>,
) {
    let section = &override_tree.durability;
    provenance.insert(
        "durability.policy".to_string(),
        provenance_entry(section.policy.is_some()),
    );
    provenance.insert(
        "durability.mode".to_string(),
        provenance_entry(section.mode.is_some()),
    );
    provenance.insert(
        "durability.log".to_string(),
        provenance_entry(section.log.is_some()),
    );
    provenance.insert(
        "durability.store_layout".to_string(),
        provenance_entry(section.store_layout.is_some()),
    );
    if let Some(policy) = &section.policy {
        config.durability.policy = policy.clone();
    }
    if let Some(mode) = section.mode {
        config.durability.policy.mode = mode;
    }
    if let Some(log) = &section.log {
        config.durability.policy.log = log.clone();
        config
            .durability
            .policy
            .checkpoints
            .compact_after_checkpoint = log.compact_after_checkpoint;
    }
    if let Some(store_layout) = &section.store_layout {
        config.durability.policy.store_layout = Some(store_layout.clone());
    }
}

fn provenance_entry(overridden: bool) -> ConfigProvenanceEntry {
    if overridden {
        ConfigProvenanceEntry {
            source: ConfigValueSource::BuilderOverride,
            detail: "explicit builder override".to_string(),
        }
    } else {
        ConfigProvenanceEntry {
            source: ConfigValueSource::ProfileDefault,
            detail: "resolved from runtime profile".to_string(),
        }
    }
}
