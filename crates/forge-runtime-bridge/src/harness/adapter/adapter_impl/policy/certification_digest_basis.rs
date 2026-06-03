use crate::routing::canonicalization::digest_string;

use super::shared_artifacts::AdmittedPolicyBundle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PolicyCertificationDigestArtifact {
    ProvenancePolicyEquivalence,
    ProvenanceReplay,
    ProvenanceDiagnostics,
    RejectionFailures,
    RejectionDiagnostics,
    AmbientLeakPolicySequence,
    AmbientLeakReplaySequence,
    AmbientLeakDiagnostics,
    SemanticRoutePlanningPolicy,
    EmptyProvenanceReport,
}

impl PolicyCertificationDigestArtifact {
    const fn digest_domain(self) -> &'static str {
        match self {
            Self::ProvenancePolicyEquivalence => "policy-provenance-equivalence",
            Self::ProvenanceReplay => "policy-provenance-replay",
            Self::ProvenanceDiagnostics => "policy-provenance-diagnostics",
            Self::RejectionFailures => "policy-rejection-certification",
            Self::RejectionDiagnostics => "policy-rejection-diagnostics",
            Self::AmbientLeakPolicySequence => "policy-ambient-leak-certification",
            Self::AmbientLeakReplaySequence => "policy-ambient-leak-replay",
            Self::AmbientLeakDiagnostics => "policy-ambient-leak-diagnostics",
            Self::SemanticRoutePlanningPolicy => "semantic-route-planning-policy",
            Self::EmptyProvenanceReport => "policy-empty-provenance-report",
        }
    }
}

pub(super) fn provenance_policy_equivalence_digest(
    deterministic: &AdmittedPolicyBundle,
    optimized: &AdmittedPolicyBundle,
) -> String {
    digest_policy_certification_basis(
        PolicyCertificationDigestArtifact::ProvenancePolicyEquivalence,
        [deterministic.contract.digest(), optimized.contract.digest()],
    )
}

pub(super) fn provenance_replay_digest(
    deterministic: &AdmittedPolicyBundle,
    optimized: &AdmittedPolicyBundle,
) -> String {
    digest_policy_certification_basis(
        PolicyCertificationDigestArtifact::ProvenanceReplay,
        [
            deterministic.replay_bundle.digest(),
            optimized.replay_bundle.digest(),
        ],
    )
}

pub(super) fn provenance_diagnostics_digest(
    deterministic: &AdmittedPolicyBundle,
    optimized: &AdmittedPolicyBundle,
) -> String {
    digest_policy_certification_basis(
        PolicyCertificationDigestArtifact::ProvenanceDiagnostics,
        [
            deterministic.provenance.digest(),
            optimized.provenance.digest(),
        ],
    )
}

pub(super) fn rejection_failure_digest(
    optimized_authoritative: &crate::facade::BridgePolicyRejection,
    replay_conflict: &crate::facade::BridgePolicyRejection,
) -> String {
    digest_policy_certification_basis(
        PolicyCertificationDigestArtifact::RejectionFailures,
        [optimized_authoritative.digest(), replay_conflict.digest()],
    )
}

pub(super) fn rejection_diagnostics_digest(
    optimized_authoritative: &crate::facade::BridgePolicyRejection,
    replay_conflict: &crate::facade::BridgePolicyRejection,
) -> String {
    let optimized_authoritative_basis = policy_rejection_diagnostic_basis(optimized_authoritative);
    let replay_conflict_basis = policy_rejection_diagnostic_basis(replay_conflict);
    digest_policy_certification_basis(
        PolicyCertificationDigestArtifact::RejectionDiagnostics,
        [
            optimized_authoritative_basis.as_str(),
            replay_conflict_basis.as_str(),
        ],
    )
}

pub(super) fn ambient_leak_policy_sequence_digest(
    preview_before: &AdmittedPolicyBundle,
    authoritative_middle: &AdmittedPolicyBundle,
    preview_after: &AdmittedPolicyBundle,
) -> String {
    digest_policy_certification_basis(
        PolicyCertificationDigestArtifact::AmbientLeakPolicySequence,
        [
            preview_before.contract.digest(),
            authoritative_middle.contract.digest(),
            preview_after.contract.digest(),
        ],
    )
}

