use super::super::support::*;
use crate::authoring::{AspectFieldSelector, AuthoredResultShapeField};
use crate::ordinary::read::{
    current, declare, WorthQueryReadNextAction, WorthQueryReadOutcome,
    WorthQueryReadRelationshipProof, WorthQueryReadRelationshipProofs,
};
use crate::policy_basis::{BranchAccessGrant, PolicyEpoch, PolicyRuleSnapshot};
use crate::tenant_basis::{SchemaVariantSnapshot, TenantBasisEpoch, TenantBindingSnapshot};

#[test]
fn ordinary_read_matches_internal_phase_chain_result_and_receipt_identity() {
    let declaration = declare(local_identity_read).expect("ordinary declaration should build");
    let declaration_identity = declaration.identity().as_str().to_string();
    let mut ordinary_workspace = read_runtime()
        .workspace("ordinary-read-parity")
        .expect("ordinary workspace should open");
    let ordinary = declaration
        .using(current())
        .run(&mut ordinary_workspace)
        .into_result()
        .expect("ordinary read should execute")
        .into_result();

    let mut oracle_workspace = read_runtime()
        .workspace("internal-read-parity")
        .expect("oracle workspace should open");
    let oracle = oracle_workspace
        .compose_read(local_identity_read)
        .expect("internal phase-chain oracle should execute");

    assert_eq!(declaration_identity, ordinary.receipt().read_graph_digest());
    assert_eq!(ordinary.rows(), oracle.rows());
    assert_eq!(
        ordinary.receipt().query_digest(),
        oracle.receipt().query_digest()
    );
    assert_eq!(
        ordinary.receipt().read_graph_digest(),
        oracle.receipt().read_graph_digest()
    );
    assert_eq!(ordinary.receipt().breadth(), oracle.receipt().breadth());
}

#[test]
fn ordinary_read_exposes_success_without_phase_artifacts() {
    let declaration = declare(local_identity_read).expect("ordinary declaration should build");
    let mut workspace = read_runtime()
        .workspace("ordinary-read-outcome")
        .expect("ordinary workspace should open");

    match declaration.using(current()).run(&mut workspace) {
        WorthQueryReadOutcome::Completed(completion) => {
            assert!(!completion.result().receipt().query_digest().is_empty());
            assert_eq!(
                completion
                    .context_receipt()
                    .counters()
                    .graph_authority_admitted_count(),
                1
            );
        }
        WorthQueryReadOutcome::Stopped(stop) => {
            panic!("ordinary read unexpectedly stopped: {:?}", stop.source())
        }
    }
}

#[test]
fn ordinary_read_lowers_policy_tenant_and_relationship_authority_once() {
    let declaration = declare(local_manager_relationship_read)
        .expect("relationship declaration should canonicalize");
    let policy_tenant = admitted_policy_tenant_inputs(1, true);
    let relationships = WorthQueryReadRelationshipProofs::bounded(
        [WorthQueryReadRelationshipProof::direct_edge(
            manager_relation_name(),
        )],
        1,
        1,
    )
    .expect("bounded relationship proof should declare");
    let context = current()
        .under_policy_tenant(
            policy_tenant.policy,
            policy_tenant.tenant,
            policy_tenant.branch,
            policy_tenant.schema,
        )
        .with_relationship_proofs(relationships);
    let mut workspace = read_runtime()
        .workspace("ordinary-read-policy-tenant-relationship")
        .expect("ordinary workspace should open");

    let completion = declaration
        .using(context)
        .run(&mut workspace)
        .into_result()
        .expect("fully admitted read should execute");
    let counters = completion.context_receipt().counters();

    assert_eq!(counters.canonical_query_identity_read_count(), 1);
    assert_eq!(counters.policy_tenant_admission_attempt_count(), 1);
    assert_eq!(counters.policy_tenant_admitted_count(), 1);
    assert_eq!(counters.relationship_proof_admission_attempt_count(), 1);
    assert_eq!(counters.relationship_proof_admitted_count(), 1);
    assert_eq!(counters.graph_authority_admission_attempt_count(), 1);
    assert_eq!(counters.graph_authority_admitted_count(), 1);
    assert!(completion
        .context_receipt()
        .policy_tenant_admission_digest()
        .is_some());
    assert!(completion
        .context_receipt()
        .relationship_proof_admission_digest()
        .is_some());
}

