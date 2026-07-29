use super::*;

#[test]
fn every_owner_has_one_closed_fact_family_and_only_query_can_reset() {
    let expected = [
        (
            UiProducedFactOwner::SourceIngress,
            UiProducedFactFamily::AuthoredSource,
            UiProducedFactResetPosture::NoReset,
        ),
        (
            UiProducedFactOwner::HostViewport,
            UiProducedFactFamily::HostViewport,
            UiProducedFactResetPosture::NoReset,
        ),
        (
            UiProducedFactOwner::HostDeviceScale,
            UiProducedFactFamily::HostDeviceScale,
            UiProducedFactResetPosture::NoReset,
        ),
        (
            UiProducedFactOwner::MeasurementExchange,
            UiProducedFactFamily::Measurement,
            UiProducedFactResetPosture::NoReset,
        ),
        (
            UiProducedFactOwner::QueryBinding,
            UiProducedFactFamily::Query,
            UiProducedFactResetPosture::OwnerIssuedReset,
        ),
        (
            UiProducedFactOwner::ScrollRuntimeState,
            UiProducedFactFamily::CommittedScrollExtent,
            UiProducedFactResetPosture::NoReset,
        ),
        (
            UiProducedFactOwner::PortalRuntimeState,
            UiProducedFactFamily::CommittedPortalAnchor,
            UiProducedFactResetPosture::NoReset,
        ),
    ];

    for (owner, family, reset) in expected {
        let contract = UiProducedFactContract::for_owner(owner);
        assert_eq!(contract.owner(), owner);
        assert_eq!(contract.family(), family);
        assert_eq!(contract.reset_posture(), reset);
    }
}
