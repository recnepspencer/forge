use crate::source::{
    WorthUiArtifactCapabilityReferenceInspection, WorthUiArtifactHandle,
    WorthUiArtifactIdentitySeed, WorthUiArtifactNodeKind, WorthUiArtifactSourceOrigin,
    WorthUiDurableStateEligibility, WorthUiQueryInspectionLink,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiArtifactNodeInspection {
    handle: WorthUiArtifactHandle,
    node_kind: WorthUiArtifactNodeKind,
    source_origin: WorthUiArtifactSourceOrigin,
    identity_seed: WorthUiArtifactIdentitySeed,
    durable_state_eligibility: WorthUiDurableStateEligibility,
    capability_references: Vec<WorthUiArtifactCapabilityReferenceInspection>,
    query_inspection_links: Vec<WorthUiQueryInspectionLink>,
}

impl WorthUiArtifactNodeInspection {
    pub(crate) fn new(
        handle: WorthUiArtifactHandle,
        node_kind: WorthUiArtifactNodeKind,
        source_origin: WorthUiArtifactSourceOrigin,
        identity_seed: WorthUiArtifactIdentitySeed,
        durable_state_eligibility: WorthUiDurableStateEligibility,
        capability_references: Vec<WorthUiArtifactCapabilityReferenceInspection>,
        query_inspection_links: Vec<WorthUiQueryInspectionLink>,
    ) -> Self {
        Self {
            handle,
            node_kind,
            source_origin,
            identity_seed,
            durable_state_eligibility,
            capability_references,
            query_inspection_links,
        }
    }

    pub(crate) fn handle(&self) -> &WorthUiArtifactHandle {
        &self.handle
    }

    pub(crate) fn node_kind(&self) -> WorthUiArtifactNodeKind {
        self.node_kind
    }

    pub(crate) fn source_origin(&self) -> &WorthUiArtifactSourceOrigin {
        &self.source_origin
    }

    pub(crate) fn capability_references(&self) -> &[WorthUiArtifactCapabilityReferenceInspection] {
        &self.capability_references
    }

    pub(crate) fn query_inspection_links(&self) -> &[WorthUiQueryInspectionLink] {
        &self.query_inspection_links
    }
}