#[test]
fn equivalent_relationship_declaration_order_converges_on_context_identity() {
    let first_receipt = run_relationship_context([
        WorthQueryReadRelationshipProof::direct_edge(manager_relation_name()),
        WorthQueryReadRelationshipProof::tenant_membership(),
    ]);
    let second_receipt = run_relationship_context([
        WorthQueryReadRelationshipProof::tenant_membership(),
        WorthQueryReadRelationshipProof::direct_edge(manager_relation_name()),
    ]);

    assert_eq!(first_receipt.digest(), second_receipt.digest());
    assert_eq!(
        first_receipt.relationship_proof_admission_digest(),
        second_receipt.relationship_proof_admission_digest()
    );
}

#[test]
fn denied_policy_stops_before_graph_authority_or_runtime_execution() {
    let declaration = declare(local_identity_read).expect("read declaration should canonicalize");
    let policy_tenant = admitted_policy_tenant_inputs(1, false);
    let context = current().under_policy_tenant(
        policy_tenant.policy,
        policy_tenant.tenant,
        policy_tenant.branch,
        policy_tenant.schema,
    );
    let mut workspace = read_runtime()
        .workspace("ordinary-read-policy-denied")
        .expect("ordinary workspace should open");

    let stop = declaration
        .using(context)
        .run(&mut workspace)
        .into_result()
        .expect_err("denied policy must stop the read");
    let counters = stop
        .context_denial()
        .expect("policy denial must remain a context denial")
        .counters();

    assert_eq!(
        stop.next_action(),
        WorthQueryReadNextAction::SupplyPolicyAuthority
    );
    assert_eq!(counters.canonical_query_identity_read_count(), 1);
    assert_eq!(counters.policy_tenant_admission_attempt_count(), 1);
    assert_eq!(counters.policy_tenant_admitted_count(), 0);
    assert_eq!(counters.relationship_proof_admission_attempt_count(), 0);
    assert_eq!(counters.relationship_proof_admitted_count(), 0);
    assert_eq!(counters.graph_authority_admission_attempt_count(), 0);
    assert_eq!(counters.graph_authority_admitted_count(), 0);
    assert!(stop.context_receipt().is_none());
}

#[test]
fn narrowing_policy_stops_until_narrowing_context_is_declared() {
    let declaration = declare(local_identity_read).expect("read declaration should canonicalize");
    let policy = PolicyRuleSnapshot::synthetic_authority_with_posture(
        "ordinary-narrowing-policy",
        "ordinary-narrowing-rules",
        PolicyEpoch::Synthetic(1),
        true,
        true,
        false,
    );
    let context = current().under_policy_tenant(
        policy.clone(),
        TenantBindingSnapshot::synthetic_direct(
            "tenant-a",
            "main",
            "schema-a",
            TenantBasisEpoch::Synthetic(7),
        ),
        BranchAccessGrant::synthetic_granted("main", &policy),
        SchemaVariantSnapshot::synthetic_authority("tenant-a", "schema-a", "exact"),
    );
    let mut workspace = read_runtime()
        .workspace("ordinary-read-policy-narrowing-context-required")
        .expect("ordinary workspace should open");

    let stop = declaration
        .using(context)
        .run(&mut workspace)
        .into_result()
        .expect_err("narrowing policy must not execute without narrowing context");
    let counters = stop
        .context_denial()
        .expect("missing narrowing context must remain a context denial")
        .counters();

    assert_eq!(
        stop.next_action(),
        WorthQueryReadNextAction::SupplyPolicyNarrowingContext
    );
    assert_eq!(counters.policy_tenant_admission_attempt_count(), 1);
    assert_eq!(counters.policy_tenant_admitted_count(), 1);
    assert_eq!(counters.graph_authority_admission_attempt_count(), 0);
}

#[test]
fn relationship_read_without_relationship_authority_stops_before_other_admission() {
    let declaration = declare(local_manager_relationship_read)
        .expect("relationship read declaration should canonicalize");
    let mut workspace = read_runtime()
        .workspace("ordinary-read-missing-relationship-authority")
        .expect("ordinary workspace should open");

    let stop = declaration
        .using(current())
        .run(&mut workspace)
        .into_result()
        .expect_err("relationship read must require relationship authority");
    let counters = stop
        .context_denial()
        .expect("missing relationship authority must be a context denial")
        .counters();

    assert_eq!(
        stop.next_action(),
        WorthQueryReadNextAction::SupplyRelationshipProofAuthority
    );
    assert_eq!(counters.canonical_query_identity_read_count(), 1);
    assert_eq!(counters.policy_tenant_admission_attempt_count(), 0);
    assert_eq!(counters.relationship_proof_admission_attempt_count(), 0);
    assert_eq!(counters.graph_authority_admission_attempt_count(), 0);
}

