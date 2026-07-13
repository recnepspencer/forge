use serde_json::Value;
use worth_foundational::facade::{
    BoundaryArtifactField, BoundaryArtifactId, BoundaryArtifactLocator, DiagnosticRichnessProfile,
    FoundationalBoundaryEvidenceExecutedReceiptArtifact,
    FoundationalBoundaryEvidenceFreshnessPosture, FoundationalBoundaryEvidenceProvenanceArtifact,
    FoundationalBoundaryEvidenceProvenanceFrontDoor, FoundationalBoundaryEvidenceReceiptBoundary,
    FoundationalBoundaryEvidenceReceiptFrontDoor, FoundationalBoundaryEvidenceSourceBasis,
    FoundationalCommitId, FoundationalCommitParentBasis, FoundationalCommitParentageLocator,
    FoundationalTransitionLocator,
};
use worth_proof::TransitionOutcome;

use crate::{WorthServerQueryHandoffDenial, WorthServerQueryHandoffDenialCode};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerMetadataNormalizationReceipt {
    metadata_identity: String,
    canonical_filename: String,
    diagnostics_profile: DiagnosticRichnessProfile,
    source_kind: String,
    normalized_key_paths: Vec<String>,
    provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
    receipt: FoundationalBoundaryEvidenceExecutedReceiptArtifact,
    canonical_digest: String,
}

impl WorthServerMetadataNormalizationReceipt {
    pub(crate) fn from_manifest(
        metadata_identity: &str,
        canonical_filename: &str,
        metadata_body: &Value,
        diagnostics_profile: DiagnosticRichnessProfile,
        denial_code: WorthServerQueryHandoffDenialCode,
    ) -> Result<Self, WorthServerQueryHandoffDenial> {
        let mut normalized_key_paths = Vec::new();
        normalize_value(
            metadata_body,
            "",
            &mut normalized_key_paths,
            diagnostics_profile,
            denial_code,
        )?;
        normalized_key_paths.sort();
        Ok(build_receipt(
            metadata_identity,
            canonical_filename,
            diagnostics_profile,
            "inline_manifest",
            normalized_key_paths,
        ))
    }

    pub(crate) fn observed(
        metadata_identity: &str,
        canonical_filename: &str,
        diagnostics_profile: DiagnosticRichnessProfile,
    ) -> Self {
        build_receipt(
            metadata_identity,
            canonical_filename,
            diagnostics_profile,
            "observed_truth",
            Vec::new(),
        )
    }

    pub fn metadata_identity(&self) -> &str {
        &self.metadata_identity
    }

    pub fn canonical_filename(&self) -> &str {
        &self.canonical_filename
    }

    pub fn diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        self.diagnostics_profile
    }

    pub fn source_kind(&self) -> &str {
        &self.source_kind
    }

    pub fn normalized_key_paths(&self) -> &[String] {
        &self.normalized_key_paths
    }

    pub fn provenance(&self) -> &FoundationalBoundaryEvidenceProvenanceArtifact {
        &self.provenance
    }

    pub fn receipt(&self) -> &FoundationalBoundaryEvidenceExecutedReceiptArtifact {
        &self.receipt
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}

pub(crate) fn validate_manifest_metadata_normalization(
    metadata_body: &Value,
    diagnostics_profile: DiagnosticRichnessProfile,
) -> Result<(), WorthServerQueryHandoffDenial> {
    WorthServerMetadataNormalizationReceipt::from_manifest(
        "validation-only",
        "validation-only",
        metadata_body,
        diagnostics_profile,
        WorthServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
    )
    .map(|_| ())
}

fn build_receipt(
    metadata_identity: &str,
    canonical_filename: &str,
    diagnostics_profile: DiagnosticRichnessProfile,
    source_kind: &str,
    normalized_key_paths: Vec<String>,
) -> WorthServerMetadataNormalizationReceipt {
    let provenance = build_provenance(
        metadata_identity,
        canonical_filename,
        source_kind,
        &normalized_key_paths,
    );
    let receipt = FoundationalBoundaryEvidenceReceiptFrontDoor
        .publication(receipt_boundary(
            metadata_identity,
            canonical_filename,
            source_kind,
            &normalized_key_paths,
        ))
        .with_provenance(provenance.clone());
    let canonical_digest = format!(
        "worth-server-metadata-normalization-receipt-v1|identity={metadata_identity}|filename={canonical_filename}|source={source_kind}|keys={}|diagnostics={diagnostics_profile:?}|receipt_kind={:?}|provenance_locality={:?}",
        normalized_key_paths.join(","),
        receipt.receipt_kind(),
        provenance.locality(),
    );
    WorthServerMetadataNormalizationReceipt {
        metadata_identity: metadata_identity.to_string(),
        canonical_filename: canonical_filename.to_string(),
        diagnostics_profile,
        source_kind: source_kind.to_string(),
        normalized_key_paths,
        provenance,
        receipt,
        canonical_digest,
    }
}

