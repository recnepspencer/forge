use crate::durability::data::RecoveryAuthorityContinuityMismatch;

pub(crate) fn recovery_basis_mismatch(
    checkpoint_manifest: Option<&crate::durability::data::DurableCheckpointManifest>,
    runtime_registry: &crate::schema::data::RelationalSchemaRegistry,
    runtime_profile: crate::config::data::RelationalRuntimeProfile,
    runtime_name: &str,
    primary_schema_version_id: crate::schema::data::SchemaVersionId,
) -> Option<RecoveryAuthorityContinuityMismatch> {
    let manifest = checkpoint_manifest?;
    if manifest.schema_version != primary_schema_version_id {
        return Some(RecoveryAuthorityContinuityMismatch::SchemaRegistryShape {
            expected_primary_schema_version: manifest.schema_version,
            found_primary_schema_version: primary_schema_version_id,
            expected_entity_kind_count: runtime_registry.entity_kinds.len(),
            found_entity_kind_count: runtime_registry.entity_kinds.len(),
            expected_relation_kind_count: runtime_registry.relation_kinds.len(),
            found_relation_kind_count: runtime_registry.relation_kinds.len(),
        });
    }
    if manifest.profile != runtime_profile {
        return Some(RecoveryAuthorityContinuityMismatch::RuntimeProfile {
            expected: format!("{:?}", manifest.profile),
            found: format!("{runtime_profile:?}"),
        });
    }
    if manifest.runtime_name != runtime_name {
        return Some(RecoveryAuthorityContinuityMismatch::RuntimeName {
            expected: manifest.runtime_name.clone(),
            found: runtime_name.to_string(),
        });
    }
    None
}
