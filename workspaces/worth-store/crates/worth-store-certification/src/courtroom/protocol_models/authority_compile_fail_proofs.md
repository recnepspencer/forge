# Protocol courtroom authority inversion proofs

Certification can observe a published replication outcome but cannot construct
one. The failure is field privacy on the resolved production type:

```compile_fail
use worth_store_replication::PublishedReplication;

fn certification_cannot_issue_owner_outcome() -> PublishedReplication {
    PublishedReplication {}
}
```

Test-support fixtures remain hostile inputs, not production authority. This
fails because the real publication boundary requires the concrete current
authority witness rather than anything supplied by test support:

```compile_fail
use worth_store_replication::{
    ReplicationAdmissionRuntime, ReplicationPublicationReadiness,
};
use worth_store_test_support::harness::physical_isolation::reclaim::ReclaimFixture;

fn fixture_cannot_publish(
    runtime: &mut ReplicationAdmissionRuntime,
    readiness: ReplicationPublicationReadiness,
    fixture: &ReclaimFixture,
) {
    let _ = runtime.publish(readiness, fixture);
}
```
