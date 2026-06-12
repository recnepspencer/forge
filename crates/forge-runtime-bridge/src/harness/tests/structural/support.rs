use forge_harness::facade::ScenarioPlan;
use forge_harness::facade::{ExecutionProfile, ExecutionRequest, HarnessAdapter, RunRecord};

use crate::facade::{
    BridgeTruthViewSelector, SnapshotReadRecord, SnapshotReadRequest,
    StructuralFingerprintEquivalenceContract, StructuralFingerprintFamily,
    StructuralFingerprintNormalizationRule, StructuralFingerprintOmissionPolicy,
    StructuralFingerprintOrderingRule, StructuralIdentityDeclaration,
    StructuralIdentityDeclarationIdentity, StructuralSchemaIdentity, StructuralTruthViewBasis,
    TruthSnapshotIdentity,
};
use crate::harness::adapter::{BridgeHarnessAdapter, BridgeHarnessTargetId};
use crate::harness::fixtures::{BridgeHarnessFixture, SnapshotFixture};

use super::super::support::{committed_patch_on_branch, registration};

pub(super) fn structural_fixture(
    name: &str,
) -> forge_harness::facade::ScenarioFixture<BridgeHarnessFixture> {
    ScenarioPlan::new(
        name,
        BridgeHarnessFixture::new(vec![registration()])
            .with_policy(crate::facade::BridgeRuntimePolicy::development())
            .with_structural_declaration(remap_declaration(
                StructuralIdentityDeclarationIdentity::new("structural:analysis-remap"),
            ))
            .with_structural_declaration(branch_declaration(
                StructuralIdentityDeclarationIdentity::new("structural:analysis-branch-compare"),
            ))
            .with_structural_declaration(branch_head_declaration(
                StructuralIdentityDeclarationIdentity::new(
                    "structural:analysis-branch-head-compare",
                ),
            ))
            .with_committed_patch(committed_patch_on_branch(
                crate::truth_identity_fixtures::truth_branch_fixture("analysis"),
                crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
                crate::truth_identity_fixtures::truth_patch_fixture("patch-a"),
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
                forge_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ))
            .with_committed_patch(committed_patch_on_branch(
                crate::truth_identity_fixtures::truth_branch_fixture("analysis"),
                crate::truth_identity_fixtures::truth_commit_fixture("commit-b"),
                crate::truth_identity_fixtures::truth_patch_fixture("patch-b"),
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-b"),
                forge_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ))
            .with_committed_patch(committed_patch_on_branch(
                crate::truth_identity_fixtures::truth_branch_fixture("left"),
                crate::truth_identity_fixtures::truth_commit_fixture("commit-left-a"),
                crate::truth_identity_fixtures::truth_patch_fixture("patch-left-a"),
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
                forge_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ))
            .with_committed_patch(committed_patch_on_branch(
                crate::truth_identity_fixtures::truth_branch_fixture("right"),
                crate::truth_identity_fixtures::truth_commit_fixture("commit-right-b"),
                crate::truth_identity_fixtures::truth_patch_fixture("patch-right-b"),
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-b"),
                forge_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ))
            .with_snapshot(structural_snapshot(
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
                "alice",
            ))
            .with_snapshot(structural_snapshot(
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-b"),
                "bob",
            )),
    )
    .declare_input("structural")
    .declare_observation("structural")
    .compile()
}

fn structural_snapshot(snapshot_identity: TruthSnapshotIdentity, value: &str) -> SnapshotFixture {
    let mismatch_identity = snapshot_identity.as_str().to_owned();
    SnapshotFixture::new(
        snapshot_identity,
        vec![
            structural_snapshot_record(
                "entity-1",
                forge_foundational::facade::AspectValue::String((value).into()),
            ),
            structural_snapshot_record(
                "entity-2",
                forge_foundational::facade::AspectValue::String((value).into()),
            ),
            structural_snapshot_record(
                "entity-3",
                forge_foundational::facade::AspectValue::String(
                    (format!("shape-mismatch-{mismatch_identity}")).into(),
                ),
            ),
        ],
    )
}

fn structural_snapshot_record(
    entity_identity: &str,
    value: forge_foundational::facade::AspectValue,
) -> SnapshotReadRecord {
    SnapshotReadRecord::for_request(
        &SnapshotReadRequest::for_coarse(
            entity_identity,
            crate::snapshot::SnapshotReadContract::scalar(
                forge_foundational::facade::AspectKey::new("profile")
                    .expect("valid structural snapshot aspect key"),
                forge_foundational::facade::ScalarAspectType::String,
            ),
        ),
        value,
    )
}

fn remap_declaration(
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
            crate::truth_identity_fixtures::truth_branch_fixture("analysis"),
            crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
        )),
    )
}

