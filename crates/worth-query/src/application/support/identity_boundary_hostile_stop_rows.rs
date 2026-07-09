use super::WorthQueryIdentityBoundaryHostileMatrixRow;
use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};
use crate::runtime::{
    WorthQueryAuthorityLane, WorthQueryEffectPolicy,
    WorthQueryGraphCompositionDomainInvariantDenial,
    WorthQueryGraphCompositionDomainInvariantSummary, WorthQueryPreviewBasisAdmission,
    WorthQueryRuntimeError, WorthQueryRuntimeEvidenceAuthority, WorthQueryRuntimeFacadeFamily,
    WorthQueryRuntimeFamilySupportStatus, WorthQueryRuntimeFamilyTeachingPosture,
    WorthQueryRuntimeSupportDenial, WorthQueryStopClass,
};
use crate::session_label::WorthQuerySessionLabel;

pub(super) fn family_admission_message_rewording_stability_row(
) -> WorthQueryIdentityBoundaryHostileMatrixRow {
    let first_error =
        WorthQueryRuntimeError::UnsupportedFacadeFamily(WorthQueryRuntimeSupportDenial::new(
            WorthQueryRuntimeFacadeFamily::Temporal,
            WorthQueryRuntimeFamilySupportStatus::Supported,
            Some(WorthQueryRuntimeFamilyTeachingPosture::SupportGateOnly),
            "first temporal wording",
        ));
    let second_error =
        WorthQueryRuntimeError::UnsupportedFacadeFamily(WorthQueryRuntimeSupportDenial::new(
            WorthQueryRuntimeFacadeFamily::Temporal,
            WorthQueryRuntimeFamilySupportStatus::Supported,
            Some(WorthQueryRuntimeFamilyTeachingPosture::SupportGateOnly),
            "second temporal wording",
        ));
    let first_message = first_error.to_string();
    let second_message = second_error.to_string();
    let certified = matches!(
        first_error.stop_class(),
        WorthQueryStopClass::FamilyAdmissionDenied {
            family: WorthQueryRuntimeFacadeFamily::Temporal,
            status: WorthQueryRuntimeFamilySupportStatus::Supported,
            teaching_posture: Some(WorthQueryRuntimeFamilyTeachingPosture::SupportGateOnly),
            ..
        }
    ) && matches!(
        second_error.stop_class(),
        WorthQueryStopClass::FamilyAdmissionDenied {
            family: WorthQueryRuntimeFacadeFamily::Temporal,
            status: WorthQueryRuntimeFamilySupportStatus::Supported,
            teaching_posture: Some(WorthQueryRuntimeFamilyTeachingPosture::SupportGateOnly),
            ..
        }
    ) && first_message != second_message;
    WorthQueryIdentityBoundaryHostileMatrixRow::new(
        "family-admission-message-rewording-stability",
        certified,
        witness_digest(
            "family-admission-message-rewording-stability",
            certified,
            [first_message.as_str(), second_message.as_str()],
        ),
    )
}

pub(super) fn graph_domain_invariant_message_rewording_stability_row(
) -> WorthQueryIdentityBoundaryHostileMatrixRow {
    let first_error = WorthQueryRuntimeError::GraphCompositionDomainInvariantDenied(
        graph_domain_invariant_denial("graph domain invariant failed first"),
    );
    let second_error = WorthQueryRuntimeError::GraphCompositionDomainInvariantDenied(
        graph_domain_invariant_denial("graph domain invariant failed second"),
    );
    let first_message = first_error.to_string();
    let second_message = second_error.to_string();
    let certified = matches!(
        first_error.stop_class(),
        WorthQueryStopClass::GraphCompositionDomainInvariantDenied { .. }
    ) && matches!(
        second_error.stop_class(),
        WorthQueryStopClass::GraphCompositionDomainInvariantDenied { .. }
    ) && first_message != second_message;
    WorthQueryIdentityBoundaryHostileMatrixRow::new(
        "graph-domain-invariant-message-rewording-stability",
        certified,
        witness_digest(
            "graph-domain-invariant-message-rewording-stability",
            certified,
            [first_message.as_str(), second_message.as_str()],
        ),
    )
}

pub(super) fn session_label_render_collision_distinctness_row(
) -> WorthQueryIdentityBoundaryHostileMatrixRow {
    let left = WorthQuerySessionLabel::scoped_strs("worth.kernel", ["preview"]).expect("label");
    let right = WorthQuerySessionLabel::scoped_strs("worth", ["kernel", "preview"]).expect("label");
    let certified =
        left.display() == right.display() && left.identity_digest() != right.identity_digest();
    WorthQueryIdentityBoundaryHostileMatrixRow::new(
        "session-label-render-collision-distinctness",
        certified,
        witness_digest(
            "session-label-render-collision-distinctness",
            certified,
            [
                left.identity_digest().as_str(),
                right.identity_digest().as_str(),
            ],
        ),
    )
}

pub(super) fn session_label_same_family_replay_collision_row(
) -> WorthQueryIdentityBoundaryHostileMatrixRow {
    let label = test_session_label("stop-class-collision");
    let error = WorthQueryRuntimeError::SessionLabelCollision {
        authority_lane: WorthQueryAuthorityLane::BranchLocalTruth,
        label: label.clone(),
    };
    let certified = matches!(
        error.stop_class(),
        WorthQueryStopClass::SessionLabelCollision {
            authority_lane: WorthQueryAuthorityLane::BranchLocalTruth,
            label: collision_label,
        } if collision_label == &label
    );
    WorthQueryIdentityBoundaryHostileMatrixRow::new(
        "session-label-same-family-replay-collision",
        certified,
        witness_digest(
            "session-label-same-family-replay-collision",
            certified,
            [label.identity_digest().as_str()],
        ),
    )
}

