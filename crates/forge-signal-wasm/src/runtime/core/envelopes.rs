use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use forge_signal::facade::adapters::{
    branch_state_proof_report, merge_plan_proof_report, merge_result_proof_report,
    replay_artifact_proof_report, replay_parity_proof_report, runtime_proof_report,
    BranchStateProofReport, MergePlanProofReport, MergeResultProofReport, ReplayArtifactProofInput,
    ReplayArtifactProofReport, ReplayParityProofReport, RuntimeProofReport,
    BRANCH_STATE_PROOF_BASIS_VERSION,
};
use forge_signal::facade::adapters::{BranchMergePlan, BranchMergeResult};
use forge_signal::facade::history::RuntimeBranchId;

use crate::boundary::errors::ForgeSignalJsError;
use crate::recipe::model::SourceSpec;
use crate::runtime::adapters::{
    RuntimeDefinitionEnvelope, RuntimeEnvelope, UnavailableCallbackArtifact,
};
use crate::runtime::summaries::RuntimeSnapshotEnvelope;

use super::merge::build_branch_state_proof_basis;
use super::state::{
    BranchRuntimeState, CallbackDiagnosticState, CatalogEntry, DenseGridFamily, RuntimeStore,
    StoredRecipeDefinition, WebRuntimeMetrics, WebSignalKind,
};
use super::RuntimeCore;

#[derive(Clone)]
pub(crate) struct ExactRuntimeRestoreArtifact {
    snapshot: RuntimeSnapshotEnvelope,
    store: RuntimeStore,
    callback_diagnostics: BTreeMap<String, CallbackDiagnosticState>,
    catalog: BTreeMap<String, CatalogEntry>,
    web_signals: BTreeMap<String, WebSignalKind>,
    nodes_by_id: BTreeMap<forge_signal::facade::NodeId, String>,
    dense_grids: BTreeMap<String, Arc<DenseGridFamily>>,
    branch_states: BTreeMap<u64, BranchRuntimeState>,
    snapshot_states: BTreeMap<u64, BranchRuntimeState>,
    runtime_snapshots: BTreeMap<u64, forge_signal::facade::history::RuntimeSnapshot>,
    policy: crate::runtime::policy::RuntimePolicySpec,
    web_metrics: WebRuntimeMetrics,
}

const CALLBACK_UNAVAILABLE_FOR_PORTABLE_EXPORT: &str =
    "computeCallbackUnavailableForPortableExport";
#[cfg_attr(not(test), allow(dead_code))]
const CALLBACK_UNAVAILABLE_FOR_RUNTIME_ENVELOPE_IMPORT: &str =
    "computeCallbackUnavailableForRuntimeEnvelopeImport";

