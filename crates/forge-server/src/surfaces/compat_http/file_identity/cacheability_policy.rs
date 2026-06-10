use forge_foundational::facade::{
    BoundaryArtifactField, BoundaryArtifactId, BoundaryArtifactLocator, DiagnosticRichnessProfile,
    FoundationalBoundaryEvidenceFreshnessPosture, FoundationalBoundaryEvidenceProvenanceArtifact,
    FoundationalBoundaryEvidenceProvenanceFrontDoor, FoundationalBoundaryEvidenceSourceBasis,
};
use forge_proof::TransitionOutcome;

use crate::ForgeServerCompatibilityCachePolicy;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerCacheabilityPolicy {
    surface_kind: String,
    diagnostics_profile: DiagnosticRichnessProfile,
    cache_control: String,
    vary: Vec<String>,
    publicly_reusable: bool,
    intermediary_reuse_safe: bool,
    branch_scoped: bool,
    auth_scoped: bool,
    remask_safe_for_shared_caches: bool,
    provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
    canonical_digest: String,
}

impl ForgeServerCacheabilityPolicy {
    pub(crate) fn from_compatibility_policy(
        surface_kind: &str,
        diagnostics_profile: DiagnosticRichnessProfile,
        metadata_identity: &str,
        branch_digest: &str,
        compatibility_policy: &ForgeServerCompatibilityCachePolicy,
        remask_safe_for_shared_caches: bool,
    ) -> Self {
        let provenance = build_provenance(surface_kind, metadata_identity, branch_digest);
        let canonical_digest = format!(
            "forge-server-cacheability-policy-v1|surface={surface_kind}|cache_control={}|vary={}|public={}|intermediary_safe=false|branch_scoped=true|auth_scoped=true|remask_safe={remask_safe_for_shared_caches}|diagnostics={diagnostics_profile:?}|provenance_locality={:?}",
            compatibility_policy.cache_control(),
            compatibility_policy.vary().join(","),
            compatibility_policy.publicly_reusable(),
            provenance.locality(),
        );
        Self {
            surface_kind: surface_kind.to_string(),
            diagnostics_profile,
            cache_control: compatibility_policy.cache_control().to_string(),
            vary: compatibility_policy.vary().to_vec(),
            publicly_reusable: compatibility_policy.publicly_reusable(),
            intermediary_reuse_safe: false,
            branch_scoped: true,
            auth_scoped: true,
            remask_safe_for_shared_caches,
            provenance,
            canonical_digest,
        }
    }

    pub(crate) fn scoped_private(
        surface_kind: &str,
        diagnostics_profile: DiagnosticRichnessProfile,
        metadata_identity: &str,
        branch_digest: &str,
    ) -> Self {
        let vary = vec![
            "authorization".to_string(),
            "x-forge-branch".to_string(),
            "x-forge-diagnostics".to_string(),
        ];
        let cache_control = "private, no-store".to_string();
        let provenance = build_provenance(surface_kind, metadata_identity, branch_digest);
        let canonical_digest = format!(
            "forge-server-cacheability-policy-v1|surface={surface_kind}|cache_control={cache_control}|vary={}|public=false|intermediary_safe=false|branch_scoped=true|auth_scoped=true|remask_safe=false|diagnostics={diagnostics_profile:?}|provenance_locality={:?}",
            vary.join(","),
            provenance.locality(),
        );
        Self {
            surface_kind: surface_kind.to_string(),
            diagnostics_profile,
            cache_control,
            vary,
            publicly_reusable: false,
            intermediary_reuse_safe: false,
            branch_scoped: true,
            auth_scoped: true,
            remask_safe_for_shared_caches: false,
            provenance,
            canonical_digest,
        }
    }

    pub fn surface_kind(&self) -> &str {
        &self.surface_kind
    }

    pub fn diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        self.diagnostics_profile
    }

    pub fn cache_control(&self) -> &str {
        &self.cache_control
    }

    pub fn vary(&self) -> &[String] {
        &self.vary
    }

    pub fn publicly_reusable(&self) -> bool {
        self.publicly_reusable
    }

    pub fn intermediary_reuse_safe(&self) -> bool {
        self.intermediary_reuse_safe
    }

    pub fn branch_scoped(&self) -> bool {
        self.branch_scoped
    }

    pub fn auth_scoped(&self) -> bool {
        self.auth_scoped
    }

    pub fn remask_safe_for_shared_caches(&self) -> bool {
        self.remask_safe_for_shared_caches
    }

    pub fn provenance(&self) -> &FoundationalBoundaryEvidenceProvenanceArtifact {
        &self.provenance
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}

fn build_provenance(
    surface_kind: &str,
    metadata_identity: &str,
    branch_digest: &str,
) -> FoundationalBoundaryEvidenceProvenanceArtifact {
    let source_basis =
        FoundationalBoundaryEvidenceSourceBasis::boundary_artifact(BoundaryArtifactLocator::new(
            BoundaryArtifactId::new(boundary_artifact_id(&[
                "forge-server.file-identity.cacheability".to_string(),
                surface_kind.to_string(),
                metadata_identity.to_string(),
                branch_digest.to_string(),
            ])),
            BoundaryArtifactField::Basis,
        ));
    match FoundationalBoundaryEvidenceProvenanceFrontDoor
        .branch_local(source_basis)
        .with_freshness(FoundationalBoundaryEvidenceFreshnessPosture::FreshRetained)
    {
        TransitionOutcome::Success(provenance) => provenance,
        outcome => panic!("cacheability provenance construction should be admitted: {outcome:?}"),
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
