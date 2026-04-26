use forge_relational::facade::runtime::RelationalRuntime;
use forge_runtime_bridge::facade::RuntimeBridge;
use serde_json::Value;

use crate::identity::hash_parts;
use crate::memory_workspace::{ForgeQueryMutationReceipt, ForgeQueryWorkspaceError};

use super::{ForgeQueryAuthorityLane, ForgeQueryWriteReceipt};

pub trait ForgeQueryIntentAuthorityAdapter {
    fn execute_intent(
        &mut self,
        bridge: &RuntimeBridge,
        relational_runtime: Option<&mut RelationalRuntime>,
        declaration: &ForgeQueryIntentDeclaration,
    ) -> Result<ForgeQueryIntentExecution, ForgeQueryWorkspaceError>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryIntentSourceLane {
    UserAuthored,
    EffectTriggered,
    PreviewLocal,
    BranchLocal,
    DerivedRuntime,
}

impl ForgeQueryIntentSourceLane {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UserAuthored => "user-authored",
            Self::EffectTriggered => "effect-triggered",
            Self::PreviewLocal => "preview-local",
            Self::BranchLocal => "branch-local",
            Self::DerivedRuntime => "derived-runtime",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryIntentDeclaration {
    name: String,
    strategy_name: String,
    strategy_version: String,
    input_contract: String,
    input: Value,
    source_lane: ForgeQueryIntentSourceLane,
    target_lane: ForgeQueryAuthorityLane,
}

impl ForgeQueryIntentDeclaration {
    pub fn strategy_commit(
        name: impl Into<String>,
        strategy_name: impl Into<String>,
        strategy_version: impl Into<String>,
        input_contract: impl Into<String>,
        input: Value,
    ) -> Self {
        Self {
            name: name.into(),
            strategy_name: strategy_name.into(),
            strategy_version: strategy_version.into(),
            input_contract: input_contract.into(),
            input,
            source_lane: ForgeQueryIntentSourceLane::UserAuthored,
            target_lane: ForgeQueryAuthorityLane::AuthoritativeTruth,
        }
    }

    pub fn with_source_lane(mut self, source_lane: ForgeQueryIntentSourceLane) -> Self {
        self.source_lane = source_lane;
        self
    }

