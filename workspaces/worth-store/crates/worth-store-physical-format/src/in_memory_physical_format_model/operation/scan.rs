use crate::{
    InMemoryPhysicalFormatModelCounterSnapshot, MinimalManifestVerifierReport,
    OfflinePhysicalVerifier, PhysicalHeaderAuthority, PlatformPhysicalModelLayoutReport,
    PlatformPhysicalScanReport,
};
use worth_store_contracts::RoadmapScope;

use super::denials::InMemoryPhysicalFormatModelDenial;
use super::restore::map_verifier_denial_for_restore;
use super::storage::InMemoryPhysicalFormatModelStorage;
use super::{
    InMemoryPhysicalFormatModel, InMemoryPhysicalFormatModelDenialKind,
    PlatformPhysicalDegradedExactScanReceipt, PlatformPhysicalHiddenScanDenialReceipt,
    PlatformPhysicalLayoutAccessIntent, PlatformPhysicalLayoutAccessRequest,
};

impl InMemoryPhysicalFormatModel {
    pub fn execute_admitted_degraded_exact_scan(
        &mut self,
        ready: super::PlatformPhysicalDegradedExactScanReady,
    ) -> Result<
        super::PlatformPhysicalDegradedExecutionObservation,
        InMemoryPhysicalFormatModelDenial,
    > {
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
    ) -> Result<PlatformPhysicalScanReport, InMemoryPhysicalFormatModelDenial> {
        let verifier_report = verify_persisted_layout_for_scan(&self.storage, &self.headers)?;
        let model_report = collect_model_layout_observation(&self.storage);
        self.counters = self.counters.with_scan_materializations().with_scan();
        Ok(construct_scan_report(
            model_report,
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
    ) -> Result<PlatformPhysicalDegradedExactScanReceipt, InMemoryPhysicalFormatModelDenial> {
        if !matches!(
            request.intent(),
            PlatformPhysicalLayoutAccessIntent::ExplicitDegradedExactScan
        ) || request.budget_rows() == 0
        {
            return Err(InMemoryPhysicalFormatModelDenial::new(
                InMemoryPhysicalFormatModelDenialKind::OfflineVerifierDenied,
            ));
        }
        let scan = self.scan_physical_layout()?;
        let observed_rows = scan.model_report().discovered_references().len() as u64;
        if observed_rows > request.budget_rows() {
            return Err(InMemoryPhysicalFormatModelDenial::new(
                InMemoryPhysicalFormatModelDenialKind::OfflineVerifierDenied,
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
    storage: &InMemoryPhysicalFormatModelStorage,
    headers: &PhysicalHeaderAuthority,
) -> Result<MinimalManifestVerifierReport, InMemoryPhysicalFormatModelDenial> {
    OfflinePhysicalVerifier::for_canonical_physical_format(headers.clone())
        .verify(&storage.persisted_layout())
        .map_err(map_verifier_denial_for_restore)
}

pub(crate) fn collect_model_layout_observation(
    storage: &InMemoryPhysicalFormatModelStorage,
) -> PlatformPhysicalModelLayoutReport {
    PlatformPhysicalModelLayoutReport::new(
        storage.model_discovered_references(),
        storage.model_traversal_report(),
    )
}

pub(crate) fn construct_scan_report(
    model_report: PlatformPhysicalModelLayoutReport,
    verifier_report: MinimalManifestVerifierReport,
    counters: InMemoryPhysicalFormatModelCounterSnapshot,
    scope: RoadmapScope,
) -> PlatformPhysicalScanReport {
    PlatformPhysicalScanReport::new(model_report, verifier_report, counters, scope)
}
