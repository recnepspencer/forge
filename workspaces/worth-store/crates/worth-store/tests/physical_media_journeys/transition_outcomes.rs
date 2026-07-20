use super::*;

use worth_store::physical_runtime::certification::MediaFaultDirective;
use worth_store::physical_runtime::MediaAdmissionInspectionCause;
use worth_store_physical_backend::{
    MediaOperationRole, MediaQualificationDenial, MediaQualificationRebindRequired,
};

fn success(
    runtime: worth_store::physical_runtime::AdmittedPhysicalRuntime,
    admission: FilesystemMediaAdmission,
) -> worth_store::physical_runtime::MediaOwnedPhysicalRuntime {
    match runtime.try_admit_filesystem_media(admission).into_raw() {
        TransitionOutcome::Success(media) => media,
        _ => panic!("expected media admission success"),
    }
}

#[test]
fn every_media_transition_category_has_a_real_reachable_authority_fate() {
    let parent = tempfile::tempdir().unwrap();

    let success_root = parent.path().join("success");
    let media = success(admit_runtime(&success_root), media_admission());
    assert!(matches!(media.close(), MediaShutdownOutcome::Released(_)));

    let denied_root = parent.path().join("denied");
    let denied_runtime = admit_runtime(&denied_root);
    let denied_identity = denied_runtime.runtime_identity();
    let denied = match denied_runtime
        .try_admit_filesystem_media(FilesystemMediaAdmission::production(
            FilesystemAccessPosture::UnmanagedWritersPossible,
        ))
        .into_raw()
    {
        TransitionOutcome::Denied(denied) => denied,
        _ => panic!("unmanaged writers must be a definite denial"),
    };
    assert!(matches!(
        denied.reason(),
        MediaQualificationDenial::UnmanagedWriterPosture { .. }
    ));
    let denied_runtime = denied.into_runtime();
    assert_eq!(denied_runtime.runtime_identity(), denied_identity);
    denied_runtime.abort();

    let deferred_root = parent.path().join("deferred");
    let mut held = spawn_lease_holder(&deferred_root);
    let deferred_runtime = admit_runtime(&deferred_root);
    let deferred_identity = deferred_runtime.runtime_identity();
    let deferred = match deferred_runtime
        .try_admit_filesystem_media(media_admission())
        .into_raw()
    {
        TransitionOutcome::Deferred(deferred) => deferred,
        _ => panic!("real lease contention must defer"),
    };
    let deferred_counters = deferred.reason().counters();
    assert_eq!(deferred_counters.ownership_contentions(), 1);
    assert_eq!(
        deferred_counters.completed_operations_for(MediaOperationRole::CreateDirectory),
        0
    );
    assert_eq!(
        deferred_counters.completed_operations_for(MediaOperationRole::CreateMutationLease),
        0
    );
    assert!(deferred_counters.is_conserved());
    let deferred_runtime = deferred.into_runtime();
    assert_eq!(deferred_runtime.runtime_identity(), deferred_identity);
    deferred_runtime.abort();
    use std::io::Write;
    held.stdin.as_mut().unwrap().write_all(&[1]).unwrap();
    drop(held.stdin.take());
    assert!(held.wait().unwrap().success());

    let stale_root = parent.path().join("stale");
    let media = success(admit_runtime(&stale_root), media_admission());
    let stale_report = media.qualification_report();
    media.close();
    std::fs::rename(&stale_root, parent.path().join("stale-previous")).unwrap();
    let stale_runtime = admit_runtime(&stale_root);
    let stale_identity = stale_runtime.runtime_identity();
    let stale = match stale_runtime
        .try_admit_filesystem_media(media_admission().require_current_profile(stale_report))
        .into_raw()
    {
        TransitionOutcome::Stale(stale) => stale,
        _ => panic!("replaced root identity must stale the prior report"),
    };
    let stale_counters = stale.reason().counters();
    assert_eq!(
        stale_counters.attempts_for(MediaOperationRole::ObserveRootProfile),
        1
    );
    assert_eq!(stale_counters.completed_operations(), 1);
    assert_eq!(stale_counters.denied_before_effect(), 0);
    assert!(stale_counters.is_conserved());
    let stale_runtime = stale.into_runtime();
    assert_eq!(stale_runtime.runtime_identity(), stale_identity);
    stale_runtime.abort();

    let rebind_root = parent.path().join("rebind");
    let media = success(admit_runtime(&rebind_root), media_admission());
    let report = media
        .qualification_report()
        .with_profile_digest_for_certification([0xA5; 32]);
    media.close();
    let rebind_runtime = admit_runtime(&rebind_root);
    let rebind_identity = rebind_runtime.runtime_identity();
    let rebind = match rebind_runtime
        .try_admit_filesystem_media(media_admission().require_current_profile(report))
        .into_raw()
    {
        TransitionOutcome::RebindRequired(rebind) => rebind,
        _ => panic!("profile-contract drift must require rebind"),
    };
    let MediaQualificationRebindRequired::BackendProfileChanged { counters } = rebind.reason()
    else {
        panic!("profile drift must retain its semantic rebind reason");
    };
    assert_eq!(
        counters.attempts_for(MediaOperationRole::ObserveRootProfile),
        1
    );
    assert!(counters.is_conserved());
    let rebind_runtime = rebind.into_runtime();
    assert_eq!(rebind_runtime.runtime_identity(), rebind_identity);
    rebind_runtime.abort();

    let failed_root = parent.path().join("failed");
    let established = success(admit_runtime(&failed_root), media_admission());
    let failed_store = established.store_identity();
    established.close();
    let admission = media_admission();
    let faults = admission.fault_schedule_authority();
    let schedule = faults
        .schedule(vec![faults
            .rule(
                MediaOperationRole::OpenExisting,
                2,
                MediaFaultDirective::FailBefore {
                    kind: std::io::ErrorKind::PermissionDenied,
                    raw_os_error: None,
                },
            )
            .for_store(failed_store)])
        .unwrap();
    let failed_runtime = admit_runtime(&failed_root);
    let failed_identity = failed_runtime.runtime_identity();
    let failed = match failed_runtime
        .try_admit_filesystem_media(admission.with_fault_schedule(schedule))
        .into_raw()
    {
        TransitionOutcome::Failed(failed) => failed,
        _ => panic!("post-owner identity failure must consume authority"),
    };
    assert_eq!(failed.runtime_identity(), failed_identity);
    assert!(matches!(
        failed.cause(),
        MediaAdmissionInspectionCause::BackendFailure(_)
    ));
    admit_runtime(&failed_root).abort();
}

