use crate::runtime::{
    WorthQueryExistingTruthBindingDenial, WorthQueryExistingTruthProbeDenial,
    WorthQueryExistingTruthProbeRequest,
};

#[derive(Clone, Debug, PartialEq)]
pub enum WorthQueryExistingTruthProbeRoutingPreflight {
    Admitted,
    BindingDenied(WorthQueryExistingTruthBindingDenial),
    ProbeDenied(WorthQueryExistingTruthProbeDenial),
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryExistingTruthProbeIntentSeed {
    request: WorthQueryExistingTruthProbeRequest,
    preflight: WorthQueryExistingTruthProbeRoutingPreflight,
}

impl WorthQueryExistingTruthProbeIntentSeed {
    pub fn new(
        request: WorthQueryExistingTruthProbeRequest,
        preflight: WorthQueryExistingTruthProbeRoutingPreflight,
    ) -> Self {
        Self { request, preflight }
    }

    pub fn request(&self) -> &WorthQueryExistingTruthProbeRequest {
        &self.request
    }

    pub fn preflight(&self) -> &WorthQueryExistingTruthProbeRoutingPreflight {
        &self.preflight
    }

    pub fn request_label(&self) -> String {
        format!(
            "probe.existing.{}",
            self.request.binding().authoritative_identity().as_str()
        )
    }

    pub fn request_input_digest(&self) -> String {
        self.request.request_digest().to_string()
    }
}
