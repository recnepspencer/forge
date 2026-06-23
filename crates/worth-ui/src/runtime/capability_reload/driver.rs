use std::collections::BTreeSet;

use crate::capability::CapabilitySnapshot;
use crate::runtime::{
    WorthUiAdmittedCapabilityReloadBatch, WorthUiCapabilityPreparedReload,
    WorthUiCapabilityReloadEvidence, WorthUiCapabilityReloadFamilyKind,
    WorthUiCapabilityReloadFamilyRow, WorthUiCapabilityReloadRequest, WorthUiCapabilityReloadStage,
    WorthUiCapabilityReloadStatus, WorthUiRuntimeFactSet, WorthUiRuntimeHost,
};

use super::{
    WorthUiAppearanceDelta, WorthUiCapabilityFamilyDelta, WorthUiCapabilityReloadDenialCode,
    WorthUiCapabilityReloadFamilyCounters, WorthUiCommandDelta, WorthUiCommandProjectionDelta,
    WorthUiComponentDelta, WorthUiDensityDelta, WorthUiThemeTokenDelta,
};

impl WorthUiRuntimeHost {
    /// Prepares a transitional capability-family source adapter reload.
    ///
    /// This path remains for older proof slices that still lower family-local
    /// source text directly. Phase 23 establishes authored source-package
    /// ingress as the ordinary reload boundary; this adapter is explicitly not
    /// that boundary.
    pub fn prepare_capability_reload(
        &self,
        request: WorthUiCapabilityReloadRequest,
    ) -> WorthUiCapabilityPreparedReload {
        let before = self.inspect_active();
        let active_snapshot = self.active_state_for_read().capability_snapshot();
        let request_digest = request.source_digest();
        let requests = request.flattened();

        match admit_capability_reload_batch(active_snapshot, requests) {
            Ok(admitted) => self.prepare_admitted_capability_batch(
                before.snapshot_digest(),
                request_digest,
                admitted,
            ),
            Err(denial) => WorthUiCapabilityPreparedReload::new(
                self.instance_id().raw(),
                WorthUiCapabilityReloadEvidence::denied_for_family(
                    self.instance_id().raw(),
                    before.snapshot_digest(),
                    request_digest,
                    denial.family,
                    denial.stage,
                    denial.detail,
                    denial.denial_code,
                    denial.counters,
                ),
                None,
            ),
        }
    }

    fn prepare_admitted_capability_batch(
        &self,
        active_snapshot_digest_before: u64,
        request_digest: u64,
        admission: CapabilityReloadBatchAdmission,
    ) -> WorthUiCapabilityPreparedReload {
        let candidate_snapshot_digest = admission.candidate_snapshot.digest().as_u64();
        let status = if candidate_snapshot_digest == active_snapshot_digest_before {
            WorthUiCapabilityReloadStatus::EquivalentNoOp
        } else {
            WorthUiCapabilityReloadStatus::ReadyForFrameBoundary
        };
        let admitted_batch = WorthUiAdmittedCapabilityReloadBatch::new(
            admission.candidate_snapshot,
            admission.family_rows.clone(),
            admission.changed_facts.clone(),
            active_snapshot_digest_before,
            candidate_snapshot_digest,
        );
        let changed_facts = admitted_batch.changed_facts().clone();
        let evidence = WorthUiCapabilityReloadEvidence::from_family_rows(
            self.instance_id().raw(),
            status,
            active_snapshot_digest_before,
            Some(candidate_snapshot_digest),
            request_digest,
            admission.family_rows,
            changed_facts,
        );
        let admitted_batch = match status {
            WorthUiCapabilityReloadStatus::ReadyForFrameBoundary => Some(admitted_batch),
            _ => None,
        };
        WorthUiCapabilityPreparedReload::new(self.instance_id().raw(), evidence, admitted_batch)
    }
}

struct CapabilityReloadBatchAdmission {
    candidate_snapshot: CapabilitySnapshot,
    family_rows: Vec<WorthUiCapabilityReloadFamilyRow>,
    changed_facts: WorthUiRuntimeFactSet,
}

struct CapabilityReloadAdmissionDenial {
    family: WorthUiCapabilityReloadFamilyKind,
    stage: WorthUiCapabilityReloadStage,
    detail: String,
    denial_code: Option<WorthUiCapabilityReloadDenialCode>,
    counters: WorthUiCapabilityReloadFamilyCounters,
}

