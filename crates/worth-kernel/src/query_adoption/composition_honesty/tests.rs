use super::*;

fn current_sources() -> Vec<WorthKernelCompositionSourceStatus> {
    WorthKernelCompositionSourceStatus::current().expect("composition source statuses")
}

#[test]
fn composition_report_consumes_lower_crate_query_evidence() {
    let report = current_kernel_composition_honesty_report()
        .expect("kernel composition honesty should be evidence backed");

    assert_eq!(report.kernel_composition_source_count(), 3);
    assert_eq!(
        report.lower_crate_receipt_family_count(),
        LOWER_CRATE_RECEIPT_FAMILY_COUNT
    );
    assert_eq!(
        report.kernel_workload_receipt_family_count(),
        WorkloadEvidenceStage::AUTHORITY_STAGES.len()
    );
    assert_eq!(report.spatial_workload_support_pin_row_count(), 1);
    assert_eq!(
        report.representative_workload_evidence_row_count(),
        WorkloadEvidenceStage::AUTHORITY_STAGES.len()
    );
    assert_eq!(report.representative_spatial_receipt_identity_count(), 7);
    assert!(!report.evidence_report_identity().is_empty());
    assert!(!report.digest_participation_identity().is_empty());
}

#[test]
fn composition_denies_missing_topology_receipts() {
    let mut sources = current_sources();
    sources.retain(|source| source.kind != WorthKernelCompositionSourceKind::Topology);

    let error = WorthKernelCompositionHonestyReport::from_sources(sources)
        .expect_err("topology receipt source must be mandatory");

    assert!(matches!(
        error,
        WorthKernelCompositionHonestyError::MissingTopologyReceipts
    ));
}

#[test]
fn composition_denies_missing_spatial_receipts() {
    let mut sources = current_sources();
    sources.retain(|source| source.kind != WorthKernelCompositionSourceKind::Spatial);

    let error = WorthKernelCompositionHonestyReport::from_sources(sources)
        .expect_err("spatial receipt source must be mandatory");

    assert!(matches!(
        error,
        WorthKernelCompositionHonestyError::MissingSpatialReceipts
    ));
}

#[test]
fn composition_denies_missing_kernel_receipts() {
    let mut sources = current_sources();
    sources.retain(|source| source.kind != WorthKernelCompositionSourceKind::Kernel);

    let error = WorthKernelCompositionHonestyReport::from_sources(sources)
        .expect_err("kernel Query evidence source must be mandatory");

    assert!(matches!(
        error,
        WorthKernelCompositionHonestyError::MissingKernelReceipts
    ));
}

#[test]
fn composition_denies_duplicated_lower_crate_sources() {
    let mut sources = current_sources();
    let topology = sources
        .iter()
        .find(|source| source.kind == WorthKernelCompositionSourceKind::Topology)
        .expect("topology source")
        .clone();
    sources.push(topology);

    let error = WorthKernelCompositionHonestyReport::from_sources(sources)
        .expect_err("duplicated lower-crate authority source must be denied");

    assert!(matches!(
        error,
        WorthKernelCompositionHonestyError::DuplicateCompositionSource(
            WorthKernelCompositionSourceKind::Topology
        )
    ));
}

#[test]
fn composition_denies_stale_support_pins() {
    let mut sources = current_sources();
    let topology = sources
        .iter_mut()
        .find(|source| source.kind == WorthKernelCompositionSourceKind::Topology)
        .expect("topology source");
    topology.support_blocking_finding_count = 1;

    let error = WorthKernelCompositionHonestyReport::from_sources(sources)
        .expect_err("stale support pin must deny composition proof");

    assert!(matches!(
        error,
        WorthKernelCompositionHonestyError::StaleSupportPins {
            source: WorthKernelCompositionSourceKind::Topology,
            blocking_finding_count: 1
        }
    ));
}

#[test]
fn composition_denies_forged_evidence_reports() {
    let mut sources = current_sources();
    let spatial = sources
        .iter_mut()
        .find(|source| source.kind == WorthKernelCompositionSourceKind::Spatial)
        .expect("spatial source");
    spatial.evidence_report_identity = "forged-spatial-evidence".to_string();

    let error = WorthKernelCompositionHonestyReport::from_sources(sources)
        .expect_err("forged evidence identity must deny composition proof");

    assert!(matches!(
        error,
        WorthKernelCompositionHonestyError::ForgedEvidenceReport {
            source: WorthKernelCompositionSourceKind::Spatial,
            field: "evidence_report_identity"
        }
    ));
}

#[test]
fn composition_denies_lost_spatial_workload_cardinality() {
    let mut sources = current_sources();
    let spatial = sources
        .iter_mut()
        .find(|source| source.kind == WorthKernelCompositionSourceKind::Spatial)
        .expect("spatial source");
    spatial.workload_support_pin_row_count = 0;

    let error = WorthKernelCompositionHonestyReport::from_sources(sources)
        .expect_err("kernel must preserve spatial workload support cardinality");

    assert!(matches!(
        error,
        WorthKernelCompositionHonestyError::MissingSpatialReceipts
    ));
}

#[test]
fn composition_denies_representative_workload_missing_spatial_receipts() {
    let snapshot = WorthKernelRepresentativeWorkloadSnapshot::current()
        .expect("representative workload snapshot")
        .with_missing_spatial_receipts();

    let error = WorthKernelCompositionHonestyReport::from_sources_and_workload_snapshot(
        current_sources(),
        snapshot,
    )
    .expect_err("representative workload must carry spatial receipt identities");

    assert!(matches!(
        error,
        WorthKernelCompositionHonestyError::MissingSpatialReceipts
    ));
}
