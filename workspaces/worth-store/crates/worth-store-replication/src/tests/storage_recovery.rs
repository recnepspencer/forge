use worth_store_physical_backend::{
    ProductionStorageBoundarySeam, ScriptedStorageBoundaryControl, StorageBoundaryFault,
};

use super::{progress_directory, source};
use crate::{
    admit_replication_publication_readiness, ReplicationAdmissionRuntime, ReplicationDeliveryKind,
    ReplicationPeerCapacity, ReplicationPublicationDenial, ReplicationPublicationOutcomeView,
};

#[test]
fn torn_first_snapshot_reopens_as_no_committed_peer_progress() {
    let directory = progress_directory("first-torn-injection");
    let first = source(90, 7, "lineage-a", 0, 1, "sha256:first-torn");
    let authority = first.security_scope().current_authority().clone();
    let mut runtime = ReplicationAdmissionRuntime::open(
        &directory,
        &authority,
        ReplicationPeerCapacity::new(1).unwrap(),
    )
    .unwrap();
    let readiness = admit_replication_publication_readiness(
        runtime
            .observe_progress(first)
            .into_observed_progress()
            .unwrap(),
    );
    let control = ScriptedStorageBoundaryControl::inject(
        ProductionStorageBoundarySeam::ReplicationProgressSnapshotWrite,
        StorageBoundaryFault::TearWrite { retained_bytes: 8 },
    );
    assert!(matches!(
        runtime
            .publish_with_boundary_control(readiness, &authority, &control)
            .view(),
        ReplicationPublicationOutcomeView::Denied(ReplicationPublicationDenial::ProgressStoreIo)
    ));
    drop(runtime);

    let reopened = ReplicationAdmissionRuntime::open(
        &directory,
        &authority,
        ReplicationPeerCapacity::new(1).unwrap(),
    )
    .unwrap();
    let observed = reopened
        .observe_progress(source(90, 7, "lineage-a", 0, 1, "sha256:first-torn"))
        .into_observed_progress()
        .unwrap();
    assert_eq!(observed.delivery_kind(), ReplicationDeliveryKind::Fresh);
}

#[test]
fn interruption_after_snapshot_durability_reopens_as_committed_progress() {
    let directory = progress_directory("durable-injection");
    let first = source(100, 7, "lineage-a", 0, 1, "sha256:durable-interrupted");
    let authority = first.security_scope().current_authority().clone();
    let mut runtime = ReplicationAdmissionRuntime::open(
        &directory,
        &authority,
        ReplicationPeerCapacity::new(1).unwrap(),
    )
    .unwrap();
    let readiness = admit_replication_publication_readiness(
        runtime
            .observe_progress(first)
            .into_observed_progress()
            .unwrap(),
    );
    let control = ScriptedStorageBoundaryControl::inject(
        ProductionStorageBoundarySeam::ReplicationProgressSnapshotDurable,
        StorageBoundaryFault::Interrupt,
    );
    assert!(matches!(
        runtime
            .publish_with_boundary_control(readiness, &authority, &control)
            .view(),
        ReplicationPublicationOutcomeView::Denied(ReplicationPublicationDenial::ProgressStoreIo)
    ));
    drop(runtime);

    let reopened = ReplicationAdmissionRuntime::open(
        &directory,
        &authority,
        ReplicationPeerCapacity::new(1).unwrap(),
    )
    .unwrap();
    let resumed = reopened
        .observe_progress(source(101, 7, "lineage-a", 1, 2, "sha256:after-reopen"))
        .into_observed_progress()
        .unwrap();
    assert_eq!(resumed.delivery_kind(), ReplicationDeliveryKind::Resumed);
}

#[test]
fn generated_first_snapshot_tears_reopen_without_inventing_peer_progress() {
    let encoded_snapshot_bytes = first_snapshot_encoded_bytes();
    for retained_bytes in 0..encoded_snapshot_bytes {
        let directory = progress_directory(&format!("generated-first-tear-{retained_bytes}"));
        let first = source(200, 7, "lineage-a", 0, 1, "sha256:generated-first");
        let authority = first.security_scope().current_authority().clone();
        let mut runtime = ReplicationAdmissionRuntime::open(
            &directory,
            &authority,
            ReplicationPeerCapacity::new(1).unwrap(),
        )
        .unwrap();
        let readiness = admit_replication_publication_readiness(
            runtime
                .observe_progress(first)
                .into_observed_progress()
                .unwrap(),
        );
        let control = ScriptedStorageBoundaryControl::inject(
            ProductionStorageBoundarySeam::ReplicationProgressSnapshotWrite,
            StorageBoundaryFault::TearWrite { retained_bytes },
        );
        assert!(runtime
            .publish_with_boundary_control(readiness, &authority, &control)
            .into_result()
            .is_err());
        drop(runtime);

        let reopened = ReplicationAdmissionRuntime::open(
            &directory,
            &authority,
            ReplicationPeerCapacity::new(1).unwrap(),
        )
        .unwrap();
        let observed = reopened
            .observe_progress(source(200, 7, "lineage-a", 0, 1, "sha256:generated-first"))
            .into_observed_progress()
            .unwrap();
        assert_eq!(observed.delivery_kind(), ReplicationDeliveryKind::Fresh);
    }
}

fn first_snapshot_encoded_bytes() -> u64 {
    let directory = progress_directory("generated-first-tear-size-probe");
    let first = source(200, 7, "lineage-a", 0, 1, "sha256:generated-first");
    let authority = first.security_scope().current_authority().clone();
    let mut runtime = ReplicationAdmissionRuntime::open(
        &directory,
        &authority,
        ReplicationPeerCapacity::new(1).unwrap(),
    )
    .unwrap();
    let readiness = admit_replication_publication_readiness(
        runtime
            .observe_progress(first)
            .into_observed_progress()
            .unwrap(),
    );
    runtime
        .publish(readiness, &authority)
        .into_result()
        .unwrap();
    std::fs::metadata(directory.join("replication-progress-1.snapshot"))
        .unwrap()
        .len()
}
