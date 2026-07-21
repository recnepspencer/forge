use super::*;

#[test]
fn omitted_stage_and_typed_output_drift_localize_without_digest_oracles() {
    let original = trace(vec![stage("start"), stage("finish")]);
    let omitted = trace(vec![stage("start")]);
    assert_eq!(
        compare_exact_workflow_traces(&original, &omitted, Default::default()),
        WorthQueryReplayComparison::Diverged(WorthQueryReplayDivergence::StageSet)
    );

    let mut changed = trace(vec![stage("start"), stage("finish")]);
    changed.stages[1].output = WorthQueryWorkflowSemanticValue::Text("changed".into());
    assert_eq!(
        compare_exact_workflow_traces(&original, &changed, Default::default()),
        WorthQueryReplayComparison::Diverged(WorthQueryReplayDivergence::Output {
            stage: "finish".into()
        })
    );
}

#[test]
fn effect_drift_is_never_hidden_by_the_diagnostic_noise_allowance() {
    let original = trace(vec![stage("start")]);
    let mut changed = trace(vec![stage("start")]);
    changed.stages[0]
        .effects
        .push(WorthQueryEffectTraceMeaning {
            family: WorthQueryOperationEffectFamily::Mutation,
            mutation: None,
        });
    assert_eq!(
        compare_exact_workflow_traces(
            &original,
            &changed,
            WorthQueryReplayNoiseContract {
                diagnostic_warnings: true
            }
        ),
        WorthQueryReplayComparison::Diverged(WorthQueryReplayDivergence::Effect {
            stage: "start".into()
        })
    );
}

#[test]
fn invariant_identity_cannot_hide_different_effect_coverage() {
    let mut original = trace(vec![stage("mutate")]);
    original.stages[0].effects = vec![mutation_effect(1), mutation_effect(2)];
    original.stages[0].invariants = vec![WorthQueryInvariantTraceMeaning {
        invariant_role: "mutation-accepted".into(),
        installed_invariant_identity: "invariant:v1".into(),
        effect_indices: vec![0],
    }];
    let mut changed = original.clone();
    changed.stages[0].invariants[0].effect_indices = vec![1];

    assert_eq!(
        compare_exact_workflow_traces(&original, &changed, Default::default()),
        WorthQueryReplayComparison::Diverged(WorthQueryReplayDivergence::Invariant {
            stage: "mutate".into()
        })
    );
}

#[test]
fn same_effect_family_on_a_different_target_is_semantic_drift() {
    let mut original = trace(vec![stage("mutate")]);
    original.stages[0].effects.push(mutation_effect(1));
    let mut changed = trace(vec![stage("mutate")]);
    changed.stages[0].effects.push(mutation_effect(2));

    assert_eq!(
        compare_exact_workflow_traces(&original, &changed, Default::default()),
        WorthQueryReplayComparison::Diverged(WorthQueryReplayDivergence::Effect {
            stage: "mutate".into()
        })
    );
}

fn mutation_effect(local_slot: u64) -> WorthQueryEffectTraceMeaning {
    let target = crate::memory_workspace::WorthQueryEntityIdentity::from_relational_record(
        worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts::entity(1, local_slot, 0),
    );
    WorthQueryEffectTraceMeaning {
        family: WorthQueryOperationEffectFamily::Mutation,
        mutation: Some(WorthQueryMutationTraceMeaning {
            target_entity: Some(target),
            target_collection: Some("Vertex".into()),
            deltas: Vec::new(),
            declared_aspect_operations: Vec::new(),
            declared_aspect_value_digest: None,
            naming: None,
            continuity: None,
        }),
    }
}

#[test]
fn only_explicit_diagnostic_noise_can_converge() {
    let original = trace(vec![stage("start")]);
    let mut changed = trace(vec![stage("start")]);
    changed.stages[0]
        .warnings
        .push(WorthQueryWorkflowStageWarning::Advisory(
            "provider timing".into(),
        ));
    assert!(matches!(
        compare_exact_workflow_traces(&original, &changed, Default::default()),
        WorthQueryReplayComparison::Diverged(WorthQueryReplayDivergence::Diagnostic { .. })
    ));
    assert_eq!(
        compare_exact_workflow_traces(
            &original,
            &changed,
            WorthQueryReplayNoiseContract {
                diagnostic_warnings: true
            }
        ),
        WorthQueryReplayComparison::Equivalent
    );
}

#[test]
fn lineage_divergence_is_typed_and_localized_to_its_stage() {
    let original = trace(vec![stage("publish")]);
    let mut changed = trace(vec![stage("publish")]);
    changed.stages[0]
        .lineage
        .push(WorthQueryLineageTraceMeaning {
            outcome: lineage_outcome(),
            effect_indices: vec![0],
        });
    assert_eq!(
        compare_exact_workflow_traces(&original, &changed, Default::default()),
        WorthQueryReplayComparison::Diverged(WorthQueryReplayDivergence::Lineage {
            stage: "publish".into()
        })
    );
}

fn lineage_outcome() -> InstalledIdentityEvolutionOutcome {
    let continuity = crate::runtime::WorthQueryContinuityMutationEvidence::test_only(
        crate::runtime::WorthQueryContinuityMutationFamily::RebindExistingTarget,
        crate::runtime::WorthQueryContinuityOutcomeClass::ContinuesAsSingleSuccessor,
        "entity:1",
        vec!["entity:2".into()],
        None,
        Some("Vertex"),
    );
    let context =
        crate::identity_evolution::IdentityEvolutionQueryContext::lineage_traversal_for_test(
            crate::identity::CanonicalQueryDigest::from_parts(&["semantic-trace-query".into()]),
            crate::identity::BasisDigest::from_parts(&["semantic-trace-basis".into()]),
            crate::identity_evolution::LineageTraversalDescriptor::direct_successor_exact(
                continuity
                    .prior_authoritative_identity()
                    .evidence_identity()
                    .as_str(),
                continuity.successor_authoritative_identities()[0]
                    .evidence_identity()
                    .as_str(),
            ),
        );
    let admitted = crate::identity_evolution::admit_identity_evolution_query(context)
        .expect("test lineage admission");
    let artifact = crate::identity_evolution::execute_admitted_identity_evolution_query(&admitted)
        .expect("test lineage execution");
    InstalledIdentityEvolutionOutcome::from_execution(
        artifact,
        Some(continuity),
        None,
        crate::identity_evolution::InstalledIdentityEvolutionBinding {
            operation_identity: "operation",
            run_identity: "run",
            stage_identity: "publish",
            effect_receipt_identity: "effect:1".into(),
            establishing_entity_targets: Vec::new(),
        },
    )
    .expect("matching engine and mutation continuity evidence")
}

fn trace(stages: Vec<WorthQueryWorkflowStageTraceSemantics>) -> WorthQueryWorkflowTraceSemantics {
    WorthQueryWorkflowTraceSemantics {
        operation_identity: "operation".into(),
        stages,
        publication: None,
    }
}

fn stage(identity: &str) -> WorthQueryWorkflowStageTraceSemantics {
    WorthQueryWorkflowStageTraceSemantics {
        stage_identity: identity.into(),
        predecessor_stage_identities: Vec::new(),
        input: WorthQueryWorkflowSemanticValue::NotRequired,
        output: WorthQueryWorkflowSemanticValue::Text(identity.into()),
        result_state: None,
        warnings: Vec::new(),
        effects: Vec::new(),
        invariants: Vec::new(),
        conditional_path: Vec::new(),
        lineage: Vec::new(),
    }
}
