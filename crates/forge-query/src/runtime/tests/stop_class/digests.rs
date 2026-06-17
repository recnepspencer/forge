use super::super::support::*;
use super::completeness_support::{
    representative_runtime_generated_stop_errors, representative_runtime_stop_errors,
    runtime_error_variant_key, stop_class_variant_key,
};
use super::consumer_support::routing::route_consumer_stop_class;
use super::consumer_support::runtime_errors::temporal_public_family_admission_error;

mod consumer_route_keys;
use consumer_route_keys::consumer_stop_route_key;

fn compose_certification_sequence_digest(
    tag: &'static str,
    values: impl IntoIterator<Item = impl AsRef<str>>,
) -> String {
    crate::ForgeQueryEvidenceIdentity::compose(
        crate::ForgeQueryEvidenceScope::RuntimeHostileCertificationArtifact,
    )
    .field_value_sequence(crate::ForgeQueryEvidenceTag::new(tag), values)
    .seal()
    .as_str()
    .to_string()
}

#[test]
fn typed_stop_class_taxonomy_phase_three_outputs_are_non_empty_and_stable() {
    let representative_errors = representative_runtime_stop_errors();
    let stop_class_digest = digest_stop_class_outputs(&representative_errors);
    let support_denial_digest = digest_support_denial_output(&representative_errors);
    let preview_promotion_digest = digest_preview_promotion_output(&representative_errors);
    let intent_denial_digest = digest_intent_denial_output(&representative_errors);
    let failure_digest = digest_failure_output(&representative_errors);

    assert!(!stop_class_digest.is_empty());
    assert!(!support_denial_digest.is_empty());
    assert!(!preview_promotion_digest.is_empty());
    assert!(!intent_denial_digest.is_empty());
    assert!(!failure_digest.is_empty());

    assert_ne!(stop_class_digest, support_denial_digest);
    assert_ne!(stop_class_digest, preview_promotion_digest);
    assert_ne!(stop_class_digest, intent_denial_digest);
    assert_ne!(support_denial_digest, preview_promotion_digest);
    assert_ne!(preview_promotion_digest, intent_denial_digest);
    assert_ne!(intent_denial_digest, failure_digest);
}

#[test]
fn typed_stop_class_matching_phase_four_outputs_are_non_empty_and_stable() {
    let representative_errors = representative_runtime_stop_errors();
    let runtime_generated_errors = representative_runtime_generated_stop_errors();
    let first_public_family_error = temporal_public_family_admission_error(
        "phase-four-digest-public-family-first",
        "first temporal wording",
    );
    let second_public_family_error = temporal_public_family_admission_error(
        "phase-four-digest-public-family-second",
        "second temporal wording",
    );

    let consumer_stop_route_digest = digest_consumer_stop_routes(&representative_errors);
    let public_family_admission_digest = digest_public_family_admission(&first_public_family_error);
    let runtime_generated_route_digest = digest_consumer_stop_routes(&runtime_generated_errors);
    let message_drift_probe_digest =
        digest_message_drift_probe(&first_public_family_error, &second_public_family_error);
    let failure_digest = digest_failure_output(&representative_errors);

    assert!(!consumer_stop_route_digest.is_empty());
    assert!(!public_family_admission_digest.is_empty());
    assert!(!runtime_generated_route_digest.is_empty());
    assert!(!message_drift_probe_digest.is_empty());
    assert!(!failure_digest.is_empty());

    assert_ne!(consumer_stop_route_digest, public_family_admission_digest);
    assert_ne!(consumer_stop_route_digest, runtime_generated_route_digest);
    assert_ne!(consumer_stop_route_digest, message_drift_probe_digest);
    assert_ne!(
        public_family_admission_digest,
        runtime_generated_route_digest
    );
    assert_ne!(public_family_admission_digest, message_drift_probe_digest);
    assert_ne!(runtime_generated_route_digest, message_drift_probe_digest);
}

fn digest_stop_class_outputs(errors: &[ForgeQueryRuntimeError]) -> String {
    compose_certification_sequence_digest(
        "stop_class_output",
        errors.iter().map(|error| {
            format!(
                "{}:{}",
                runtime_error_variant_key(error),
                stop_class_variant_key(error.stop_class())
            )
        }),
    )
}

fn digest_support_denial_output(errors: &[ForgeQueryRuntimeError]) -> String {
    let denial = errors
        .iter()
        .find_map(|error| match error.stop_class() {
            ForgeQueryStopClass::FamilyAdmissionDenied {
                family,
                status,
                teaching_posture,
                reason,
            } => Some(
                crate::ForgeQueryEvidenceIdentity::compose(
                    crate::ForgeQueryEvidenceScope::RuntimeHostileCertificationArtifact,
                )
                .field_shape(crate::ForgeQueryEvidenceTag::new("family"), family.as_str())
                .field_shape(crate::ForgeQueryEvidenceTag::new("status"), status.as_str())
                .field_shape(
                    crate::ForgeQueryEvidenceTag::new("teaching_posture"),
                    teaching_posture
                        .map(ForgeQueryRuntimeFamilyTeachingPosture::as_str)
                        .unwrap_or("none"),
                )
                .field_value(crate::ForgeQueryEvidenceTag::new("reason"), reason)
                .seal()
                .as_str()
                .to_string(),
            ),
            _ => None,
        })
        .expect("phase-3 representatives should contain a family admission denial");

    assert!(!denial.is_empty());
    denial
}

