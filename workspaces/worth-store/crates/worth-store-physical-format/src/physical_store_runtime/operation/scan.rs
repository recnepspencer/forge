use crate::{
    MinimalManifestVerifierReport, OfflinePhysicalVerifier, PhysicalHeaderAuthority,
    PhysicalStoreRuntimeCounterSnapshot, PlatformPhysicalRuntimeLayoutReport,
    PlatformPhysicalScanReport,
};
use worth_store_contracts::RoadmapScope;

use super::denials::PhysicalStoreRuntimeDenial;
use super::reopen::map_verifier_denial_for_reopen;
use super::storage::PhysicalStoreRuntimeStorage;
use super::{
    PhysicalStoreRuntime, PhysicalStoreRuntimeDenialKind, PlatformPhysicalDegradedExactScanReceipt,
    PlatformPhysicalHiddenScanDenialReceipt, PlatformPhysicalLayoutAccessIntent,
    PlatformPhysicalLayoutAccessRequest,
};

impl PhysicalStoreRuntime {
    pub fn execute_admitted_degraded_exact_scan(
        &mut self,
        ready: super::PlatformPhysicalDegradedExactScanReady,
    ) -> Result<super::PlatformPhysicalDegradedExecutionObservation, PhysicalStoreRuntimeDenial>
    {
        let allocations_before = self.counters().allocation_events();
        let request = PlatformPhysicalLayoutAccessRequest::explicit_degraded_exact_scan(
            ready.admitted_rows(),
        );
        let scan = self.execute_explicit_degraded_exact_scan(request)?;
        Ok(super::PlatformPhysicalDegradedExecutionObservation::issue(
            ready.budget(),
            scan,
            self.counters().allocation_events() - allocations_before,
        ))
    }

    pub fn scan_physical_layout(
        &mut self,
    ) -> Result<PlatformPhysicalScanReport, PhysicalStoreRuntimeDenial> {
        let verifier_report = verify_persisted_layout_for_scan(&self.storage, &self.headers)?;
        let runtime_report = collect_runtime_layout_observation(&self.storage);
        self.counters = self.counters.with_scan_materializations().with_scan();
        Ok(construct_scan_report(
            runtime_report,
            verifier_report,
            self.counters,
            self.scope,
        ))
    }

    pub fn reject_hidden_broad_scan(
        &mut self,
        request: PlatformPhysicalLayoutAccessRequest,
    ) -> PlatformPhysicalHiddenScanDenialReceipt {
        debug_assert!(matches!(
            request.intent(),
            PlatformPhysicalLayoutAccessIntent::HiddenBroadScan
        ));
        let counters_before = self.counters;
        self.counters = self.counters.with_full_store_materialization_rejection();
        PlatformPhysicalHiddenScanDenialReceipt::from_rejected_request(
            request,
            counters_before,
            self.counters,
        )
    }

    pub fn execute_explicit_degraded_exact_scan(
        &mut self,
        request: PlatformPhysicalLayoutAccessRequest,
    ) -> Result<PlatformPhysicalDegradedExactScanReceipt, PhysicalStoreRuntimeDenial> {
        if !matches!(
            request.intent(),
            PlatformPhysicalLayoutAccessIntent::ExplicitDegradedExactScan
        ) || request.budget_rows() == 0
        {
            return Err(PhysicalStoreRuntimeDenial::new(
                PhysicalStoreRuntimeDenialKind::OfflineVerifierDenied,
            ));
        }
        let scan = self.scan_physical_layout()?;
        let observed_rows = scan.runtime_report().discovered_references().len() as u64;
        if observed_rows > request.budget_rows() {
            return Err(PhysicalStoreRuntimeDenial::new(
                PhysicalStoreRuntimeDenialKind::OfflineVerifierDenied,
            ));
        }
        Ok(PlatformPhysicalDegradedExactScanReceipt::new(
            request,
            observed_rows,
            scan.counters(),
        ))
    }
}

pub(crate) fn verify_persisted_layout_for_scan(
    storage: &PhysicalStoreRuntimeStorage,
    headers: &PhysicalHeaderAuthority,
) -> Result<MinimalManifestVerifierReport, PhysicalStoreRuntimeDenial> {
    OfflinePhysicalVerifier::for_canonical_physical_format(headers.clone())
        .verify(&storage.persisted_layout())
        .map_err(map_verifier_denial_for_reopen)
}

pub(crate) fn collect_runtime_layout_observation(
    storage: &PhysicalStoreRuntimeStorage,
) -> PlatformPhysicalRuntimeLayoutReport {
    PlatformPhysicalRuntimeLayoutReport::new(
        storage.runtime_discovered_references(),
        storage.runtime_traversal_report(),
    )
}

pub(crate) fn construct_scan_report(
    runtime_report: PlatformPhysicalRuntimeLayoutReport,
    verifier_report: MinimalManifestVerifierReport,
    counters: PhysicalStoreRuntimeCounterSnapshot,
    scope: RoadmapScope,
) -> PlatformPhysicalScanReport {
    PlatformPhysicalScanReport::new(runtime_report, verifier_report, counters, scope)
}