fn admit_capability_reload_batch(
    active_snapshot: &CapabilitySnapshot,
    requests: Vec<WorthUiCapabilityReloadRequest>,
) -> Result<CapabilityReloadBatchAdmission, CapabilityReloadAdmissionDenial> {
    reject_duplicate_families(&requests)?;
    let mut candidate_snapshot = active_snapshot.clone();
    let mut family_rows = Vec::new();
    let mut changed_facts = WorthUiRuntimeFactSet::empty();

    for request in requests {
        let request_digest = request.source_digest();
        let family_delta = derive_family_delta(&candidate_snapshot, &request)?;
        let (family, next_snapshot, counters, family_facts, component_reload_receipt) =
            family_delta.into_parts();
        let candidate_changed = next_snapshot.digest() != candidate_snapshot.digest();
        changed_facts.extend(family_facts.facts().cloned());
        family_rows.push(
            WorthUiCapabilityReloadFamilyRow::admitted_with_component_reload_receipt(
                family,
                request_digest,
                counters,
                candidate_changed,
                component_reload_receipt,
            ),
        );
        candidate_snapshot = next_snapshot;
    }

    Ok(CapabilityReloadBatchAdmission {
        candidate_snapshot,
        family_rows,
        changed_facts,
    })
}

fn reject_duplicate_families(
    requests: &[WorthUiCapabilityReloadRequest],
) -> Result<(), CapabilityReloadAdmissionDenial> {
    let mut seen_families = BTreeSet::new();
    for request in requests {
        let family = request.family_kind();
        if !seen_families.insert(family) {
            return Err(CapabilityReloadAdmissionDenial {
                family,
                stage: WorthUiCapabilityReloadStage::DuplicateCapabilityFamily,
                detail: format!("duplicate capability reload family `{}`", family.token()),
                denial_code: None,
                counters: WorthUiCapabilityReloadFamilyCounters::default(),
            });
        }
    }
    Ok(())
}

fn derive_family_delta(
    active_snapshot: &CapabilitySnapshot,
    request: &WorthUiCapabilityReloadRequest,
) -> Result<WorthUiCapabilityFamilyDelta, CapabilityReloadAdmissionDenial> {
    match request {
        WorthUiCapabilityReloadRequest::ThemeTokens(theme_tokens) => {
            WorthUiThemeTokenDelta::derive(active_snapshot, theme_tokens)
                .map(WorthUiThemeTokenDelta::into_family_delta)
                .map_err(|denial| CapabilityReloadAdmissionDenial {
                    family: WorthUiCapabilityReloadFamilyKind::ThemeTokens,
                    stage: denial.stage(),
                    detail: denial.detail(),
                    denial_code: None,
                    counters: WorthUiCapabilityReloadFamilyCounters::new(1, 0, 0, 0, 0, 0),
                })
        }
        WorthUiCapabilityReloadRequest::Commands(commands) => {
            WorthUiCommandDelta::derive(active_snapshot, commands)
                .map(WorthUiCommandDelta::into_family_delta)
                .map_err(|denial| CapabilityReloadAdmissionDenial {
                    family: WorthUiCapabilityReloadFamilyKind::Commands,
                    stage: denial.stage(),
                    detail: denial.detail(),
                    denial_code: None,
                    counters: WorthUiCapabilityReloadFamilyCounters::new(1, 0, 0, 0, 0, 0),
                })
        }
        WorthUiCapabilityReloadRequest::CommandProjections(projections) => {
            WorthUiCommandProjectionDelta::derive(active_snapshot, projections)
                .map(WorthUiCommandProjectionDelta::into_family_delta)
                .map_err(|denial| CapabilityReloadAdmissionDenial {
                    family: WorthUiCapabilityReloadFamilyKind::CommandProjections,
                    stage: denial.stage(),
                    detail: denial.detail(),
                    denial_code: None,
                    counters: WorthUiCapabilityReloadFamilyCounters::new(1, 0, 0, 0, 0, 0),
                })
        }
        WorthUiCapabilityReloadRequest::Components(components) => {
            WorthUiComponentDelta::derive(active_snapshot, components)
                .map(WorthUiComponentDelta::into_family_delta)
                .map_err(|denial| CapabilityReloadAdmissionDenial {
                    family: WorthUiCapabilityReloadFamilyKind::Components,
                    stage: denial.stage(),
                    detail: denial.detail(),
                    denial_code: None,
                    counters: denial.counters(),
                })
        }
        WorthUiCapabilityReloadRequest::Appearance(appearance) => {
            WorthUiAppearanceDelta::derive(active_snapshot, appearance)
                .map(WorthUiAppearanceDelta::into_family_delta)
                .map_err(|denial| CapabilityReloadAdmissionDenial {
                    family: WorthUiCapabilityReloadFamilyKind::Appearance,
                    stage: denial.stage(),
                    detail: denial.detail(),
                    denial_code: denial.denial_code(),
                    counters: denial.counters(),
                })
        }
        WorthUiCapabilityReloadRequest::Density(density) => {
            WorthUiDensityDelta::derive(active_snapshot, density)
                .map(WorthUiDensityDelta::into_family_delta)
                .map_err(|denial| CapabilityReloadAdmissionDenial {
                    family: WorthUiCapabilityReloadFamilyKind::Density,
                    stage: denial.stage(),
                    detail: denial.detail(),
                    denial_code: None,
                    counters: denial.counters(),
                })
        }
        WorthUiCapabilityReloadRequest::Batch(_) => unreachable!("batch requests are flattened"),
    }
}
