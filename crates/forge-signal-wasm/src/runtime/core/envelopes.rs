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

use super::merge::build_branch_state_proof_basis;
use super::state::StoredRecipeDefinition;
use super::RuntimeCore;

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
}
