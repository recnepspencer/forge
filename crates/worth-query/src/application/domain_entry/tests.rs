use super::{WorthQueryDomainEntryChecked, WorthQueryDomainEntryMarker};
use crate::application::{
    WorthQueryApplicationFacade, WorthQueryCapabilityFamily, WorthQueryConfig,
    WorthQueryConfigSectionFamily, WorthQueryQueryConfig, WorthQueryRelationalConfig,
    WorthQueryRuntimeBridgeConfig, WorthQuerySignalConfig,
};
use crate::runtime::{WorthQueryRuntimeFacadeFamily, WorthQueryRuntimeFamilySupportStatus};

const ENTRY_CAPABILITIES: &[WorthQueryCapabilityFamily] = &[
    WorthQueryCapabilityFamily::QueryComposition,
    WorthQueryCapabilityFamily::QueryContext,
    WorthQueryCapabilityFamily::IdentityEvolution,
    WorthQueryCapabilityFamily::PreviewSession,
    WorthQueryCapabilityFamily::WorkflowOrchestration,
    WorthQueryCapabilityFamily::HistoricalEvaluation,
];

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct LocalSpatialDomain;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct LocalTopologyDomain;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct LocalDeferredArtifactDomain;

impl WorthQueryDomainEntryMarker for LocalSpatialDomain {
    fn domain_key(&self) -> &'static str {
        "test.spatial"
    }

    fn display_name(&self) -> &'static str {
        "LocalSpatialDomain"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        ENTRY_CAPABILITIES
    }
}

impl WorthQueryDomainEntryMarker for LocalTopologyDomain {
    fn domain_key(&self) -> &'static str {
        "test.topology"
    }

    fn display_name(&self) -> &'static str {
        "LocalTopologyDomain"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        ENTRY_CAPABILITIES
    }
}

impl WorthQueryDomainEntryMarker for LocalDeferredArtifactDomain {
    fn domain_key(&self) -> &'static str {
        "test.deferred_artifact"
    }

    fn display_name(&self) -> &'static str {
        "LocalDeferredArtifactDomain"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::DurableArtifacts]
    }
}

#[test]
fn domain_entry_support_snapshot_matches_support_report_truth() {
    let facade = WorthQueryApplicationFacade::runtime_backed_default();
    let snapshot = facade.domain_entry_support_snapshot();
    let report = facade.support_report();

    assert_eq!(
        snapshot.admitted_capability_families(),
        report.admitted_capability_families()
    );
    assert_eq!(
        snapshot.deferred_capability_families(),
        report.deferred_capability_families()
    );
    assert_eq!(
        snapshot.unsupported_capability_families(),
        report.unsupported_capability_families()
    );
    assert_eq!(snapshot.section_postures(), report.section_postures());
    assert_eq!(
        snapshot.validated_config_digest(),
        report.validated_config_digest()
    );
    assert_ne!(snapshot.snapshot_digest(), report.report_digest());
    assert_eq!(
        snapshot.runtime_support_matrix().backend_posture().as_str(),
        "primary"
    );
    assert_eq!(
        snapshot
            .runtime_support_matrix()
            .row_for_family(WorthQueryRuntimeFacadeFamily::Temporal)
            .expect("temporal runtime row should exist")
            .status(),
        WorthQueryRuntimeFamilySupportStatus::Supported
    );
    assert_eq!(
        snapshot
            .runtime_support_matrix()
            .row_for_family(WorthQueryRuntimeFacadeFamily::AsyncResource)
            .expect("async runtime row should exist")
            .status(),
        WorthQueryRuntimeFamilySupportStatus::Supported
    );
}

#[test]
fn domain_entry_root_and_proof_root_share_entry_meaning() {
    let facade = WorthQueryApplicationFacade::runtime_backed_default();
    let root = facade.domain(LocalSpatialDomain);
    let proof = facade.domain_proof_root(LocalSpatialDomain);

    assert_eq!(root.domain_key(), "test.spatial");
    assert_eq!(root.display_name(), "LocalSpatialDomain");
    assert_eq!(proof.domain_key(), root.domain_key());
    assert_eq!(proof.display_name(), root.display_name());
    assert_eq!(proof.support_snapshot(), root.support_snapshot());

    let topology = facade.domain(LocalTopologyDomain);
    assert_eq!(topology.domain_key(), "test.topology");
}

#[test]
fn admitted_checked_entry_matches_ordinary_entry_support_posture() {
    let facade = WorthQueryApplicationFacade::runtime_backed_default();
    let ordinary = facade.domain(LocalSpatialDomain);
    let checked = facade.domain_checked(LocalSpatialDomain);

    match checked {
        WorthQueryDomainEntryChecked::Admitted(admitted) => {
            assert_eq!(ordinary.domain_key(), admitted.domain_key());
            assert_eq!(ordinary.display_name(), admitted.display_name());
            assert_eq!(ordinary.support_snapshot(), admitted.support_snapshot());
        }
        other => panic!("expected admitted checked entry, got {other:?}"),
    }
}

#[test]
fn checked_domain_entry_defers_when_only_deferred_capabilities_block() {
    let facade = WorthQueryApplicationFacade::runtime_backed_default();
    let checked = facade.domain_checked(LocalDeferredArtifactDomain);

    match checked {
        WorthQueryDomainEntryChecked::Deferred(deferred) => {
            assert_eq!(
                deferred.blocking_capability_families(),
                &[WorthQueryCapabilityFamily::DurableArtifacts]
            );
            assert_eq!(deferred.marker(), LocalDeferredArtifactDomain);
        }
        other => panic!("expected deferred checked entry, got {other:?}"),
    }
}

#[test]
fn checked_domain_entry_marks_unsupported_when_query_section_is_disabled() {
    let facade = WorthQueryApplicationFacade::new(
        WorthQueryConfig::runtime_backed_default()
            .with_query(WorthQueryQueryConfig::disabled())
            .with_relational(WorthQueryRelationalConfig::disabled())
            .with_signal(WorthQuerySignalConfig::disabled())
            .with_runtime_bridge(WorthQueryRuntimeBridgeConfig::disabled()),
    )
    .expect("query-disabled config remains valid when dependents are disabled too");
    let checked = facade.domain_checked(LocalSpatialDomain);

    match checked {
        WorthQueryDomainEntryChecked::Unsupported(unsupported) => {
            assert!(unsupported
                .blocking_capability_families()
                .contains(&WorthQueryCapabilityFamily::QueryComposition));
            assert!(unsupported
                .support_snapshot()
                .section_postures()
                .iter()
                .any(|posture| {
                    posture.section() == WorthQueryConfigSectionFamily::Query && !posture.enabled()
                }));
        }
        other => panic!("expected unsupported checked entry, got {other:?}"),
    }
}
