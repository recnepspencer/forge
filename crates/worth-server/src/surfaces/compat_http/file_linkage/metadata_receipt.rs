use worth_foundational::facade::{
    BoundaryHandle, EquivalenceBasisId, FoundationalBoundaryEvidenceExecutedReceiptArtifact,
    FoundationalBoundaryEvidenceProvenanceArtifact, FoundationalBoundaryEvidenceReceiptBoundary,
    FoundationalBoundaryEvidenceReceiptFrontDoor, FoundationalCommitId,
    FoundationalCommitParentBasis, FoundationalCommitParentageLocator,
    FoundationalTransitionLocator,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerFileMetadataTruthKind {
    ObservedRead,
    ObservedInspection,
    CommittedMutation,
}

impl WorthServerFileMetadataTruthKind {
    pub fn truth_observed(self) -> bool {
        matches!(self, Self::ObservedRead | Self::ObservedInspection)
    }

    pub fn truth_committed(self) -> bool {
        matches!(self, Self::CommittedMutation)
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::ObservedRead => "observed_read",
            Self::ObservedInspection => "observed_inspection",
            Self::CommittedMutation => "committed_mutation",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerFileMetadataReceipt {
    metadata_identity: String,
    tenant_id: String,
    workspace_digest: String,
    branch_digest: String,
    operation_name: String,
    truth_kind: WorthServerFileMetadataTruthKind,
    truth_digest: String,
    basis_digest: Option<String>,
    receipt: FoundationalBoundaryEvidenceExecutedReceiptArtifact,
    canonical_digest: String,
}

pub(crate) struct WorthServerFileMetadataReceiptParts {
    pub(crate) tenant_id: String,
    pub(crate) workspace_digest: String,
    pub(crate) branch_digest: String,
    pub(crate) operation_name: String,
    pub(crate) truth_kind: WorthServerFileMetadataTruthKind,
    pub(crate) truth_digest: String,
    pub(crate) basis_digest: Option<String>,
    pub(crate) provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
}

impl WorthServerFileMetadataReceipt {
    pub(crate) fn new(parts: WorthServerFileMetadataReceiptParts) -> Self {
        let WorthServerFileMetadataReceiptParts {
            tenant_id,
            workspace_digest,
            branch_digest,
            operation_name,
            truth_kind,
            truth_digest,
            basis_digest,
            provenance,
        } = parts;
        let tenant_id = tenant_id.trim().to_string();
        let workspace_digest = workspace_digest.trim().to_string();
        let branch_digest = branch_digest.trim().to_string();
        let operation_name = operation_name.trim().to_string();
        let truth_digest = truth_digest.trim().to_string();
        let metadata_identity = format!(
            "worth-server-file-metadata-identity-v1|tenant={tenant_id}|workspace={workspace_digest}|branch={branch_digest}|operation={operation_name}"
        );
        let receipt = FoundationalBoundaryEvidenceReceiptFrontDoor
            .publication(receipt_boundary(
                "file-metadata",
                &metadata_identity,
                &truth_digest,
            ))
            .with_provenance(provenance);
        let canonical_digest = format!(
            "worth-server-file-metadata-receipt-v1|identity={metadata_identity}|truth_kind={}|truth={truth_digest}|basis={}|receipt_kind={:?}|receipt_locality={:?}",
            truth_kind.as_str(),
            basis_digest.as_deref().unwrap_or("none"),
            receipt.receipt_kind(),
            receipt.locality(),
        );
        Self {
            metadata_identity,
            tenant_id,
            workspace_digest,
            branch_digest,
            operation_name,
            truth_kind,
            truth_digest,
            basis_digest,
            receipt,
            canonical_digest,
        }
    }

    pub fn metadata_identity(&self) -> &str {
        &self.metadata_identity
    }

    pub fn tenant_id(&self) -> &str {
        &self.tenant_id
    }

    pub fn workspace_digest(&self) -> &str {
        &self.workspace_digest
    }

    pub fn branch_digest(&self) -> &str {
        &self.branch_digest
    }

    pub fn operation_name(&self) -> &str {
        &self.operation_name
    }

    pub fn truth_kind(&self) -> WorthServerFileMetadataTruthKind {
        self.truth_kind
    }

    pub fn truth_observed(&self) -> bool {
        self.truth_kind.truth_observed()
    }

    pub fn truth_committed(&self) -> bool {
        self.truth_kind.truth_committed()
    }

    pub fn truth_digest(&self) -> &str {
        &self.truth_digest
    }

    pub fn basis_digest(&self) -> Option<&str> {
        self.basis_digest.as_deref()
    }

    pub fn receipt(&self) -> &FoundationalBoundaryEvidenceExecutedReceiptArtifact {
        &self.receipt
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}

fn receipt_boundary(
    boundary_family: &str,
    metadata_identity: &str,
    truth_digest: &str,
) -> FoundationalBoundaryEvidenceReceiptBoundary {
    let commit_id = FoundationalCommitId::new(BoundaryHandle::new(boundary_artifact_id(&[
        "worth-server.file-linkage.metadata.commit".to_string(),
        boundary_family.to_string(),
        metadata_identity.to_string(),
        truth_digest.to_string(),
    ])));
    let parent_basis =
        FoundationalCommitParentBasis::new(EquivalenceBasisId::new(boundary_artifact_id(&[
            "worth-server.file-linkage.metadata.parent".to_string(),
            boundary_family.to_string(),
            metadata_identity.to_string(),
            truth_digest.to_string(),
        ])));
    FoundationalBoundaryEvidenceReceiptBoundary::transition(
        FoundationalTransitionLocator::CommitParentage(FoundationalCommitParentageLocator::new(
            commit_id,
            parent_basis,
        )),
    )
}

fn boundary_artifact_id(parts: &[String]) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    for part in parts {
        for byte in part.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= 0x1f;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
