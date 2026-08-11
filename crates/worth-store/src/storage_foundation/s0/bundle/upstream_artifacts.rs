use super::super::evidence::{S0ArtifactKind, S0CanonicalArtifactSpec};
use super::aggregate_construction::CertifiedBundleRequest;
use super::validation::artifact_spec;

pub(super) fn build_upstream_artifact_specs(
    request: &CertifiedBundleRequest<'_>,
) -> Vec<S0CanonicalArtifactSpec> {
    let mut artifacts = build_matrix_artifact_specs(request);
    artifacts.extend(build_claim_artifact_specs(request));
    artifacts.extend(build_audit_artifact_specs(request));
    artifacts.extend(build_readiness_artifact_specs(request));
    artifacts
}

fn build_matrix_artifact_specs(
    request: &CertifiedBundleRequest<'_>,
) -> Vec<S0CanonicalArtifactSpec> {
    vec![
        artifact_spec(
            S0ArtifactKind::BackendCapabilityMatrix,
            request
                .backend_matrix
                .matrix()
                .envelope()
                .deterministic_digest()
                .clone(),
        ),
        artifact_spec(
            S0ArtifactKind::MilestonePhysicalStatusMatrix,
            request
                .milestone_matrix
                .matrix()
                .envelope()
                .deterministic_digest()
                .clone(),
        ),
    ]
}

fn build_claim_artifact_specs(
    request: &CertifiedBundleRequest<'_>,
) -> Vec<S0CanonicalArtifactSpec> {
    vec![
        artifact_spec(
            S0ArtifactKind::SemanticPhysicalClaimReport,
            request
                .claim_report
                .report()
                .envelope()
                .deterministic_digest()
                .clone(),
        ),
        artifact_spec(
            S0ArtifactKind::DeferredPhysicalGuaranteeMap,
            request
                .deferred_map
                .map()
                .envelope()
                .deterministic_digest()
                .clone(),
        ),
    ]
}

fn build_audit_artifact_specs(
    request: &CertifiedBundleRequest<'_>,
) -> Vec<S0CanonicalArtifactSpec> {
    vec![
        artifact_spec(
            S0ArtifactKind::TerminologyRiskReport,
            request
                .terminology_report
                .report()
                .envelope()
                .deterministic_digest()
                .clone(),
        ),
        artifact_spec(
            S0ArtifactKind::TestMigrationNotes,
            request
                .migration_notes
                .report()
                .envelope()
                .deterministic_digest()
                .clone(),
        ),
    ]
}

fn build_readiness_artifact_specs(
    request: &CertifiedBundleRequest<'_>,
) -> Vec<S0CanonicalArtifactSpec> {
    vec![
        artifact_spec(
            S0ArtifactKind::HarnessMaturityReport,
            request
                .harness_report
                .report()
                .envelope()
                .deterministic_digest()
                .clone(),
        ),
        artifact_spec(
            S0ArtifactKind::S1HandoffReadiness,
            request
                .s1_handoff
                .handoff()
                .envelope()
                .deterministic_digest()
                .clone(),
        ),
    ]
}
