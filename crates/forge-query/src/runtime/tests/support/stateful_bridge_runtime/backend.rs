use super::super::*;
use super::state::StatefulBridgeState;
use super::verification::{probe_existing_truth, verify_existing_truth_assertion};
use super::writes::{apply_command, external_row_text};
use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::runtime::backend::build_bridge_authority_bundle;

use std::cell::RefCell;
use std::rc::Rc;

use crate::declarative_live::DeclarativeLiveViewShape;
use crate::memory_workspace::{
    ForgeQueryCommitIdentity, ForgeQueryEntityIdentity, ForgeQueryLivePatch,
    ForgeQueryLiveViewHandle, ForgeQuerySnapshotIdentity,
};
use crate::subscription::SubscriptionActivationInput;
use forge_runtime_bridge::facade::RelationalBridgeRecordIdentityParts;

type SharedState = Rc<RefCell<StatefulBridgeState>>;

pub(super) struct StatefulBridgeRuntimeBackend {
    state: SharedState,
    support_profile: ForgeQueryRuntimeSupportProfile,
}

impl StatefulBridgeRuntimeBackend {
    pub(super) fn new(
        state: SharedState,
        support_profile: ForgeQueryRuntimeSupportProfile,
    ) -> Self {
        Self {
            state,
            support_profile,
        }
    }
}

impl ForgeQueryRuntimeBackend for StatefulBridgeRuntimeBackend {
    fn support_profile(&self) -> ForgeQueryRuntimeSupportProfile {
        self.support_profile.clone()
    }

