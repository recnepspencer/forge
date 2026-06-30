use serde::Serialize;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize)]
pub enum ReplayUndoSemanticGraphPriorProofClass {
    TopologyDerivedInvalidationExecutionReceipt,
    SpatialEvidenceLookupExecutionReceipt,
}

impl ReplayUndoSemanticGraphPriorProofClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TopologyDerivedInvalidationExecutionReceipt => {
                "topology-derived-invalidation-execution-receipt"
            }
            Self::SpatialEvidenceLookupExecutionReceipt => {
                "spatial-evidence-lookup-execution-receipt"
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ReplayUndoSemanticGraphPriorProofIdentity {
    class: ReplayUndoSemanticGraphPriorProofClass,
    digest: String,
}

impl ReplayUndoSemanticGraphPriorProofIdentity {
    fn new(class: ReplayUndoSemanticGraphPriorProofClass, digest: impl Into<String>) -> Self {
        let digest = digest.into();
        assert!(
            !digest.trim().is_empty(),
            "replay/undo prior proof identity requires a non-empty digest"
        );
        Self { class, digest }
    }

    pub const fn class(&self) -> ReplayUndoSemanticGraphPriorProofClass {
        self.class
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn digest_part(&self) -> String {
        format!("{}:{}", self.class.as_str(), self.digest)
    }
}

pub fn admit_topology_derived_invalidation_prior_proof_identity(
    execution_receipt_digest: &str,
) -> ReplayUndoSemanticGraphPriorProofIdentity {
    ReplayUndoSemanticGraphPriorProofIdentity::new(
        ReplayUndoSemanticGraphPriorProofClass::TopologyDerivedInvalidationExecutionReceipt,
        execution_receipt_digest,
    )
}

pub fn admit_spatial_evidence_lookup_prior_proof_identity(
    execution_receipt_digest: &str,
) -> ReplayUndoSemanticGraphPriorProofIdentity {
    ReplayUndoSemanticGraphPriorProofIdentity::new(
        ReplayUndoSemanticGraphPriorProofClass::SpatialEvidenceLookupExecutionReceipt,
        execution_receipt_digest,
    )
}