#[test]
fn backend_denial_after_preflight_returns_exact_runtime_and_counters() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("pre-effect-backend-denial");
    let runtime = admit_runtime(&root);
    let runtime_identity = runtime.runtime_identity();
    let admission = media_admission();
    let faults = admission.fault_schedule_authority();
    let schedule = faults
        .schedule(vec![faults.rule(
            MediaOperationRole::OpenRootParent,
            1,
            MediaFaultDirective::FailBefore {
                kind: std::io::ErrorKind::PermissionDenied,
                raw_os_error: None,
            },
        )])
        .unwrap();
    let denial = match runtime
        .try_admit_filesystem_media(admission.with_fault_schedule(schedule))
        .into_raw()
    {
        TransitionOutcome::Denied(denial) => denial,
        _ => panic!("a proven pre-effect backend denial must return C.3 authority"),
    };
    let MediaQualificationDenial::OwnerPreEffect {
        denial: cause,
        counters,
    } = denial.reason()
    else {
        panic!("backend denial must retain its exact effect posture");
    };
    let worth_store_physical_backend::FilesystemMediaOwnerAdmissionDenial::Confinement(cause) =
        cause
    else {
        panic!("first-call denial must retain confinement cause");
    };
    assert_eq!(cause.io_kind(), Some(std::io::ErrorKind::PermissionDenied));
    assert_eq!(counters.denied_before_effect(), 1);
    assert_eq!(counters.completed_operations(), 1);
    assert!(counters.is_conserved());
    assert!(!root.exists());
    let runtime = denial.into_runtime();
    assert_eq!(runtime.runtime_identity(), runtime_identity);
    runtime.abort();
}
