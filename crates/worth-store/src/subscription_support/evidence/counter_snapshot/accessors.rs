use super::SubscriptionSupportCounterSnapshot;

impl SubscriptionSupportCounterSnapshot {
    pub fn declarations_admitted(&self) -> u64 {
        self.declarations_admitted
    }

    pub fn declarations_rejected(&self) -> u64 {
        self.declarations_rejected
    }

    pub fn budget_denials(&self) -> u64 {
        self.budget_denials
    }

    pub fn artifacts_published(&self) -> u64 {
        self.artifacts_published
    }

    pub fn artifacts_fetched(&self) -> u64 {
        self.artifacts_fetched
    }

    pub fn family_catalog_lookups(&self) -> u64 {
        self.family_catalog_lookups
    }

    pub fn lookup_keys_used(&self) -> u64 {
        self.lookup_keys_used
    }

    pub fn rows_read(&self) -> u64 {
        self.rows_read
    }

    pub fn duplicate_retries(&self) -> u64 {
        self.duplicate_retries
    }

    pub fn identity_collisions(&self) -> u64 {
        self.identity_collisions
    }

    pub fn access_structure_debts(&self) -> u64 {
        self.access_structure_debts
    }

    pub fn malformed_support_records(&self) -> u64 {
        self.malformed_support_records
    }

    pub fn exact_classifications(&self) -> u64 {
        self.exact_classifications
    }

    pub fn degraded_classifications(&self) -> u64 {
        self.degraded_classifications
    }

    pub fn rebuild_required_classifications(&self) -> u64 {
        self.rebuild_required_classifications
    }

    pub fn denied_classifications(&self) -> u64 {
        self.denied_classifications
    }

    pub fn restart_reconstruction_count(&self) -> u64 {
        self.restart_reconstruction_count
    }

    pub fn restart_shards_touched(&self) -> u64 {
        self.restart_shards_touched
    }

    pub fn restart_global_scan_count(&self) -> u64 {
        self.restart_global_scan_count
    }

    pub fn rebuild_basis_plan_count(&self) -> u64 {
        self.rebuild_basis_plan_count
    }

    pub fn runtime_handoff_count(&self) -> u64 {
        self.runtime_handoff_count
    }

    pub fn operational_verdict_translation_count(&self) -> u64 {
        self.operational_verdict_translation_count
    }

    pub fn operational_verdict_translation_rejections(&self) -> u64 {
        self.operational_verdict_translation_rejections
    }

    pub fn support_action_envelope_publications(&self) -> u64 {
        self.support_action_envelope_publications
    }

    pub fn support_action_recovery_count(&self) -> u64 {
        self.support_action_recovery_count
    }

    pub fn support_action_interrupted_recovery_count(&self) -> u64 {
        self.support_action_recovery_count
    }

    pub fn support_hidden_exact_loss_count(&self) -> u64 {
        self.support_hidden_exact_loss_count
    }

    pub fn support_hot_path_rejections(&self) -> u64 {
        self.support_hot_path_rejections
    }

    pub fn support_payload_budget_rejection_count(&self) -> u64 {
        self.support_payload_budget_rejection_count
    }

    pub fn support_batch_receipt_reuse_count(&self) -> u64 {
        self.support_batch_receipt_reuse_count
    }

    pub fn support_store_global_debt_rejections(&self) -> u64 {
        self.support_store_global_debt_rejections
    }

    pub fn support_global_scan_recovery_rejection_count(&self) -> u64 {
        self.support_global_scan_recovery_rejection_count
    }

    pub fn support_retention_plan_count(&self) -> u64 {
        self.support_retention_plan_count
    }

    pub fn support_retention_affected_entries(&self) -> u64 {
        self.support_retention_affected_entries
    }

    pub fn support_retained_family_count(&self) -> u64 {
        self.support_retained_family_count
    }

    pub fn support_reclaimed_family_count(&self) -> u64 {
        self.support_reclaimed_family_count
    }

    pub fn support_compacted_basis_count(&self) -> u64 {
        self.support_compacted_basis_count
    }

    pub fn support_expired_family_count(&self) -> u64 {
        self.support_expired_family_count
    }

    pub fn support_reclaim_consequence_count(&self) -> u64 {
        self.support_reclaim_consequence_count
    }

    pub fn support_policy_expiration_count(&self) -> u64 {
        self.support_policy_expiration_count
    }

    pub fn support_compatibility_plan_count(&self) -> u64 {
        self.support_compatibility_plan_count
    }

    pub fn support_compatibility_affected_entries(&self) -> u64 {
        self.support_compatibility_affected_entries
    }

    pub fn support_manifest_admission_count(&self) -> u64 {
        self.support_manifest_admission_count
    }

    pub fn support_compatibility_receipt_binding_count(&self) -> u64 {
        self.support_compatibility_receipt_binding_count
    }

    pub fn support_exact_compatible_migration_count(&self) -> u64 {
        self.support_exact_compatible_migration_count
    }

    pub fn support_degraded_compatibility_count(&self) -> u64 {
        self.support_degraded_compatibility_count
    }

    pub fn support_version_skew_rejection_count(&self) -> u64 {
        self.support_version_skew_rejection_count
    }

    pub fn support_portability_plan_count(&self) -> u64 {
        self.support_portability_plan_count
    }

    pub fn support_portability_manifest_entries(&self) -> u64 {
        self.support_portability_manifest_entries
    }

    pub fn support_portability_required_basis_count(&self) -> u64 {
        self.support_portability_required_basis_count
    }

    pub fn support_portability_omitted_support_count(&self) -> u64 {
        self.support_portability_omitted_support_count
    }

    pub fn support_replication_inclusion_count(&self) -> u64 {
        self.support_replication_inclusion_count
    }

    pub fn support_replication_omission_count(&self) -> u64 {
        self.support_replication_omission_count
    }

    pub fn support_import_admission_count(&self) -> u64 {
        self.support_import_admission_count
    }

    pub fn support_import_rejection_count(&self) -> u64 {
        self.support_import_rejection_count
    }

    pub fn support_capsule_manifest_budget_denial_count(&self) -> u64 {
        self.support_capsule_manifest_budget_denial_count
    }

    pub fn support_maintenance_descriptor_count(&self) -> u64 {
        self.support_maintenance_descriptor_count
    }

    pub fn support_maintenance_delay_count(&self) -> u64 {
        self.support_maintenance_delay_count
    }

    pub fn support_maintenance_rebuild_debt_count(&self) -> u64 {
        self.support_maintenance_rebuild_debt_count
    }

    pub fn support_maintenance_refresh_count(&self) -> u64 {
        self.support_maintenance_refresh_count
    }

    pub fn support_maintenance_compatibility_migration_count(&self) -> u64 {
        self.support_maintenance_compatibility_migration_count
    }

    pub fn support_maintenance_degradation_recovery_count(&self) -> u64 {
        self.support_maintenance_degradation_recovery_count
    }

    pub fn support_maintenance_coalesced_duplicate_count(&self) -> u64 {
        self.support_maintenance_coalesced_duplicate_count
    }

    pub fn support_maintenance_interrupted_restart_recovery_count(&self) -> u64 {
        self.support_maintenance_interrupted_restart_recovery_count
    }
}
