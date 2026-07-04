use crate::evidence::{
    evidence_authority_binding, evidence_handle, evidence_identity, evidence_ref,
    UiEvidenceAuthorityBinding, UiEvidenceAuthorityGeneration, UiEvidenceAuthorityKind,
    UiEvidenceFamily, UiEvidenceIdentity, UiEvidenceMaterializationPosture, UiEvidenceRef,
    UiEvidenceRetentionPosture,
};
use crate::graph::UiGraphSnapshot;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiGraphEvidenceRecord {
    identity: UiEvidenceIdentity,
    authority_binding: UiEvidenceAuthorityBinding,
    reference: UiEvidenceRef,
}

impl UiGraphEvidenceRecord {
    #[allow(dead_code)]
    pub(crate) fn for_snapshot(snapshot: &UiGraphSnapshot, graph_node_digest: u64) -> Self {
        let identity = evidence_identity(UiEvidenceFamily::Graph, graph_node_digest);
        let authority_binding = evidence_authority_binding(
            UiEvidenceAuthorityKind::GraphSnapshot,
            snapshot.generation().as_u64(),
            UiEvidenceAuthorityGeneration::new(snapshot.generation().as_u64()),
            None,
        );
        let handle = evidence_handle(UiEvidenceFamily::Graph, identity, graph_node_digest);

        Self {
            identity,
            authority_binding,
            reference: evidence_ref(
                UiEvidenceFamily::Graph,
                identity,
                authority_binding,
                UiEvidenceMaterializationPosture::SummaryAvailable,
                UiEvidenceRetentionPosture::CurrentGenerationOnly,
                handle,
            ),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn reference(&self) -> UiEvidenceRef {
        self.reference
    }
}

#[cfg(test)]
mod tests {
    use crate::facade::WorthUi;
    use crate::graph::UiGraphWorldProfile;
    use worth_ui_dsl::{
        UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey, UiDslSourceProvenance,
        UiDslStructuralToken, WorthUiDslPackage,
    };

    use super::{UiEvidenceFamily, UiGraphEvidenceRecord};

    #[test]
    fn graph_evidence_refs_bind_the_real_snapshot_generation() {
        let app = WorthUi::app()
            .with_dsl_package(
                WorthUiDslPackage::named("worth-ui.runtime.graph.evidence")
                    .with_semantic_artifact_spec(
                        UiDslSemanticArtifactSpec::new(
                            UiDslSemanticKey::new("ui.graph.evidence.record"),
                            UiDslSemanticFamily::Control,
                            UiDslSourceProvenance::file_authored(
                                "app/graph_evidence_record_tests.wui",
                                0,
                            ),
                        )
                        .with_structural_token(UiDslStructuralToken::new("control:test")),
                    ),
            )
            .freeze();
        let snapshot = app.graph_snapshot();
        let graph_node_digest = snapshot.nodes()[0].graph_node_identity().digest();
        let record = UiGraphEvidenceRecord::for_snapshot(snapshot, graph_node_digest);
        let evidence_ref = record.reference();

        assert_eq!(
            snapshot.world_profile(),
            &UiGraphWorldProfile::authoritative()
        );
        assert_eq!(evidence_ref.family(), UiEvidenceFamily::Graph);
        assert_eq!(
            evidence_ref.authority_generation().as_u64(),
            snapshot.generation().as_u64()
        );
        assert_eq!(
            evidence_ref
                .authority_binding()
                .artifact_identity()
                .digest(),
            snapshot.generation().as_u64()
        );
    }
}
