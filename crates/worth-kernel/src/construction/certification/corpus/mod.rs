mod cases;
mod closeout;
mod compound;
mod execution;
mod family_boundary_drift_report;
mod family_boundary_report;
mod ordering;
mod parity;
mod rejection_witnesses;
mod replay_siege;
mod replay_siege_builder;
mod replay_siege_report;
mod row_support;
mod simplex_exhaustion_witness_report;
mod simplex_ladder_report;
mod simplex_registry;

#[cfg(test)]
pub use compound::{
    prepare_primitive_construction_compound_milestone_closeout_report,
    PrimitiveConstructionCompoundAdversarialSiegeError,
    PrimitiveConstructionCompoundMilestoneCloseoutReport,
};
pub use family_boundary_report::{
    prepare_primitive_construction_family_boundary_report,
    PrimitiveConstructionFamilyBoundaryReport, PrimitiveConstructionFamilyBoundaryReportError,
};
pub use replay_siege::{
    prepare_primitive_construction_corpus_replay_siege, PrimitiveConstructionCorpusReplaySiegeError,
};
pub use replay_siege_report::{
    PrimitiveConstructionCorpusParameterRole, PrimitiveConstructionCorpusReplaySiegeReport,
    PrimitiveConstructionCorpusReplaySiegeRow,
};
#[cfg(test)]
pub use simplex_exhaustion_witness_report::{
    prepare_primitive_construction_simplex_realization_exhaustion_witness_report,
    PrimitiveConstructionSimplexRealizationExhaustionWitnessReport,
};
#[cfg(test)]
pub use simplex_ladder_report::{
    prepare_primitive_construction_simplex_realization_strategy_ladder_report,
    PrimitiveConstructionSimplexRealizationLadderReportError,
    PrimitiveConstructionSimplexRealizationStrategyLadderReport,
};
#[cfg(test)]
pub(crate) use simplex_registry::{
    required_simplex_exhaustion_witness_kinds, required_simplex_ladder_scenarios,
};

#[cfg(test)]
mod tests;
