use forge_runtime_bridge::facade::BridgeMutationAuthorityBundle;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryMutationCausalityEvidence {
    causality_digest: String,
    truth_trigger_digest: String,
    route_digest: String,
    evaluation_surface_digest: String,
    truth_view_digest: String,
}

impl ForgeQueryMutationCausalityEvidence {
    pub(in crate::runtime) fn from_bridge(bundle: &BridgeMutationAuthorityBundle) -> Self {
        let causality = bundle.causality();
        Self {
            causality_digest: causality.causality_digest().to_string(),
            truth_trigger_digest: causality.truth_trigger_digest().to_string(),
            route_digest: causality.route_digest().to_string(),
            evaluation_surface_digest: causality.evaluation_surface_digest().to_string(),
            truth_view_digest: causality.truth_view_digest().to_string(),
        }
    }

    pub fn causality_digest(&self) -> &str {
        &self.causality_digest
    }

    pub fn truth_trigger_digest(&self) -> &str {
        &self.truth_trigger_digest
    }

    pub fn route_digest(&self) -> &str {
        &self.route_digest
    }

    pub fn evaluation_surface_digest(&self) -> &str {
        &self.evaluation_surface_digest
    }

    pub fn truth_view_digest(&self) -> &str {
        &self.truth_view_digest
    }
}
