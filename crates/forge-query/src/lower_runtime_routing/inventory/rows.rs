use super::{
    ForgeQueryLowerRuntimeDirectImportAudit, ForgeQueryLowerRuntimeDirectImportAuditRow,
    ForgeQueryLowerRuntimeDirectImportPosture, ForgeQueryLowerRuntimeGapRegistry,
    ForgeQueryLowerRuntimeGapRegistryRow, ForgeQueryLowerRuntimeSeamKey,
};

const GAP_ROWS: &[ForgeQueryLowerRuntimeGapRegistryRow] = &[];

const DIRECT_IMPORT_AUDIT_ROWS: &[ForgeQueryLowerRuntimeDirectImportAuditRow] = &[
    ForgeQueryLowerRuntimeDirectImportAuditRow::new(
        ForgeQueryLowerRuntimeSeamKey::RuntimeBackendBoundaryModules,
        "crates/forge-query/src/runtime/backend/*",
        ForgeQueryLowerRuntimeDirectImportPosture::RuntimeBackendBoundary,
        "backend-boundary modules may import lower-runtime facades because they are the canonical ordinary crossing hub",
    ),
    ForgeQueryLowerRuntimeDirectImportAuditRow::new(
        ForgeQueryLowerRuntimeSeamKey::HistoricalBridgeLoweringModule,
        "crates/forge-query/src/historical/bridge_lowering.rs",
        ForgeQueryLowerRuntimeDirectImportPosture::AllowedAdapter,
        "historical policy lowering remains an allowed Query boundary adapter",
    ),
    ForgeQueryLowerRuntimeDirectImportAuditRow::new(
        ForgeQueryLowerRuntimeSeamKey::EffectBackedRelationalMutation,
        "crates/forge-query/src/effect_lifecycle/execution.rs",
        ForgeQueryLowerRuntimeDirectImportPosture::AllowedAdapter,
        "effect-backed relational mutation execution remains an allowed Query boundary adapter over admitted relational authority",
    ),
    ForgeQueryLowerRuntimeDirectImportAuditRow::new(
        ForgeQueryLowerRuntimeSeamKey::EffectBackedRelationalMerge,
        "crates/forge-query/src/effect_lifecycle/execution.rs",
        ForgeQueryLowerRuntimeDirectImportPosture::AllowedAdapter,
        "effect-backed relational merge execution remains an allowed Query boundary adapter over admitted relational authority",
    ),
    ForgeQueryLowerRuntimeDirectImportAuditRow::new(
        ForgeQueryLowerRuntimeSeamKey::ProjectionConsumptionSourceModule,
        "crates/forge-query/src/projection_consumption/source/mod.rs",
        ForgeQueryLowerRuntimeDirectImportPosture::AllowedAdapter,
        "projection-consumption source intake remains an allowed Query boundary adapter",
    ),
    ForgeQueryLowerRuntimeDirectImportAuditRow::new(
        ForgeQueryLowerRuntimeSeamKey::CausalBuilderBridgeModule,
        "crates/forge-query/src/runtime/inspection/causal/builder_bridge.rs",
        ForgeQueryLowerRuntimeDirectImportPosture::AllowedAdapter,
        "causal bridge materialization remains an allowed Query boundary adapter",
    ),
    ForgeQueryLowerRuntimeDirectImportAuditRow::new(
        ForgeQueryLowerRuntimeSeamKey::FrontierSignalAdapterModule,
        "crates/forge-query/src/frontier_signal_adapter.rs",
        ForgeQueryLowerRuntimeDirectImportPosture::AllowedAdapter,
        "frontier evidence intake remains an allowed Query boundary adapter over signal facade receipts",
    ),
    ForgeQueryLowerRuntimeDirectImportAuditRow::new(
        ForgeQueryLowerRuntimeSeamKey::EffectExecutionBridgeModule,
        "crates/forge-query/src/effect_lifecycle/execution_bridge.rs",
        ForgeQueryLowerRuntimeDirectImportPosture::AllowedAdapter,
        "bridge writeback execution remains an allowed Query boundary adapter over the bridge admitted execution contract",
    ),
    ForgeQueryLowerRuntimeDirectImportAuditRow::new(
        ForgeQueryLowerRuntimeSeamKey::IntentRuntimeExecution,
        "crates/forge-query/src/runtime/backend/intent_authority.rs",
        ForgeQueryLowerRuntimeDirectImportPosture::AllowedAdapter,
        "installed intent authority remains an allowed backend-boundary adapter over runtime bridge and relational authority contracts",
    ),
    ForgeQueryLowerRuntimeDirectImportAuditRow::new(
        ForgeQueryLowerRuntimeSeamKey::DownstreamQueryRuntimeBoundarySubtree,
        "crates/worth-topo/src/projection/runtime_boundary/*",
        ForgeQueryLowerRuntimeDirectImportPosture::DownstreamRuntimeBoundarySubtree,
        "downstream Query-integrated projection code may import lower-runtime facades only inside the declared runtime-boundary subtree",
    ),
];

pub fn forge_query_lower_runtime_gap_registry() -> ForgeQueryLowerRuntimeGapRegistry {
    ForgeQueryLowerRuntimeGapRegistry::new(GAP_ROWS)
}

pub fn forge_query_lower_runtime_direct_import_audit() -> ForgeQueryLowerRuntimeDirectImportAudit {
    ForgeQueryLowerRuntimeDirectImportAudit::new(DIRECT_IMPORT_AUDIT_ROWS)
}
