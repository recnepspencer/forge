use crate::data::error::SignalError;

#[cfg(feature = "parallel")]
use super::{verify_locality_case, FinancialCanonicalCaseIdentity, FinancialLocalityCaseEvidence};
#[cfg(feature = "parallel")]
use crate::tests::domains::fintech::world::strategy_work_projection;
#[cfg(feature = "parallel")]
use crate::tests::domains::fintech::world::{FinancialLocalityScenario, FinancialWorldDefinition};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg(not(feature = "parallel"))]
pub(in crate::tests::domains::fintech) struct MeasurementGap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) enum TraversalStrategyDecision {
    #[cfg(feature = "parallel")]
    CurrentStrategyCertified,
    #[cfg(not(feature = "parallel"))]
    InsufficientEvidence(MeasurementGap),
}

#[derive(Debug)]
pub(in crate::tests::domains::fintech) struct InvalidationStrategyReport {
    decision: TraversalStrategyDecision,
    #[cfg(feature = "parallel")]
    deterministic_case: Option<FinancialCanonicalCaseIdentity>,
    #[cfg(feature = "parallel")]
    optimized_case: Option<FinancialCanonicalCaseIdentity>,
    #[cfg(feature = "parallel")]
    canonical_work_items: u64,
}

impl InvalidationStrategyReport {
    pub(in crate::tests::domains::fintech) const fn decision(&self) -> TraversalStrategyDecision {
        self.decision
    }

    #[cfg(feature = "parallel")]
    pub(in crate::tests::domains::fintech) const fn canonical_work_items(&self) -> u64 {
        self.canonical_work_items
    }

    #[cfg(feature = "parallel")]
    pub(in crate::tests::domains::fintech) fn case_identities(
        &self,
    ) -> Option<(
        &FinancialCanonicalCaseIdentity,
        &FinancialCanonicalCaseIdentity,
    )> {
        self.deterministic_case
            .as_ref()
            .zip(self.optimized_case.as_ref())
    }
}

#[cfg(not(feature = "parallel"))]
pub(in crate::tests::domains::fintech) fn certify_current_strategy(
    _seed: u64,
) -> Result<InvalidationStrategyReport, SignalError> {
    Ok(InvalidationStrategyReport {
        decision: TraversalStrategyDecision::InsufficientEvidence(MeasurementGap),
    })
}

#[cfg(feature = "parallel")]
pub(in crate::tests::domains::fintech) fn certify_current_strategy(
    seed: u64,
) -> Result<InvalidationStrategyReport, SignalError> {
    let definition = || {
        FinancialWorldDefinition::dense_market_close(
            seed,
            1_000,
            crate::tests::domains::fintech::world::DensityRatio::FourInFive,
        )
    };
    let deterministic = verify_locality_case(
        definition(),
        0,
        crate::facade::DiagnosticsTier::Operational,
        crate::logic::planner::StageExecutor::Serial,
    )?;
    let optimized = verify_locality_case(
        definition(),
        0,
        crate::facade::DiagnosticsTier::Operational,
        crate::logic::planner::StageExecutor::balanced_parallel(),
    )?;
    certify_equivalent_streams(deterministic, optimized)
}

#[cfg(feature = "parallel")]
fn certify_equivalent_streams(
    deterministic: FinancialLocalityCaseEvidence,
    optimized: FinancialLocalityCaseEvidence,
) -> Result<InvalidationStrategyReport, SignalError> {
    if deterministic.scenario() != FinancialLocalityScenario::DenseMarketClose
        || deterministic.scenario() != optimized.scenario()
        || deterministic.scale() != optimized.scale()
    {
        return Err(SignalError::internal(
            "strategy comparison mixed financial case identity",
        ));
    }
    if strategy_work_projection(deterministic.performed_work())
        != strategy_work_projection(optimized.performed_work())
    {
        return Err(SignalError::internal(
            "strategies performed different canonical admitted work",
        ));
    }
    if deterministic.counters() != optimized.counters()
        || deterministic.necessary_evaluations() != optimized.necessary_evaluations()
        || deterministic.identity() != optimized.identity()
    {
        return Err(SignalError::internal(
            "strategies committed different performed truth or evidence",
        ));
    }
    let canonical_work_items = deterministic.performed_work().len() as u64;
    Ok(InvalidationStrategyReport {
        decision: TraversalStrategyDecision::CurrentStrategyCertified,
        deterministic_case: Some(deterministic.identity().clone()),
        optimized_case: Some(optimized.identity().clone()),
        canonical_work_items,
    })
}

#[cfg(all(test, feature = "parallel"))]
mod tests {
    use super::*;

    #[test]
    fn deterministic_and_optimized_modes_consume_the_same_canonical_work() {
        let report = certify_current_strategy(41).unwrap();
        assert_eq!(
            report.decision(),
            TraversalStrategyDecision::CurrentStrategyCertified
        );
        assert!(report.canonical_work_items() > 0);
        let (deterministic, optimized) = report.case_identities().unwrap();
        assert_eq!(deterministic, optimized);
    }

    #[test]
    fn strategy_report_rejects_a_mixed_financial_work_stream() {
        let deterministic = verify_locality_case(
            FinancialWorldDefinition::dense_market_close(
                41,
                1_000,
                crate::tests::domains::fintech::world::DensityRatio::OneInOneHundred,
            ),
            0,
            crate::facade::DiagnosticsTier::Operational,
            crate::logic::planner::StageExecutor::Serial,
        )
        .unwrap();
        let optimized = verify_locality_case(
            FinancialWorldDefinition::dense_market_close(
                41,
                1_000,
                crate::tests::domains::fintech::world::DensityRatio::FourInFive,
            ),
            0,
            crate::facade::DiagnosticsTier::Operational,
            crate::logic::planner::StageExecutor::balanced_parallel(),
        )
        .unwrap();

        assert!(certify_equivalent_streams(deterministic, optimized).is_err());
    }
}
