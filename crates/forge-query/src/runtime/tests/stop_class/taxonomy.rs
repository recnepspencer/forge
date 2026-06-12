use super::super::support::*;

fn existing_binding() -> ForgeQueryExistingTruthTargetBinding {
    ForgeQueryExistingTruthTargetBinding::direct_entity("authority:task-1", "Task:1")
        .expect("binding should build")
        .in_target_collection("Task")
        .expect("collection should build")
}

#[test]
fn runtime_stop_class_taxonomy_covers_manual_runtime_error_variants() {
    let binding = existing_binding();
    let assertion_denial = ForgeQueryExistingTruthAssertionDenial::new(
        &binding,
        ForgeQueryExistingTruthAssertionDenialKind::MissingAssertedAspect,
        Some("status.value".to_string()),
        Some("\"open\"".to_string()),
        None,
        "missing asserted aspect",
    );
    let probe_denial = ForgeQueryExistingTruthProbeDenial::new(
        &binding,
        ForgeQueryExistingTruthProbeDenialKind::MissingProbedAspect,
        Some("status.value".to_string()),
        "missing probed aspect",
    );
    let binding_denial = ForgeQueryExistingTruthBindingDenial::new(
        &binding,
        ForgeQueryExistingTruthBindingDenialKind::CollectionMismatch,
        "wrong collection",
    );
    let continuity_intent =
        ForgeQueryContinuityMutationIntent::rebind_existing_target("authority:task-1", "Task:2")
            .expect("continuity intent should build");
    let continuity_denial = ForgeQueryContinuityMutationDenial::new(
        &continuity_intent,
        Some(&binding),
        ForgeQueryContinuityMutationDenialKind::RequiresExistingTruthBinding,
        "continuity requires binding",
    );
    let graph_denial = ForgeQueryGraphCompositionDenial::new(
        ForgeQueryGraphCompositionDenialKind::DuplicateSymbolDeclaration,
        Some("task_symbol".to_string()),
        Some("Task".to_string()),
        "duplicate symbol",
    );
    let graph_domain_denial = ForgeQueryGraphCompositionDomainInvariantDenial::from_contributed(
        "graph.family",
        "graph domain invariant failed",
        ForgeQueryGraphCompositionDomainInvariantSummary::from_parts(
            vec!["Task".to_string()],
            vec!["task_symbol".to_string()],
            vec!["same_batch_entity_relation_identity_edges".to_string()],
            vec!["mixed_existing_target_followup_mutation".to_string()],
            "program-digest".to_string(),
            "breadth-digest".to_string(),
            "components=1".to_string(),
        ),
    );
    let naming_intent =
        ForgeQueryNamingMutationIntent::attach_new_target("attachment-1").expect("intent");
    let naming_denial = ForgeQueryNamingMutationDenial::new(
        &naming_intent,
        ForgeQueryNamingMutationDenialKind::RequiresSameBatchTargetReference,
        "naming needs a same-batch target",
    );
    let target_reference =
        ForgeQuerySymbolicTargetReference::new("task_symbol").expect("reference should build");
    let symbolic_denial = ForgeQuerySymbolicTargetReferenceDenial::new(
        &target_reference,
        ForgeQuerySymbolicTargetReferenceDenialKind::UnresolvedSameBatchTarget,
        "same-batch target unresolved",
    );
    let read_denial = ForgeQueryReadDenial::new(
        ForgeQueryReadDenialKind::ValidationDenied,
        "read validation failed",
    );
    let effect_policy_denial = ForgeQueryEffectPolicy::DeriveOnly
        .admit(
            ForgeQueryEffectAction::WriteIntent,
            ForgeQueryAuthorityLane::AuthoritativeTruth,
        )
        .expect_err("derive-only write intent should deny");
    let support_denial = ForgeQueryRuntimeSupportDenial::new(
        ForgeQueryRuntimeFacadeFamily::Temporal,
        ForgeQueryRuntimeFamilySupportStatus::Unsupported,
        Some(ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly),
        "temporal gate is closed",
    );

    let cases = vec![
        ForgeQueryRuntimeError::MissingBackend,
        ForgeQueryRuntimeError::MissingRuntimeBridge,
        ForgeQueryRuntimeError::MissingSchemaAdapter,
        ForgeQueryRuntimeError::MissingSourceAdapter,
        ForgeQueryRuntimeError::MissingWriteAuthority,
        ForgeQueryRuntimeError::MissingSignalSink,
        ForgeQueryRuntimeError::MissingSubscriptionActivation,
        ForgeQueryRuntimeError::MissingPreviewBasis,
        ForgeQueryRuntimeError::MissingInspectorEvidence,
        ForgeQueryRuntimeError::MissingIntentAuthority,
        ForgeQueryRuntimeError::ExistingTruthAssertionDenied(assertion_denial),
        ForgeQueryRuntimeError::ExistingTruthProbeDenied(probe_denial),
        ForgeQueryRuntimeError::MutationBindingDenied(binding_denial),
        ForgeQueryRuntimeError::MutationContinuityDenied(continuity_denial),
        ForgeQueryRuntimeError::GraphCompositionDenied(graph_denial),
        ForgeQueryRuntimeError::GraphCompositionDomainInvariantDenied(graph_domain_denial),
        ForgeQueryRuntimeError::MutationNamingDenied(naming_denial),
        ForgeQueryRuntimeError::MutationTargetReferenceDenied(symbolic_denial),
        ForgeQueryRuntimeError::ReadCompositionDenied(read_denial),
        ForgeQueryRuntimeError::Workspace(ForgeQueryWorkspaceError::new("workspace failed")),
        ForgeQueryRuntimeError::Program(ForgeQueryProgramError::new("program failed")),
        ForgeQueryRuntimeError::UnknownProgram("missing.program".to_string()),
        ForgeQueryRuntimeError::UnknownOperation {
            program_id: "program.id".to_string(),
            operation_id: "operation.id".to_string(),
        },
        ForgeQueryRuntimeError::MissingLiveView("view.live".to_string()),
        ForgeQueryRuntimeError::MissingLiveSubscription("sub.live".to_string()),
        ForgeQueryRuntimeError::MissingDerivedView("view.derived".to_string()),
        ForgeQueryRuntimeError::MissingEffect("effect.name".to_string()),
        ForgeQueryRuntimeError::MissingPendingWriteIntent("effect.name".to_string()),
        ForgeQueryRuntimeError::RetainedRowDecode {
            view_name: "view.retained".to_string(),
            stage: "decode",
            message: "decode failed".to_string(),
        },
        ForgeQueryRuntimeError::ComputedDeclaration {
            view_name: "view.computed".to_string(),
            stage: "declare",
            message: "computed failed".to_string(),
        },
        ForgeQueryRuntimeError::EffectDeclaration {
            effect_name: "effect.declare".to_string(),
            stage: "declare",
            message: "effect failed".to_string(),
        },
        ForgeQueryRuntimeError::LiveSubscriptionInstallation {
            view_name: "view.live".to_string(),
            stage: "install",
            message: "install failed".to_string(),
        },
        ForgeQueryRuntimeError::UnsupportedAuthority("authoritative-lane".to_string()),
        ForgeQueryRuntimeError::EffectPolicyDenied(effect_policy_denial),
        ForgeQueryRuntimeError::InvariantRegistration {
            stage: "registration",
            message: "registration failed".to_string(),
        },
        ForgeQueryRuntimeError::SessionLabelCollision {
            authority_lane: ForgeQueryAuthorityLane::PreviewTruth,
            label: test_session_label("stop-class-collision"),
        },
        ForgeQueryRuntimeError::PreviewOperationEffectDenied {
            label: "preview-label".to_string(),
            stage: "effect-admission",
            message: "preview declaration denied".to_string(),
        },
        ForgeQueryRuntimeError::UnsupportedFacadeFamily(support_denial),
    ];

    for error in cases {
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
            | ForgeQueryStopClass::Workspace { .. }
            | ForgeQueryStopClass::Program { .. }
            | ForgeQueryStopClass::RuntimeLookupFailed { .. }
            | ForgeQueryStopClass::MissingRuntimeArtifact { .. }
            | ForgeQueryStopClass::RuntimeDeclarationFailed { .. }
            | ForgeQueryStopClass::SessionLabelCollision { .. }
            | ForgeQueryStopClass::UnsupportedAuthority { .. }
            | ForgeQueryStopClass::EffectPolicyDenied { .. }
            | ForgeQueryStopClass::FamilyAdmissionDenied { .. } => {}
            other => panic!("unexpected stop class: {other:?}"),
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
            &ForgeQueryNamingMutationIntent::attach_new_target("attachment-1").expect("intent"),
            ForgeQueryNamingMutationDenialKind::RequiresSameBatchTargetReference,
            "naming needs a same-batch target",
        ),
        ForgeQueryNamingMutationDenial::new(
            &ForgeQueryNamingMutationIntent::remove("attachment-1", "Task:1")
                .expect("remove intent"),
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
