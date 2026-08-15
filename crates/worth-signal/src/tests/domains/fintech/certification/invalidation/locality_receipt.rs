use super::performed_work_validation::require_expected_performed_work;
use super::{
    verified_locality_case_identity, ExpectedLocalityCounterRow, FinancialCanonicalCaseIdentity,
    FinancialLocalityExpectationManifest, FreshFinancialLocalityRecompute,
};
use crate::data::error::SignalError;
use crate::data::telemetry::{InvalidationPerformedCounter, SignalInvalidationRealizedCounters};
use crate::facade::DiagnosticsTier;
use crate::logic::planner::StageExecutor;
#[cfg(feature = "parallel")]
use crate::tests::domains::fintech::world::FinancialPerformedCanonicalWork;
use crate::tests::domains::fintech::world::{
    compile_financial_locality_world, compile_financial_locality_world_at_tier,
    FinancialLocalityScenario, FinancialWorldDefinition, LocalityLane, LocalityScaleTuple,
    LocalitySemanticOutputId,
};

#[derive(Clone, Debug)]
pub(in crate::tests::domains::fintech) struct FinancialLocalityCaseEvidence {
    scenario: FinancialLocalityScenario,
    scale: LocalityScaleTuple,
    lane: LocalityLane,
    identity: FinancialCanonicalCaseIdentity,
    counters: SignalInvalidationRealizedCounters,
    canonical_work_items: usize,
    #[cfg(feature = "parallel")]
    performed_work: FinancialPerformedCanonicalWork,
    necessary_evaluations: Vec<LocalitySemanticOutputId>,
    measurement: FinancialLocalityCaseMeasurement,
}

#[derive(Clone, Copy, Debug)]
pub(in crate::tests::domains::fintech) struct FinancialLocalityCaseMeasurement {
    seed: u64,
    elapsed: std::time::Duration,
    peak_batch_memory_items: u64,
}

impl FinancialLocalityCaseMeasurement {
    pub(in crate::tests::domains::fintech) const fn seed(self) -> u64 {
        self.seed
    }

    pub(in crate::tests::domains::fintech) const fn elapsed(self) -> std::time::Duration {
        self.elapsed
    }

    pub(in crate::tests::domains::fintech) const fn peak_batch_memory_items(self) -> u64 {
        self.peak_batch_memory_items
    }
}

impl FinancialLocalityCaseEvidence {
    pub(in crate::tests::domains::fintech) const fn scenario(&self) -> FinancialLocalityScenario {
        self.scenario
    }

    pub(in crate::tests::domains::fintech) const fn scale(&self) -> LocalityScaleTuple {
        self.scale
    }

    pub(in crate::tests::domains::fintech) const fn lane(&self) -> LocalityLane {
        self.lane
    }

    pub(in crate::tests::domains::fintech) const fn counters(
        &self,
    ) -> SignalInvalidationRealizedCounters {
        self.counters
    }

    pub(in crate::tests::domains::fintech) const fn canonical_work_items(&self) -> usize {
        self.canonical_work_items
    }

    #[cfg(feature = "parallel")]
    pub(in crate::tests::domains::fintech) fn performed_work(
        &self,
    ) -> &FinancialPerformedCanonicalWork {
        &self.performed_work
    }

    pub(in crate::tests::domains::fintech) fn necessary_evaluations(
        &self,
    ) -> &[LocalitySemanticOutputId] {
        &self.necessary_evaluations
    }

    pub(in crate::tests::domains::fintech) fn identity(&self) -> &FinancialCanonicalCaseIdentity {
        &self.identity
    }

    pub(in crate::tests::domains::fintech) const fn measurement(
        &self,
    ) -> FinancialLocalityCaseMeasurement {
        self.measurement
    }

    pub(super) fn into_identity(self) -> FinancialCanonicalCaseIdentity {
        self.identity
    }
}