    pub fn with_target_lane(mut self, target_lane: ForgeQueryAuthorityLane) -> Self {
        self.target_lane = target_lane;
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn strategy_name(&self) -> &str {
        &self.strategy_name
    }

    pub fn strategy_version(&self) -> &str {
        &self.strategy_version
    }

    pub fn input_contract(&self) -> &str {
        &self.input_contract
    }

    pub fn input(&self) -> &Value {
        &self.input
    }

    pub fn source_lane(&self) -> ForgeQueryIntentSourceLane {
        self.source_lane
    }

    pub fn target_lane(&self) -> ForgeQueryAuthorityLane {
        self.target_lane
    }

    pub fn input_digest(&self) -> String {
        let input = serde_json::to_string(&self.input)
            .unwrap_or_else(|error| format!("unserializable-intent-input:{error}"));
        hash_parts(&[
            "forge_query_intent_input_v1".to_string(),
            format!("name:{}", self.name),
            format!("strategy:{}", self.strategy_name),
            format!("version:{}", self.strategy_version),
            format!("contract:{}", self.input_contract),
            format!("input:{input}"),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentExecution {
    strategy_identity: String,
    strategy_version: String,
    strategy_descriptor_digest: String,
    canonical_input_digest: String,
    produced_mutation_digest: String,
    invariant_evidence: Vec<String>,
    mutation_receipt: ForgeQueryMutationReceipt,
}

impl ForgeQueryIntentExecution {
    pub fn admitted(
        strategy_identity: impl Into<String>,
        strategy_version: impl Into<String>,
        strategy_descriptor_digest: impl Into<String>,
        canonical_input_digest: impl Into<String>,
        produced_mutation_digest: impl Into<String>,
        invariant_evidence: impl IntoIterator<Item = impl Into<String>>,
        mutation_receipt: ForgeQueryMutationReceipt,
    ) -> Self {
        Self {
            strategy_identity: strategy_identity.into(),
            strategy_version: strategy_version.into(),
            strategy_descriptor_digest: strategy_descriptor_digest.into(),
            canonical_input_digest: canonical_input_digest.into(),
            produced_mutation_digest: produced_mutation_digest.into(),
            invariant_evidence: invariant_evidence.into_iter().map(Into::into).collect(),
            mutation_receipt,
        }
    }

    pub fn strategy_identity(&self) -> &str {
        &self.strategy_identity
    }

    pub fn strategy_version(&self) -> &str {
        &self.strategy_version
    }

    pub fn strategy_descriptor_digest(&self) -> &str {
        &self.strategy_descriptor_digest
    }

    pub fn canonical_input_digest(&self) -> &str {
        &self.canonical_input_digest
    }

    pub fn produced_mutation_digest(&self) -> &str {
        &self.produced_mutation_digest
    }

    pub fn invariant_evidence(&self) -> &[String] {
        &self.invariant_evidence
    }

    pub fn mutation_receipt(&self) -> &ForgeQueryMutationReceipt {
        &self.mutation_receipt
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentReceipt {
    intent_name: String,
    strategy_identity: String,
    strategy_version: String,
    strategy_descriptor_digest: String,
    canonical_input_digest: String,
    produced_mutation_digest: String,
    invariant_evidence: Vec<String>,
    source_lane: ForgeQueryIntentSourceLane,
    target_lane: ForgeQueryAuthorityLane,
    commit_identity: String,
    snapshot_token: String,
    affected_live_view_ids: Vec<String>,
    affected_derived_view_ids: Vec<String>,
    considered_computed_view_count: usize,
    considered_effect_count: usize,
    delivered_effect_count: usize,
    pending_write_intent_count: usize,
    receipt_digest: String,
}

impl ForgeQueryIntentReceipt {
    pub(crate) fn new(
        declaration: &ForgeQueryIntentDeclaration,
        execution: ForgeQueryIntentExecution,
        write_receipt: &ForgeQueryWriteReceipt,
    ) -> Self {
        let affected_live_view_ids = write_receipt.affected_live_view_ids().to_vec();
        let affected_derived_view_ids = write_receipt.affected_derived_view_ids().to_vec();
        let commit_identity = write_receipt.commit_identity().to_string();
        let snapshot_token = write_receipt.snapshot_token().to_string();
        let receipt_digest = hash_parts(&[
            "forge_query_intent_receipt_v1".to_string(),
            format!("intent:{}", declaration.name()),
            format!("strategy:{}", execution.strategy_identity()),
            format!("version:{}", execution.strategy_version()),
            format!("descriptor:{}", execution.strategy_descriptor_digest()),
            format!("input:{}", execution.canonical_input_digest()),
            format!("mutation:{}", execution.produced_mutation_digest()),
            format!("source:{}", declaration.source_lane().as_str()),
            format!("target:{}", declaration.target_lane()),
            format!("commit:{commit_identity}"),
            format!("snapshot:{snapshot_token}"),
            format!("live:{}", affected_live_view_ids.join("|")),
            format!("derived:{}", affected_derived_view_ids.join("|")),
        ]);
        Self {
            intent_name: declaration.name().to_string(),
            strategy_identity: execution.strategy_identity,
            strategy_version: execution.strategy_version,
            strategy_descriptor_digest: execution.strategy_descriptor_digest,
            canonical_input_digest: execution.canonical_input_digest,
            produced_mutation_digest: execution.produced_mutation_digest,
            invariant_evidence: execution.invariant_evidence,
            source_lane: declaration.source_lane(),
            target_lane: declaration.target_lane(),
            commit_identity,
            snapshot_token,
            affected_live_view_ids,
            affected_derived_view_ids,
            considered_computed_view_count: write_receipt.considered_computed_view_count(),
            considered_effect_count: write_receipt.considered_effect_count(),
            delivered_effect_count: write_receipt.delivered_effect_count(),
            pending_write_intent_count: write_receipt.pending_write_intent_count(),
            receipt_digest,
        }
    }

    pub fn intent_name(&self) -> &str {
        &self.intent_name
    }

    pub fn strategy_identity(&self) -> &str {
        &self.strategy_identity
    }

    pub fn strategy_version(&self) -> &str {
        &self.strategy_version
    }

    pub fn strategy_descriptor_digest(&self) -> &str {
        &self.strategy_descriptor_digest
    }

    pub fn canonical_input_digest(&self) -> &str {
        &self.canonical_input_digest
    }

    pub fn produced_mutation_digest(&self) -> &str {
        &self.produced_mutation_digest
    }

    pub fn invariant_evidence(&self) -> &[String] {
        &self.invariant_evidence
    }

    pub fn source_lane(&self) -> ForgeQueryIntentSourceLane {
        self.source_lane
    }

    pub fn target_lane(&self) -> ForgeQueryAuthorityLane {
        self.target_lane
    }

    pub fn commit_identity(&self) -> &str {
        &self.commit_identity
    }

    pub fn snapshot_token(&self) -> &str {
        &self.snapshot_token
    }

    pub fn affected_live_view_ids(&self) -> &[String] {
        &self.affected_live_view_ids
    }

    pub fn affected_derived_view_ids(&self) -> &[String] {
        &self.affected_derived_view_ids
    }

    pub fn considered_computed_view_count(&self) -> usize {
        self.considered_computed_view_count
    }

    pub fn considered_effect_count(&self) -> usize {
        self.considered_effect_count
    }

    pub fn delivered_effect_count(&self) -> usize {
        self.delivered_effect_count
    }

    pub fn pending_write_intent_count(&self) -> usize {
        self.pending_write_intent_count
    }

    pub fn receipt_digest(&self) -> &str {
        &self.receipt_digest
    }
}
