use super::{execute_merge_request, MergeHarnessExecution, MergeHarnessTarget};
use crate::facade::{
    BridgeMappingId, BridgeMappingRegistration, BridgeMergeConsumptionClass,
    BridgeMergeDenialClass, BridgeMergePrecedenceStage, BridgeMergeRoutingOutcomeClass,
    BridgeMergeStructuralAdvisoryDisposition, CoarseRoutingMode, MappingSelector,
    MergeHistoryDeclarationIdentity, RuntimeBridgeBuilder, SignalInvalidationScope,
    TruthPatchScope,
};
use crate::harness::fixtures::{
    BridgeHarnessFixture, InMemoryRelationalBridgeSource, RecordingSignalBridgeSink,
};

fn runtime_with_merge(
    declaration: crate::facade::MergeHistoryDeclaration,
) -> crate::facade::RuntimeBridge {
    let source = InMemoryRelationalBridgeSource::default();
    RuntimeBridgeBuilder::new()
        .with_relational_source(source.clone())
        .with_truth_branch_head_source(source.clone())
        .with_continuity_lineage_source(source)
        .with_signal_sink(RecordingSignalBridgeSink::default())
        .register_mapping(registration())
        .register_merge(declaration)
        .build()
        .expect("bridge runtime should build with merge declaration")
}

fn fixture_with_merge(declaration: crate::facade::MergeHistoryDeclaration) -> BridgeHarnessFixture {
    BridgeHarnessFixture::new(vec![registration()]).with_merge_declaration(declaration)
}

fn registration() -> BridgeMappingRegistration {
    BridgeMappingRegistration::new(
        BridgeMappingId::admit_bridge_owned("profile-name"),
        TruthPatchScope::for_entity_field(
            MappingSelector::exact("user"),
            worth_foundational::facade::AspectKey::new("profile").expect("valid native aspect key"),
            worth_foundational::facade::FieldKey::new("name".to_owned())
                .expect("valid native field key"),
        ),
        crate::snapshot::SnapshotReadContract::scalar(
            worth_foundational::facade::AspectKey::new("profile").expect("valid native aspect key"),
            worth_foundational::facade::ScalarAspectType::String,
        ),
        SignalInvalidationScope::admit_bridge_owned("signal.profile"),
        CoarseRoutingMode::Direct,
    )
}

fn merge_declaration(
    declaration_identity: MergeHistoryDeclarationIdentity,
    class: BridgeMergeConsumptionClass,
) -> crate::facade::MergeHistoryDeclaration {
    let authority_artifact_identity = format!("merge-artifact:{}", declaration_identity.as_str());
    crate::facade::MergeHistoryDeclaration::new(
        declaration_identity,
        class,
        crate::facade::BridgeMergeOntologyMappingSurface::direct_phase_m9_0("rel-merge-v1"),
        crate::facade::BridgeMergeAuthorityBasis::new(
            crate::facade::BridgeMergeAuthorityBasisKind::OrderedMergeCommit,
            authority_artifact_identity,
            "rel-merge-v1",
            "schema-policy-v1",
            crate::facade::BridgeMergeParentOrderProof::new(vec![
                crate::truth_identity_fixtures::truth_commit_fixture("parent-a"),
                crate::truth_identity_fixtures::truth_commit_fixture("parent-b"),
            ]),
        ),
    )
}

