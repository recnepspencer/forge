use serde_json::json;

use crate::facade::{
    ForgeQueryDerivedPatch, ForgeQueryDerivedView, ForgeQueryDerivedViewMaintainer,
    ForgeQueryDerivedViewMaterialization, ForgeQueryMutationDelta,
};
use crate::memory_workspace::ForgeQueryCommitIdentity;

pub(super) struct TranscriptMaintainer {
    pub(super) prefix: &'static str,
    pub(super) replace: bool,
}

impl ForgeQueryDerivedViewMaintainer for TranscriptMaintainer {
    fn maintain(
        &mut self,
        view: &ForgeQueryDerivedView,
        delta: &ForgeQueryMutationDelta,
        materialization: &mut ForgeQueryDerivedViewMaterialization,
    ) -> ForgeQueryDerivedPatch {
        let row = json!({
            "family": self.prefix,
            "entity": delta.entity_identity.terminal_projection_for_reporting().to_string(),
            "view": view.name(),
        });
        if self.replace {
            materialization.replace_rows([row.clone()]);
        } else {
            materialization.push_row(row.clone());
        }
        ForgeQueryDerivedPatch::incremental(
            view.name(),
            transcript_commit_identity("transcript-derived-commit", self.prefix),
            delta.entity_identity.clone(),
            if view.produced_aspects().is_empty() {
                delta.aspect_paths.clone()
            } else {
                view.produced_aspects().to_vec()
            },
            row,
        )
    }
}

fn transcript_commit_identity(namespace: &str, evidence: &str) -> ForgeQueryCommitIdentity {
    ForgeQueryCommitIdentity::from_relational_commit_id(stable_transcript_position(
        namespace, evidence,
    ))
}

fn stable_transcript_position(namespace: &str, evidence: &str) -> u64 {
    let mut acc = 14_695_981_039_346_656_037_u64;
    for byte in namespace.bytes().chain(evidence.bytes()) {
        acc ^= u64::from(byte);
        acc = acc.wrapping_mul(1_099_511_628_211_u64);
    }
    acc
}