fn branch_declaration(
    declaration_identity: StructuralIdentityDeclarationIdentity,
) -> StructuralIdentityDeclaration {
    StructuralIdentityDeclaration::branch_comparison(
        declaration_identity,
        StructuralSchemaIdentity::new("schema:geometry"),
        StructuralFingerprintEquivalenceContract::new(
            StructuralSchemaIdentity::new("schema:geometry"),
            StructuralFingerprintFamily::BranchComparisonFingerprint,
            "geometry-branch-v1",
            StructuralFingerprintNormalizationRule::SchemaDeclaredCanonicalForm,
            StructuralFingerprintOrderingRule::SchemaDeclaredCanonicalOrder,
            StructuralFingerprintOmissionPolicy::SchemaDeclaredOmissionPolicy,
        ),
        StructuralTruthViewBasis::explicit_branch_pair(
            BridgeTruthViewSelector::branch_snapshot(
                crate::truth_identity_fixtures::truth_branch_fixture("left"),
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
            ),
            BridgeTruthViewSelector::branch_snapshot(
                crate::truth_identity_fixtures::truth_branch_fixture("right"),
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-b"),
            ),
        ),
    )
}

fn branch_head_declaration(
    declaration_identity: StructuralIdentityDeclarationIdentity,
) -> StructuralIdentityDeclaration {
    StructuralIdentityDeclaration::branch_comparison(
        declaration_identity,
        StructuralSchemaIdentity::new("schema:geometry"),
        StructuralFingerprintEquivalenceContract::new(
            StructuralSchemaIdentity::new("schema:geometry"),
            StructuralFingerprintFamily::BranchComparisonFingerprint,
            "geometry-branch-head-v1",
            StructuralFingerprintNormalizationRule::SchemaDeclaredCanonicalForm,
            StructuralFingerprintOrderingRule::SchemaDeclaredCanonicalOrder,
            StructuralFingerprintOmissionPolicy::SchemaDeclaredOmissionPolicy,
        ),
        StructuralTruthViewBasis::explicit_branch_pair(
            BridgeTruthViewSelector::branch_head(
                crate::truth_identity_fixtures::truth_branch_fixture("left"),
            ),
            BridgeTruthViewSelector::branch_head(
                crate::truth_identity_fixtures::truth_branch_fixture("right"),
            ),
        ),
    )
}

pub(super) fn exact_target() -> BridgeHarnessTargetId {
    BridgeHarnessTargetId::structural_remap_exact(StructuralIdentityDeclarationIdentity::new(
        "structural:analysis-remap",
    ))
}

pub(super) fn ambiguous_target() -> BridgeHarnessTargetId {
    BridgeHarnessTargetId::structural_remap_ambiguous(StructuralIdentityDeclarationIdentity::new(
        "structural:analysis-remap",
    ))
}

pub(super) fn no_safe_match_target() -> BridgeHarnessTargetId {
    BridgeHarnessTargetId::structural_remap_no_safe_match(
        StructuralIdentityDeclarationIdentity::new("structural:analysis-remap"),
    )
}

pub(super) fn lineage_divergence_target() -> BridgeHarnessTargetId {
    BridgeHarnessTargetId::structural_remap_lineage_divergence(
        StructuralIdentityDeclarationIdentity::new("structural:analysis-remap"),
    )
}

pub(super) fn identity_conflict_target() -> BridgeHarnessTargetId {
    BridgeHarnessTargetId::structural_remap_identity_conflict(
        StructuralIdentityDeclarationIdentity::new("structural:analysis-remap"),
    )
}

pub(super) fn remap_replay_target() -> BridgeHarnessTargetId {
    BridgeHarnessTargetId::structural_remap_replay(StructuralIdentityDeclarationIdentity::new(
        "structural:analysis-remap",
    ))
}

pub(super) fn branch_compare_target() -> BridgeHarnessTargetId {
    BridgeHarnessTargetId::structural_branch_compare(StructuralIdentityDeclarationIdentity::new(
        "structural:analysis-branch-compare",
    ))
}

pub(super) fn branch_replay_target() -> BridgeHarnessTargetId {
    BridgeHarnessTargetId::structural_branch_replay(StructuralIdentityDeclarationIdentity::new(
        "structural:analysis-branch-compare",
    ))
}

pub(super) fn branch_head_compare_target() -> BridgeHarnessTargetId {
    BridgeHarnessTargetId::structural_branch_compare(StructuralIdentityDeclarationIdentity::new(
        "structural:analysis-branch-head-compare",
    ))
}

pub(super) fn branch_head_replay_target() -> BridgeHarnessTargetId {
    BridgeHarnessTargetId::structural_branch_replay(StructuralIdentityDeclarationIdentity::new(
        "structural:analysis-branch-head-compare",
    ))
}

pub(super) fn direct_profile(name: &str) -> ExecutionProfile {
    ExecutionProfile::development(name)
}

pub(super) fn forensic_profile(name: &str) -> ExecutionProfile {
    ExecutionProfile::forensic(name)
}

pub(super) fn execute_structural_run(
    profile: ExecutionProfile,
    request_name: &str,
    target: BridgeHarnessTargetId,
) -> RunRecord<BridgeHarnessTargetId> {
    let adapter = BridgeHarnessAdapter;
    let fixture = structural_fixture("bridge-structural-matrix");
    let mut runtime = adapter
        .create_runtime()
        .expect("structural harness runtime");
    adapter
        .prepare_runtime(&mut runtime, &profile)
        .expect("structural harness prepare");
    adapter
        .load_fixture(&mut runtime, &fixture)
        .expect("structural harness load fixture");
    adapter
        .execute(
            &mut runtime,
            &fixture,
            &ExecutionRequest::target(request_name, target),
            &profile,
        )
        .expect("structural harness execution")
}
