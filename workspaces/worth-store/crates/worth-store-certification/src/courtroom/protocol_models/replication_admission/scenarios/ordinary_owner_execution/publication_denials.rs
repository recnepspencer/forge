use super::*;

pub(super) fn collect_publication_denials(
    actions: &mut BTreeSet<ReplicationAdmissionAction>,
) {
    collect_current_authority_publication_denial(actions);
    collect_stale_progress_publication_denial(actions);
    collect_peer_capacity_publication_denial(actions);
    collect_progress_store_publication_denial(actions);
}

fn collect_current_authority_publication_denial(
    actions: &mut BTreeSet<ReplicationAdmissionAction>,
) {
    let source = admitted_source(SourceSpec::standard(30, 30, 40));
    let mut runtime = current_runtime();
    let progress = runtime
        .observe_progress(source)
        .into_observed_progress()
        .unwrap();
    let readiness = admit_replication_publication_readiness(progress);
    let foreign = readmitted_foreign_wal_security_scope_for_test();
    let denied = runtime.publish(readiness, foreign.current_authority());
    actions.insert(map_replication_publication_outcome(&denied));
}

fn collect_stale_progress_publication_denial(actions: &mut BTreeSet<ReplicationAdmissionAction>) {
    let mut runtime = current_runtime();
    let first = publication_readiness(&runtime, admitted_source(SourceSpec::standard(31, 30, 40)));
    let stale = publication_readiness(&runtime, admitted_source(SourceSpec::standard(32, 30, 50)));
    let authority = first.source().security_scope().current_authority().clone();
    runtime.publish(first, &authority).into_result().unwrap();
    let denied = runtime.publish(stale, &authority);
    actions.insert(map_replication_publication_outcome(&denied));
}

fn collect_peer_capacity_publication_denial(actions: &mut BTreeSet<ReplicationAdmissionAction>) {
    let scope = readmitted_wal_security_scope_for_test();
    let mut runtime = open_runtime(ReplicationPeerCapacity::new(1).unwrap());
    let first = publication_readiness(&runtime, admitted_source(SourceSpec::standard(33, 30, 40)));
    runtime
        .publish(first, scope.current_authority())
        .into_result()
        .unwrap();
    let second = publication_readiness(
        &runtime,
        admitted_source(SourceSpec {
            peer: "peer-capacity-denied",
            ..SourceSpec::standard(34, 30, 40)
        }),
    );
    let denied = runtime.publish(second, scope.current_authority());
    actions.insert(map_replication_publication_outcome(&denied));
}

fn collect_progress_store_publication_denial(actions: &mut BTreeSet<ReplicationAdmissionAction>) {
    let scope = readmitted_wal_security_scope_for_test();
    let directory = unique_progress_directory();
    let mut runtime = ReplicationAdmissionRuntime::open(
        &directory,
        scope.current_authority(),
        ReplicationPeerCapacity::new(1).unwrap(),
    )
    .unwrap();
    let log = directory.join("replication-progress.lock");
    std::fs::remove_file(&log).unwrap();
    std::fs::create_dir(&log).unwrap();
    let readiness =
        publication_readiness(&runtime, admitted_source(SourceSpec::standard(35, 30, 40)));
    let denied = runtime.publish(readiness, scope.current_authority());
    actions.insert(map_replication_publication_outcome(&denied));
}
