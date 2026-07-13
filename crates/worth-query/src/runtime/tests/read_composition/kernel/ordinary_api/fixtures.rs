use super::super::super::support::*;
use crate::authoring::{AspectFieldSelector, AuthoredResultShapeField};
use crate::ordinary::read::{
    current, declare, WorthQueryReadRelationshipProof, WorthQueryReadRelationshipProofs,
};
use crate::policy_basis::{BranchAccessGrant, PolicyEpoch, PolicyRuleSnapshot};
use crate::tenant_basis::{SchemaVariantSnapshot, TenantBasisEpoch, TenantBindingSnapshot};

pub(super) fn local_identity_read(
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

pub(super) fn local_manager_relationship_read(
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

pub(super) struct PolicyTenantInputs {
    pub(super) policy: PolicyRuleSnapshot,
    pub(super) tenant: TenantBindingSnapshot,
    pub(super) branch: BranchAccessGrant,
    pub(super) schema: SchemaVariantSnapshot,
}

pub(super) fn admitted_policy_tenant_inputs(epoch: u64, admits_query: bool) -> PolicyTenantInputs {
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

pub(super) fn run_policy_context(
    epoch: u64,
) -> crate::ordinary::read::WorthQueryReadContextReceipt {
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

pub(super) fn run_relationship_context<const PROOF_COUNT: usize>(
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
