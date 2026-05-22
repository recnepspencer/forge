use forge_foundational::facade::{
    foundational_diagnostic_boundary_artifact_subject, foundational_diagnostic_code,
    foundational_diagnostic_locator_boundary_artifact, foundational_diagnostic_scope,
    BoundaryArtifactField, BoundaryArtifactId, BoundaryArtifactLocator,
    FoundationalDiagnosticCodeId, FoundationalDiagnosticDenialClass,
    FoundationalDiagnosticEvidencePosture, FoundationalDiagnosticScopeId,
    FoundationalDiagnosticSemanticLabelSet, FoundationalDiagnosticSeverity,
    FoundationalDiagnosticWidenedFalloutPosture,
};

use super::capabilities::SpatialBlockedCapability;
use super::declared_analysis::{
    SpatialIntentConflictClass, SpatialIntentEscalation, SpatialIntentExplanationClass,
};
use super::materialization::SpatialArbitrationMaterializationDenial;

pub(crate) fn arbitration_scope(
) -> Result<FoundationalDiagnosticScopeId, SpatialArbitrationMaterializationDenial> {
    foundational_diagnostic_scope("worth.spatial.arbitration")
        .map_err(SpatialArbitrationMaterializationDenial::Primitive)
}

pub(crate) fn arbitration_subject() -> forge_foundational::facade::FoundationalDiagnosticSubject {
    foundational_diagnostic_boundary_artifact_subject(
        BoundaryArtifactId::new(3101),
        BoundaryArtifactField::Payload,
    )
}

pub(crate) fn arbitration_locator() -> forge_foundational::facade::FoundationalDiagnosticLocator {
    foundational_diagnostic_locator_boundary_artifact(BoundaryArtifactLocator::new(
        BoundaryArtifactId::new(3102),
        BoundaryArtifactField::Payload,
    ))
}

pub(crate) fn request_boundary_artifact() -> BoundaryArtifactLocator {
    BoundaryArtifactLocator::new(
        BoundaryArtifactId::new(3100),
        BoundaryArtifactField::Payload,
    )
}

pub(crate) fn support_boundary_artifact() -> BoundaryArtifactLocator {
    BoundaryArtifactLocator::new(
        BoundaryArtifactId::new(3103),
        BoundaryArtifactField::Payload,
    )
}

pub(crate) fn code(
    value: &'static str,
) -> Result<FoundationalDiagnosticCodeId, SpatialArbitrationMaterializationDenial> {
    foundational_diagnostic_code(value).map_err(SpatialArbitrationMaterializationDenial::Primitive)
}

pub(crate) fn semantic_labels(
    conflict_class: SpatialIntentConflictClass,
    escalation: SpatialIntentEscalation,
    policy_profile_name: &'static str,
) -> Result<FoundationalDiagnosticSemanticLabelSet, SpatialArbitrationMaterializationDenial> {
    let mut labels = vec![code("worth.spatial.arbitration")?];
    labels.push(code(match conflict_class {
        SpatialIntentConflictClass::SingleClearIntent => {
            "worth.spatial.arbitration.single_clear_intent"
        }
        SpatialIntentConflictClass::MultiplePlausibleIntents => {
            "worth.spatial.arbitration.multiple_plausible_intents"
        }
        SpatialIntentConflictClass::UnsafeToAssume => "worth.spatial.arbitration.unsafe_to_assume",
        SpatialIntentConflictClass::BlockedCandidateSet => {
            "worth.spatial.arbitration.blocked_candidate_set"
        }
    })?);
    labels.push(code(match escalation {
        SpatialIntentEscalation::AutoResolve(_) => {
            "worth.spatial.arbitration.escalation.auto_resolve"
        }
        SpatialIntentEscalation::PreserveCandidates => {
            "worth.spatial.arbitration.escalation.preserve_candidates"
        }
        SpatialIntentEscalation::AskForClarification => {
            "worth.spatial.arbitration.escalation.ask_for_clarification"
        }
        SpatialIntentEscalation::BlockedByMissingCapability(_) => {
            "worth.spatial.arbitration.escalation.blocked_missing_capability"
        }
    })?);
    labels.push(code(match policy_profile_name {
        "conservative_exact_modeling" => "worth.spatial.policy.conservative_exact_modeling",
        "bim_host_friendly" => "worth.spatial.policy.bim_host_friendly",
        "ask_first_arbitration" => "worth.spatial.policy.ask_first_arbitration",
        "aggressive_snap" => "worth.spatial.policy.aggressive_snap",
        "high_fidelity_preview" => "worth.spatial.policy.high_fidelity_preview",
        _ => "worth.spatial.policy.custom",
    })?);
    Ok(FoundationalDiagnosticSemanticLabelSet::new(labels))
}

