use super::*;
use crate::authoring::{
    AspectFieldSelector, AuthoredResultShapeField, DetailQueryBuilder, DetailResultShapeBuilder,
};
use crate::domain_installation::{
    WorthQueryInstalledDomainExecutionDrift, WorthQueryInstalledDomainExecutionDriftKind,
    WorthQueryInstalledDomainExecutionNextAction, WorthQueryInstalledDomainLiveCheckpointOutcome,
    WorthQueryInstalledDomainLiveCloseOutcome, WorthQueryInstalledDomainLiveOpenOutcome,
    WorthQueryInstalledDomainLiveResumeOutcome,
};
use crate::ordinary::read::{
    current, project_facts, WorthQueryProjectionOutcome, WorthQueryProjectionViolation,
};

#[test]
fn installed_live_checkpoint_resume_and_close_preserve_the_package_witness() {
    let runtime = installed_runtime();
    let handle = runtime.domain(InstalledDomain).unwrap();
    let expected = handle.authority_witness().witness_identity().clone();
    let mut workspace = runtime.workspace("installed-domain-live").unwrap();
    let live = open_live(&handle, &mut workspace, "installed.identity");
    assert_eq!(
        live.installation_receipt()
            .installed_authority()
            .witness_identity(),
        &expected
    );
    assert_live_projection(&live, &mut workspace, &expected);
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
    let resumed = resumed.into_handle();
    assert_live_projection(&resumed, &mut workspace, &expected);
    let closed = match resumed.close(&mut workspace) {
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
fn installed_live_projection_rejects_an_equivalent_foreign_installation() {
    let owner = installed_runtime();
    let owner_handle = owner.domain(InstalledDomain).unwrap();
    let owner_witness = owner_handle.authority_witness().witness_identity().clone();
    let mut owner_workspace = owner.workspace("installed-live-projection-owner").unwrap();
    let owner_live = open_live(&owner_handle, &mut owner_workspace, "installed.owner");

    let foreign = installed_runtime();
    let foreign_handle = foreign.domain(InstalledDomain).unwrap();
    let mut foreign_workspace = foreign
        .workspace("installed-live-projection-foreign")
        .unwrap();
    let foreign_live = open_live(&foreign_handle, &mut foreign_workspace, "installed.foreign");
    let foreign_read = match foreign_live.read(&mut foreign_workspace) {
        Ok(read) => read,
        Err(_) => panic!("foreign live read must succeed in its owning workspace"),
    };

    let projection = owner_live.project(&foreign_read, project_facts().entity_identities());
    assert!(matches!(
        projection.outcome(),
        WorthQueryProjectionOutcome::Violation(
            WorthQueryProjectionViolation::LiveInstallationMismatch { .. }
        )
    ));
    assert_eq!(
        projection
            .receipt()
            .installed_authority()
            .witness_identity(),
        &owner_witness
    );
    assert!(matches!(
        owner_live.close(&mut owner_workspace),
        WorthQueryInstalledDomainLiveCloseOutcome::Closed(_)
    ));
    assert!(matches!(
        foreign_live.close(&mut foreign_workspace),
        WorthQueryInstalledDomainLiveCloseOutcome::Closed(_)
    ));
}

#[test]
fn foreign_runtime_live_resume_retains_the_continuation_for_its_owner() {
    let owner = installed_runtime();
    let handle = owner.domain(InstalledDomain).unwrap();
    let mut owner_workspace = owner.workspace("installed-domain-live-owner").unwrap();
    let foreign = installed_runtime();
    let mut foreign_workspace = foreign.workspace("installed-domain-live-foreign").unwrap();
    let live = open_live(&handle, &mut owner_workspace, "installed.foreign-resume");
    let continuation = match live.checkpoint(&mut owner_workspace) {
        WorthQueryInstalledDomainLiveCheckpointOutcome::Checkpointed(continuation) => continuation,
        _ => panic!("installed live checkpoint must succeed"),
    };
    let continuation = match continuation.resume(&mut foreign_workspace) {
        WorthQueryInstalledDomainLiveResumeOutcome::AuthorityStopped(continuation, drift) => {
            assert_drift(
                &drift,
                WorthQueryInstalledDomainExecutionDriftKind::ForeignRuntime,
                WorthQueryInstalledDomainExecutionNextAction::UseOwningRuntime,
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
fn generation_turnover_prevents_live_continuation_revival() {
    let runtime = installed_runtime();
    let handle = runtime.domain(InstalledDomain).unwrap();
    let mut workspace = runtime.workspace("installed-domain-live-stale").unwrap();
    let live = open_live(&handle, &mut workspace, "installed.stale-resume");
    let continuation = match live.checkpoint(&mut workspace) {
        WorthQueryInstalledDomainLiveCheckpointOutcome::Checkpointed(continuation) => continuation,
        _ => panic!("installed live checkpoint must succeed"),
    };
    workspace
        .replace_domain_installation_with_successor_generation()
        .unwrap();

    let WorthQueryInstalledDomainLiveResumeOutcome::AuthorityStopped(_, drift) =
        continuation.resume(&mut workspace)
    else {
        panic!("a stale installed continuation must not revive")
    };
    assert_drift(
        &drift,
        WorthQueryInstalledDomainExecutionDriftKind::StaleInstallation,
        WorthQueryInstalledDomainExecutionNextAction::RebindCurrentInstallation,
    );
}

fn open_live(
    handle: &crate::domain_installation::WorthQueryInstalledDomainHandle<InstalledDomain>,
    workspace: &mut crate::runtime::WorthQueryWorkspace,
    name: &'static str,
) -> crate::domain_installation::WorthQueryInstalledDomainLiveHandle<InstalledDomain> {
    match handle
        .live(name, identity_read)
        .unwrap()
        .using(current())
        .open(workspace)
        .unwrap()
    {
        WorthQueryInstalledDomainLiveOpenOutcome::Opened(handle) => handle,
        WorthQueryInstalledDomainLiveOpenOutcome::Stopped(stop) => {
            panic!("installed live open stopped: {:?}", stop.stop().source())
        }
    }
}

fn assert_live_projection(
    live: &crate::domain_installation::WorthQueryInstalledDomainLiveHandle<InstalledDomain>,
    workspace: &mut crate::runtime::WorthQueryWorkspace,
    expected_witness: &crate::WorthQueryEvidenceIdentity,
) {
    let read = match live.read(workspace) {
        Ok(read) => read,
        Err(_) => panic!("installed live read must succeed"),
    };
    let projection = live.project(&read, project_facts().entity_identities());
    assert!(
        matches!(
            projection.outcome(),
            WorthQueryProjectionOutcome::Completed(_)
        ),
        "live projection outcome: {:#?}",
        projection.outcome()
    );
    assert_eq!(
        projection
            .receipt()
            .installed_authority()
            .witness_identity(),
        expected_witness
    );
}

fn assert_drift(
    drift: &WorthQueryInstalledDomainExecutionDrift,
    expected_kind: WorthQueryInstalledDomainExecutionDriftKind,
    expected_action: WorthQueryInstalledDomainExecutionNextAction,
) {
    assert_eq!(drift.kind(), expected_kind);
    assert_eq!(drift.next_action(), expected_action);
    assert_eq!(drift.counters().planning_attempts(), 0);
    assert_eq!(drift.counters().lower_runtime_attempts(), 0);
    assert_eq!(drift.counters().execution_attempts(), 0);
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
