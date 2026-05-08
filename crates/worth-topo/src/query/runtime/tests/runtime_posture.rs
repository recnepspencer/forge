use crate::query::{
    TopologyRuntimePostureCapability, TopologyRuntimePostureStatus, TopologyRuntimeSupport,
};

#[test]
fn current_head_runtime_posture_rows_freeze_admitted_and_denied_capabilities() {
    let support = TopologyRuntimeSupport::current_head_authoritative();

    assert_eq!(
        support.runtime_posture_rows().len(),
        TopologyRuntimePostureCapability::ALL.len()
    );
    for capability in TopologyRuntimePostureCapability::ALL {
        let row = support
            .runtime_posture_rows()
            .iter()
            .find(|row| row.capability() == capability)
            .expect("current-head posture row should exist");
        let expected_status = match capability {
            TopologyRuntimePostureCapability::CurrentHeadLiveReads
            | TopologyRuntimePostureCapability::PostWriteMaterialization
            | TopologyRuntimePostureCapability::AuthoritativeWrites => {
                TopologyRuntimePostureStatus::Admitted
            }
            TopologyRuntimePostureCapability::CurrentHeadMaterialization
            | TopologyRuntimePostureCapability::HistoricalBasis => {
                TopologyRuntimePostureStatus::Denied
            }
        };
        assert_eq!(row.status(), expected_status);
        assert!(!row.row_digest().is_empty());
    }
}

#[test]
fn snapshot_runtime_posture_rows_freeze_historical_read_only_capabilities() {
    let support = TopologyRuntimeSupport::snapshot_read_only();

    assert_eq!(
        support.runtime_posture_rows().len(),
        TopologyRuntimePostureCapability::ALL.len()
    );
    for capability in TopologyRuntimePostureCapability::ALL {
        let expected_status = match capability {
            TopologyRuntimePostureCapability::HistoricalBasis => {
                TopologyRuntimePostureStatus::Admitted
            }
            TopologyRuntimePostureCapability::CurrentHeadLiveReads
            | TopologyRuntimePostureCapability::CurrentHeadMaterialization
            | TopologyRuntimePostureCapability::PostWriteMaterialization
            | TopologyRuntimePostureCapability::AuthoritativeWrites => {
                TopologyRuntimePostureStatus::Denied
            }
        };
        assert_eq!(support.runtime_posture_status(capability), expected_status);
    }
}