pub(crate) fn candidate_support_code(
    explanation: SpatialIntentExplanationClass,
) -> Result<FoundationalDiagnosticCodeId, SpatialArbitrationMaterializationDenial> {
    code(match explanation {
        SpatialIntentExplanationClass::AuthoredBaseline => {
            "worth.spatial.arbitration.candidate.authored_baseline"
        }
        SpatialIntentExplanationClass::RelationInferred => {
            "worth.spatial.arbitration.candidate.relation_inferred"
        }
        SpatialIntentExplanationClass::BlockedFutureCapability => {
            "worth.spatial.arbitration.candidate.blocked_future_capability"
        }
        SpatialIntentExplanationClass::UnsafeBoundary => {
            "worth.spatial.arbitration.candidate.unsafe_boundary"
        }
        SpatialIntentExplanationClass::PolicyPreferred => {
            "worth.spatial.arbitration.candidate.policy_preferred"
        }
    })
}

pub(crate) fn blocked_capability_code(
    capability: SpatialBlockedCapability,
) -> Result<FoundationalDiagnosticCodeId, SpatialArbitrationMaterializationDenial> {
    code(match capability {
        SpatialBlockedCapability::MergeBoolean => {
            "worth.spatial.arbitration.capability.merge_boolean"
        }
        SpatialBlockedCapability::SubtractBoolean => {
            "worth.spatial.arbitration.capability.subtract_boolean"
        }
        SpatialBlockedCapability::CutOpening => "worth.spatial.arbitration.capability.cut_opening",
        SpatialBlockedCapability::Join => "worth.spatial.arbitration.capability.join",
        SpatialBlockedCapability::HostAttach => "worth.spatial.arbitration.capability.host_attach",
    })
}

pub(crate) fn decision_code(
    escalation: SpatialIntentEscalation,
) -> Result<FoundationalDiagnosticCodeId, SpatialArbitrationMaterializationDenial> {
    code(match escalation {
        SpatialIntentEscalation::AutoResolve(_) => {
            "worth.spatial.arbitration.accepted.auto_resolve"
        }
        SpatialIntentEscalation::PreserveCandidates => {
            "worth.spatial.arbitration.denied.preserve_candidates"
        }
        SpatialIntentEscalation::AskForClarification => {
            "worth.spatial.arbitration.denied.ask_for_clarification"
        }
        SpatialIntentEscalation::BlockedByMissingCapability(_) => {
            "worth.spatial.arbitration.denied.blocked_missing_capability"
        }
    })
}

pub(crate) fn decision_severity(
    escalation: SpatialIntentEscalation,
) -> FoundationalDiagnosticSeverity {
    match escalation {
        SpatialIntentEscalation::AutoResolve(_) => FoundationalDiagnosticSeverity::Info,
        SpatialIntentEscalation::BlockedByMissingCapability(_) => {
            FoundationalDiagnosticSeverity::Advisory
        }
        SpatialIntentEscalation::PreserveCandidates
        | SpatialIntentEscalation::AskForClarification => FoundationalDiagnosticSeverity::Denial,
    }
}

pub(crate) fn denial_class(
    escalation: SpatialIntentEscalation,
) -> Option<FoundationalDiagnosticDenialClass> {
    match escalation {
        SpatialIntentEscalation::AutoResolve(_) => None,
        SpatialIntentEscalation::BlockedByMissingCapability(_) => {
            Some(FoundationalDiagnosticDenialClass::UnsupportedDenied)
        }
        SpatialIntentEscalation::PreserveCandidates
        | SpatialIntentEscalation::AskForClarification => {
            Some(FoundationalDiagnosticDenialClass::DomainDenied)
        }
    }
}

pub(crate) fn evidence_posture(
    explanation: SpatialIntentExplanationClass,
) -> FoundationalDiagnosticEvidencePosture {
    match explanation {
        SpatialIntentExplanationClass::AuthoredBaseline => {
            FoundationalDiagnosticEvidencePosture::RetainedDirect
        }
        SpatialIntentExplanationClass::RelationInferred
        | SpatialIntentExplanationClass::PolicyPreferred => {
            FoundationalDiagnosticEvidencePosture::Summarized
        }
        SpatialIntentExplanationClass::BlockedFutureCapability
        | SpatialIntentExplanationClass::UnsafeBoundary => {
            FoundationalDiagnosticEvidencePosture::Reconstructed
        }
    }
}

pub(crate) fn widened_posture(
    explanation: SpatialIntentExplanationClass,
) -> FoundationalDiagnosticWidenedFalloutPosture {
    match explanation {
        SpatialIntentExplanationClass::BlockedFutureCapability
        | SpatialIntentExplanationClass::UnsafeBoundary => {
            FoundationalDiagnosticWidenedFalloutPosture::WidenedExpected
        }
        _ => FoundationalDiagnosticWidenedFalloutPosture::NotWidened,
    }
}
