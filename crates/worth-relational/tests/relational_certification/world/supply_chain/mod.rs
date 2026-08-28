mod baseline_audit;
mod certified_baseline;
mod comparison;
mod comparison_ancestry;
mod comparison_entities;
mod comparison_relations;
mod comparison_schema;
mod comparison_state;
mod compiler;
mod definition;
mod definition_entities;
mod definition_relations;
mod delta;
mod expected_digest;
mod expected_observation;
mod handles;
mod observation;
mod observation_debug;
mod oracle;
mod production_world;
mod program;
mod program_schema;
mod read_footprint;
mod relation_comparison;
mod runtime_driver;
mod scale;
mod scenario_delta_vocabulary;
mod scenarios;
mod schema;
mod schema_validation;
mod schema_vocabulary;
mod semantic_key;
mod trace;

pub(crate) use baseline_audit::audit as audit_supply_chain_baseline;
pub(crate) use baseline_audit::BaselineAuditError;
pub(crate) use certified_baseline::{
    assert_oracle_matches, canonical_empty_supply_chain_runtime, certified_supply_chain_world,
};
pub(crate) use comparison::{compare, ComparisonMismatch, ObservedSupplyChainState};
pub(crate) use compiler::{
    compile_supply_chain_baseline, compile_supply_chain_baseline_with_budget,
    compile_supply_chain_baseline_with_budget_and_invariant_catalog_and_custom_invariants,
    compile_supply_chain_baseline_with_custom_invariant,
    compile_supply_chain_baseline_with_invariant_catalog, SupplyChainCompilationError,
};
pub(crate) use definition::SupplyChainWorldDefinition;
pub(crate) use delta::{
    DeltaId, DeltaIdentityBasis, DeltaPostcondition, DeltaPrecondition, SupplyChainScenarioDelta,
};
pub(crate) use expected_digest::{canonical_bytes, digest};
pub(crate) use expected_observation::ExpectedSupplyChainObservation;
pub(crate) use handles::{HandleBindingError, SupplyChainSemanticHandles};
pub(crate) use observation::{
    observe as observe_supply_chain, observe_observation as observe_supply_chain_observation,
    observe_snapshot as observe_supply_chain_snapshot, ObservationError,
};
pub(crate) use oracle::{
    apply, apply_from_parent, insert_vessel, next_vessel_key, reject_duplicate_relation,
    vessel_call_signs, AcceptedDelta, AncestryError, OracleAncestry, OracleApplicationError,
    OracleBranch, OracleState, UniqueEntityFieldOracleError,
};
pub(crate) use production_world::ProductionSeededSupplyChainWorld;
pub(crate) use program::{
    entity_kind_id, relation_client_key, relation_kind_id, CompiledSupplyChainProgram,
    SupplyChainProgramError,
};
pub(crate) use program_schema::schema_registry_with_altered_port_contract;
pub(crate) use read_footprint::DeltaReadFootprint;
pub(crate) use relation_comparison::validate_relation_vector;
pub(crate) use runtime_driver::{
    commit_branch_batch, commit_branch_batch_with_result, commit_main_batch,
    commit_supply_chain_delta, fork_supply_chain_branch_from_main, head_for_supply_chain_branch,
    head_for_supply_chain_identity, lower_cargo_footprint_batch, lower_hazard_v2_batch,
    lower_supply_chain_production_delta, snapshot_for_supply_chain_identity,
    SupplyChainProductionDeltaLoweringError,
};
pub(crate) use scale::{
    CostBudgetError, CostDimension, ScaleName, SupplyChainCostInputs, SupplyChainScale,
};
pub(crate) use scenarios::{
    hazard_v2_transition, BaselineName, BranchCreationIntent, BranchIntentError,
    RetentionObligationError, RetentionObligationKind, SupplyChainBaseline,
};
pub(crate) use schema::{
    BookingStatus, EntityRecord, HazardClass, InspectionResult, OperatingPosture, Region,
    RelationEdge, SchemaError, SchemaVersion, SupplyChainSchema, VesselClass, VoyageStatus,
};
pub(crate) use semantic_key::{
    AbsenceKind, Anchor, BranchLabel, EntityKey, EntityKind, FieldKey, RelationKey, RelationKind,
    SemanticPath,
};
pub(crate) use trace::{MutationId, MutationOperation, SemanticTrace, TraceReplayError};
