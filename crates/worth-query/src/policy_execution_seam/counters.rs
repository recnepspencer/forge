#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PolicyAwareSeamCounters {
    seam_admission_count: usize,
    seam_denial_count: usize,
    authorized_projection_width: usize,
    relationship_proof_topology_width: usize,
    tenant_schema_basis_count: usize,
    delivery_width: usize,
    live_relevance_field_width: usize,
    plan_digest_part_count: usize,
    executor_semantic_rediscovery_count: usize,
    raw_plan_bypass_denial_count: usize,
    raw_diff_scrub_denial_count: usize,
    raw_live_relevance_denial_count: usize,
    delivery_overexposure_denial_count: usize,
    placeholder_masking_denial_count: usize,
    store_backed_deferred_count: usize,
    durable_cursor_deferred_count: usize,
    durable_artifact_reload_deferred_count: usize,
    durable_delivery_metadata_deferred_count: usize,
    durable_overclaim_denial_count: usize,
    per_row_allocation_denial_count: usize,
    cross_tenant_fanout_denial_count: usize,
    saved_query_policy_bypass_denial_count: usize,
    unsupported_policy_workflow_composition_denial_count: usize,
    policy_epoch_drift_readmission_count: usize,
    tenant_basis_drift_readmission_count: usize,
    policy_sparse_to_burst_readmission_count: usize,
    policy_dense_restart_debt_count: usize,
}

