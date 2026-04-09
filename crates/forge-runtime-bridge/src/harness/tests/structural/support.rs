use forge_harness::facade::ScenarioPlan;
use forge_harness::facade::{ExecutionProfile, ExecutionRequest, HarnessAdapter, RunRecord};

use crate::facade::{
    BridgeTruthViewSelector, SnapshotReadRecord, StructuralFingerprintEquivalenceContract,
    StructuralFingerprintFamily, StructuralFingerprintNormalizationRule,
    StructuralFingerprintOmissionPolicy, StructuralFingerprintOrderingRule,
    StructuralIdentityDeclaration, StructuralIdentityDeclarationIdentity, StructuralSchemaIdentity,
    StructuralTruthViewBasis, TruthBranchIdentity, TruthSnapshotIdentity,
};
use crate::harness::adapter::BridgeHarnessAdapter;
use crate::harness::fixtures::{BridgeHarnessFixture, SnapshotFixture};

use super::super::support::{committed_patch_on_branch, registration};

pub(super) fn structural_fixture(
    name: &str,
) -> forge_harness::facade::ScenarioFixture<BridgeHarnessFixture> {
    ScenarioPlan::new(
        name,
        BridgeHarnessFixture::new(vec![registration()])
            .with_policy(crate::facade::BridgeRuntimePolicy::development())
            .with_structural_declaration(remap_declaration("structural:analysis-remap"))
            .with_structural_declaration(branch_declaration("structural:analysis-branch-compare"))
            .with_structural_declaration(branch_head_declaration(
                "structural:analysis-branch-head-compare",
            ))
            .with_committed_patch(committed_patch_on_branch(
                "analysis",
                "commit-a",
                "patch-a",
                "snapshot-a",
                "name",
            ))
            .with_committed_patch(committed_patch_on_branch(
                "analysis",
                "commit-b",
                "patch-b",
                "snapshot-b",
                "name",
            ))
            .with_committed_patch(committed_patch_on_branch(
                "left",
                "commit-left-a",
                "patch-left-a",
                "snapshot-a",
                "name",
            ))
            .with_committed_patch(committed_patch_on_branch(
                "right",
                "commit-right-b",
                "patch-right-b",
                "snapshot-b",
                "name",
            ))
            .with_snapshot(structural_snapshot("snapshot-a", "alice"))
            .with_snapshot(structural_snapshot("snapshot-b", "bob")),
    )
    .declare_input("structural")
    .declare_observation("structural")
    .compile()
}

fn structural_snapshot(snapshot_identity: &str, value: &str) -> SnapshotFixture {
    SnapshotFixture::new(
        TruthSnapshotIdentity::new(snapshot_identity),
        vec![
            SnapshotReadRecord::new("entity-1:profile", value.as_bytes().to_vec()),
            SnapshotReadRecord::new("entity-2:profile", value.as_bytes().to_vec()),
            SnapshotReadRecord::new(
                "entity-3:profile",
                format!("shape-mismatch-{snapshot_identity}").into_bytes(),
            ),
        ],
    )
}

fn remap_declaration(declaration_id: &str) -> StructuralIdentityDeclaration {
    StructuralIdentityDeclaration::advisory_remap(
        StructuralIdentityDeclarationIdentity::new(declaration_id),
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

fn branch_declaration(declaration_id: &str) -> StructuralIdentityDeclaration {
    StructuralIdentityDeclaration::branch_comparison(
        StructuralIdentityDeclarationIdentity::new(declaration_id),
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
                TruthBranchIdentity::new("left"),
                TruthSnapshotIdentity::new("snapshot-a"),
            ),
            BridgeTruthViewSelector::branch_snapshot(
                TruthBranchIdentity::new("right"),
                TruthSnapshotIdentity::new("snapshot-b"),
            ),
        ),
    )
}

fn branch_head_declaration(declaration_id: &str) -> StructuralIdentityDeclaration {
    StructuralIdentityDeclaration::branch_comparison(
        StructuralIdentityDeclarationIdentity::new(declaration_id),
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
            BridgeTruthViewSelector::branch_head(TruthBranchIdentity::new("left")),
            BridgeTruthViewSelector::branch_head(TruthBranchIdentity::new("right")),
        ),
    )
}

pub(super) fn exact_target() -> String {
    "structural-remap-exact:structural:analysis-remap".to_string()
}

pub(super) fn ambiguous_target() -> String {
    "structural-remap-ambiguous:structural:analysis-remap".to_string()
}

pub(super) fn no_safe_match_target() -> String {
    "structural-remap-no-safe-match:structural:analysis-remap".to_string()
}

pub(super) fn lineage_divergence_target() -> String {
    "structural-remap-lineage-divergence:structural:analysis-remap".to_string()
}

pub(super) fn identity_conflict_target() -> String {
    "structural-remap-identity-conflict:structural:analysis-remap".to_string()
}

pub(super) fn remap_replay_target() -> String {
    "structural-remap-replay:structural:analysis-remap".to_string()
}

pub(super) fn branch_compare_target() -> String {
    "structural-branch-compare:structural:analysis-branch-compare".to_string()
}

pub(super) fn branch_replay_target() -> String {
    "structural-branch-replay:structural:analysis-branch-compare".to_string()
}

pub(super) fn branch_head_compare_target() -> String {
    "structural-branch-compare:structural:analysis-branch-head-compare".to_string()
}

pub(super) fn branch_head_replay_target() -> String {
    "structural-branch-replay:structural:analysis-branch-head-compare".to_string()
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
    target: String,
) -> RunRecord<String> {
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
