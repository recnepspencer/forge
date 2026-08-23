use std::sync::Arc;

use super::{RelationalBranchRootCaptureDenial, RelationalBranchVisibilityCommitment};
use crate::history::data::{BranchId, CommitId};
use crate::publication::patch::data::PatchStreamPosition;
use crate::schema::data::SchemaId;
use crate::tests::support::{create_entity_outcome, persisted_runtime_with_test_schema};

fn committed_root_and_envelope() -> (
    crate::runtime::RelationalRuntime,
    Arc<super::RelationalBranchRoot>,
    crate::history::data::CanonicalCommitEnvelope,
) {
    let mut runtime = persisted_runtime_with_test_schema();
    let performed = create_entity_outcome(&mut runtime, "branch-root-axis-binding");
    let root = runtime
        .history
        .branch_cell(&BranchId("main".to_owned()))
        .and_then(|cell| cell.root())
        .cloned()
        .expect("performed commit installs one complete branch root");
    let envelope = runtime
        .replay()
        .canonical_commit_envelope(performed.commit.commit_id)
        .expect("performed commit owns one canonical envelope")
        .clone();
    (runtime, root, envelope)
}

#[test]
fn publication_cost_distinguishes_new_and_reused_schema_authority() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "schema-authority-allocation-owner");
    let first = runtime
        .history
        .branch_cell(&BranchId("main".to_owned()))
        .and_then(|cell| cell.root())
        .cloned()
        .expect("first commit installs a root");
    create_entity_outcome(&mut runtime, "schema-authority-reuse");
    let second = runtime
        .history
        .branch_cell(&BranchId("main".to_owned()))
        .and_then(|cell| cell.root())
        .cloned()
        .expect("second commit installs a root");

    assert_eq!(first.publication_cost().new_schema_authorities, 1);
    assert_eq!(first.publication_cost().reused_schema_authorities, 0);
    assert_eq!(second.publication_cost().new_schema_authorities, 0);
    assert_eq!(second.publication_cost().reused_schema_authorities, 1);
    assert_eq!(
        first.schema_authority().allocation_id(),
        second.schema_authority().allocation_id()
    );
}

#[test]
fn relink_rejects_schema_authority_mutation() {
    let (runtime, root, mut envelope) = committed_root_and_envelope();
    envelope.schema_authority.primary_schema_id = Some(SchemaId("mutated-schema".to_owned()));

    let denial = root
        .relink_canonical_envelope(Arc::new(envelope), &runtime.services.symbols)
        .expect_err("canonical schema authority mutation cannot be readmitted");

    assert!(matches!(
        denial,
        RelationalBranchRootCaptureDenial::SchemaRootMismatch { .. }
    ));
}

#[test]
fn relink_rejects_visibility_tuple_mutation() {
    let (runtime, root, mut envelope) = committed_root_and_envelope();
    envelope.patch.position = PatchStreamPosition(envelope.patch.position.0 + 1);

    let denial = root
        .relink_canonical_envelope(Arc::new(envelope), &runtime.services.symbols)
        .expect_err("visibility tuple mutation cannot replace the committed envelope");

    assert!(matches!(
        denial,
        RelationalBranchRootCaptureDenial::VisibilityCommitmentMismatch { .. }
    ));
}

#[test]
fn completeness_recomputes_visibility_commitment() {
    let (runtime, root, mut altered_envelope) = committed_root_and_envelope();
    assert!(root.is_complete(&runtime.services.symbols));
    altered_envelope.patch.position = PatchStreamPosition(altered_envelope.patch.position.0 + 1);
    let mut corrupted = root.as_ref().clone();
    let committed = corrupted
        .committed
        .as_mut()
        .expect("performed root owns committed axes");
    committed.axes.visibility = RelationalBranchVisibilityCommitment::for_root(
        &altered_envelope,
        committed.axes.storage_root,
        committed.axes.schema_root,
        committed.axes.correctness_index,
    );

    assert!(
        !corrupted.is_complete(&runtime.services.symbols),
        "completeness must reject a commitment to any different visible tuple"
    );
}

#[test]
fn visibility_commitment_binds_every_visible_tuple_axis() {
    let (_, _, envelope) = committed_root_and_envelope();
    let storage_root = [17; 32];
    let schema_root = [29; 32];
    let baseline = RelationalBranchVisibilityCommitment::for_root(
        &envelope,
        storage_root,
        schema_root,
        super::RelationalRootCorrectnessIndex::AuthoritativeFallback,
    )
    .digest();
    assert_eq!(
        baseline,
        independent_visibility_digest(&envelope, storage_root, schema_root, 0),
        "the production commitment must match an independent complete-tuple oracle"
    );

    let mut mutants = Vec::new();
    mutants.push(independent_visibility_digest(
        &envelope,
        [18; 32],
        schema_root,
        0,
    ));
    mutants.push(independent_visibility_digest(
        &envelope,
        storage_root,
        [30; 32],
        0,
    ));
    mutants.push(independent_visibility_digest(
        &envelope,
        storage_root,
        schema_root,
        1,
    ));
    let mut commit = envelope.clone();
    commit.commit.commit_id = CommitId(commit.commit.commit_id.0 + 1);
    mutants.push(production_visibility(&commit, storage_root, schema_root));
    let mut version = envelope.clone();
    version.commit.version_id.0 += 1;
    mutants.push(production_visibility(&version, storage_root, schema_root));
    let mut parents = envelope.clone();
    parents.commit.parents = vec![CommitId(41), CommitId(43)];
    mutants.push(production_visibility(&parents, storage_root, schema_root));
    parents.commit.parents.reverse();
    mutants.push(production_visibility(&parents, storage_root, schema_root));
    let mut branch = envelope.clone();
    branch.branch_context = BranchId("visibility-mutant".to_owned());
    mutants.push(production_visibility(&branch, storage_root, schema_root));
    let mut patch = envelope.clone();
    patch.patch.position = PatchStreamPosition(patch.patch.position.0 + 1);
    mutants.push(production_visibility(&patch, storage_root, schema_root));

    assert!(
        mutants.into_iter().all(|mutant| mutant != baseline),
        "every truth, schema, correctness, commit, ancestry, branch, and patch axis must turn the commitment red"
    );
}

fn production_visibility(
    envelope: &crate::history::data::CanonicalCommitEnvelope,
    storage_root: [u8; 32],
    schema_root: [u8; 32],
) -> [u8; 32] {
    RelationalBranchVisibilityCommitment::for_root(
        envelope,
        storage_root,
        schema_root,
        super::RelationalRootCorrectnessIndex::AuthoritativeFallback,
    )
    .digest()
}

fn independent_visibility_digest(
    envelope: &crate::history::data::CanonicalCommitEnvelope,
    storage_root: [u8; 32],
    schema_root: [u8; 32],
    correctness_tag: u8,
) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    let mut digest = Sha256::new();
    digest.update(b"worth.relational.branch-visibility.v1\0");
    digest.update(storage_root);
    digest.update(schema_root);
    digest.update([correctness_tag]);
    digest.update(envelope.commit.commit_id.0.to_be_bytes());
    digest.update(envelope.commit.version_id.0.to_be_bytes());
    digest.update((envelope.commit.parents.len() as u64).to_be_bytes());
    for parent in &envelope.commit.parents {
        digest.update(parent.0.to_be_bytes());
    }
    digest.update((envelope.branch_context.0.len() as u64).to_be_bytes());
    digest.update(envelope.branch_context.0.as_bytes());
    digest.update(envelope.patch.position.0.to_be_bytes());
    digest.finalize().into()
}
