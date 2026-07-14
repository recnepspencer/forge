use super::{DegradedScanLoweringBasis, DegradedScanReady, StaleDegradedExactScan};
use crate::access::execution::DegradedScanAdmissionDenied;
use crate::planning::{AccessPlanIdentity, SelectedDegradedExactScan};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DegradedScanRebindTrace {
    stale_plan: AccessPlanIdentity,
    replacement_plan: AccessPlanIdentity,
}

impl DegradedScanRebindTrace {
    pub const fn stale_plan(&self) -> &AccessPlanIdentity {
        &self.stale_plan
    }

    pub const fn replacement_plan(&self) -> &AccessPlanIdentity {
        &self.replacement_plan
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DegradedScanRebindAdmission {
    stale_basis: DegradedScanLoweringBasis,
    replacement_plan: AccessPlanIdentity,
    current: crate::CurrentLayoutMaterialization,
    trace: DegradedScanRebindTrace,
}

impl DegradedScanRebindAdmission {
    pub(in crate::access::execution::degraded_scan) fn into_ready_basis(
        self,
    ) -> (crate::CurrentLayoutMaterialization, DegradedScanRebindTrace) {
        (self.current, self.trace)
    }
}

pub(super) fn admit(
    stale: &StaleDegradedExactScan,
    replacement: &SelectedDegradedExactScan,
) -> Result<DegradedScanRebindAdmission, DegradedScanAdmissionDenied> {
    validate_equivalent_request(stale, replacement)?;
    let replacement_materialization = replacement.materialization().clone();
    let expected_frontier = stale.stale_materialization().observed_frontier().clone();
    let current = replacement_materialization
        .clone()
        .require_current_at(expected_frontier)
        .map_err(
            |_| DegradedScanAdmissionDenied::ReplacementFrontierMismatch {
                basis: stale.basis().clone(),
                expected: Box::new(
                    stale
                        .stale_materialization()
                        .observed_frontier()
                        .source()
                        .clone(),
                ),
                actual: Box::new(replacement_materialization.source().clone()),
            },
        )?;
    let trace = DegradedScanRebindTrace {
        stale_plan: stale.selected().fingerprint().clone(),
        replacement_plan: replacement.fingerprint().clone(),
    };
    Ok(DegradedScanRebindAdmission {
        stale_basis: stale.basis().clone(),
        replacement_plan: replacement.fingerprint().clone(),
        current,
        trace,
    })
}

pub(super) fn rebind(
    stale: StaleDegradedExactScan,
    replacement: SelectedDegradedExactScan,
    admission: DegradedScanRebindAdmission,
) -> Result<DegradedScanReady, DegradedScanAdmissionDenied> {
    if stale.basis() != &admission.stale_basis
        || replacement.fingerprint() != &admission.replacement_plan
    {
        return Err(DegradedScanAdmissionDenied::RebindAdmissionMismatch {
            basis: stale.basis().clone(),
            expected_replacement: replacement.fingerprint().clone(),
            admitted_replacement: admission.replacement_plan,
        });
    }
    validate_equivalent_request(&stale, &replacement)?;
    Ok(super::admit_rebound_ready(
        super::lower(replacement),
        admission,
    ))
}

fn validate_equivalent_request(
    stale: &StaleDegradedExactScan,
    replacement: &SelectedDegradedExactScan,
) -> Result<(), DegradedScanAdmissionDenied> {
    let stale_selected = stale.selected();
    if stale_selected.admitted_family() != replacement.admitted_family() {
        return Err(DegradedScanAdmissionDenied::ReplacementAuthorityMismatch {
            basis: stale.basis().clone(),
            expected: stale_selected.admitted_family(),
            actual: replacement.admitted_family(),
        });
    }
    if stale_selected.request_identity() != replacement.request_identity() {
        return Err(DegradedScanAdmissionDenied::ReplacementRequestMismatch {
            basis: stale.basis().clone(),
            expected: Box::new(stale_selected.request_identity()),
            actual: Box::new(replacement.request_identity()),
        });
    }
    if stale_selected.intent() != replacement.intent() {
        return Err(DegradedScanAdmissionDenied::ReplacementIntentMismatch {
            basis: stale.basis().clone(),
            expected: stale_selected.intent(),
            actual: replacement.intent(),
        });
    }
    Ok(())
}
