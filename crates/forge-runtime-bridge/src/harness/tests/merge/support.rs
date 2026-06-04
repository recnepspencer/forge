use forge_harness::facade::ScenarioPlan;

use crate::facade::{
    BridgeMergeConsumptionClass, BridgeMergeOntologyMappingEntry,
    BridgeMergeOntologyMappingSurface, BridgeMergeStructuralAdvisoryDisposition,
    CanonicalRelationalMergeClass, MergeHistoryDeclarationIdentity,
};
use crate::harness::fixtures::{
    BridgeHarnessFixture, InMemoryRelationalBridgeSource, RecordingSignalBridgeSink,
};

pub(super) fn runtime_with_merge(
    declaration: crate::facade::MergeHistoryDeclaration,
) -> crate::facade::RuntimeBridge {
    let source = InMemoryRelationalBridgeSource::default();
    crate::facade::RuntimeBridgeBuilder::new()
        .with_relational_source(source.clone())
        .with_truth_branch_head_source(source.clone())
        .with_continuity_lineage_source(source)
        .with_signal_sink(RecordingSignalBridgeSink::default())
        .register_mapping(super::super::support::registration())
        .register_merge(declaration)
        .build()
        .expect("bridge runtime should build with merge declaration")
}

pub(super) fn merge_fixture(
    scenario: &str,
    declaration: crate::facade::MergeHistoryDeclaration,
) -> forge_harness::facade::ScenarioFixture<BridgeHarnessFixture> {
    ScenarioPlan::new(
        scenario,
        BridgeHarnessFixture::new(vec![super::super::support::registration()])
            .with_policy(crate::facade::BridgeRuntimePolicy::development())
            .with_merge_declaration(declaration),
    )
    .declare_input("merge")
    .declare_observation("merge")
    .compile()
}

pub(super) fn many_to_one_mapping_declaration(
    declaration_identity: MergeHistoryDeclarationIdentity,
) -> crate::facade::MergeHistoryDeclaration {
    let authority_artifact_identity = format!("merge-artifact:{}", declaration_identity.as_str());
    crate::facade::MergeHistoryDeclaration::new(
        declaration_identity,
        BridgeMergeConsumptionClass::AspectReconciliationMerge,
        BridgeMergeOntologyMappingSurface::new(
            "rel-merge-v1",
            vec![
                BridgeMergeOntologyMappingEntry::direct_wrapper(
                    CanonicalRelationalMergeClass::AspectReconciliation,
                    BridgeMergeConsumptionClass::AspectReconciliationMerge,
                ),
                BridgeMergeOntologyMappingEntry::direct_wrapper(
                    CanonicalRelationalMergeClass::PolicyResolvedConflict,
                    BridgeMergeConsumptionClass::AspectReconciliationMerge,
                ),
            ],
        ),
        crate::facade::BridgeMergeAuthorityBasis::new(
            crate::facade::BridgeMergeAuthorityBasisKind::OrderedMergeCommit,
            authority_artifact_identity,
            "rel-merge-v1",
            "schema-policy-v1",
            crate::facade::BridgeMergeParentOrderProof::new(vec![
                crate::facade::TruthCommitIdentity::new("parent-a"),
                crate::facade::TruthCommitIdentity::new("parent-b"),
            ]),
        ),
    )
    .with_structural_advisory(BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent)
}
