use crate::{
    MinimalManifestVerifierReport, OfflinePhysicalVerifier, PhysicalHeaderAuthority,
    PlatformPhysicalFacadeCounterSnapshot, PlatformPhysicalRuntimeLayoutReport,
    PlatformPhysicalScanReport,
};
use forge_store_contracts::RoadmapScope;

use super::denials::PlatformPhysicalFacadeDenial;
use super::map_verifier_denial_for_reopen;
use super::storage::PlatformPhysicalFacadeStorage;

pub(crate) fn verify_persisted_layout_for_scan(
    storage: &PlatformPhysicalFacadeStorage,
    headers: &PhysicalHeaderAuthority,
) -> Result<MinimalManifestVerifierReport, PlatformPhysicalFacadeDenial> {
    OfflinePhysicalVerifier::s1(headers.clone())
        .verify(&storage.persisted_layout())
        .map_err(map_verifier_denial_for_reopen)
}

pub(crate) fn collect_runtime_layout_observation(
    storage: &PlatformPhysicalFacadeStorage,
) -> PlatformPhysicalRuntimeLayoutReport {
    PlatformPhysicalRuntimeLayoutReport::new(
        storage.runtime_discovered_references(),
        storage.runtime_traversal_report(),
    )
}

pub(crate) fn construct_scan_report(
    runtime_report: PlatformPhysicalRuntimeLayoutReport,
    verifier_report: MinimalManifestVerifierReport,
    counters: PlatformPhysicalFacadeCounterSnapshot,
    scope: RoadmapScope,
) -> PlatformPhysicalScanReport {
    PlatformPhysicalScanReport::new(runtime_report, verifier_report, counters, scope)
}