    fn current_snapshot_identity(&self) -> ForgeQuerySnapshotIdentity {
        let state = self.state.borrow();
        ForgeQuerySnapshotIdentity::preview(
            ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::RuntimeStateSnapshot)
                .field_usize(
                    ForgeQueryEvidenceTag::new("stateful_bridge_snapshot_sequence"),
                    state.next_snapshot_token,
                )
                .seal(),
        )
    }

    fn admit_live_view_declaration(
        &self,
        name: &str,
        request: &DeclarativeLiveQueryRequest,
        schema_view: &QuerySchemaView,
    ) -> Result<LiveViewDeclarationAdmissionBoundaryReceipt, ForgeQueryWorkspaceError> {
        TestSchemaAdapter.admit_live_view(name, request, schema_view)
    }

    fn declare_live_view(
        &mut self,
        name: String,
        request: DeclarativeLiveQueryRequest,
        _schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryLiveViewHandle, ForgeQueryWorkspaceError> {
        self.state
            .borrow_mut()
            .live_views
            .insert(name.clone(), request.target().to_string());
        Ok(ForgeQueryLiveViewHandle::new(name))
    }

    fn write(
        &mut self,
        command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let mut state = self.state.borrow_mut();
        let collection = command
            .declared_collection_ref()
            .map(str::to_string)
            .or_else(|| {
                command
                    .existing_truth_binding()
                    .and_then(|binding| binding.target_collection().map(str::to_string))
            })
            .or_else(|| {
                command.declared_entity_identity_ref().and_then(|identity| {
                    state
                        .collection_by_identity
                        .get(&identity.terminal_projection_for_reporting())
                        .cloned()
                })
            })
            .ok_or_else(|| {
                ForgeQueryWorkspaceError::new("stateful bridge could not resolve collection")
            })?;
        let (entity_identity, entity_identity_text) = match command.mutation_family() {
            ForgeQueryMutationFamily::Insert => {
                state.next_entity_identity += 1;
                let identity = ForgeQueryEntityIdentity::from_relational_record(
                    RelationalBridgeRecordIdentityParts::entity(
                        1,
                        state.next_entity_identity as u64,
                        0,
                    ),
                );
                let identity_text = identity.terminal_projection_for_reporting();
                (identity, identity_text)
            }
            _ => {
                let identity = command
                    .declared_entity_identity_ref()
                    .cloned()
                    .or_else(|| {
                        command
                            .existing_truth_binding()
                            .map(|binding| binding.resolved_target_identity().clone())
                    })
                    .or_else(|| {
                        command.symbolic_target_reference().and_then(|reference| {
                            state.identity_by_symbol.get(reference.symbol()).cloned()
                        })
                    })
                    .ok_or_else(|| {
                        ForgeQueryWorkspaceError::new(
                            "stateful bridge could not resolve target entity identity",
                        )
                    })?;
                let identity_text = command
                    .symbolic_target_reference()
                    .and_then(|reference| state.identity_text_by_symbol.get(reference.symbol()))
                    .cloned()
                    .unwrap_or_else(|| identity.terminal_projection_for_reporting());
                (identity, identity_text)
            }
        };
        let mutation_kind = apply_command(
            &mut state,
            &command,
            &collection,
            &entity_identity,
            &entity_identity_text,
        )?;
        state.next_commit_identity += 1;
        state.next_snapshot_token += 1;
        let commit_identity =
            ForgeQueryCommitIdentity::from_relational_commit_id(state.next_commit_identity as u64);
        let snapshot_identity = ForgeQuerySnapshotIdentity::from_relational_snapshot(
            forge_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts::new(
                1,
                state.next_snapshot_token as u64,
            ),
        );
        let bridge_authority = build_bridge_authority_bundle(
            &state.bridge,
            &snapshot_identity,
            &command,
            &collection,
            &entity_identity,
            mutation_kind.clone(),
        )?;
        Ok(test_mutation_receipt_with_bridge_authority(
            commit_identity,
            snapshot_identity,
            collection,
            entity_identity,
            mutation_kind,
            command.declared_aspect_paths(),
            bridge_authority,
        ))
    }

    fn write_batch(
        &mut self,
        commands: Vec<ForgeQueryWriteCommand>,
    ) -> Result<Vec<ForgeQueryMutationReceipt>, ForgeQueryWorkspaceError> {
        let mut receipts = Vec::with_capacity(commands.len());
        for command in commands {
            receipts.push(self.write(command)?);
        }
        Ok(receipts)
    }

    fn admit_existing_truth_binding(
        &self,
        binding: &ForgeQueryExistingTruthTargetBinding,
    ) -> Result<(), ForgeQueryExistingTruthBindingDenial> {
        let state = self.state.borrow();
        if let Some(expected_collection) = binding.target_collection() {
            if !state.installed_collections.contains(expected_collection) {
                return Err(ForgeQueryExistingTruthBindingDenial::new(
                    binding,
                    ForgeQueryExistingTruthBindingDenialKind::CollectionMismatch,
                    format!("declared target collection `{expected_collection}` is not installed"),
                ));
            }
        }
        let resolved_target_identity = binding
            .resolved_target_identity()
            .terminal_projection_for_reporting();
        let Some(actual_collection) = state.collection_by_identity.get(&resolved_target_identity)
        else {
            return Err(ForgeQueryExistingTruthBindingDenial::new(
                binding,
                ForgeQueryExistingTruthBindingDenialKind::ResolvedTargetMissing,
                format!(
                    "resolved target `{}` is not present in authoritative truth",
                    resolved_target_identity
                ),
            ));
        };
        if let Some(expected_collection) = binding.target_collection() {
            if actual_collection != expected_collection {
                return Err(ForgeQueryExistingTruthBindingDenial::new(
                    binding,
                    ForgeQueryExistingTruthBindingDenialKind::CollectionMismatch,
                    format!(
                        "resolved target `{}` belongs to collection `{actual_collection}`, not `{expected_collection}`",
                        resolved_target_identity
                    ),
                ));
            }
        }
        Ok(())
    }

    fn verify_existing_truth_assertion(
        &self,
        binding: &ForgeQueryExistingTruthTargetBinding,
        aspects: &[ForgeQueryAspectValue],
    ) -> Result<ForgeQueryVerifiedExistingTruthAssertion, ForgeQueryExistingTruthAssertionDenial>
    {
        let state = self.state.borrow();
        let snapshot_identity = self.current_snapshot_identity();
        verify_existing_truth_assertion(&state, binding, aspects, snapshot_identity)
    }

    fn probe_existing_truth(
        &self,
        request: &ForgeQueryExistingTruthProbeRequest,
    ) -> Result<ForgeQueryExistingTruthProbe, ForgeQueryExistingTruthProbeDenial> {
        probe_existing_truth(&self.state.borrow(), request)
    }

    fn execute_intent(
        &mut self,
        _declaration: &ForgeQueryIntentDeclaration,
    ) -> Result<ForgeQueryIntentExecution, ForgeQueryRuntimeError> {
        Err(ForgeQueryRuntimeError::MissingIntentAuthority)
    }

    fn live_entities(&self, view_name: &str) -> Vec<ForgeQueryEntity> {
        let state = self.state.borrow();
        let Some(collection) = state.live_views.get(view_name) else {
            return Vec::new();
        };
        let Some(rows) = state.rows_by_collection.get(collection) else {
            return Vec::new();
        };
        rows.iter()
            .map(|(identity, external_row)| {
                ForgeQueryEntity::from_external_projection(
                    state
                        .identity_by_storage_key
                        .get(identity)
                        .cloned()
                        .unwrap_or_else(|| {
                            crate::memory_workspace::admit_authored_entity_label(identity)
                        }),
                    external_row.clone(),
                )
            })
            .collect()
    }

    fn drain_live_patches(&mut self, _view_name: &str) -> Vec<ForgeQueryLivePatch> {
        Vec::new()
    }

    fn affected_live_view_ids(&self, receipt: &ForgeQueryMutationReceipt) -> Vec<String> {
        let state = self.state.borrow();
        let mut affected = receipt
            .deltas
            .iter()
            .flat_map(|delta| {
                state
                    .live_views
                    .iter()
                    .filter(move |(_, collection)| *collection == &delta.collection)
                    .map(|(name, _)| name.clone())
            })
            .collect::<Vec<_>>();
        affected.sort();
        affected.dedup();
        affected
    }

    fn install_live_subscription(
        &mut self,
        view_name: &str,
        activation: &SubscriptionActivationInput,
    ) -> Result<SubscriptionActivationReceipt, ForgeQueryWorkspaceError> {
        let receipt = TestSubscriptionActivation.admit_activation(view_name, activation)?;
        Ok(receipt.activation_receipt().clone())
    }

    fn admit_preview_basis(
        &self,
        label: &ForgeQuerySessionLabel,
        effect_policy: ForgeQueryEffectPolicy,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryPreviewBasisAdmission, ForgeQueryWorkspaceError> {
        TestPreviewBasis.admit_preview_basis(label, effect_policy, authority)
    }

    fn inspect_write_receipt(
        &self,
        receipt: &ForgeQueryWriteReceipt,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryRuntimeInspectionEvidence, ForgeQueryWorkspaceError> {
        TestInspectorEvidence.inspect_write_receipt(receipt, authority)
    }

    fn grouped_baseline_members(
        &self,
        request: &DeclarativeLiveQueryRequest,
    ) -> Result<Option<Vec<(String, String)>>, ForgeQueryWorkspaceError> {
        let DeclarativeLiveViewShape::KanbanGrouped { grouping_aspect } = request.view_shape()
        else {
            return Ok(None);
        };
        let grouping_aspect_text = grouping_aspect.as_str();
        let identity_path = request
            .projection()
            .iter()
            .find(|field| field.aspect() == "identity")
            .map(|field| format!("{}.{}", field.aspect(), field.field()))
            .unwrap_or_else(|| "identity.id".to_string());
        let grouping_path = request
            .projection()
            .iter()
            .find(|field| field.aspect() == grouping_aspect_text)
            .map(|field| format!("{}.{}", field.aspect(), field.field()))
            .unwrap_or_else(|| format!("{grouping_aspect_text}.value"));
        let members = self
            .state
            .borrow()
            .rows_by_collection
            .get(request.target())
            .into_iter()
            .flat_map(|rows| rows.iter())
            .filter_map(|(entity_identity, external_row)| {
                let member = external_row_text(external_row, &identity_path)
                    .unwrap_or_else(|| entity_identity.clone());
                let lane = external_row_text(external_row, &grouping_path)?;
                Some((member, lane))
            })
            .collect::<Vec<_>>();
        Ok(Some(members))
    }
}
