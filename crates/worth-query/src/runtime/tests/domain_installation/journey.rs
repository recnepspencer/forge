use super::*;
use crate::authoring::{
    AspectFieldSelector, AuthoredResultShapeField, DetailQueryBuilder, DetailResultShapeBuilder,
};
use crate::domain_installation::{
    WorthQueryInstalledDomainCapabilityKind, WorthQueryInstalledDomainExecutionDriftKind,
    WorthQueryInstalledDomainLiveCheckpointOutcome, WorthQueryInstalledDomainLiveCloseOutcome,
    WorthQueryInstalledDomainLiveOpenOutcome, WorthQueryInstalledDomainLiveResumeOutcome,
};
use crate::ordinary::read::WorthQueryReadNextAction;
use crate::ordinary::read::{current, project_facts};
use crate::policy_basis::{BranchAccessGrant, PolicyEpoch, PolicyRuleSnapshot};
use crate::runtime::{WorthQueryAspectTouch, WorthQueryAuthoredAspectValue};
use crate::session_label::WorthQuerySessionLabel;
use crate::tenant_basis::{SchemaVariantSnapshot, TenantBasisEpoch, TenantBindingSnapshot};

#[test]
fn installed_read_projection_and_receipts_carry_one_authority_witness() {
    let runtime = installed_runtime();
    let handle = runtime.domain(InstalledDomain).unwrap();
    let expected = handle.authority_witness().witness_identity().clone();
    let mut workspace = runtime.workspace("installed-domain-read").unwrap();

    let completion = handle
        .read(identity_read)
        .unwrap()
        .using(current())
        .run(&mut workspace)
        .unwrap()
        .into_result()
        .unwrap();
    assert_eq!(
        completion.receipt().capability(),
        WorthQueryInstalledDomainCapabilityKind::Read
    );
    assert_eq!(
        completion
            .receipt()
            .installed_authority()
            .witness_identity(),
        &expected
    );

    let projection = completion.project(project_facts().entity_identities());
    assert_eq!(
        projection.receipt().capability(),
        WorthQueryInstalledDomainCapabilityKind::Projection
    );
    assert_eq!(
        projection
            .receipt()
            .installed_authority()
            .witness_identity(),
        &expected
    );
}

#[test]
fn foreign_workspace_denies_before_the_ordinary_read_journey_begins() {
    let owner = installed_runtime();
    let handle = owner.domain(InstalledDomain).unwrap();
    let foreign = installed_runtime();
    let mut workspace = foreign.workspace("foreign-installed-domain").unwrap();

    let stop = handle
        .read(identity_read)
        .unwrap()
        .using(current())
        .run(&mut workspace)
        .unwrap_err();
    assert_eq!(
        stop.stop().kind(),
        WorthQueryInstalledDomainExecutionDriftKind::ForeignRuntime
    );
    assert_eq!(
        stop.installed_authority().witness_identity(),
        handle.authority_witness().witness_identity()
    );
}

#[test]
fn installed_live_checkpoint_resume_and_close_preserve_the_package_witness() {
    let runtime = installed_runtime();
    let handle = runtime.domain(InstalledDomain).unwrap();
    let expected = handle.authority_witness().witness_identity().clone();
    let mut workspace = runtime.workspace("installed-domain-live").unwrap();
    let live = match handle
        .live("installed.identity", identity_read)
        .unwrap()
        .using(current())
        .open(&mut workspace)
        .unwrap()
    {
        WorthQueryInstalledDomainLiveOpenOutcome::Opened(handle) => handle,
        WorthQueryInstalledDomainLiveOpenOutcome::Stopped(stop) => {
            panic!("installed live open stopped: {:?}", stop.stop().source())
        }
    };
    assert_eq!(
        live.installation_receipt()
            .installed_authority()
            .witness_identity(),
        &expected
    );
    let continuation = match live.checkpoint(&mut workspace) {
        WorthQueryInstalledDomainLiveCheckpointOutcome::Checkpointed(continuation) => continuation,
        _ => panic!("installed live checkpoint must succeed"),
    };
    assert_eq!(
        continuation
            .checkpoint_receipt()
            .installed_authority()
            .witness_identity(),
        &expected
    );
    let resumed = match continuation.resume(&mut workspace) {
        WorthQueryInstalledDomainLiveResumeOutcome::Resumed(completion) => completion,
        _ => panic!("installed live resume must succeed"),
    };
    assert_eq!(
        resumed
            .execution_receipt()
            .installed_authority()
            .witness_identity(),
        &expected
    );
    let closed = match resumed.into_handle().close(&mut workspace) {
        WorthQueryInstalledDomainLiveCloseOutcome::Closed(receipt) => receipt,
        _ => panic!("installed live close must succeed"),
    };
    assert_eq!(
        closed
            .execution_receipt()
            .installed_authority()
            .witness_identity(),
        &expected
    );
}

