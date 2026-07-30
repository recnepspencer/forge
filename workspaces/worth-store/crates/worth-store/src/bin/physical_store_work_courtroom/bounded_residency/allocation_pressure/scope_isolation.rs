use std::num::NonZeroU64;

use worth_store::physical_runtime::{
    CertificationScopeAdmissionFailure, CertificationScopePressure,
    PhysicalOperationAllocationScope as Scope, PhysicalResidencyCertification,
    PhysicalResidencyDimension, ServingPhysicalRuntime,
};

use super::super::configuration::BoundedResidencyConfiguration;
use super::ScopeIsolationEvidence;

const SCOPES: [Scope; 7] = [
    Scope::ForegroundRead,
    Scope::ForegroundWrite,
    Scope::Recovery,
    Scope::Scrub,
    Scope::Maintenance,
    Scope::Verification,
    Scope::Blob,
];
const GLOBAL_FILL_SCOPES: [Scope; 4] = [
    Scope::Recovery,
    Scope::Scrub,
    Scope::Maintenance,
    Scope::Verification,
];
const GLOBAL_DENIAL_SCOPE: Scope = Scope::Blob;

struct IndividualScopeEvidence {
    denials: u32,
    all_effect_free: bool,
}

struct GlobalEnvelopeEvidence {
    requested: u64,
    current: u64,
    limit: u64,
    effect_free: bool,
}

pub(super) fn prove(
    serving: &ServingPhysicalRuntime,
    configuration: BoundedResidencyConfiguration,
) -> Result<ScopeIsolationEvidence, String> {
    let certification = serving.certification_physical_residency();
    let individual = prove_individual_scopes(&certification, configuration)?;
    let global = prove_combined_envelope(&certification, configuration)?;
    let counters = require_terminal_release(serving)?;
    Ok(ScopeIsolationEvidence {
        admitted_scopes: SCOPES.len() as u32,
        exact_scope_denials: individual.denials,
        global_envelope_denied: true,
        global_denial_requested: global.requested,
        global_denial_current: global.current,
        global_denial_limit: global.limit,
        peak_operation_bytes: counters.peak_operation_bytes(),
        terminal_operation_bytes: counters.active_operation_bytes(),
        all_effect_free: individual.all_effect_free && global.effect_free,
    })
}

fn prove_individual_scopes(
    certification: &PhysicalResidencyCertification,
    configuration: BoundedResidencyConfiguration,
) -> Result<IndividualScopeEvidence, String> {
    let mut evidence = IndividualScopeEvidence {
        denials: 0,
        all_effect_free: true,
    };
    for scope in SCOPES {
        evidence.all_effect_free &= prove_scope(certification, configuration, scope)?;
        evidence.denials = evidence.denials.saturating_add(1);
    }
    Ok(evidence)
}

fn prove_scope(
    certification: &PhysicalResidencyCertification,
    configuration: BoundedResidencyConfiguration,
    scope: Scope,
) -> Result<bool, String> {
    let ceiling = configuration.scope_bytes(scope);
    let held = certification
        .admit_operation_scope(scope, nonzero(ceiling)?)
        .map_err(|failure| format!("C.6 {scope:?} ceiling admission failed: {failure:?}"))?;
    if held.scope() != scope || held.bytes() != ceiling {
        return Err(format!(
            "C.6 {scope:?} admission returned foreign scope truth"
        ));
    }
    let pressure = denied_pressure(
        certification.admit_operation_scope(scope, NonZeroU64::MIN),
        "one-past-scope",
    )?;
    if pressure.dimension() != PhysicalResidencyDimension::OperationScope(scope)
        || pressure.scope() != scope
        || pressure.requested() != 1
        || pressure.current() != ceiling
        || pressure.limit() != ceiling
    {
        return Err(format!(
            "C.6 {scope:?} one-past denial reported foreign pressure"
        ));
    }
    Ok(!pressure.effect_may_have_started())
}

