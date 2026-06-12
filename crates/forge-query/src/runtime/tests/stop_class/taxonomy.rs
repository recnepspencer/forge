use super::super::support::*;
use super::completeness_support::{existing_binding, representative_runtime_stop_errors};

#[test]
fn runtime_stop_class_taxonomy_covers_representative_runtime_error_variants() {
    for error in representative_runtime_stop_errors() {
        match error.stop_class() {
            ForgeQueryStopClass::MissingRuntimeComponent { .. }
            | ForgeQueryStopClass::ExistingTruthAssertionDenied { .. }
            | ForgeQueryStopClass::ExistingTruthProbeDenied { .. }
            | ForgeQueryStopClass::MutationBindingDenied { .. }
            | ForgeQueryStopClass::MutationContinuityDenied { .. }
            | ForgeQueryStopClass::GraphCompositionDenied { .. }
            | ForgeQueryStopClass::GraphCompositionDomainInvariantDenied { .. }
            | ForgeQueryStopClass::MutationNamingDenied { .. }
            | ForgeQueryStopClass::MutationTargetReferenceDenied { .. }
            | ForgeQueryStopClass::ReadCompositionDenied { .. }
            | ForgeQueryStopClass::ReadCompositionDomainInvariantDenied { .. }
            | ForgeQueryStopClass::Workspace { .. }
            | ForgeQueryStopClass::Program { .. }
            | ForgeQueryStopClass::RuntimeLookupFailed { .. }
            | ForgeQueryStopClass::MissingRuntimeArtifact { .. }
            | ForgeQueryStopClass::SharedReadStaleBasis { .. }
            | ForgeQueryStopClass::RuntimeDeclarationFailed { .. }
            | ForgeQueryStopClass::PreviewOperationEffectDenied { .. }
            | ForgeQueryStopClass::SessionLabelCollision { .. }
            | ForgeQueryStopClass::UnsupportedAuthorityRequirement { .. }
            | ForgeQueryStopClass::ExistingTruthAssertionRequiresAuthorityLane { .. }
            | ForgeQueryStopClass::IntentCommitDenied { .. }
            | ForgeQueryStopClass::IntentExecutionRoutingFailed { .. }
            | ForgeQueryStopClass::EffectPolicyDenied { .. }
            | ForgeQueryStopClass::PreviewPromotionDenied { .. }
            | ForgeQueryStopClass::FamilyAdmissionDenied { .. } => {}
        }
    }
}

#[test]
fn runtime_stop_class_ignores_runtime_declaration_message_wording() {
    let first = ForgeQueryRuntimeError::ComputedDeclaration {
        view_name: "tasks.typed-stop-class".to_string(),
        stage: "shape-materialization",
        message: "first wording".to_string(),
    };
    let second = ForgeQueryRuntimeError::ComputedDeclaration {
        view_name: "tasks.typed-stop-class".to_string(),
        stage: "shape-materialization",
        message: "second wording".to_string(),
    };

    for error in [first, second] {
        match error.stop_class() {
            ForgeQueryStopClass::RuntimeDeclarationFailed {
                kind, name, stage, ..
            } => {
                assert_eq!(
                    kind,
                    ForgeQueryRuntimeDeclarationFailureKind::ComputedDeclaration
                );
                assert_eq!(name, "tasks.typed-stop-class");
                assert_eq!(stage, "shape-materialization");
            }
            other => panic!("expected computed declaration stop class, got {other:?}"),
        }
    }
}

#[test]
fn runtime_stop_class_preserves_multiple_denial_kinds_within_the_same_family() {
    let binding = existing_binding();

    let assertion_denials = [
        ForgeQueryExistingTruthAssertionDenial::new(
            &binding,
            ForgeQueryExistingTruthAssertionDenialKind::MissingAssertedAspect,
            Some("status.value".to_string()),
            Some("\"open\"".to_string()),
            None,
            "missing asserted aspect",
        ),
        ForgeQueryExistingTruthAssertionDenial::new(
            &binding,
            ForgeQueryExistingTruthAssertionDenialKind::AssertedValueMismatch,
            Some("status.value".to_string()),
            Some("\"open\"".to_string()),
            Some("\"closed\"".to_string()),
            "asserted value mismatch",
        ),
    ];

    for denial in assertion_denials {
        let expected_kind = denial.kind();
        match ForgeQueryRuntimeError::ExistingTruthAssertionDenied(denial).stop_class() {
            ForgeQueryStopClass::ExistingTruthAssertionDenied { denial } => {
                assert_eq!(denial.kind(), expected_kind);
            }
            other => panic!("expected assertion-denial stop class, got {other:?}"),
        }
    }

    let read_denials = [
        ForgeQueryReadDenial::new(
            ForgeQueryReadDenialKind::ValidationDenied,
            "read validation failed",
        ),
        ForgeQueryReadDenial::new(
            ForgeQueryReadDenialKind::ExecutionDenied,
            "read execution failed",
        ),
    ];

    for denial in read_denials {
        let expected_kind = denial.kind().clone();
        match ForgeQueryRuntimeError::ReadCompositionDenied(denial).stop_class() {
            ForgeQueryStopClass::ReadCompositionDenied { denial } => {
                assert_eq!(denial.kind(), &expected_kind);
            }
            other => panic!("expected read-denial stop class, got {other:?}"),
        }
    }

    let naming_denials = [
        ForgeQueryNamingMutationDenial::new(
            &ForgeQueryNamingMutationIntent::attach_new_target(
                crate::runtime::ForgeQueryMutationAuthorityIdentity::naming_attachment(
                    crate::runtime::ForgeQueryNamingAttachmentAuthorityLabel::new("attachment-1")
                        .expect("naming attachment authority label"),
                )
                .expect("naming attachment identity"),
            ),
            ForgeQueryNamingMutationDenialKind::RequiresSameBatchTargetReference,
            "naming needs a same-batch target",
        ),
        ForgeQueryNamingMutationDenial::new(
            &ForgeQueryNamingMutationIntent::remove(
                crate::runtime::ForgeQueryMutationAuthorityIdentity::naming_attachment(
                    crate::runtime::ForgeQueryNamingAttachmentAuthorityLabel::new("attachment-1")
                        .expect("naming attachment authority label"),
                )
                .expect("naming attachment identity"),
                crate::runtime::ForgeQueryMutationAuthorityIdentity::naming_prior_authority(
                    crate::runtime::ForgeQueryNamingPriorAuthorityLabel::new("attachment-1")
                        .expect("naming prior authority label"),
                )
                .expect("naming prior authority identity"),
            ),
            ForgeQueryNamingMutationDenialKind::RequiresDeleteFamily,
            "naming requires delete family",
        ),
    ];

    for denial in naming_denials {
        let expected_kind = denial.kind();
        match ForgeQueryRuntimeError::MutationNamingDenied(denial).stop_class() {
            ForgeQueryStopClass::MutationNamingDenied { denial } => {
                assert_eq!(denial.kind(), expected_kind);
            }
            other => panic!("expected naming-denial stop class, got {other:?}"),
        }
    }
}