fn digest_preview_promotion_output(errors: &[ForgeQueryRuntimeError]) -> String {
    let preview_digests = errors
        .iter()
        .filter_map(|error| match error.stop_class() {
            ForgeQueryStopClass::PreviewPromotionDenied { kind, evidence } => {
                Some(format!("{}:{}", kind.as_str(), evidence.denial_digest()))
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(
        preview_digests.len(),
        4,
        "phase-3 representatives should cover every preview-promotion denial kind"
    );

    compose_certification_sequence_digest("preview_promotion_output", preview_digests)
}

fn digest_intent_denial_output(errors: &[ForgeQueryRuntimeError]) -> String {
    let intent_digests = errors
        .iter()
        .filter_map(|error| match error.stop_class() {
            ForgeQueryStopClass::IntentCommitDenied { evidence, .. } => {
                Some(format!("commit:{}", evidence.denial_digest().as_str()))
            }
            ForgeQueryStopClass::IntentExecutionRoutingFailed {
                evidence, source, ..
            } => Some(format!(
                "routing:{}:{}",
                evidence.failure_digest(),
                stop_class_variant_key(source.stop_class())
            )),
            _ => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(
        intent_digests.len(),
        2,
        "phase-3 representatives should cover both intent stop paths"
    );

    compose_certification_sequence_digest("intent_denial_output", intent_digests)
}

fn digest_failure_output(errors: &[ForgeQueryRuntimeError]) -> String {
    let representative_failures = errors
        .iter()
        .filter_map(|error| match error.stop_class() {
            ForgeQueryStopClass::RuntimeDeclarationFailed {
                kind,
                name,
                stage,
                message,
            } => Some(format!(
                "declaration:{}:{}:{}:{}",
                declaration_kind_key(kind),
                name,
                stage,
                message
            )),
            ForgeQueryStopClass::PreviewOperationEffectDenied {
                label,
                stage,
                message,
            } => Some(format!(
                "preview_operation_effect_denied:{}:{}:{}",
                label.identity_digest().as_str(),
                stage,
                message
            )),
            ForgeQueryStopClass::UnsupportedAuthorityRequirement { requirement } => {
                Some(format!("authority_requirement:{}", requirement.as_str()))
            }
            ForgeQueryStopClass::ExistingTruthAssertionRequiresAuthorityLane { required_lane } => {
                Some(format!(
                    "existing_truth_assertion_requires_authority_lane:{}",
                    required_lane.as_str()
                ))
            }
            ForgeQueryStopClass::Workspace { error } => Some(format!("workspace:{error}")),
            ForgeQueryStopClass::Program { error } => Some(format!("program:{error}")),
            _ => None,
        })
        .collect::<Vec<_>>();

    compose_certification_sequence_digest("failure_output", representative_failures)
}

fn digest_consumer_stop_routes(errors: &[ForgeQueryRuntimeError]) -> String {
    compose_certification_sequence_digest(
        "consumer_stop_route",
        errors
            .iter()
            .map(|error| consumer_stop_route_key(&route_consumer_stop_class(error))),
    )
}

fn digest_public_family_admission(error: &ForgeQueryRuntimeError) -> String {
    match error.stop_class() {
        ForgeQueryStopClass::FamilyAdmissionDenied {
            family,
            status,
            teaching_posture,
            reason,
        } => crate::ForgeQueryEvidenceIdentity::compose(
            crate::ForgeQueryEvidenceScope::RuntimeHostileCertificationArtifact,
        )
        .field_shape(crate::ForgeQueryEvidenceTag::new("family"), family.as_str())
        .field_shape(crate::ForgeQueryEvidenceTag::new("status"), status.as_str())
        .field_shape(
            crate::ForgeQueryEvidenceTag::new("teaching_posture"),
            teaching_posture
                .map(ForgeQueryRuntimeFamilyTeachingPosture::as_str)
                .unwrap_or("none"),
        )
        .field_value(crate::ForgeQueryEvidenceTag::new("reason"), reason)
        .seal()
        .as_str()
        .to_string(),
        other => panic!("expected public family admission denial, got {other:?}"),
    }
}

fn digest_message_drift_probe(
    first_error: &ForgeQueryRuntimeError,
    second_error: &ForgeQueryRuntimeError,
) -> String {
    let first_probe = first_error.to_string().contains("first temporal wording");
    let second_probe = second_error.to_string().contains("first temporal wording");
    assert!(
        first_probe,
        "the first wording probe should match before drift"
    );
    assert!(
        !second_probe,
        "the first wording probe should fail after presentation drift"
    );

    crate::ForgeQueryEvidenceIdentity::compose(
        crate::ForgeQueryEvidenceScope::RuntimeHostileCertificationArtifact,
    )
    .field_value(
        crate::ForgeQueryEvidenceTag::new("typed_first_route"),
        consumer_stop_route_key(&route_consumer_stop_class(first_error)),
    )
    .field_value(
        crate::ForgeQueryEvidenceTag::new("typed_second_route"),
        consumer_stop_route_key(&route_consumer_stop_class(second_error)),
    )
    .field_bool(
        crate::ForgeQueryEvidenceTag::new("probe_first"),
        first_probe,
    )
    .field_bool(
        crate::ForgeQueryEvidenceTag::new("probe_second"),
        second_probe,
    )
    .seal()
    .as_str()
    .to_string()
}

fn declaration_kind_key(kind: ForgeQueryRuntimeDeclarationFailureKind) -> &'static str {
    match kind {
        ForgeQueryRuntimeDeclarationFailureKind::RetainedRowDecode => "retained_row_decode",
        ForgeQueryRuntimeDeclarationFailureKind::ComputedDeclaration => "computed_declaration",
        ForgeQueryRuntimeDeclarationFailureKind::EffectDeclaration => "effect_declaration",
        ForgeQueryRuntimeDeclarationFailureKind::LiveSubscriptionInstallation => {
            "live_subscription_installation"
        }
        ForgeQueryRuntimeDeclarationFailureKind::InvariantRegistration => "invariant_registration",
    }
}