pub(super) fn ambient_leak_replay_sequence_digest(
    preview_before: &AdmittedPolicyBundle,
    authoritative_middle: &AdmittedPolicyBundle,
    preview_after: &AdmittedPolicyBundle,
) -> String {
    digest_policy_certification_basis(
        PolicyCertificationDigestArtifact::AmbientLeakReplaySequence,
        [
            preview_before.replay_bundle.digest(),
            authoritative_middle.replay_bundle.digest(),
            preview_after.replay_bundle.digest(),
        ],
    )
}

pub(super) fn ambient_leak_diagnostics_digest(
    preview_before: &AdmittedPolicyBundle,
    authoritative_middle: &AdmittedPolicyBundle,
    preview_after: &AdmittedPolicyBundle,
) -> String {
    digest_policy_certification_basis(
        PolicyCertificationDigestArtifact::AmbientLeakDiagnostics,
        [
            preview_before.provenance.digest(),
            authoritative_middle.provenance.digest(),
            preview_after.provenance.digest(),
        ],
    )
}

pub(super) fn semantic_route_planning_policy_digest(
    route_policy: &crate::facade::BridgeRoutePlanningPolicy,
) -> String {
    let execution_class = format!("{:?}", route_policy.execution_class());
    let diagnostics_tier = format!("{:?}", route_policy.diagnostics_tier());
    let route_artifacts = route_policy.route_artifacts().to_string();
    let replay_artifacts = route_policy.replay_artifacts().to_string();
    digest_policy_certification_basis(
        PolicyCertificationDigestArtifact::SemanticRoutePlanningPolicy,
        [
            execution_class.as_str(),
            diagnostics_tier.as_str(),
            route_artifacts.as_str(),
            replay_artifacts.as_str(),
        ],
    )
}

pub(super) fn empty_provenance_report_digest() -> String {
    digest_policy_certification_basis(PolicyCertificationDigestArtifact::EmptyProvenanceReport, [])
}

fn digest_policy_certification_basis<const N: usize>(
    artifact: PolicyCertificationDigestArtifact,
    evidence: [&str; N],
) -> String {
    digest_string(artifact.digest_domain(), &evidence.join("|")).to_string()
}

fn policy_rejection_diagnostic_basis(rejection: &crate::facade::BridgePolicyRejection) -> String {
    format!(
        "policy-rejection-diagnostic|declaration={}|kind={:?}|stage={:?}|field={:?}|primary={:?}|secondary={:?}|digest={}",
        rejection.declaration_identity().as_str(),
        rejection.kind(),
        rejection.stage(),
        rejection.field_kind(),
        rejection.primary_source(),
        rejection.conflicting_source(),
        rejection.digest(),
    )
}

#[cfg(test)]
mod tests {
    use super::{
        digest_policy_certification_basis, empty_provenance_report_digest,
        PolicyCertificationDigestArtifact,
    };

    #[test]
    fn closed_policy_certification_artifacts_separate_identical_evidence() {
        let evidence = ["policy-a", "policy-b"];

        let policy_digest = digest_policy_certification_basis(
            PolicyCertificationDigestArtifact::ProvenancePolicyEquivalence,
            evidence,
        );
        let replay_digest = digest_policy_certification_basis(
            PolicyCertificationDigestArtifact::ProvenanceReplay,
            evidence,
        );

        assert_ne!(policy_digest, replay_digest);
        assert!(policy_digest.starts_with("policy-provenance-equivalence:sha256:"));
        assert!(replay_digest.starts_with("policy-provenance-replay:sha256:"));
    }

    #[test]
    fn empty_provenance_report_digest_uses_named_terminal_projection_artifact() {
        let empty_report_digest = empty_provenance_report_digest();

        assert!(empty_report_digest.starts_with("policy-empty-provenance-report:sha256:"));
    }
}
