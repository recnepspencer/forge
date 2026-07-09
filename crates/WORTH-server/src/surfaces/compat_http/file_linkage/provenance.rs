use worth_foundational::facade::{
    BoundaryArtifactField, BoundaryArtifactId, BoundaryArtifactLocator, DiagnosticRichnessProfile,
    FoundationalBoundaryEvidenceFreshnessPosture, FoundationalBoundaryEvidenceProvenanceArtifact,
    FoundationalBoundaryEvidenceProvenanceFrontDoor, FoundationalBoundaryEvidenceSourceBasis,
};
use worth_proof::TransitionOutcome;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerFileTransferDisposition {
    MetadataOnlyObservation,
    VerifiedIngress,
    SelectedEgress,
    HeadOnlyEgress,
}

impl WorthServerFileTransferDisposition {
    pub fn byte_motion_observed(self) -> bool {
        matches!(self, Self::VerifiedIngress | Self::SelectedEgress)
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::MetadataOnlyObservation => "metadata_only_observation",
            Self::VerifiedIngress => "verified_ingress",
            Self::SelectedEgress => "selected_egress",
            Self::HeadOnlyEgress => "head_only_egress",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerFileTransferProvenance {
    metadata_identity: String,
    tenant_id: String,
    workspace_digest: String,
    branch_digest: String,
    operation_name: String,
    diagnostics_profile: DiagnosticRichnessProfile,
    disposition: WorthServerFileTransferDisposition,
    content_type: Option<String>,
    bytes_selected: u64,
    range_honored: bool,
    provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
    canonical_digest: String,
}

impl WorthServerFileTransferProvenance {
    pub(crate) fn new(
        metadata_identity: impl Into<String>,
        tenant_id: impl Into<String>,
        workspace_digest: impl Into<String>,
        branch_digest: impl Into<String>,
        operation_name: impl Into<String>,
        diagnostics_profile: DiagnosticRichnessProfile,
        disposition: WorthServerFileTransferDisposition,
        content_type: Option<String>,
        bytes_selected: u64,
        range_honored: bool,
    ) -> Self {
        let metadata_identity = metadata_identity.into().trim().to_string();
        let tenant_id = tenant_id.into().trim().to_string();
        let workspace_digest = workspace_digest.into().trim().to_string();
        let branch_digest = branch_digest.into().trim().to_string();
        let operation_name = operation_name.into().trim().to_string();
        let provenance = build_provenance(
            disposition,
            &metadata_identity,
            &workspace_digest,
            &branch_digest,
            &operation_name,
        );
        let canonical_digest = format!(
            "worth-server-file-transfer-provenance-v1|identity={metadata_identity}|tenant={tenant_id}|workspace={workspace_digest}|branch={branch_digest}|operation={operation_name}|disposition={}|content_type={}|bytes={bytes_selected}|range_honored={range_honored}|diagnostics={:?}|locality={:?}|freshness={:?}|source_kind={:?}",
            disposition.as_str(),
            content_type.as_deref().unwrap_or("none"),
            diagnostics_profile,
            provenance.locality(),
            provenance.freshness_posture(),
            provenance.source_basis().kind(),
        );
        Self {
            metadata_identity,
            tenant_id,
            workspace_digest,
            branch_digest,
            operation_name,
            diagnostics_profile,
            disposition,
            content_type,
            bytes_selected,
            range_honored,
            provenance,
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

    pub fn diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        self.diagnostics_profile
    }

    pub fn disposition(&self) -> WorthServerFileTransferDisposition {
        self.disposition
    }

    pub fn byte_motion_observed(&self) -> bool {
        self.disposition.byte_motion_observed()
    }

    pub fn content_type(&self) -> Option<&str> {
        self.content_type.as_deref()
    }

    pub fn bytes_selected(&self) -> u64 {
        self.bytes_selected
    }

    pub fn range_honored(&self) -> bool {
        self.range_honored
    }

    pub fn provenance(&self) -> &FoundationalBoundaryEvidenceProvenanceArtifact {
        &self.provenance
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}

fn build_provenance(
    disposition: WorthServerFileTransferDisposition,
    metadata_identity: &str,
    workspace_digest: &str,
    branch_digest: &str,
    operation_name: &str,
) -> FoundationalBoundaryEvidenceProvenanceArtifact {
    let source_basis =
        FoundationalBoundaryEvidenceSourceBasis::boundary_artifact(BoundaryArtifactLocator::new(
            BoundaryArtifactId::new(boundary_artifact_id(&[
                "worth-server.file-linkage.transfer".to_string(),
                disposition.as_str().to_string(),
                metadata_identity.to_string(),
                workspace_digest.to_string(),
                branch_digest.to_string(),
                operation_name.to_string(),
            ])),
            BoundaryArtifactField::Basis,
        ));
    match FoundationalBoundaryEvidenceProvenanceFrontDoor
        .branch_local(source_basis)
        .with_freshness(FoundationalBoundaryEvidenceFreshnessPosture::FreshRetained)
    {
        TransitionOutcome::Success(provenance) => provenance,
        outcome => panic!("file transfer provenance construction should be admitted: {outcome:?}"),
    }
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