pub(super) fn joined_string_evidence_identity_collapse_row(
) -> WorthQueryIdentityBoundaryHostileMatrixRow {
    let authority = WorthQueryRuntimeEvidenceAuthority::new();
    let left = WorthQueryPreviewBasisAdmission::new(
        &authority,
        test_session_label("preview|basis"),
        WorthQueryEffectPolicy::SandboxedWriteIntent,
        crate::runtime::WorthQueryBasisAdmissionEvidenceRow::rows_from_values([
            "alpha",
            "beta|gamma",
        ]),
    );
    let right = WorthQueryPreviewBasisAdmission::new(
        &authority,
        test_session_label("preview"),
        WorthQueryEffectPolicy::SandboxedWriteIntent,
        crate::runtime::WorthQueryBasisAdmissionEvidenceRow::rows_from_values([
            "basis|alpha",
            "beta|gamma",
        ]),
    );
    let naive_left = ["preview|basis", "alpha", "beta|gamma"].join("|");
    let naive_right = ["preview", "basis|alpha", "beta|gamma"].join("|");
    let certified =
        naive_left == naive_right && left.admission_identity() != right.admission_identity();
    WorthQueryIdentityBoundaryHostileMatrixRow::new(
        "joined-string-evidence-identity-collapses-distinct-fields",
        certified,
        witness_digest(
            "joined-string-evidence-identity-collapses-distinct-fields",
            certified,
            [naive_left.as_str(), naive_right.as_str()],
        ),
    )
}

pub(super) fn consumer_message_substring_routing_drift_row(
) -> WorthQueryIdentityBoundaryHostileMatrixRow {
    let first_error =
        WorthQueryRuntimeError::UnsupportedFacadeFamily(WorthQueryRuntimeSupportDenial::new(
            WorthQueryRuntimeFacadeFamily::Temporal,
            WorthQueryRuntimeFamilySupportStatus::Supported,
            Some(WorthQueryRuntimeFamilyTeachingPosture::SupportGateOnly),
            "first temporal wording",
        ));
    let second_error =
        WorthQueryRuntimeError::UnsupportedFacadeFamily(WorthQueryRuntimeSupportDenial::new(
            WorthQueryRuntimeFacadeFamily::Temporal,
            WorthQueryRuntimeFamilySupportStatus::Supported,
            Some(WorthQueryRuntimeFamilyTeachingPosture::SupportGateOnly),
            "public runtime DX remains support-gated",
        ));
    let first_message = first_error.to_string();
    let second_message = second_error.to_string();
    let first_contains_support_gated = first_message.contains("support-gated");
    let second_contains_support_gated = second_message.contains("support-gated");
    let certified = matches!(
        first_error.stop_class(),
        WorthQueryStopClass::FamilyAdmissionDenied {
            family: WorthQueryRuntimeFacadeFamily::Temporal,
            status: WorthQueryRuntimeFamilySupportStatus::Supported,
            teaching_posture: Some(WorthQueryRuntimeFamilyTeachingPosture::SupportGateOnly),
            ..
        }
    ) && matches!(
        second_error.stop_class(),
        WorthQueryStopClass::FamilyAdmissionDenied {
            family: WorthQueryRuntimeFacadeFamily::Temporal,
            status: WorthQueryRuntimeFamilySupportStatus::Supported,
            teaching_posture: Some(WorthQueryRuntimeFamilyTeachingPosture::SupportGateOnly),
            ..
        }
    ) && first_contains_support_gated != second_contains_support_gated;
    WorthQueryIdentityBoundaryHostileMatrixRow::new(
        "consumer-message-substring-routing-drifts",
        certified,
        witness_digest(
            "consumer-message-substring-routing-drifts",
            certified,
            [first_message.as_str(), second_message.as_str()],
        ),
    )
}

fn graph_domain_invariant_denial(message: &str) -> WorthQueryGraphCompositionDomainInvariantDenial {
    WorthQueryGraphCompositionDomainInvariantDenial::from_contributed(
        "graph.family",
        message,
        WorthQueryGraphCompositionDomainInvariantSummary::from_parts(
            vec!["Task".to_string()],
            vec!["task_symbol".to_string()],
            vec!["same_batch_entity_relation_identity_edges".to_string()],
            vec!["mixed_existing_target_followup_mutation".to_string()],
            graph_domain_invariant_fixture_digest("program"),
            graph_domain_invariant_fixture_digest("breadth"),
            "components=1".to_string(),
        ),
    )
}

fn graph_domain_invariant_fixture_digest(
    role: &'static str,
) -> crate::evidence_identity::WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::MutationEvidenceAggregateDigest)
        .field_shape(
            WorthQueryEvidenceTag::new("role"),
            "graph-domain-invariant-fixture",
        )
        .field_shape(WorthQueryEvidenceTag::new("fixture"), role)
        .seal()
}

fn test_session_label(label: &str) -> WorthQuerySessionLabel {
    WorthQuerySessionLabel::scoped_strs("worth-query-identity-boundary", [label]).expect("label")
}

fn witness_digest<'a>(
    row_name: &'static str,
    certified: bool,
    evidence: impl IntoIterator<Item = &'a str>,
) -> String {
    worth_query_evidence_identity(WorthQueryEvidenceScope::RuntimeHostileCertificationArtifact)
        .field_shape(WorthQueryEvidenceTag::new("row_name"), row_name)
        .field_bool(WorthQueryEvidenceTag::new("certified"), certified)
        .field_value_sequence(WorthQueryEvidenceTag::new("evidence"), evidence)
        .seal()
        .as_str()
        .to_string()
}