#[test]
fn installed_read_preserves_authority_across_an_ordinary_cross_basis_stop() {
    let runtime = installed_runtime();
    let handle = runtime.domain(InstalledDomain).unwrap();
    let expected = handle.authority_witness().witness_identity().clone();
    let mut workspace = runtime.workspace("installed-domain-cross-basis").unwrap();
    let policy = PolicyRuleSnapshot::synthetic_authority_with_query_admission(
        "installed-domain-policy",
        "installed-domain-rules",
        PolicyEpoch::Synthetic(1),
        true,
    );
    let branch = BranchAccessGrant::synthetic_granted("main", &policy);
    let context = current().under_policy_tenant(
        policy,
        TenantBindingSnapshot::synthetic_direct(
            "tenant-a",
            "other-branch",
            "schema-a",
            TenantBasisEpoch::Synthetic(7),
        ),
        branch,
        SchemaVariantSnapshot::synthetic_authority("tenant-a", "schema-a", "exact"),
    );

    let stopped = handle
        .read(identity_read)
        .unwrap()
        .using(context)
        .run(&mut workspace)
        .unwrap()
        .into_result();
    let Err(stop) = stopped else {
        panic!("cross-basis context must stop before planning")
    };

    assert_eq!(stop.installed_authority().witness_identity(), &expected);
    assert_eq!(
        stop.capability(),
        WorthQueryInstalledDomainCapabilityKind::Read
    );
    assert_eq!(
        stop.stop().next_action(),
        WorthQueryReadNextAction::SupplyFreshBasis
    );
    assert_eq!(stop.stop().journey_counters().planning_attempt_count(), 0);
    assert_eq!(
        stop.stop()
            .journey_counters()
            .lower_runtime_execution_attempt_count(),
        0
    );
}

#[test]
fn foreign_runtime_live_resume_retains_the_continuation_for_its_owner() {
    let owner = installed_runtime();
    let handle = owner.domain(InstalledDomain).unwrap();
    let mut owner_workspace = owner.workspace("installed-domain-live-owner").unwrap();
    let foreign = installed_runtime();
    let mut foreign_workspace = foreign.workspace("installed-domain-live-foreign").unwrap();
    let live = match handle
        .live("installed.foreign-resume", identity_read)
        .unwrap()
        .using(current())
        .open(&mut owner_workspace)
        .unwrap()
    {
        WorthQueryInstalledDomainLiveOpenOutcome::Opened(handle) => handle,
        WorthQueryInstalledDomainLiveOpenOutcome::Stopped(stop) => {
            panic!("installed live open stopped: {:?}", stop.stop().source())
        }
    };
    let continuation = match live.checkpoint(&mut owner_workspace) {
        WorthQueryInstalledDomainLiveCheckpointOutcome::Checkpointed(continuation) => continuation,
        _ => panic!("installed live checkpoint must succeed"),
    };
    let continuation = match continuation.resume(&mut foreign_workspace) {
        WorthQueryInstalledDomainLiveResumeOutcome::AuthorityStopped(continuation, drift) => {
            assert_eq!(
                drift.kind(),
                WorthQueryInstalledDomainExecutionDriftKind::ForeignRuntime
            );
            continuation
        }
        _ => panic!("foreign runtime must not resume an installed continuation"),
    };
    let resumed = match continuation.resume(&mut owner_workspace) {
        WorthQueryInstalledDomainLiveResumeOutcome::Resumed(completion) => completion,
        _ => panic!("owning runtime must still resume the retained continuation"),
    };
    assert!(matches!(
        resumed.into_handle().close(&mut owner_workspace),
        WorthQueryInstalledDomainLiveCloseOutcome::Closed(_)
    ));
}

