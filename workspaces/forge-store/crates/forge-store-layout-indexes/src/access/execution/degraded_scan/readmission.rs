use forge_proof::raw::{
    CheckedReadmitLoweredForExecutionReadyTransition, ContextualTransition,
    LoweredReadmissionContext, LoweredReadmissionReadiness, TransitionOutcome,
};

use super::{DegradedScanLoweringBasis, DegradedScanReady, StaleDegradedExactScan};
use crate::access::execution::transition_authority::{
    readiness_authority, readmission_authority, ExecutionReadinessAuthority,
    ExecutionReadinessDeferred, ReadmissionAuthority,
};
use crate::access::execution::DegradedScanAdmissionDenied;
use crate::materialization::{CurrentLayoutMaterialization, LayoutCoverageWitness};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DegradedScanReadmission {
    basis: DegradedScanLoweringBasis,
    current: CurrentLayoutMaterialization,
}

pub(super) fn admit_stale(
    stale: &StaleDegradedExactScan,
    current: CurrentLayoutMaterialization,
) -> Result<DegradedScanReadmission, DegradedScanAdmissionDenied> {
    validate_current(stale.selected(), stale.basis(), &current)?;
    Ok(DegradedScanReadmission {
        basis: stale.basis().clone(),
        current,
    })
}

pub(super) fn readmit(
    stale: StaleDegradedExactScan,
    admission: DegradedScanReadmission,
) -> Result<DegradedScanReady, DegradedScanAdmissionDenied> {
    validate_admission(
        stale.basis(),
        stale.selected(),
        &admission.basis,
        &admission.current,
    )?;
    let outcome = CheckedReadmitLoweredForExecutionReadyTransition.transition(
        stale.recipe(),
        LoweredReadmissionReadiness::<
            crate::planning::SelectedDegradedExactScan,
            DegradedScanLoweringBasis,
            DegradedScanLoweringBasis,
            ReadmissionAuthority,
            &'static str,
            ExecutionReadinessAuthority,
            DegradedScanAdmissionDenied,
            ExecutionReadinessDeferred,
            ExecutionReadinessDeferred,
        >::ready(LoweredReadmissionContext::new(
            stale.basis().clone(),
            readmission_authority(),
            "degraded-scan-readmitted-ready",
            readiness_authority(),
        )),
    );
    match outcome {
        TransitionOutcome::Success(recipe) => {
            Ok(DegradedScanReady::from_recipe(recipe, admission.current))
        }
        TransitionOutcome::Denied(denial) => Err(denial),
        _ => unreachable!("admitted degraded scan evidence resolves to current readiness"),
    }
}

fn validate_admission(
    basis: &DegradedScanLoweringBasis,
    selected: &crate::planning::SelectedDegradedExactScan,
    actual_basis: &DegradedScanLoweringBasis,
    current: &CurrentLayoutMaterialization,
) -> Result<(), DegradedScanAdmissionDenied> {
    if basis != actual_basis {
        return Err(DegradedScanAdmissionDenied::ReadmissionWitnessMismatch {
            basis: basis.clone(),
            expected: basis.clone(),
            actual: actual_basis.clone(),
        });
    }
    let expected = exact_coverage(selected);
    let coverage = current.materialization().coverage();
    if &expected != coverage {
        return Err(
            DegradedScanAdmissionDenied::ReadmissionCurrentCoverageMismatch {
                basis: basis.clone(),
                expected,
                actual: coverage.clone(),
            },
        );
    }
    Ok(())
}

fn validate_current(
    selected: &crate::planning::SelectedDegradedExactScan,
    basis: &DegradedScanLoweringBasis,
    current: &CurrentLayoutMaterialization,
) -> Result<(), DegradedScanAdmissionDenied> {
    let materialization = current.materialization();
    let exact = materialization.coverage();
    let expected = exact_coverage(selected);
    let family = materialization.family();
    let actual_family = family.declaration().family();
    if actual_family != expected.family() || exact.family() != expected.family() {
        return Err(DegradedScanAdmissionDenied::LifecycleFamilyMismatch {
            basis: basis.clone(),
            expected: expected.family(),
            actual: actual_family,
        });
    }
    let expected_authority = selected.admitted_family().security_identity();
    let expected_store = selected.admitted_family().authority_identity();
    if family.security_identity() != expected_authority
        || family.authority_identity() != expected_store
    {
        return Err(
            DegradedScanAdmissionDenied::ArtifactFamilyAuthorityMismatch {
                basis: basis.clone(),
                expected_security: expected_authority,
                actual_security: family.security_identity(),
                expected_store,
                actual_store: family.authority_identity(),
            },
        );
    }
    if exact != &expected {
        return Err(DegradedScanAdmissionDenied::CurrentCoverageMismatch {
            basis: basis.clone(),
            expected,
            actual: exact.clone(),
        });
    }
    Ok(())
}

fn exact_coverage(selected: &crate::planning::SelectedDegradedExactScan) -> LayoutCoverageWitness {
    selected
        .materialization()
        .expect("degraded scan retains admitted materialization")
        .coverage()
        .require_exact()
        .expect("degraded scan retains exact admitted coverage")
}
