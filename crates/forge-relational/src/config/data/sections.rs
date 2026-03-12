use serde::{Deserialize, Serialize};

use crate::diagnostics::data::RelationalDiagnosticsProfile;
use crate::history::data::{BranchId, HistoryRetentionClass, VersionGraphPolicy};
use crate::logic::commit::CommitAuthorityContract;
use crate::logic::planning::{PlanningContract, RelationalExecutionModel};
use crate::payloads::data::PayloadPolicy;
use crate::schema::data::RelationalSchemaRegistry;
use crate::symbols::data::{SymbolPolicy, SymbolTableSnapshot};
use crate::validation::data::InvariantCatalog;

use super::policies::{
    AdjacencyPolicy, CascadeDeletePolicy, CompiledLanePolicy, CrossContextPolicy,
    DurabilityPolicy, MvccConfig, PublicationConfig, RetentionPolicy, StorageLayoutConfig,
    VisibilityCachePolicy,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionConfig {
    pub runtime_name: String,
    pub execution_model: RelationalExecutionModel,
    pub planning: PlanningContract,
    pub commit_authority: CommitAuthorityContract,
    pub compiled_lane_policy: CompiledLanePolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticsConfig {
    pub profile: RelationalDiagnosticsProfile,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryConfig {
    pub version_graph_policy: VersionGraphPolicy,
    pub retention: HistoryRetentionClass,
    pub main_branch: BranchId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaConfig {
    pub registry: RelationalSchemaRegistry,
    pub invariant_catalog: InvariantCatalog,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentityConfig {
    pub symbol_policy: SymbolPolicy,
    pub symbol_table: SymbolTableSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageConfig {
    pub initial_entity_capacity: usize,
    pub initial_relation_capacity: usize,
    pub mvcc: MvccConfig,
    pub retention: RetentionPolicy,
    pub layout: StorageLayoutConfig,
    pub payload_policy: PayloadPolicy,
    pub adjacency_policy: AdjacencyPolicy,
    pub cross_context_policy: CrossContextPolicy,
    pub cascade_delete_policy: CascadeDeletePolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisibilityConfig {
    pub cache_policy: VisibilityCachePolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationSection {
    pub policy: PublicationConfig,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurabilityConfig {
    pub policy: DurabilityPolicy,
}