#[test]
fn installed_mutation_and_workflow_receipts_retain_the_package_witness() {
    let runtime = installed_runtime();
    let handle = runtime.domain(InstalledDomain).unwrap();
    let expected = handle.authority_witness().witness_identity().clone();
    let mut workspace = runtime.workspace("installed-domain-mutation").unwrap();
    let context = crate::ordinary::mutation::authoritative(&workspace).unwrap();
    let mutation = handle
        .mutation(|builder| {
            builder
                .set_aspect(
                    WorthQueryAspectTouch::from_authoring_ingress_text("identity.id")?,
                    WorthQueryAuthoredAspectValue::string("installed-task"),
                )
                .build_insert("Task")
        })
        .unwrap()
        .using(context)
        .run(&mut workspace)
        .unwrap();
    let crate::domain_installation::WorthQueryInstalledDomainMutationOutcome::Completed(mutation) =
        mutation
    else {
        panic!("installed mutation must complete")
    };
    assert_eq!(
        mutation.receipt().installed_authority().witness_identity(),
        &expected
    );

    let label = WorthQuerySessionLabel::scoped_strs("installed-domain", ["workflow"]).unwrap();
    let context = crate::ordinary::workflow::preview(&workspace, label.clone()).unwrap();
    let workflow = handle
        .mutation(|builder| {
            builder
                .set_aspect(
                    WorthQueryAspectTouch::from_authoring_ingress_text("identity.id")?,
                    WorthQueryAuthoredAspectValue::string("installed-preview-task"),
                )
                .build_insert("Task")
        })
        .unwrap()
        .workflow(label)
        .using(context)
        .run(&mut workspace)
        .unwrap();
    let crate::domain_installation::WorthQueryInstalledDomainWorkflowOutcome::Completed(workflow) =
        workflow
    else {
        panic!("installed workflow must complete")
    };
    assert_eq!(
        workflow.receipt().installed_authority().witness_identity(),
        &expected
    );
}

#[test]
fn installed_operational_and_rich_inspection_share_operational_evidence() {
    let runtime = installed_runtime();
    let handle = runtime.domain(InstalledDomain).unwrap();
    let expected = handle.authority_witness().witness_identity().clone();
    let mut workspace = runtime.workspace("installed-domain-inspection").unwrap();
    let operational_source = handle
        .read(identity_read)
        .unwrap()
        .using(current())
        .run(&mut workspace)
        .unwrap()
        .into_result()
        .unwrap();
    let rich_source = handle
        .read(identity_read)
        .unwrap()
        .using(current())
        .run(&mut workspace)
        .unwrap()
        .into_result()
        .unwrap();
    let basis = crate::basis_lifecycle::basis_lifecycle()
        .historical_snapshot("installed-domain-inspection", true)
        .inspect()
        .unwrap();
    let operational = operational_source
        .inspect()
        .using(crate::ordinary::inspection::inspection_basis(basis.clone()))
        .run(&workspace)
        .unwrap();
    let rich = rich_source
        .inspect()
        .with_rich_inspection()
        .using(crate::ordinary::inspection::inspection_basis(basis))
        .run(&workspace)
        .unwrap();
    assert_eq!(
        operational
            .receipt()
            .installed_authority()
            .witness_identity(),
        &expected
    );
    assert_eq!(
        operational.outcome().settled().unwrap().receipt(),
        rich.outcome().settled().unwrap().receipt()
    );
    assert!(operational
        .outcome()
        .settled()
        .unwrap()
        .materialization()
        .is_none());
    assert!(rich
        .outcome()
        .settled()
        .unwrap()
        .materialization()
        .is_some());
}

fn identity_read<Output>(
    read: WorthQueryReadBuilder<Output>,
) -> Result<Output, crate::runtime::WorthQueryReadDenial> {
    read.local_detail(
        "user",
        schema(),
        |query: DetailQueryBuilder| {
            query.project(AspectFieldSelector::new("identity", "id").unwrap())
        },
        |shape: DetailResultShapeBuilder| {
            shape.field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        },
    )
}