fn normalize_value(
    value: &Value,
    parent_path: &str,
    normalized_key_paths: &mut Vec<String>,
    diagnostics_profile: DiagnosticRichnessProfile,
    denial_code: WorthServerQueryHandoffDenialCode,
) -> Result<(), WorthServerQueryHandoffDenial> {
    match value {
        Value::Object(map) => {
            let mut seen = std::collections::BTreeSet::new();
            for (raw_key, child) in map {
                let normalized_key =
                    normalize_metadata_key(raw_key, diagnostics_profile, denial_code)?;
                if !seen.insert(normalized_key.clone()) {
                    return Err(WorthServerQueryHandoffDenial::new(
                        denial_code,
                        diagnostics_profile,
                        format!(
                            "metadata normalization detected ambiguous keys that collapse to `{normalized_key}`"
                        ),
                    ));
                }
                let path = if parent_path.is_empty() {
                    normalized_key
                } else {
                    format!("{parent_path}.{normalized_key}")
                };
                normalized_key_paths.push(path.clone());
                normalize_value(
                    child,
                    &path,
                    normalized_key_paths,
                    diagnostics_profile,
                    denial_code,
                )?;
            }
            Ok(())
        }
        Value::Array(values) => {
            for child in values {
                normalize_value(
                    child,
                    parent_path,
                    normalized_key_paths,
                    diagnostics_profile,
                    denial_code,
                )?;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

fn normalize_metadata_key(
    raw_key: &str,
    diagnostics_profile: DiagnosticRichnessProfile,
    denial_code: WorthServerQueryHandoffDenialCode,
) -> Result<String, WorthServerQueryHandoffDenial> {
    let trimmed = raw_key.trim();
    if trimmed.is_empty() {
        return Err(WorthServerQueryHandoffDenial::new(
            denial_code,
            diagnostics_profile,
            "metadata keys may not be blank after normalization",
        ));
    }
    if trimmed.contains('/') || trimmed.contains('\\') {
        return Err(WorthServerQueryHandoffDenial::new(
            denial_code,
            diagnostics_profile,
            "metadata keys may not contain path-like separators",
        ));
    }
    if trimmed.chars().any(|ch| ch.is_control() || !ch.is_ascii()) {
        return Err(WorthServerQueryHandoffDenial::new(
            denial_code,
            diagnostics_profile,
            "metadata keys must stay ASCII-printable so canonical normalization is portability-safe",
        ));
    }
    Ok(trimmed.to_ascii_lowercase())
}

fn build_provenance(
    metadata_identity: &str,
    canonical_filename: &str,
    source_kind: &str,
    normalized_key_paths: &[String],
) -> FoundationalBoundaryEvidenceProvenanceArtifact {
    let source_basis =
        FoundationalBoundaryEvidenceSourceBasis::boundary_artifact(BoundaryArtifactLocator::new(
            BoundaryArtifactId::new(boundary_artifact_id(&[
                "worth-server.file-identity.metadata-normalization".to_string(),
                metadata_identity.to_string(),
                canonical_filename.to_string(),
                source_kind.to_string(),
                normalized_key_paths.join(","),
            ])),
            BoundaryArtifactField::Basis,
        ));
    match FoundationalBoundaryEvidenceProvenanceFrontDoor
        .branch_local(source_basis)
        .with_freshness(FoundationalBoundaryEvidenceFreshnessPosture::FreshRetained)
    {
        TransitionOutcome::Success(provenance) => provenance,
        outcome => {
            panic!("metadata normalization provenance construction should be admitted: {outcome:?}")
        }
    }
}

fn receipt_boundary(
    metadata_identity: &str,
    canonical_filename: &str,
    source_kind: &str,
    normalized_key_paths: &[String],
) -> FoundationalBoundaryEvidenceReceiptBoundary {
    let key_digest = normalized_key_paths.join(",");
    let commit_id = FoundationalCommitId::new(worth_foundational::facade::BoundaryHandle::new(
        boundary_artifact_id(&[
            "worth-server.file-identity.metadata-normalization.commit".to_string(),
            metadata_identity.to_string(),
            canonical_filename.to_string(),
            source_kind.to_string(),
            key_digest.clone(),
        ]),
    ));
    let parent_basis = FoundationalCommitParentBasis::new(
        worth_foundational::facade::EquivalenceBasisId::new(boundary_artifact_id(&[
            "worth-server.file-identity.metadata-normalization.parent".to_string(),
            metadata_identity.to_string(),
            canonical_filename.to_string(),
            source_kind.to_string(),
            key_digest,
        ])),
    );
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
