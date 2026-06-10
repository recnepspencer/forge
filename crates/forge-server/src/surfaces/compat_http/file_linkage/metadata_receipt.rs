use forge_foundational::facade::{
    BoundaryHandle, EquivalenceBasisId, FoundationalBoundaryEvidenceExecutedReceiptArtifact,
    FoundationalBoundaryEvidenceProvenanceArtifact, FoundationalBoundaryEvidenceReceiptBoundary,
    FoundationalBoundaryEvidenceReceiptFrontDoor, FoundationalCommitId,
    FoundationalCommitParentBasis, FoundationalCommitParentageLocator,
    FoundationalTransitionLocator,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerFileMetadataTruthKind {
    ObservedRead,
    ObservedInspection,
    CommittedMutation,
}

impl ForgeServerFileMetadataTruthKind {
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
pub struct ForgeServerFileMetadataReceipt {
    metadata_identity: String,
    tenant_id: String,
    workspace_digest: String,
    branch_digest: String,
    operation_name: String,
    truth_kind: ForgeServerFileMetadataTruthKind,
    truth_digest: String,
    basis_digest: Option<String>,
    receipt: FoundationalBoundaryEvidenceExecutedReceiptArtifact,
    canonical_digest: String,
}

impl ForgeServerFileMetadataReceipt {
    pub(crate) fn new(
        tenant_id: impl Into<String>,
        workspace_digest: impl Into<String>,
        branch_digest: impl Into<String>,
        operation_name: impl Into<String>,
        truth_kind: ForgeServerFileMetadataTruthKind,
        truth_digest: impl Into<String>,
        basis_digest: Option<String>,
        provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
    ) -> Self {
        let tenant_id = tenant_id.into().trim().to_string();
        let workspace_digest = workspace_digest.into().trim().to_string();
        let branch_digest = branch_digest.into().trim().to_string();
        let operation_name = operation_name.into().trim().to_string();
        let truth_digest = truth_digest.into().trim().to_string();
        let metadata_identity = format!(
            "forge-server-file-metadata-identity-v1|tenant={tenant_id}|workspace={workspace_digest}|branch={branch_digest}|operation={operation_name}"
        );
        let receipt = FoundationalBoundaryEvidenceReceiptFrontDoor
            .publication(receipt_boundary(
                "file-metadata",
                &metadata_identity,
                &truth_digest,
            ))
            .with_provenance(provenance);
        let canonical_digest = format!(
            "forge-server-file-metadata-receipt-v1|identity={metadata_identity}|truth_kind={}|truth={truth_digest}|basis={}|receipt_kind={:?}|receipt_locality={:?}",
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

    pub fn truth_kind(&self) -> ForgeServerFileMetadataTruthKind {
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
        "forge-server.file-linkage.metadata.commit".to_string(),
        boundary_family.to_string(),
        metadata_identity.to_string(),
        truth_digest.to_string(),
    ])));
    let parent_basis =
        FoundationalCommitParentBasis::new(EquivalenceBasisId::new(boundary_artifact_id(&[
            "forge-server.file-linkage.metadata.parent".to_string(),
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
