mod baseline;
mod compiler;
mod definition;
mod fixture_compiler;
mod fixture_projection;
#[cfg(test)]
mod fixture_projection_tests;
mod market_inputs;
mod positions;
mod reference_finance;
mod semantic_projection;

pub(super) use baseline::{
    FinancialComparatorProfile, FinancialReproductionTuple, FinancialScaleTuple,
    FinancialScenarioIdentity,
};
pub(super) use compiler::{
    compile_financial_world, CompiledFinancialWorld, FinancialBranchLifecycleCompletion,
    FinancialDependencyRewireEvidence, FinancialEvaluationLedger, FinancialFactorSequenceEvidence,
    FinancialQuoteTranslationEvidence, FinancialSemanticHandles,
};
pub(super) use definition::{
    FinancialConditionPolicy, FinancialConsumerRole, FinancialWorldDefinition,
};
pub(super) use fixture_compiler::{compile_runtime_fixture, compile_unseeded_runtime_fixture};
pub(super) use fixture_projection::{
    FinancialFixtureProjection, FixtureAggregateState, FixtureMarketPoint, FixtureScenarioShock,
};
use market_inputs::{
    Currency, FinancialMarketInputs, FixedPrice, QuoteId, VolatilityBucket, FIXED_SCALE,
};
pub(super) use market_inputs::{CurveBucket, FxPair, MarketFactorKey};
use positions::FinancialPosition;
pub(super) use positions::PositionKind;
pub(super) use positions::{
    FinancialAspect, FinancialComparatorPolicy, FinancialOutputEquivalencePolicy, InstrumentId,
};
pub(super) use reference_finance::{
    reference_position_result, FinancialAmount, PositionFinancialResult,
};
pub(super) use semantic_projection::{
    FinancialEconomicSnapshot, FinancialSemanticProjection, SemanticOutputKey,
};
