use crate::runtime::{
    ForgeQueryExistingTruthBindingDenial, ForgeQueryExistingTruthProbeDenial,
    ForgeQueryExistingTruthProbeRequest,
};

#[derive(Clone, Debug, PartialEq)]
pub enum ForgeQueryExistingTruthProbeRoutingPreflight {
    Admitted,
    BindingDenied(ForgeQueryExistingTruthBindingDenial),
    ProbeDenied(ForgeQueryExistingTruthProbeDenial),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryExistingTruthProbeIntentSeed {
    request: ForgeQueryExistingTruthProbeRequest,
    preflight: ForgeQueryExistingTruthProbeRoutingPreflight,
}

impl ForgeQueryExistingTruthProbeIntentSeed {
    pub fn new(
        request: ForgeQueryExistingTruthProbeRequest,
        preflight: ForgeQueryExistingTruthProbeRoutingPreflight,
    ) -> Self {
        Self { request, preflight }
    }

    pub fn request(&self) -> &ForgeQueryExistingTruthProbeRequest {
        &self.request
    }

    pub fn preflight(&self) -> &ForgeQueryExistingTruthProbeRoutingPreflight {
        &self.preflight
    }

    pub fn request_label(&self) -> String {
        format!(
            "probe.existing.{}",
            self.request.binding().authoritative_identity()
        )
    }

    pub fn request_input_digest(&self) -> String {
        self.request.request_digest().to_string()
    }
}
