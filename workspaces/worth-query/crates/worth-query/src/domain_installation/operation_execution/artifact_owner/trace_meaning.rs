use super::{
    WorthQueryArtifactDisposition, WorthQueryArtifactHandleCore,
    WorthQueryArtifactSemanticProjection,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryArtifactTraceMeaning {
    handle_identity: String,
    occurrence_identity: String,
    contract_identity: String,
    semantic_projection: WorthQueryArtifactSemanticProjection,
    disposition: WorthQueryArtifactDisposition,
}

impl WorthQueryArtifactTraceMeaning {
    pub fn handle_identity(&self) -> &str {
        &self.handle_identity
    }

    pub fn occurrence_identity(&self) -> &str {
        &self.occurrence_identity
    }

    pub fn contract_identity(&self) -> &str {
        &self.contract_identity
    }

    pub fn semantic_projection(&self) -> &WorthQueryArtifactSemanticProjection {
        &self.semantic_projection
    }

    pub const fn disposition(&self) -> WorthQueryArtifactDisposition {
        self.disposition
    }

    pub(crate) fn canonical_part(&self) -> String {
        crate::identity::hash_parts(&[
            "worth_query_artifact_trace_meaning_v1".into(),
            format!("handle:{}", self.handle_identity),
            format!("occurrence:{}", self.occurrence_identity),
            format!("contract:{}", self.contract_identity),
            format!(
                "projection:{}",
                self.semantic_projection.canonical_identity()
            ),
            format!("disposition:{}", self.disposition.canonical_name()),
        ])
    }

    pub(crate) fn set_disposition(&mut self, disposition: WorthQueryArtifactDisposition) {
        self.disposition = disposition;
    }

    pub(crate) fn semantic_replay_eq(&self, candidate: &Self) -> bool {
        self.contract_identity == candidate.contract_identity
            && self.semantic_projection == candidate.semantic_projection
            && self.disposition == candidate.disposition
    }
}

impl WorthQueryArtifactHandleCore {
    pub(super) fn trace_meaning(&self) -> WorthQueryArtifactTraceMeaning {
        WorthQueryArtifactTraceMeaning {
            handle_identity: self.handle_identity.clone(),
            occurrence_identity: self.owner.binding().occurrence_identity.clone(),
            contract_identity: self
                .owner
                .binding()
                .contract
                .contract()
                .identity()
                .as_str()
                .to_owned(),
            semantic_projection: self.owner.semantic_projection().clone(),
            disposition: self.disposition,
        }
    }
}
