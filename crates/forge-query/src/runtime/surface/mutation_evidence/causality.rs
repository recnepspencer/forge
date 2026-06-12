use crate::runtime::{ForgeQueryBridgeMutationArtifactIdentity, ForgeQueryMutationEvidenceDigest};
use forge_runtime_bridge::facade::BridgeMutationAuthorityBundle;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryMutationCausalityEvidence {
    causality_digest: ForgeQueryMutationEvidenceDigest,
    truth_trigger_digest: ForgeQueryMutationEvidenceDigest,
    route_digest: ForgeQueryMutationEvidenceDigest,
    evaluation_surface_digest: ForgeQueryMutationEvidenceDigest,
    truth_view_digest: ForgeQueryMutationEvidenceDigest,
}

impl ForgeQueryMutationCausalityEvidence {
    pub(in crate::runtime) fn from_bridge(bundle: &BridgeMutationAuthorityBundle) -> Self {
        let causality = bundle.causality();
        Self {
            causality_digest: imported_artifact("causality-root", causality.causality_digest()),
            truth_trigger_digest: imported_artifact(
                "causality-truth-trigger",
                causality.truth_trigger_digest(),
            ),
            route_digest: imported_artifact("causality-route", causality.route_digest()),
            evaluation_surface_digest: imported_artifact(
                "causality-evaluation-surface",
                causality.evaluation_surface_digest(),
            ),
            truth_view_digest: imported_artifact(
                "causality-truth-view",
                causality.truth_view_digest(),
            ),
        }
    }

    pub fn causality_digest(&self) -> &ForgeQueryMutationEvidenceDigest {
        &self.causality_digest
    }

    pub fn truth_trigger_digest(&self) -> &ForgeQueryMutationEvidenceDigest {
        &self.truth_trigger_digest
    }

    pub fn route_digest(&self) -> &ForgeQueryMutationEvidenceDigest {
        &self.route_digest
    }

    pub fn evaluation_surface_digest(&self) -> &ForgeQueryMutationEvidenceDigest {
        &self.evaluation_surface_digest
    }

    pub fn truth_view_digest(&self) -> &ForgeQueryMutationEvidenceDigest {
        &self.truth_view_digest
    }
}

fn imported_artifact(
    role: &'static str,
    artifact: impl Into<String>,
) -> ForgeQueryMutationEvidenceDigest {
    let artifact = ForgeQueryBridgeMutationArtifactIdentity::imported(role, artifact);
    ForgeQueryMutationEvidenceDigest::source_identity(role, artifact.evidence_identity())
}