#[test]
fn merge_execute_certification_retains_typed_success_evidence_before_terminal_export() {
    let declaration = merge_declaration(
        MergeHistoryDeclarationIdentity::admit_bridge_owned("merge:typed-execute-certification"),
        BridgeMergeConsumptionClass::AspectReconciliationMerge,
    )
    .with_structural_advisory(BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent);
    let runtime = runtime_with_merge(declaration.clone());
    let execution = execute_merge_request(
        &runtime,
        &fixture_with_merge(declaration.clone()),
        MergeHarnessTarget::Execute {
            declaration_identity: declaration.declaration_identity().clone(),
        },
    )
    .expect("merge execution should certify");

    let MergeHarnessExecution::Execute {
        certification_bundle,
    } = execution
    else {
        panic!("expected execute certification");
    };

    assert_eq!(
        certification_bundle.support_matrix().outcome_class(),
        BridgeMergeRoutingOutcomeClass::ContinuityCandidate
    );
    assert!(certification_bundle.support_matrix().continuity_published());
    assert!(certification_bundle.support_matrix().remap_published());
    assert_eq!(
        certification_bundle
            .ontology_mapping_report()
            .bridge_class(),
        BridgeMergeConsumptionClass::AspectReconciliationMerge
    );
    assert_eq!(
        certification_bundle
            .counter_snapshot()
            .merge_replay_request_count(),
        0
    );
    assert_eq!(
        certification_bundle
            .counter_snapshot()
            .merge_history_segment_scan_count(),
        1
    );
    assert_eq!(
        certification_bundle.record_evidence().bundle_digest(),
        certification_bundle.result_bundle_digest()
    );
}

#[test]
fn merge_replay_certification_retains_typed_replay_counter_evidence() {
    let declaration = merge_declaration(
        MergeHistoryDeclarationIdentity::admit_bridge_owned("merge:typed-replay-certification"),
        BridgeMergeConsumptionClass::AspectReconciliationMerge,
    )
    .with_structural_advisory(BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent);
    let runtime = runtime_with_merge(declaration.clone());
    let execution = execute_merge_request(
        &runtime,
        &fixture_with_merge(declaration.clone()),
        MergeHarnessTarget::Replay {
            declaration_identity: declaration.declaration_identity().clone(),
        },
    )
    .expect("merge replay should certify");

    let MergeHarnessExecution::Replay {
        certification_bundle,
    } = execution
    else {
        panic!("expected replay certification");
    };

    assert_eq!(
        certification_bundle.replay_digest(),
        Some(certification_bundle.result_bundle_digest())
    );
    assert_eq!(
        certification_bundle
            .counter_snapshot()
            .merge_replay_request_count(),
        1
    );
    assert_eq!(
        certification_bundle
            .counter_snapshot()
            .merge_replay_mismatch_count(),
        0
    );
}

#[test]
fn merge_denial_certification_retains_typed_stage_and_counter_evidence() {
    let declaration = merge_declaration(
        MergeHistoryDeclarationIdentity::admit_bridge_owned("merge:typed-denial-certification"),
        BridgeMergeConsumptionClass::TopologyRewireMerge,
    )
    .with_structural_advisory(BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent);
    let runtime = runtime_with_merge(declaration.clone());
    let execution = execute_merge_request(
        &runtime,
        &fixture_with_merge(declaration.clone()),
        MergeHarnessTarget::Execute {
            declaration_identity: declaration.declaration_identity().clone(),
        },
    )
    .expect("merge denial should certify");

    let MergeHarnessExecution::Execute {
        certification_bundle,
    } = execution
    else {
        panic!("expected execute certification");
    };

    assert_eq!(
        certification_bundle.support_matrix().outcome_class(),
        BridgeMergeRoutingOutcomeClass::Denied
    );
    assert_eq!(
        certification_bundle.denial_stage_report().blocked_stage(),
        Some(BridgeMergePrecedenceStage::DeletionTopologyGate)
    );
    assert_eq!(
        certification_bundle.denial_stage_report().denial_class(),
        Some(BridgeMergeDenialClass::TopologyRewireGate)
    );
    assert!(!certification_bundle.support_matrix().continuity_published());
    assert!(!certification_bundle.support_matrix().remap_published());
    assert_eq!(
        certification_bundle
            .counter_snapshot()
            .merge_topology_rewire_class_count(),
        1
    );
    assert_eq!(
        certification_bundle
            .counter_snapshot()
            .merge_widened_scan_count(),
        0
    );
}
