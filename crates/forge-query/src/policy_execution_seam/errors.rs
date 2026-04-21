use super::PolicyAwareSeamCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PolicyAwareExecutionSeamFailureClass {
    RawCanonicalQueryBypass,
    RawExecutionPlanBypass,
    RawDiffScrubForbidden,
    RawLiveRelevanceForbidden,
    DeliveryShapeOverexposure,
    PlaceholderMaskingForbidden,
    UnsupportedPolicyExecutionMode,
    StoreBackedPolicyExecutionDeferred,
    DurablePolicyCursorDeferred,
    DurablePolicyArtifactReloadDeferred,
    DurablePolicyDeliveryMetadataDeferred,
    PerRowPolicyAllocationForbidden,
    CrossTenantPolicyFanoutForbidden,
    SavedQueryPolicyBypassForbidden,
    UnsupportedPolicyWorkflowComposition,
}

impl PolicyAwareExecutionSeamFailureClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RawCanonicalQueryBypass => "raw_canonical_query_bypass",
            Self::RawExecutionPlanBypass => "raw_execution_plan_bypass",
            Self::RawDiffScrubForbidden => "raw_diff_scrub_forbidden",
            Self::RawLiveRelevanceForbidden => "raw_live_relevance_forbidden",
            Self::DeliveryShapeOverexposure => "delivery_shape_overexposure",
            Self::PlaceholderMaskingForbidden => "placeholder_masking_forbidden",
            Self::UnsupportedPolicyExecutionMode => "unsupported_policy_execution_mode",
            Self::StoreBackedPolicyExecutionDeferred => "store_backed_policy_execution_deferred",
            Self::DurablePolicyCursorDeferred => "durable_policy_cursor_deferred",
            Self::DurablePolicyArtifactReloadDeferred => "durable_policy_artifact_reload_deferred",
            Self::DurablePolicyDeliveryMetadataDeferred => {
                "durable_policy_delivery_metadata_deferred"
            }
            Self::PerRowPolicyAllocationForbidden => "per_row_policy_allocation_forbidden",
            Self::CrossTenantPolicyFanoutForbidden => "cross_tenant_policy_fanout_forbidden",
            Self::SavedQueryPolicyBypassForbidden => "saved_query_policy_bypass_forbidden",
            Self::UnsupportedPolicyWorkflowComposition => "unsupported_policy_workflow_composition",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyAwareExecutionSeamError {
    failure_class: PolicyAwareExecutionSeamFailureClass,
    message: &'static str,
    counters: PolicyAwareSeamCounters,
}

impl PolicyAwareExecutionSeamError {
    pub(crate) fn new(
        failure_class: PolicyAwareExecutionSeamFailureClass,
        message: &'static str,
        counters: PolicyAwareSeamCounters,
    ) -> Self {
        Self {
            failure_class,
            message,
            counters,
        }
    }

    pub fn failure_class(&self) -> PolicyAwareExecutionSeamFailureClass {
        self.failure_class
    }

    pub fn message(&self) -> &'static str {
        self.message
    }

    pub fn counters(&self) -> &PolicyAwareSeamCounters {
        &self.counters
    }
}
