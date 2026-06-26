use crate::runtime::{WorthUiPlanNodeInputFamily, WorthUiQueryInspectionLinks};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthUiPlanProvenanceSource {
    ReplacementClassification,
    QueryBinding,
    ComponentLoweringHook,
    LaneBoundary,
    Diagnostics,
    RenderResource,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiArtifactToPlanProvenance {
    plan_index: u32,
    identity_basis: String,
    input_family: WorthUiPlanNodeInputFamily,
    source: WorthUiPlanProvenanceSource,
    capability_reference: Option<String>,
    query_links: Option<WorthUiQueryInspectionLinks>,
}

impl WorthUiArtifactToPlanProvenance {
    pub(crate) fn new(
        plan_index: u32,
        identity_basis: String,
        input_family: WorthUiPlanNodeInputFamily,
        source: WorthUiPlanProvenanceSource,
        capability_reference: Option<String>,
        query_links: Option<WorthUiQueryInspectionLinks>,
    ) -> Self {
        Self {
            plan_index,
            identity_basis,
            input_family,
            source,
            capability_reference,
            query_links,
        }
    }

    pub fn plan_index(&self) -> u32 {
        self.plan_index
    }

    pub fn identity_basis(&self) -> &str {
        &self.identity_basis
    }

    pub fn input_family(&self) -> WorthUiPlanNodeInputFamily {
        self.input_family
    }

    pub fn source(&self) -> WorthUiPlanProvenanceSource {
        self.source
    }

    pub fn capability_reference(&self) -> Option<&str> {
        self.capability_reference.as_deref()
    }

    pub fn query_links(&self) -> Option<&WorthUiQueryInspectionLinks> {
        self.query_links.as_ref()
    }
}