#[test]
fn changed_policy_epoch_invalidates_only_context_identity() {
    let first = run_policy_context(1);
    let second = run_policy_context(2);

    assert_ne!(first.digest(), second.digest());
    assert_eq!(
        first.canonical_query_digest(),
        second.canonical_query_digest()
    );
}

fn local_identity_read(
    read: crate::runtime::WorthQueryReadBuilder,
) -> Result<crate::runtime::WorthQueryReadGraph, crate::runtime::WorthQueryReadDenial> {
    read.local_detail(
        "user",
        manager_schema(),
        |query| {
            query.project(
                AspectFieldSelector::new("identity", "id")
                    .expect("identity projection should build"),
            )
        },
        |shape| {
            shape.field(
                AuthoredResultShapeField::new("identity", "id", "id")
                    .expect("identity result field should build"),
            )
        },
    )
}

fn local_manager_relationship_read(
    read: crate::runtime::WorthQueryReadBuilder,
) -> Result<crate::runtime::WorthQueryReadGraph, crate::runtime::WorthQueryReadDenial> {
    read.local_direct_edge_detail(
        "user",
        manager_schema(),
        manager_relation_name(),
        |query| {
            query.project(
                AspectFieldSelector::new("identity", "id")
                    .expect("identity projection should build"),
            )
        },
        |shape| {
            shape.field(
                AuthoredResultShapeField::new("identity", "id", "id")
                    .expect("identity result field should build"),
            )
        },
    )
}

struct PolicyTenantInputs {
    policy: PolicyRuleSnapshot,
    tenant: TenantBindingSnapshot,
    branch: BranchAccessGrant,
    schema: SchemaVariantSnapshot,
}

fn admitted_policy_tenant_inputs(epoch: u64, admits_query: bool) -> PolicyTenantInputs {
    let policy = PolicyRuleSnapshot::synthetic_authority_with_posture(
        "ordinary-policy",
        "ordinary-rules",
        PolicyEpoch::Synthetic(epoch),
        admits_query,
        false,
        false,
    );
    let tenant = TenantBindingSnapshot::synthetic_direct(
        "tenant-a",
        "main",
        "schema-a",
        TenantBasisEpoch::Synthetic(7),
    );
    let branch = BranchAccessGrant::synthetic_granted("main", &policy);
    let schema = SchemaVariantSnapshot::synthetic_authority("tenant-a", "schema-a", "exact");
    PolicyTenantInputs {
        policy,
        tenant,
        branch,
        schema,
    }
}

fn run_policy_context(epoch: u64) -> crate::ordinary::read::WorthQueryReadContextReceipt {
    let declaration = declare(local_identity_read).expect("read declaration should canonicalize");
    let policy_tenant = admitted_policy_tenant_inputs(epoch, true);
    let context = current().under_policy_tenant(
        policy_tenant.policy,
        policy_tenant.tenant,
        policy_tenant.branch,
        policy_tenant.schema,
    );
    let mut workspace = read_runtime()
        .workspace(format!("ordinary-read-policy-epoch-{epoch}"))
        .expect("ordinary workspace should open");
    declaration
        .using(context)
        .run(&mut workspace)
        .into_result()
        .expect("policy context should execute")
        .context_receipt()
        .clone()
}

fn run_relationship_context<const PROOF_COUNT: usize>(
    proofs: [WorthQueryReadRelationshipProof; PROOF_COUNT],
) -> crate::ordinary::read::WorthQueryReadContextReceipt {
    let declaration = declare(local_manager_relationship_read)
        .expect("relationship read declaration should canonicalize");
    let policy_tenant = admitted_policy_tenant_inputs(1, true);
    let relationships = WorthQueryReadRelationshipProofs::bounded(proofs, 2, 2)
        .expect("relationship proof set should be bounded");
    let context = current()
        .under_policy_tenant(
            policy_tenant.policy,
            policy_tenant.tenant,
            policy_tenant.branch,
            policy_tenant.schema,
        )
        .with_relationship_proofs(relationships);
    let mut workspace = read_runtime()
        .workspace("ordinary-read-relationship-order")
        .expect("ordinary workspace should open");
    declaration
        .using(context)
        .run(&mut workspace)
        .into_result()
        .expect("relationship context should execute")
        .context_receipt()
        .clone()
}
