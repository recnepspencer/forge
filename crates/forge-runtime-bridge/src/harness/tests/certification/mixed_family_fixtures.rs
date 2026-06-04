use forge_harness::facade::ScenarioPlan;

use super::super::support::{
    coarse_snapshot_record, committed_patch, committed_patch_on_branch, merge_declaration,
    registration, snapshot,
};
use crate::facade::{
    BridgeSourceCapability, BridgeSourceCapabilitySet, BridgeTruthViewSelector,
    MergeHistoryDeclarationIdentity, StructuralFingerprintEquivalenceContract,
    StructuralFingerprintFamily, StructuralFingerprintNormalizationRule,
    StructuralFingerprintOmissionPolicy, StructuralFingerprintOrderingRule,
    StructuralIdentityDeclaration, StructuralIdentityDeclarationIdentity, StructuralSchemaIdentity,
    StructuralTruthViewBasis, TruthBranchIdentity, TruthSnapshotIdentity,
};
use crate::harness::fixtures::BridgeHarnessFixture;
use crate::source::{SourceDeclaration, SourceDeclarationIdentity};

pub(crate) fn mixed_stream_fixture(
    name: &str,
) -> forge_harness::facade::ScenarioFixture<BridgeHarnessFixture> {
    ScenarioPlan::new(
        name,
        BridgeHarnessFixture::new(vec![registration()])
            .with_policy(crate::facade::BridgeRuntimePolicy::development())
            .with_committed_patch(committed_patch(
                crate::facade::TruthCommitIdentity::new("commit-a"),
                crate::facade::TruthPatchIdentity::new("patch-a"),
                TruthSnapshotIdentity::new("snapshot-a"),
                forge_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ))
            .with_committed_patch(committed_patch(
                crate::facade::TruthCommitIdentity::new("commit-b"),
                crate::facade::TruthPatchIdentity::new("patch-b"),
                TruthSnapshotIdentity::new("snapshot-a"),
                forge_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ))
            .with_snapshot(snapshot(TruthSnapshotIdentity::new("snapshot-a"), "alice")),
    )
    .declare_input("stream")
    .declare_observation("stream")
    .compile()
}

fn mixed_source_declaration(declaration_identity: SourceDeclarationIdentity) -> SourceDeclaration {
    SourceDeclaration::new(
        declaration_identity,
        BridgeTruthViewSelector::historical_commit(
            TruthBranchIdentity::new("analysis"),
            crate::facade::TruthCommitIdentity::new("commit-a"),
        ),
        BridgeSourceCapabilitySet::new(vec![
            BridgeSourceCapability::SnapshotRead,
            BridgeSourceCapability::HistoricalRead,
            BridgeSourceCapability::BranchRead,
            BridgeSourceCapability::ReplayContinuityRead,
        ]),
    )
}

pub(crate) fn mixed_source_fixture(
    name: &str,
) -> forge_harness::facade::ScenarioFixture<BridgeHarnessFixture> {
    ScenarioPlan::new(
        name,
        BridgeHarnessFixture::new(vec![registration()])
            .with_policy(crate::facade::BridgeRuntimePolicy::development())
            .with_source_declaration(mixed_source_declaration(SourceDeclarationIdentity::new(
                "source:analysis-history",
            )))
            .with_source_adapter_capabilities(BridgeSourceCapabilitySet::new(vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::HistoricalRead,
                BridgeSourceCapability::BranchRead,
                BridgeSourceCapability::ReplayContinuityRead,
            ]))
            .with_committed_patch(committed_patch_on_branch(
                crate::facade::TruthBranchIdentity::new("analysis"),
                crate::facade::TruthCommitIdentity::new("commit-a"),
                crate::facade::TruthPatchIdentity::new("patch-a"),
                TruthSnapshotIdentity::new("snapshot-a"),
                forge_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ))
            .with_snapshot(snapshot(TruthSnapshotIdentity::new("snapshot-a"), "alice")),
    )
    .declare_input("source")
    .declare_observation("source")
    .compile()
}

fn mixed_structural_snapshot(
    snapshot_identity: TruthSnapshotIdentity,
    value: &str,
) -> crate::harness::fixtures::SnapshotFixture {
    let mismatch_identity = snapshot_identity.as_str().to_owned();
    crate::harness::fixtures::SnapshotFixture::new(
        snapshot_identity,
        vec![
            coarse_snapshot_record(
                "entity-1",
                "profile",
                forge_foundational::facade::AspectValue::String(value.into()),
            ),
            coarse_snapshot_record(
                "entity-2",
                "profile",
                forge_foundational::facade::AspectValue::String(value.into()),
            ),
            coarse_snapshot_record(
                "entity-3",
                "profile",
                forge_foundational::facade::AspectValue::String(
                    format!("shape-mismatch-{mismatch_identity}").into(),
                ),
            ),
        ],
    )
}

fn mixed_structural_remap_declaration(
    declaration_identity: StructuralIdentityDeclarationIdentity,
) -> StructuralIdentityDeclaration {
    StructuralIdentityDeclaration::advisory_remap(
        declaration_identity,
        StructuralSchemaIdentity::new("schema:geometry"),
        StructuralFingerprintEquivalenceContract::new(
            StructuralSchemaIdentity::new("schema:geometry"),
            StructuralFingerprintFamily::TopologyFingerprint,
            "geometry-topology-v1",
            StructuralFingerprintNormalizationRule::SchemaDeclaredCanonicalForm,
            StructuralFingerprintOrderingRule::SchemaDeclaredCanonicalOrder,
            StructuralFingerprintOmissionPolicy::SchemaDeclaredOmissionPolicy,
        ),
        StructuralTruthViewBasis::explicit_snapshot(BridgeTruthViewSelector::branch_snapshot(
            TruthBranchIdentity::new("analysis"),
            TruthSnapshotIdentity::new("snapshot-a"),
        )),
    )
}