impl RuntimeCore {
    pub fn export_definitions(&mut self) -> Result<RuntimeDefinitionEnvelope, ForgeSignalJsError> {
        let store = self.lock_store()?;
        let sources = store
            .sources
            .iter()
            .map(|(id, source)| SourceSpec {
                id: id.clone(),
                initial: source.value.clone(),
                produces_aspects: self.catalog.get(id).map(|entry| {
                    entry
                        .produced_aspects
                        .iter()
                        .map(|aspect| aspect.id())
                        .collect()
                }),
            })
            .collect();
        let recipes = store
            .recipes
            .values()
            .filter_map(|recipe| recipe.definition.exportable_spec().cloned())
            .collect();
        let source_families = store
            .source_families
            .values()
            .map(|family| family.spec.clone())
            .collect();
        let recipe_families = store
            .recipe_families
            .values()
            .map(|family| family.spec.clone())
            .collect();
        let unavailable_callbacks: Vec<UnavailableCallbackArtifact> = store
            .recipes
            .iter()
            .filter_map(|(id, recipe)| match &recipe.definition {
                StoredRecipeDefinition::Expr(_) => None,
                StoredRecipeDefinition::Callback(callback) => Some(UnavailableCallbackArtifact {
                    id: id.clone(),
                    signal_kind: self
                        .web_signals
                        .get(id)
                        .map(|kind| match kind {
                            super::state::WebSignalKind::Input => "input".to_owned(),
                            super::state::WebSignalKind::Computed => "computed".to_owned(),
                            super::state::WebSignalKind::Output => "output".to_owned(),
                        })
                        .unwrap_or_else(|| "computed".to_owned()),
                    reason: CALLBACK_UNAVAILABLE_FOR_PORTABLE_EXPORT.to_owned(),
                    current_reads: callback
                        .reads
                        .iter()
                        .map(|read| read.id().to_owned())
                        .collect(),
                }),
            })
            .collect();
        drop(store);
        self.web_metrics
            .compute_callback_missing_unavailability_count = self
            .web_metrics
            .compute_callback_missing_unavailability_count
            .saturating_add(unavailable_callbacks.len() as u64);
        Ok(RuntimeDefinitionEnvelope {
            policy: self.policy.clone(),
            sources,
            recipes,
            source_families,
            recipe_families,
            unavailable_callbacks,
        })
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub fn export_runtime_envelope(&mut self) -> Result<RuntimeEnvelope, ForgeSignalJsError> {
        Ok(RuntimeEnvelope {
            definitions: self.export_definitions()?,
            snapshot: self.snapshot()?,
        })
    }

    pub(crate) fn export_exact_runtime_restore_artifact(
        &mut self,
    ) -> Result<ExactRuntimeRestoreArtifact, ForgeSignalJsError> {
        let snapshot = self.snapshot()?;
        let store = self.lock_store()?.clone();
        let callback_diagnostics = self.lock_callback_diagnostics()?.clone();
        Ok(ExactRuntimeRestoreArtifact {
            snapshot,
            store,
            callback_diagnostics,
            catalog: self.catalog.clone(),
            web_signals: self.web_signals.clone(),
            nodes_by_id: self.nodes_by_id.clone(),
            dense_grids: self.dense_grids.clone(),
            branch_states: self.branch_states.clone(),
            snapshot_states: self.snapshot_states.clone(),
            runtime_snapshots: self.runtime_snapshots.clone(),
            policy: self.policy.clone(),
            web_metrics: self.web_metrics.clone(),
        })
    }

    pub fn runtime_proof_report(&self) -> RuntimeProofReport {
        runtime_proof_report(
            self.runtime.schema_registry().registry_digest(),
            self.runtime.merge_strategy_registry().registry_digest(),
            self.runtime
                .merge_base_strategy_registry()
                .registry_digest(),
            self.runtime
                .aspect_merge_policy_registry()
                .registry_digest(),
            self.runtime.conflict_isolation_registry().registry_digest(),
            self.runtime.conflict_policy_registry().registry_digest(),
            self.runtime.identity_matcher_registry().registry_digest(),
            self.runtime.source_only_policy_registry().registry_digest(),
            self.runtime.deletion_policy_registry().registry_digest(),
        )
    }

    pub fn branch_state_proof(
        &self,
        branch_id: u64,
    ) -> Result<BranchStateProofReport, ForgeSignalJsError> {
        let branch = self
            .runtime
            .branch_handle(RuntimeBranchId(branch_id))
            .ok_or_else(|| {
                ForgeSignalJsError::invalid_input(format!("unknown branch `{branch_id}`"))
            })?;
        let state = self.state_for_branch(branch_id);
        Ok(branch_state_proof_report(
            branch_id,
            branch.name,
            branch.head_snapshot_id.map(|id| id.0),
            BRANCH_STATE_PROOF_BASIS_VERSION,
            &build_branch_state_proof_basis(&state),
        ))
    }

    pub fn replay_parity_proof(
        &self,
        expected_branch_id: u64,
        replayed_branch_id: u64,
    ) -> Result<ReplayParityProofReport, ForgeSignalJsError> {
        let expected = self.branch_state_proof(expected_branch_id)?;
        let replayed = self.branch_state_proof(replayed_branch_id)?;
        Ok(replay_parity_proof_report(
            expected.branch_id,
            expected.branch_name,
            expected.snapshot_id,
            expected.state_digest,
            replayed.branch_id,
            replayed.branch_name,
            replayed.snapshot_id,
            replayed.state_digest,
        ))
    }

    pub fn replay_artifact_proof(
        &self,
        expected: ReplayArtifactProofInput,
        replayed_branch_id: u64,
    ) -> Result<ReplayArtifactProofReport, ForgeSignalJsError> {
        let replayed_state = self.branch_state_proof(replayed_branch_id)?;
        let runtime_proof = self.runtime_proof_report();
        Ok(replay_artifact_proof_report(
            expected,
            ReplayArtifactProofInput {
                proof_schema_version: runtime_proof.proof_schema_version.clone(),
                registry_bundle_digest: Some(runtime_proof.registry_bundle_digest),
                lowered_strategy_bundle_digest: None,
                merge_plan_digest: None,
                merge_result_digest: None,
                lineage_digest: None,
                branch_state_digest: replayed_state.state_digest,
            },
        ))
    }

    pub(super) fn merge_plan_proof_report(
        &self,
        plan: &BranchMergePlan,
    ) -> Result<MergePlanProofReport, ForgeSignalJsError> {
        Ok(merge_plan_proof_report(
            plan,
            &self.runtime_proof_report().registry_bundle_digest,
        ))
    }

    pub(super) fn merge_result_proof_report(
        &self,
        result: &BranchMergeResult,
    ) -> Result<MergeResultProofReport, ForgeSignalJsError> {
        Ok(merge_result_proof_report(result))
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub fn replace_runtime_envelope(
        &mut self,
        envelope: RuntimeEnvelope,
    ) -> Result<(), ForgeSignalJsError> {
        if !envelope.definitions.unavailable_callbacks.is_empty() {
            self.web_metrics
                .compute_callback_missing_unavailability_count = self
                .web_metrics
                .compute_callback_missing_unavailability_count
                .saturating_add(envelope.definitions.unavailable_callbacks.len() as u64);
            let ids = envelope
                .definitions
                .unavailable_callbacks
                .iter()
                .map(|artifact| artifact.id.clone())
                .collect::<Vec<_>>()
                .join(", ");
            return Err(ForgeSignalJsError::callback_failure(
                CALLBACK_UNAVAILABLE_FOR_RUNTIME_ENVELOPE_IMPORT,
                format!(
                    "runtime envelope import cannot restore callback-backed nodes without live callback registrations: {ids}"
                ),
                Some(ids),
            ));
        }
        let mut rebuilt = RuntimeCore::new(envelope.definitions.policy.clone())?;
        for family in envelope.definitions.source_families {
            rebuilt.define_source_family(family)?;
        }
        for family in envelope.definitions.recipe_families {
            rebuilt.define_keyed_recipe_family(family)?;
        }
        for source in envelope.definitions.sources {
            rebuilt.define_source(source)?;
        }
        for recipe in envelope.definitions.recipes {
            rebuilt.define_recipe(recipe)?;
        }
        rebuilt.restore_snapshot(envelope.snapshot)?;
        *self = rebuilt;
        Ok(())
    }

    pub(crate) fn replace_runtime_envelope_exact(
        &mut self,
        artifact: ExactRuntimeRestoreArtifact,
    ) -> Result<(), ForgeSignalJsError> {
        let mut rebuilt = RuntimeCore::new(artifact.policy.clone())?;
        rebuilt.catalog = artifact.catalog;
        rebuilt.web_signals = artifact.web_signals;
        rebuilt.nodes_by_id = artifact.nodes_by_id;
        rebuilt.dense_grids = artifact.dense_grids;
        rebuilt.branch_states = artifact.branch_states;
        rebuilt.snapshot_states = artifact.snapshot_states;
        rebuilt.runtime_snapshots = artifact.runtime_snapshots;
        rebuilt.web_metrics = artifact.web_metrics;
        rebuilt.store = Arc::new(Mutex::new(artifact.store));
        rebuilt.callback_diagnostics = Arc::new(Mutex::new(artifact.callback_diagnostics));
        rebuilt.restore_snapshot(artifact.snapshot)?;
        *self = rebuilt;
        Ok(())
    }
}
