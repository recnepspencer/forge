use forge_foundational::{
    foundational_diagnostic_boundary_artifact_subject, foundational_diagnostic_code,
    foundational_diagnostic_locator_boundary_artifact, foundational_diagnostic_scope,
    BoundaryArtifactField, BoundaryArtifactId, BoundaryArtifactLocator,
    FoundationalDiagnosticDenialClass, FoundationalDiagnosticEvidencePosture,
    FoundationalDiagnosticLocalityClaim, FoundationalDiagnosticOutcomeKind,
    FoundationalDiagnosticProvenanceReadyRow, FoundationalDiagnosticRow,
    FoundationalDiagnosticSemanticLabelSet, FoundationalDiagnosticSeverity,
    FoundationalDiagnosticSupportEvidencePosture, FoundationalDiagnosticSupportRow,
    FoundationalDiagnosticWidenedFalloutPosture,
};
use sha2::{Digest, Sha256};

use super::super::payloads::{
    ForgeQueryDomainCapabilityPayload, ForgeQueryDomainCapabilitySemanticPosture,
};
use super::super::targets::ForgeQueryDomainCapabilityTargetBinding;
use super::super::{
    ForgeQueryDomainCapabilityTargetKind,
    ForgeQueryMaterializationReadyDomainCapabilityContribution,
};

pub(crate) struct ForgeQueryDomainCapabilityDiagnosticRows {
    pub subject: forge_foundational::FoundationalDiagnosticSubject,
    pub scope: forge_foundational::FoundationalDiagnosticScopeId,
    pub primary_code: forge_foundational::FoundationalDiagnosticCodeId,
    pub outcome_kind: FoundationalDiagnosticOutcomeKind,
    pub required_rows: Vec<FoundationalDiagnosticRow>,
    pub standard_rows: Vec<FoundationalDiagnosticRow>,
    pub forensic_rows: Vec<FoundationalDiagnosticRow>,
}

pub(crate) fn build_rows<P, T>(
    contribution: &ForgeQueryMaterializationReadyDomainCapabilityContribution<P, T>,
) -> ForgeQueryDomainCapabilityDiagnosticRows
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    let payload = contribution.payload().payload();
    let category = payload.category();
    let semantic_posture = payload.semantic_posture();
    let target = contribution.payload().target();
    let artifact_id = boundary_artifact_id(contribution.payload().request_digest());
    let subject = foundational_diagnostic_boundary_artifact_subject(
        artifact_id,
        BoundaryArtifactField::Payload,
    );
    let locator = foundational_diagnostic_locator_boundary_artifact(BoundaryArtifactLocator::new(
        artifact_id,
        BoundaryArtifactField::Payload,
    ));
    let scope = scope_id(&format!(
        "query.domain_capabilities.{}.{}",
        category.as_str(),
        target.kind().as_str()
    ));
    let primary_code = code_id(payload.semantic_code());
    let outcome_kind = semantic_posture.outcome_kind();
    let labels = labels([
        &format!("category.{}", category.as_str()),
        &format!("target.{}", target.kind().as_str()),
        &format!("outcome.{}", outcome_kind.canonical_name()),
    ]);
    let evidence_posture = evidence_posture_for(target.kind(), semantic_posture, outcome_kind);
    let support_evidence_posture =
        support_evidence_posture_for(target.kind(), semantic_posture, outcome_kind);
    let locality = locality_for(target.kind());
    let widened = widened_for(outcome_kind);

    let required_rows = vec![
        FoundationalDiagnosticRow::Decision(
            forge_foundational::FoundationalDiagnosticDecisionRow::new(
                primary_code.clone(),
                scope.clone(),
                severity_for(outcome_kind),
                subject.clone(),
                locator.clone(),
                outcome_kind,
                labels.clone(),
                denial_class_for(outcome_kind),
                locality,
                widened,
            ),
        ),
        FoundationalDiagnosticRow::Support(FoundationalDiagnosticSupportRow::new(
            code_id(&format!("{}.support", payload.semantic_code())),
            scope.clone(),
            severity_for_support(outcome_kind),
            subject.clone(),
            locator.clone(),
            outcome_kind,
            labels.clone(),
            support_evidence_posture.clone(),
            locality,
            widened,
        )),
    ];
    let standard_rows = vec![FoundationalDiagnosticRow::ProvenanceReady(
        FoundationalDiagnosticProvenanceReadyRow::new(
            code_id(&format!("{}.provenance", payload.semantic_code())),
            scope.clone(),
            FoundationalDiagnosticSeverity::Info,
            subject.clone(),
            locator.clone(),
            outcome_kind,
            labels.clone(),
            foundational_diagnostic_locator_boundary_artifact(BoundaryArtifactLocator::new(
                boundary_artifact_id(target.binding_digest()),
                BoundaryArtifactField::Basis,
            )),
            evidence_posture,
        ),
    )];
    let forensic_rows = vec![FoundationalDiagnosticRow::Support(
        FoundationalDiagnosticSupportRow::new(
            code_id(&format!("{}.trace", payload.semantic_code())),
            scope.clone(),
            FoundationalDiagnosticSeverity::Info,
            subject.clone(),
            locator.clone(),
            outcome_kind,
            labels,
            support_evidence_posture,
            locality,
            widened,
        ),
    )];

    ForgeQueryDomainCapabilityDiagnosticRows {
        subject,
        scope,
        primary_code,
        outcome_kind,
        required_rows,
        standard_rows,
        forensic_rows,
    }
}