pub(in crate::tests::domains::fintech) fn verify_locality_case(
    definition: FinancialWorldDefinition,
    trace_index: usize,
    diagnostics_tier: DiagnosticsTier,
    executor: StageExecutor,
) -> Result<FinancialLocalityCaseEvidence, SignalError> {
    let started = std::time::Instant::now();
    report_scheduled_step(&definition, trace_index, "compile", started.elapsed());
    let mut compiled = compile_financial_locality_world_at_tier(definition, diagnostics_tier)?;
    let seed = compiled.locality_definition().seed();
    report_scheduled_step(
        compiled.definition(),
        trace_index,
        "manifest",
        started.elapsed(),
    );
    let manifest = FinancialLocalityExpectationManifest::derive_for_trace(
        compiled.locality_definition(),
        &compiled.locality_definition().action_traces()[trace_index],
        compiled.locality_graph_instance(),
    );
    let fresh = FreshFinancialLocalityRecompute::run_for_trace(
        compiled.locality_definition(),
        &compiled.locality_definition().action_traces()[trace_index],
    );
    report_scheduled_step(
        compiled.definition(),
        trace_index,
        "execute",
        started.elapsed(),
    );
    let (observation, performed) =
        compiled.observe_locality_action_trace_with_executor(trace_index, executor)?;
    report_scheduled_step(
        compiled.definition(),
        trace_index,
        "validate",
        started.elapsed(),
    );
    validate_case_results(&compiled, &manifest, &fresh, &observation)?;
    report_scheduled_step(
        compiled.definition(),
        trace_index,
        "identity",
        started.elapsed(),
    );
    let identity = verified_locality_case_identity(
        compiled.locality_definition(),
        &manifest,
        diagnostics_tier,
        performed,
    )?;
    Ok(FinancialLocalityCaseEvidence {
        scenario: manifest.scenario(),
        scale: compiled.locality_definition().scale(),
        lane: compiled
            .locality_definition()
            .workload()
            .execution_posture(),
        identity,
        counters: observation.performed_counters,
        canonical_work_items: manifest.canonical_work().len(),
        #[cfg(feature = "parallel")]
        performed_work: observation.performed_work,
        necessary_evaluations: manifest.necessary_evaluations().iter().copied().collect(),
        measurement: FinancialLocalityCaseMeasurement {
            seed,
            elapsed: started.elapsed(),
            peak_batch_memory_items: observation
                .performed_counters
                .value(InvalidationPerformedCounter::PeakBatchMemoryItems),
        },
    })
}

fn report_scheduled_step(
    definition: &FinancialWorldDefinition,
    trace_index: usize,
    step: &'static str,
    elapsed: std::time::Duration,
) {
    if definition
        .locality()
        .is_some_and(|locality| locality.workload().execution_posture() == LocalityLane::Scheduled)
    {
        eprintln!(
            "M13 scheduled step: trace={trace_index} {step} elapsed_ms={}",
            elapsed.as_millis()
        );
    }
}

pub(super) fn validate_case_results(
    compiled: &crate::tests::domains::fintech::world::CompiledFinancialWorld,
    manifest: &FinancialLocalityExpectationManifest,
    fresh: &FreshFinancialLocalityRecompute,
    observation: &crate::tests::domains::fintech::world::FinancialLocalityRedObservation,
) -> Result<(), SignalError> {
    let committed = compiled.committed_locality_financial_values()?;
    if committed != *fresh.shocked_values() {
        let drift = committed
            .iter()
            .filter_map(|(output, actual)| {
                let expected = fresh.shocked_values().get(output)?;
                (actual != expected)
                    .then_some(format!("{output:?}: actual={actual}, expected={expected}"))
            })
            .take(8)
            .collect::<Vec<_>>();
        return Err(SignalError::internal(format!(
            "incremental locality truth differs from fresh financial recompute for {:?} {:?} {:?}: {}",
            manifest.scenario(),
            compiled.locality_definition().scale(),
            manifest.action_trace(),
            drift.join(", "),
        )));
    }
    if observation.evaluated_outputs != *manifest.necessary_evaluations() {
        let unexpected = observation
            .evaluated_outputs
            .difference(manifest.necessary_evaluations())
            .take(8)
            .collect::<Vec<_>>();
        let missing = manifest
            .necessary_evaluations()
            .difference(&observation.evaluated_outputs)
            .take(8)
            .collect::<Vec<_>>();
        return Err(SignalError::internal(format!(
            "performed locality evaluations differ from necessity for {:?} {:?} {:?}: unexpected={unexpected:?}, missing={missing:?}",
            manifest.scenario(),
            compiled.locality_definition().scale(),
            manifest.action_trace(),
        )));
    }
    require_expected_performed_work(manifest, &observation.performed_work)?;
    let expected = expected_counters(&manifest);
    let drifts = InvalidationPerformedCounter::ALL
        .into_iter()
        .filter_map(|counter| {
            let actual = observation.performed_counters.value(counter);
            let expected = expected.value(counter);
            (actual != expected)
                .then(|| format!("{} actual={actual} expected={expected}", counter.name()))
        })
        .collect::<Vec<_>>();
    if !drifts.is_empty() {
        return Err(SignalError::internal(format!(
            "performed locality counters drifted for {:?} {:?} {:?}: {}",
            manifest.scenario(),
            compiled.locality_definition().scale(),
            manifest.action_trace(),
            drifts.join(", ")
        )));
    }
    Ok(())
}

fn expected_counters(
    manifest: &FinancialLocalityExpectationManifest,
) -> SignalInvalidationRealizedCounters {
    let rows = ExpectedLocalityCounterRow::ALL;
    SignalInvalidationRealizedCounters::from_values(std::array::from_fn(|index| {
        manifest.counter_manifest().value(rows[index])
    }))
}

#[cfg(test)]
mod tests;
