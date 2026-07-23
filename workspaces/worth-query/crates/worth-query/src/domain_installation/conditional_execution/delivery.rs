pub struct WorthQueryConditionalAuthoritativeChangeDeliveryRequest {
    location: worth_query_installation::facade::WorthQueryConditionalNodeLocation,
    dependency_ordinal: usize,
    committed_patch: worth_runtime_bridge::facade::RelationalCommittedPatchRequest,
}

impl WorthQueryConditionalAuthoritativeChangeDeliveryRequest {
    pub fn new(
        location: worth_query_installation::facade::WorthQueryConditionalNodeLocation,
        dependency_ordinal: usize,
        committed_patch: worth_runtime_bridge::facade::RelationalCommittedPatchRequest,
    ) -> Self {
        Self {
            location,
            dependency_ordinal,
            committed_patch,
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        worth_query_installation::facade::WorthQueryConditionalNodeLocation,
        usize,
        worth_runtime_bridge::facade::RelationalCommittedPatchRequest,
    ) {
        (self.location, self.dependency_ordinal, self.committed_patch)
    }
}

#[derive(Debug, Clone)]
pub enum WorthQueryConditionalDeliveryDenial {
    NodeNotInstalled,
    Bridge {
        kind: worth_runtime_bridge::facade::BridgeConditionalDenialKind,
        detail: String,
    },
}

impl WorthQueryConditionalDeliveryDenial {
    pub(crate) fn bridge(denial: worth_runtime_bridge::facade::BridgeConditionalDenial) -> Self {
        Self::Bridge {
            kind: denial.kind(),
            detail: denial.detail().to_string(),
        }
    }
}
