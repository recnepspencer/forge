mod baseline;
mod compiler;
mod definition;
mod fixture_compiler;
mod fixture_projection;
#[cfg(test)]
mod fixture_projection_tests;
mod locality_definition;
mod locality_scale;
mod market_inputs;
mod positions;
mod reference_finance;
mod semantic_projection;

pub(super) use baseline::{
    FinancialComparatorProfile, FinancialReproductionTuple, FinancialScaleTuple,
    FinancialScenarioIdentity,
};
pub(super) use compiler::{
    compile_financial_locality_world, compile_financial_world, CompiledFinancialWorld,
    FinancialBranchLifecycleCompletion, FinancialDependencyRewireEvidence,
    FinancialEvaluationLedger, FinancialFactorSequenceEvidence, FinancialLocalityRedObservation,
    FinancialQuoteTranslationEvidence, FinancialSemanticHandles,
};
pub(super) use definition::{
    FinancialConditionPolicy, FinancialConsumerRole, FinancialWorldDefinition,
};
pub(super) use fixture_compiler::{compile_runtime_fixture, compile_unseeded_runtime_fixture};
pub(super) use fixture_projection::{
    FinancialFixtureProjection, FixtureAggregateState, FixtureMarketPoint, FixtureScenarioShock,
};
pub(super) use locality_definition::{
    FinancialLocalityDefinition, FinancialLocalityFormula, FinancialLocalityMutation,
    FinancialLocalityOutput, LocalityScope, LocalitySemanticOutputId,
};
pub(super) use locality_scale::{
    ordinary_locality_cases, scheduled_locality_cases, FinancialLocalityScenario,
    LocalityCaseContract, LocalityScaleTuple, SparseFanoutAxis,
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
