use crate::projection::runtime_boundary::query_runtime::{
    TopologyQueryReadFamilySupportStatus, TopologyRuntimeCloseoutFamily,
    TopologyRuntimeCloseoutStatus, TopologyRuntimeSupport,
};
use crate::projection::TopologyDomainQueryRequestFamily;

#[test]
fn current_head_runtime_support_reports_query_native_read_family_admission() {
    let support = TopologyRuntimeSupport::current_head_authoritative();

    assert_eq!(
        support.query_read_family_support_rows().len(),
        TopologyDomainQueryRequestFamily::ALL.len()
    );
    for family in TopologyDomainQueryRequestFamily::ALL {
        assert_eq!(
            support.query_read_family_support_status(family),
            TopologyQueryReadFamilySupportStatus::Admitted
        );
        let row = support
            .query_read_family_support_rows()
            .iter()
            .find(|row| row.family() == family)
            .expect("current-head read family row should exist");
        assert_eq!(row.status(), TopologyQueryReadFamilySupportStatus::Admitted);
        assert!(!row.row_digest().is_empty());
    }
}

#[test]
fn snapshot_runtime_support_reports_historical_query_native_read_family_admission() {
    let support = TopologyRuntimeSupport::snapshot_read_only();

    assert_eq!(
        support.query_read_family_support_rows().len(),
        TopologyDomainQueryRequestFamily::ALL.len()
    );
    for family in TopologyDomainQueryRequestFamily::ALL {
        assert_eq!(
            support.query_read_family_support_status(family),
            TopologyQueryReadFamilySupportStatus::Admitted
        );
        let row = support
            .query_read_family_support_rows()
            .iter()
            .find(|row| row.family() == family)
            .expect("snapshot read family row should exist");
        assert_eq!(row.status(), TopologyQueryReadFamilySupportStatus::Admitted);
        assert!(row.reason().contains("historical query basis context"));
    }
}

#[test]
fn runtime_closeout_reports_current_head_completion_and_snapshot_blockers_honestly() {
    let current_head = TopologyRuntimeSupport::current_head_authoritative();
    let snapshot = TopologyRuntimeSupport::snapshot_read_only();

    assert_eq!(
        current_head.closeout().rows().len(),
        TopologyRuntimeCloseoutFamily::ALL.len()
    );
    for family in TopologyRuntimeCloseoutFamily::ALL {
        let row = current_head
            .closeout()
            .rows()
            .iter()
            .find(|row| row.family() == family)
            .expect("current-head closeout row should exist");
        assert_eq!(row.status(), TopologyRuntimeCloseoutStatus::Satisfied);
        assert!(!row.row_digest().is_empty());
    }

    assert_eq!(
        snapshot
            .closeout()
            .status(TopologyRuntimeCloseoutFamily::BridgeBackedRuntimePath),
        TopologyRuntimeCloseoutStatus::Satisfied
    );
    assert_eq!(
        snapshot
            .closeout()
            .status(TopologyRuntimeCloseoutFamily::QueryNativeTopologyReadFamilies),
        TopologyRuntimeCloseoutStatus::Satisfied
    );
    for family in [
        TopologyRuntimeCloseoutFamily::QueryNativeTopologyEditFamilies,
        TopologyRuntimeCloseoutFamily::QueryNativeGraphComposedEditLanes,
    ] {
        assert_eq!(
            snapshot.closeout().status(family),
            TopologyRuntimeCloseoutStatus::Blocked
        );
    }
    assert_eq!(
        snapshot
            .closeout()
            .status(TopologyRuntimeCloseoutFamily::MirrorReadDeletion),
        TopologyRuntimeCloseoutStatus::Satisfied
    );
}