fn prove_combined_envelope(
    certification: &PhysicalResidencyCertification,
    configuration: BoundedResidencyConfiguration,
) -> Result<GlobalEnvelopeEvidence, String> {
    let fill = exact_global_fill(configuration)?;
    let mut held = Vec::with_capacity(fill.len());
    for (scope, bytes) in fill {
        held.push(
            certification
                .admit_operation_scope(scope, bytes)
                .map_err(|failure| {
                    format!("C.6 {scope:?} envelope admission failed: {failure:?}")
                })?,
        );
    }
    let pressure = denied_pressure(
        certification.admit_operation_scope(GLOBAL_DENIAL_SCOPE, NonZeroU64::MIN),
        "one-past-global",
    )?;
    if pressure.dimension() != PhysicalResidencyDimension::OperationBytes
        || pressure.scope() != GLOBAL_DENIAL_SCOPE
        || pressure.requested() != 1
        || pressure.current() != configuration.operation_bytes()
        || pressure.limit() != configuration.operation_bytes()
    {
        return Err("C.6 combined scope pressure did not reach the global envelope".to_owned());
    }
    drop(held);
    Ok(GlobalEnvelopeEvidence {
        requested: pressure.requested(),
        current: pressure.current(),
        limit: pressure.limit(),
        effect_free: !pressure.effect_may_have_started(),
    })
}

fn exact_global_fill(
    configuration: BoundedResidencyConfiguration,
) -> Result<Vec<(Scope, NonZeroU64)>, String> {
    let ceilings = GLOBAL_FILL_SCOPES.map(|scope| (scope, configuration.scope_bytes(scope)));
    plan_global_fill(configuration.operation_bytes(), ceilings)
}

fn plan_global_fill(
    operation_bytes: u64,
    ceilings: [(Scope, u64); GLOBAL_FILL_SCOPES.len()],
) -> Result<Vec<(Scope, NonZeroU64)>, String> {
    let mut remaining = operation_bytes;
    let mut plan = Vec::with_capacity(ceilings.len());
    for (scope, ceiling) in ceilings {
        let requested = remaining.min(ceiling);
        if let Some(requested) = NonZeroU64::new(requested) {
            plan.push((scope, requested));
            remaining -= requested.get();
        }
    }
    if remaining != 0 {
        return Err(format!(
            "C.6 configured scopes cannot fill the global operation envelope: remaining={remaining}"
        ));
    }
    Ok(plan)
}

fn require_terminal_release(
    serving: &ServingPhysicalRuntime,
) -> Result<worth_store::physical_runtime::PhysicalResidencyCounterSnapshot, String> {
    let counters = serving.residency_observation().counters();
    if counters.active_operation_bytes() != 0
        || SCOPES
            .iter()
            .any(|scope| counters.active_operation_bytes_for(*scope) != 0)
    {
        return Err("C.6 scope pressure leaked live operation allocation".to_owned());
    }
    Ok(counters)
}

fn denied_pressure(
    outcome: Result<
        worth_store::physical_runtime::CertificationScopedAllocation,
        CertificationScopeAdmissionFailure,
    >,
    posture: &str,
) -> Result<CertificationScopePressure, String> {
    match outcome {
        Ok(_) => Err(format!("C.6 {posture} allocation unexpectedly admitted")),
        Err(CertificationScopeAdmissionFailure::Pressure(pressure)) => Ok(pressure),
        Err(CertificationScopeAdmissionFailure::Residency(other)) => Err(format!(
            "C.6 {posture} admission failed without pressure: {other:?}"
        )),
    }
}

fn nonzero(value: u64) -> Result<NonZeroU64, String> {
    NonZeroU64::new(value).ok_or_else(|| "C.6 scope ceiling was zero".to_owned())
}

#[cfg(test)]
mod tests {
    use super::{plan_global_fill, Scope, GLOBAL_FILL_SCOPES};

    #[test]
    fn global_fill_plan_saturates_the_hostile_envelope_and_reserves_blob() {
        let ceilings = [
            (Scope::Recovery, 2_359_296),
            (Scope::Scrub, 1_835_008),
            (Scope::Maintenance, 1_572_864),
            (Scope::Verification, 1_048_576),
        ];
        let plan = plan_global_fill(6_815_744, ceilings).unwrap();
        assert_eq!(
            plan.iter()
                .map(|(scope, bytes)| (*scope, bytes.get()))
                .collect::<Vec<_>>(),
            vec![
                (Scope::Recovery, 2_359_296),
                (Scope::Scrub, 1_835_008),
                (Scope::Maintenance, 1_572_864),
                (Scope::Verification, 1_048_576),
            ]
        );
        assert!(!plan.iter().any(|(scope, _)| *scope == Scope::Blob));
    }

    #[test]
    fn global_fill_plan_rejects_insufficient_combined_scope_capacity() {
        let ceilings = GLOBAL_FILL_SCOPES.map(|scope| (scope, 1));
        assert!(plan_global_fill(5, ceilings).is_err());
    }
}
