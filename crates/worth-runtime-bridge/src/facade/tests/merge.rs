use super::*;
use crate::facade::{
    BridgeMergeAuthoritativeLineageDisposition, BridgeMergeAuthorityBasis,
    BridgeMergeAuthorityBasisKind, BridgeMergeCausalFrontierDisposition,
    BridgeMergeConsumptionClass, BridgeMergeOntologyMappingSurface, BridgeMergeParentOrderProof,
    BridgeMergePrecedenceStage, BridgeMergeRoutingOutcomeClass, BridgeMergeSchemaPolicyDisposition,
    BridgeMergeStructuralAdvisoryDisposition, MergeHistoryDeclaration,
    MergeHistoryDeclarationIdentity,
};

mod admission;
mod publication;
mod replay;
mod routing_outcomes;

fn registered_merge(
    declaration_identity: MergeHistoryDeclarationIdentity,
    class: BridgeMergeConsumptionClass,
) -> MergeHistoryDeclaration {
    let authority_artifact_identity = format!("merge-artifact:{}", declaration_identity.as_str());
    MergeHistoryDeclaration::new(
        declaration_identity,
        class,
        BridgeMergeOntologyMappingSurface::direct_phase_m9_0("rel-merge-v1"),
        BridgeMergeAuthorityBasis::new(
            BridgeMergeAuthorityBasisKind::OrderedMergeCommit,
            authority_artifact_identity,
            "rel-merge-v1",
            "schema-policy-v1",
            BridgeMergeParentOrderProof::new(vec![
                crate::truth_identity_fixtures::truth_commit_fixture("parent-a"),
                crate::truth_identity_fixtures::truth_commit_fixture("parent-b"),
            ]),
        ),
    )
}
fn runtime_with_merge(declaration: MergeHistoryDeclaration) -> RuntimeBridge {
    RuntimeBridgeBuilder::new()
        .with_policy(BridgeRuntimePolicy::default())
        .with_relational_source(StaticSource)
        .with_source_adapter(StaticSourceAdapter)
        .with_truth_branch_head_source(StaticSource)
        .with_signal_sink(StaticSink)
        .register_source(registered_source(
            "source:analysis-snapshot",
            BridgeTruthViewSelector::branch_snapshot(
                crate::truth_identity_fixtures::truth_branch_fixture("analysis"),
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
            ),
            vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::BranchRead,
            ],
        ))
        .register_merge(declaration)
        .register_mapping(BridgeMappingRegistration::new(
            BridgeMappingId::admit_bridge_owned("mapping"),
            TruthPatchScope::for_entity_field(
                MappingSelector::exact("entity-1"),
                worth_foundational::facade::AspectKey::new("profile")
                    .expect("valid native aspect key"),
                worth_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid native field key"),
            ),
            crate::snapshot::SnapshotReadContract::scalar(
                worth_foundational::facade::AspectKey::new("profile")
                    .expect("valid native aspect key"),
                worth_foundational::facade::ScalarAspectType::String,
            ),
            SignalInvalidationScope::admit_bridge_owned("signal:profile"),
            CoarseRoutingMode::Direct,
        ))
        .build()
        .expect("runtime should build with merge declaration")
}
