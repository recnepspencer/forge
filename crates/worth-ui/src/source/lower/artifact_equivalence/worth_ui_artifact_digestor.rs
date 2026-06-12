use crate::source::{
    WorthUiArtifact, WorthUiArtifactDigest, WorthUiArtifactDigestReport,
    WorthUiArtifactEquivalenceBasis, WorthUiArtifactEquivalenceMetrics,
};

use super::worth_ui_artifact_semantic_basis::artifact_semantic_basis;

#[derive(Clone, Debug, Default)]
pub(crate) struct WorthUiArtifactDigestor;

impl WorthUiArtifactDigestor {
    pub(crate) fn digest(
        artifact: &WorthUiArtifact,
        basis: WorthUiArtifactEquivalenceBasis,
    ) -> WorthUiArtifactDigest {
        Self::digest_with_report(artifact, basis).0
    }

    pub(crate) fn digest_with_report(
        artifact: &WorthUiArtifact,
        basis: WorthUiArtifactEquivalenceBasis,
    ) -> (WorthUiArtifactDigest, WorthUiArtifactDigestReport) {
        let semantic_basis = artifact_semantic_basis(artifact);
        let mut metrics = WorthUiArtifactEquivalenceMetrics::default();
        for module_id in artifact.module_ids() {
            metrics.record_module_compared();
            let module = artifact.module(module_id).expect("artifact module");
            for _node in module.nodes() {
                metrics.record_node_compared();
                metrics.record_semantic_payload_compared();
            }
        }

        (
            WorthUiArtifactDigest::new(basis, fold_text(&semantic_basis)),
            WorthUiArtifactDigestReport::new(basis, metrics),
        )
    }
}

fn fold_text(text: &str) -> u64 {
    let mut digest = 0xcbf2_9ce4_8422_2325u64;
    for byte in text.as_bytes() {
        digest ^= u64::from(*byte);
        digest = digest.wrapping_mul(0x100_0000_01b3);
    }
    digest
}