impl PolicyAwareSeamCounters {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn admitted(
        authorized_projection_width: usize,
        relationship_proof_topology_width: usize,
        delivery_width: usize,
        live_relevance_field_width: usize,
        plan_digest_part_count: usize,
    ) -> Self {
        Self {
            seam_admission_count: 1,
            tenant_schema_basis_count: 2,
            authorized_projection_width,
            relationship_proof_topology_width,
            delivery_width,
            live_relevance_field_width,
            plan_digest_part_count,
            ..Self::default()
        }
    }
    #[cfg(test)]
    pub(crate) fn denied_raw_plan_bypass() -> Self {
        Self {
            seam_denial_count: 1,
            raw_plan_bypass_denial_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn denied_raw_diff_scrub() -> Self {
        Self {
            seam_denial_count: 1,
            raw_diff_scrub_denial_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn denied_raw_live_relevance() -> Self {
        Self {
            seam_denial_count: 1,
            raw_live_relevance_denial_count: 1,
            ..Self::default()
        }
    }
    #[cfg(test)]
    pub(crate) fn denied_delivery_overexposure() -> Self {
        Self {
            seam_denial_count: 1,
            delivery_overexposure_denial_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn denied_placeholder_masking() -> Self {
        Self {
            seam_denial_count: 1,
            placeholder_masking_denial_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn deferred_store_backed() -> Self {
        Self {
            seam_denial_count: 1,
            store_backed_deferred_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn deferred_durable_cursor() -> Self {
        Self {
            seam_denial_count: 1,
            durable_cursor_deferred_count: 1,
            durable_overclaim_denial_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn deferred_durable_artifact_reload() -> Self {
        Self {
            seam_denial_count: 1,
            durable_artifact_reload_deferred_count: 1,
            durable_overclaim_denial_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn deferred_durable_delivery_metadata() -> Self {
        Self {
            seam_denial_count: 1,
            durable_delivery_metadata_deferred_count: 1,
            durable_overclaim_denial_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn denied_per_row_allocation() -> Self {
        Self {
            seam_denial_count: 1,
            per_row_allocation_denial_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn denied_cross_tenant_fanout() -> Self {
        Self {
            seam_denial_count: 1,
            cross_tenant_fanout_denial_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn denied_saved_query_policy_bypass() -> Self {
        Self {
            seam_denial_count: 1,
            saved_query_policy_bypass_denial_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn denied_unsupported_policy_workflow_composition() -> Self {
        Self {
            seam_denial_count: 1,
            unsupported_policy_workflow_composition_denial_count: 1,
            ..Self::default()
        }
    }
    #[cfg(test)]
    pub(crate) fn denied_policy_dense_restart_debt() -> Self {
        Self {
            seam_denial_count: 1,
            raw_live_relevance_denial_count: 1,
            policy_dense_restart_debt_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn record_policy_epoch_drift_readmission(mut self) -> Self {
        self.policy_epoch_drift_readmission_count += 1;
        self
    }

    pub(crate) fn record_tenant_basis_drift_readmission(mut self) -> Self {
        self.tenant_basis_drift_readmission_count += 1;
        self
    }

    pub(crate) fn record_policy_sparse_to_burst_readmission(mut self) -> Self {
        self.policy_sparse_to_burst_readmission_count += 1;
        self
    }

    pub fn seam_admission_count(&self) -> usize {
        self.seam_admission_count
    }

    pub fn seam_denial_count(&self) -> usize {
        self.seam_denial_count
    }

    pub fn authorized_projection_width(&self) -> usize {
        self.authorized_projection_width
    }

    pub fn relationship_proof_topology_width(&self) -> usize {
        self.relationship_proof_topology_width
    }

    pub fn tenant_schema_basis_count(&self) -> usize {
        self.tenant_schema_basis_count
    }

    pub fn delivery_width(&self) -> usize {
        self.delivery_width
    }

    pub fn live_relevance_field_width(&self) -> usize {
        self.live_relevance_field_width
    }

    pub fn plan_digest_part_count(&self) -> usize {
        self.plan_digest_part_count
    }

    pub fn executor_semantic_rediscovery_count(&self) -> usize {
        self.executor_semantic_rediscovery_count
    }

    pub fn raw_plan_bypass_denial_count(&self) -> usize {
        self.raw_plan_bypass_denial_count
    }

    pub fn raw_diff_scrub_denial_count(&self) -> usize {
        self.raw_diff_scrub_denial_count
    }

    pub fn raw_live_relevance_denial_count(&self) -> usize {
        self.raw_live_relevance_denial_count
    }

    pub fn delivery_overexposure_denial_count(&self) -> usize {
        self.delivery_overexposure_denial_count
    }

    pub fn placeholder_masking_denial_count(&self) -> usize {
        self.placeholder_masking_denial_count
    }

    pub fn store_backed_deferred_count(&self) -> usize {
        self.store_backed_deferred_count
    }

    pub fn durable_cursor_deferred_count(&self) -> usize {
        self.durable_cursor_deferred_count
    }

    pub fn durable_artifact_reload_deferred_count(&self) -> usize {
        self.durable_artifact_reload_deferred_count
    }

    pub fn durable_delivery_metadata_deferred_count(&self) -> usize {
        self.durable_delivery_metadata_deferred_count
    }

    pub fn durable_overclaim_denial_count(&self) -> usize {
        self.durable_overclaim_denial_count
    }

    pub fn per_row_allocation_denial_count(&self) -> usize {
        self.per_row_allocation_denial_count
    }

    pub fn cross_tenant_fanout_denial_count(&self) -> usize {
        self.cross_tenant_fanout_denial_count
    }

    pub fn saved_query_policy_bypass_denial_count(&self) -> usize {
        self.saved_query_policy_bypass_denial_count
    }

    pub fn unsupported_policy_workflow_composition_denial_count(&self) -> usize {
        self.unsupported_policy_workflow_composition_denial_count
    }

    pub fn policy_epoch_drift_readmission_count(&self) -> usize {
        self.policy_epoch_drift_readmission_count
    }

    pub fn tenant_basis_drift_readmission_count(&self) -> usize {
        self.tenant_basis_drift_readmission_count
    }

    pub fn policy_sparse_to_burst_readmission_count(&self) -> usize {
        self.policy_sparse_to_burst_readmission_count
    }

    pub fn policy_dense_restart_debt_count(&self) -> usize {
        self.policy_dense_restart_debt_count
    }

    pub(crate) fn digest_parts(&self) -> Vec<String> {
        vec![
            format!("seam_admission:{}", self.seam_admission_count),
            format!("seam_denial:{}", self.seam_denial_count),
            format!("authorized_width:{}", self.authorized_projection_width),
            format!(
                "proof_topology_width:{}",
                self.relationship_proof_topology_width
            ),
            format!("tenant_schema_basis:{}", self.tenant_schema_basis_count),
            format!("delivery_width:{}", self.delivery_width),
            format!("live_relevance_width:{}", self.live_relevance_field_width),
            format!("plan_digest_parts:{}", self.plan_digest_part_count),
            format!(
                "executor_rediscovery:{}",
                self.executor_semantic_rediscovery_count
            ),
            format!("raw_plan_bypass:{}", self.raw_plan_bypass_denial_count),
            format!("raw_diff_scrub:{}", self.raw_diff_scrub_denial_count),
            format!(
                "raw_live_relevance:{}",
                self.raw_live_relevance_denial_count
            ),
            format!(
                "delivery_overexposure:{}",
                self.delivery_overexposure_denial_count
            ),
            format!(
                "placeholder_masking:{}",
                self.placeholder_masking_denial_count
            ),
            format!("store_deferred:{}", self.store_backed_deferred_count),
            format!(
                "durable_cursor_deferred:{}",
                self.durable_cursor_deferred_count
            ),
            format!(
                "durable_artifact_reload_deferred:{}",
                self.durable_artifact_reload_deferred_count
            ),
            format!(
                "durable_delivery_metadata_deferred:{}",
                self.durable_delivery_metadata_deferred_count
            ),
            format!(
                "durable_overclaim_denial:{}",
                self.durable_overclaim_denial_count
            ),
            format!(
                "per_row_allocation_denial:{}",
                self.per_row_allocation_denial_count
            ),
            format!(
                "cross_tenant_fanout_denial:{}",
                self.cross_tenant_fanout_denial_count
            ),
            format!(
                "saved_query_policy_bypass_denial:{}",
                self.saved_query_policy_bypass_denial_count
            ),
            format!(
                "unsupported_policy_workflow_composition_denial:{}",
                self.unsupported_policy_workflow_composition_denial_count
            ),
            format!(
                "policy_epoch_drift_readmission:{}",
                self.policy_epoch_drift_readmission_count
            ),
            format!(
                "tenant_basis_drift_readmission:{}",
                self.tenant_basis_drift_readmission_count
            ),
            format!(
                "policy_sparse_to_burst_readmission:{}",
                self.policy_sparse_to_burst_readmission_count
            ),
            format!(
                "policy_dense_restart_debt:{}",
                self.policy_dense_restart_debt_count
            ),
        ]
    }
}