fn severity_for(outcome: FoundationalDiagnosticOutcomeKind) -> FoundationalDiagnosticSeverity {
    match outcome {
        FoundationalDiagnosticOutcomeKind::Accepted => FoundationalDiagnosticSeverity::Info,
        FoundationalDiagnosticOutcomeKind::Advisory
        | FoundationalDiagnosticOutcomeKind::Deferred
        | FoundationalDiagnosticOutcomeKind::Partial => FoundationalDiagnosticSeverity::Advisory,
        FoundationalDiagnosticOutcomeKind::Unsupported
        | FoundationalDiagnosticOutcomeKind::Denied => FoundationalDiagnosticSeverity::Denial,
        FoundationalDiagnosticOutcomeKind::Mismatch => FoundationalDiagnosticSeverity::Warning,
        FoundationalDiagnosticOutcomeKind::Violation => FoundationalDiagnosticSeverity::Violation,
    }
}

fn severity_for_support(
    outcome: FoundationalDiagnosticOutcomeKind,
) -> FoundationalDiagnosticSeverity {
    match outcome {
        FoundationalDiagnosticOutcomeKind::Violation
        | FoundationalDiagnosticOutcomeKind::Denied
        | FoundationalDiagnosticOutcomeKind::Unsupported => {
            FoundationalDiagnosticSeverity::Advisory
        }
        _ => FoundationalDiagnosticSeverity::Info,
    }
}

fn denial_class_for(
    outcome: FoundationalDiagnosticOutcomeKind,
) -> Option<FoundationalDiagnosticDenialClass> {
    match outcome {
        FoundationalDiagnosticOutcomeKind::Denied => {
            Some(FoundationalDiagnosticDenialClass::DomainDenied)
        }
        FoundationalDiagnosticOutcomeKind::Unsupported => {
            Some(FoundationalDiagnosticDenialClass::UnsupportedDenied)
        }
        FoundationalDiagnosticOutcomeKind::Violation => {
            Some(FoundationalDiagnosticDenialClass::PolicyDenied)
        }
        _ => None,
    }
}

fn evidence_posture_for(
    _target_kind: ForgeQueryDomainCapabilityTargetKind,
    semantic_posture: ForgeQueryDomainCapabilitySemanticPosture,
    outcome: FoundationalDiagnosticOutcomeKind,
) -> FoundationalDiagnosticEvidencePosture {
    if semantic_posture.is_policy_or_inferred() {
        return FoundationalDiagnosticEvidencePosture::Summarized;
    }
    match outcome {
        FoundationalDiagnosticOutcomeKind::Partial
        | FoundationalDiagnosticOutcomeKind::Advisory => {
            FoundationalDiagnosticEvidencePosture::Summarized
        }
        _ => FoundationalDiagnosticEvidencePosture::RetainedDirect,
    }
}

fn support_evidence_posture_for(
    target_kind: ForgeQueryDomainCapabilityTargetKind,
    semantic_posture: ForgeQueryDomainCapabilitySemanticPosture,
    outcome: FoundationalDiagnosticOutcomeKind,
) -> FoundationalDiagnosticSupportEvidencePosture {
    FoundationalDiagnosticSupportEvidencePosture::Present(evidence_posture_for(
        target_kind,
        semantic_posture,
        outcome,
    ))
}

fn locality_for(
    target_kind: ForgeQueryDomainCapabilityTargetKind,
) -> FoundationalDiagnosticLocalityClaim {
    match target_kind {
        ForgeQueryDomainCapabilityTargetKind::IntentDeclaration
        | ForgeQueryDomainCapabilityTargetKind::AdmittedIntentPlan => {
            FoundationalDiagnosticLocalityClaim::ExactSubject
        }
        ForgeQueryDomainCapabilityTargetKind::LowerRuntimeBoundaryEnvelope => {
            FoundationalDiagnosticLocalityClaim::SubjectNeighborhood
        }
    }
}

fn widened_for(
    outcome: FoundationalDiagnosticOutcomeKind,
) -> FoundationalDiagnosticWidenedFalloutPosture {
    match outcome {
        FoundationalDiagnosticOutcomeKind::Partial
        | FoundationalDiagnosticOutcomeKind::Advisory => {
            FoundationalDiagnosticWidenedFalloutPosture::WidenedExpected
        }
        _ => FoundationalDiagnosticWidenedFalloutPosture::NotWidened,
    }
}

fn labels<const N: usize>(values: [&str; N]) -> FoundationalDiagnosticSemanticLabelSet {
    FoundationalDiagnosticSemanticLabelSet::new(values.into_iter().map(code_id))
}

fn code_id(value: &str) -> forge_foundational::FoundationalDiagnosticCodeId {
    foundational_diagnostic_code(normalize_fragment(value)).expect("normalized diagnostic code")
}

fn scope_id(value: &str) -> forge_foundational::FoundationalDiagnosticScopeId {
    foundational_diagnostic_scope(normalize_fragment(value)).expect("normalized diagnostic scope")
}

fn normalize_fragment(value: &str) -> String {
    let mut normalized = String::with_capacity(value.len());
    let mut previous_separator = true;
    for ch in value.chars().flat_map(char::to_lowercase) {
        let valid = ch.is_ascii_lowercase() || ch.is_ascii_digit();
        if valid {
            normalized.push(ch);
            previous_separator = false;
        } else if !previous_separator {
            normalized.push('.');
            previous_separator = true;
        }
    }
    while normalized.ends_with('.') {
        normalized.pop();
    }
    if normalized.is_empty() {
        "unspecified".to_string()
    } else {
        normalized
    }
}

pub(crate) fn boundary_artifact_id(value: &str) -> BoundaryArtifactId {
    let mut hasher = Sha256::new();
    hasher.update(value.as_bytes());
    let digest = hasher.finalize();
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&digest[..8]);
    BoundaryArtifactId::new(u64::from_be_bytes(bytes))
}