pub(crate) fn mixed_structural_fixture(
    name: &str,
) -> forge_harness::facade::ScenarioFixture<BridgeHarnessFixture> {
    ScenarioPlan::new(
        name,
        BridgeHarnessFixture::new(vec![registration()])
            .with_policy(crate::facade::BridgeRuntimePolicy::development())
            .with_structural_declaration(mixed_structural_remap_declaration(
                StructuralIdentityDeclarationIdentity::new("structural:analysis-remap"),
            ))
            .with_committed_patch(committed_patch_on_branch(
                crate::facade::TruthBranchIdentity::new("analysis"),
                crate::facade::TruthCommitIdentity::new("commit-a"),
                crate::facade::TruthPatchIdentity::new("patch-a"),
                TruthSnapshotIdentity::new("snapshot-a"),
                forge_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ))
            .with_committed_patch(committed_patch_on_branch(
                crate::facade::TruthBranchIdentity::new("analysis"),
                crate::facade::TruthCommitIdentity::new("commit-b"),
                crate::facade::TruthPatchIdentity::new("patch-b"),
                TruthSnapshotIdentity::new("snapshot-b"),
                forge_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ))
            .with_snapshot(mixed_structural_snapshot(
                TruthSnapshotIdentity::new("snapshot-a"),
                "alice",
            ))
            .with_snapshot(mixed_structural_snapshot(
                TruthSnapshotIdentity::new("snapshot-b"),
                "bob",
            )),
    )
    .declare_input("structural")
    .declare_observation("structural")
    .compile()
}

pub(crate) fn mixed_merge_fixture(
    name: &str,
) -> forge_harness::facade::ScenarioFixture<BridgeHarnessFixture> {
    ScenarioPlan::new(
        name,
        BridgeHarnessFixture::new(vec![registration()])
            .with_policy(crate::facade::BridgeRuntimePolicy::development())
            .with_merge_declaration(merge_declaration(
                MergeHistoryDeclarationIdentity::new("merge:m13-mixed"),
                crate::facade::BridgeMergeConsumptionClass::AspectReconciliationMerge,
                [
                    crate::facade::TruthCommitIdentity::new("parent-a"),
                    crate::facade::TruthCommitIdentity::new("parent-b"),
                ],
            ))
            .with_merge_declaration(merge_declaration(
                MergeHistoryDeclarationIdentity::new("merge:m13-topology-denial"),
                crate::facade::BridgeMergeConsumptionClass::TopologyRewireMerge,
                [
                    crate::facade::TruthCommitIdentity::new("parent-a"),
                    crate::facade::TruthCommitIdentity::new("parent-b"),
                ],
            )),
    )
    .declare_input("merge")
    .declare_observation("merge")
    .compile()
}

pub(crate) fn mixed_policy_fixture(
    name: &str,
) -> forge_harness::facade::ScenarioFixture<BridgeHarnessFixture> {
    ScenarioPlan::new(
        name,
        BridgeHarnessFixture::new(vec![registration()])
            .with_policy(crate::facade::BridgeRuntimePolicy::development())
            .with_committed_patch(committed_patch(
                crate::facade::TruthCommitIdentity::new("commit-a"),
                crate::facade::TruthPatchIdentity::new("patch-a"),
                TruthSnapshotIdentity::new("snapshot-a"),
                forge_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ))
            .with_snapshot(snapshot(TruthSnapshotIdentity::new("snapshot-a"), "alice")),
    )
    .declare_input("policy")
    .declare_observation("policy")
    .compile()
}

pub(crate) fn mixed_speculation_fixture(
    name: &str,
) -> forge_harness::facade::ScenarioFixture<BridgeHarnessFixture> {
    ScenarioPlan::new(
        name,
        BridgeHarnessFixture::new(vec![registration()])
            .with_policy(crate::facade::BridgeRuntimePolicy::development())
            .with_committed_patch(committed_patch(
                crate::facade::TruthCommitIdentity::new("commit-a"),
                crate::facade::TruthPatchIdentity::new("patch-a"),
                TruthSnapshotIdentity::new("snapshot-a"),
                forge_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ))
            .with_snapshot(snapshot(TruthSnapshotIdentity::new("snapshot-a"), "alice")),
    )
    .declare_input("speculation")
    .declare_observation("speculation")
    .compile()
}

pub(crate) fn mixed_writeback_fixture(
    name: &str,
) -> forge_harness::facade::ScenarioFixture<BridgeHarnessFixture> {
    ScenarioPlan::new(
        name,
        BridgeHarnessFixture::new(vec![registration()])
            .with_policy(crate::facade::BridgeRuntimePolicy::development())
            .with_committed_patch(committed_patch(
                crate::facade::TruthCommitIdentity::new("commit-a"),
                crate::facade::TruthPatchIdentity::new("patch-a"),
                TruthSnapshotIdentity::new("snapshot-a"),
                forge_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ))
            .with_snapshot(snapshot(TruthSnapshotIdentity::new("snapshot-a"), "alice")),
    )
    .declare_input("writeback")
    .declare_observation("writeback")
    .compile()
}
