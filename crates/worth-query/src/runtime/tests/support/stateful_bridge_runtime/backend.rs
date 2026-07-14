use super::super::*;
use super::state::StatefulBridgeState;
use super::verification::{probe_existing_truth, verify_existing_truth_assertion};
use super::writes::{
    apply_command, external_row_text_at_path, native_external_field_path_for_touch,
};
use crate::evidence_identity::{
    WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};
use crate::runtime::backend::build_bridge_authority_bundle;
use crate::runtime::WorthQueryAspectTouch;

use std::cell::RefCell;
use std::rc::Rc;

use crate::declarative_live::{DeclarativeLiveViewShape, DeclarativeProjectionField};
use crate::memory_workspace::{
    WorthQueryCommitIdentity, WorthQueryEntityIdentity, WorthQueryLivePatch,
    WorthQueryLiveViewHandle, WorthQuerySnapshotIdentity,
};
use crate::subscription::SubscriptionActivationInput;
use worth_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts;

type SharedState = Rc<RefCell<StatefulBridgeState>>;

pub(super) struct StatefulBridgeRuntimeBackend {
    state: SharedState,
    support_profile: WorthQueryRuntimeSupportProfile,
}

impl StatefulBridgeRuntimeBackend {
    pub(super) fn new(
        state: SharedState,
        support_profile: WorthQueryRuntimeSupportProfile,
    ) -> Self {
        Self {
            state,
            support_profile,
        }
    }
}

impl WorthQueryRuntimeBackend for StatefulBridgeRuntimeBackend {
    fn support_profile(&self) -> WorthQueryRuntimeSupportProfile {
        self.support_profile.clone()
    }

    fn current_snapshot_identity(&self) -> WorthQuerySnapshotIdentity {
        let state = self.state.borrow();
        WorthQuerySnapshotIdentity::preview(
            WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::RuntimeStateSnapshot)
                .field_usize(
                    WorthQueryEvidenceTag::new("stateful_bridge_snapshot_sequence"),
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
    ) -> Result<LiveViewDeclarationAdmissionBoundaryReceipt, WorthQueryWorkspaceError> {
        TestSchemaAdapter.admit_live_view(name, request, schema_view)
    }

    fn declare_live_view(
        &mut self,
        name: String,
        request: DeclarativeLiveQueryRequest,
        _schema_view: QuerySchemaView,
    ) -> Result<WorthQueryLiveViewHandle, WorthQueryWorkspaceError> {
        let live_target = WorthQueryLiveArtifactTarget::from_view_name(name.clone());
        self.state
            .borrow_mut()
            .live_views
            .insert(live_target, request.target_collection_identity());
        Ok(WorthQueryLiveViewHandle::new(name))
    }

    fn close_live_view(&mut self, name: &str) -> Result<(), WorthQueryWorkspaceError> {
        self.state
            .borrow_mut()
            .live_views
            .remove(&WorthQueryLiveArtifactTarget::from_view_name(name));
        Ok(())
    }

    fn write(
        &mut self,
        mutation: WorthQueryBackendAdmissibleMutation,
    ) -> Result<WorthQueryMutationReceipt, WorthQueryWorkspaceError> {
        let mut state = self.state.borrow_mut();
        let collection = mutation
            .declared_collection_identity()
            .map(|collection| collection.as_str().to_string())
            .or_else(|| {
                mutation.existing_truth_binding().and_then(|binding| {
                    binding
                        .terminal_target_collection_projection()
                        .map(str::to_string)
                })
            })
            .or_else(|| {
                mutation
                    .declared_entity_identity_ref()
                    .and_then(|identity| {
                        state
                            .collection_by_identity
                            .get(&identity.terminal_projection_for_reporting())
                            .cloned()
                    })
            })
            .ok_or_else(|| {
                WorthQueryWorkspaceError::new("stateful bridge could not resolve collection")
            })?;
        let (entity_identity, entity_identity_text) = match mutation.mutation_family() {
            WorthQueryMutationFamily::Insert => {
                state.next_entity_identity += 1;
                let identity = WorthQueryEntityIdentity::from_relational_record(
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
                let identity = mutation
                    .declared_entity_identity_ref()
                    .cloned()
                    .or_else(|| {
                        mutation
                            .existing_truth_binding()
                            .map(|binding| binding.resolved_target_identity().clone())
                    })
                    .or_else(|| {
                        mutation.symbolic_target_reference().and_then(|reference| {
                            state.identity_by_symbol.get(reference.symbol()).cloned()
                        })
                    })
                    .ok_or_else(|| {
                        WorthQueryWorkspaceError::new(
                            "stateful bridge could not resolve target entity identity",
                        )
                    })?;
                let identity_text = mutation
                    .symbolic_target_reference()
                    .and_then(|reference| state.identity_text_by_symbol.get(reference.symbol()))
                    .cloned()
                    .unwrap_or_else(|| identity.terminal_projection_for_reporting());
                (identity, identity_text)
            }
        };
        let mutation_kind = apply_command(
            &mut state,
            &mutation,
            &collection,
            &entity_identity,
            &entity_identity_text,
        )?;
        state.next_commit_identity += 1;
        state.next_snapshot_token += 1;
        let commit_identity =
            WorthQueryCommitIdentity::from_relational_commit_id(state.next_commit_identity as u64);
        let snapshot_identity = WorthQuerySnapshotIdentity::from_relational_snapshot(
            worth_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts::new(
                1,
                state.next_snapshot_token as u64,
            ),
        );
        let bridge_authority = build_bridge_authority_bundle(
            &state.bridge,
            &snapshot_identity,
            &mutation,
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
            mutation.declared_aspect_touches(),
            bridge_authority,
        ))
    }

    fn write_batch(
        &mut self,
        mutations: Vec<WorthQueryBackendAdmissibleMutation>,
    ) -> Result<Vec<WorthQueryMutationReceipt>, WorthQueryWorkspaceError> {
        let mut receipts = Vec::with_capacity(mutations.len());
        for mutation in mutations {
            receipts.push(self.write(mutation)?);
        }
        Ok(receipts)
    }

    fn admit_existing_truth_binding(
        &self,
        binding: &WorthQueryExistingTruthTargetBinding,
    ) -> Result<(), WorthQueryExistingTruthBindingDenial> {
        let state = self.state.borrow();
        if let Some(expected_collection) = binding.terminal_target_collection_projection() {
            if !state.installed_collections.contains(expected_collection) {
                return Err(WorthQueryExistingTruthBindingDenial::new(
                    binding,
                    WorthQueryExistingTruthBindingDenialKind::CollectionMismatch,
                    format!("declared target collection `{expected_collection}` is not installed"),
                ));
            }
        }
        let resolved_target_identity = binding
            .resolved_target_identity()
            .terminal_projection_for_reporting();
        let Some(actual_collection) = state.collection_by_identity.get(&resolved_target_identity)
        else {
            return Err(WorthQueryExistingTruthBindingDenial::new(
                binding,
                WorthQueryExistingTruthBindingDenialKind::ResolvedTargetMissing,
                format!(
                    "resolved target `{}` is not present in authoritative truth",
                    resolved_target_identity
                ),
            ));
        };
        if let Some(expected_collection) = binding.terminal_target_collection_projection() {
            if actual_collection != expected_collection {
                return Err(WorthQueryExistingTruthBindingDenial::new(
                    binding,
                    WorthQueryExistingTruthBindingDenialKind::CollectionMismatch,
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
        binding: &WorthQueryExistingTruthTargetBinding,
        aspects: &[WorthQueryAdmittedAspectValue],
    ) -> Result<WorthQueryVerifiedExistingTruthAssertion, WorthQueryExistingTruthAssertionDenial>
    {
        let state = self.state.borrow();
        let snapshot_identity = self.current_snapshot_identity();
        verify_existing_truth_assertion(&state, binding, aspects, snapshot_identity)
    }

    fn probe_existing_truth(
        &self,
        request: &WorthQueryExistingTruthProbeRequest,
    ) -> Result<WorthQueryExistingTruthProbe, WorthQueryExistingTruthProbeDenial> {
        probe_existing_truth(&self.state.borrow(), request)
    }

    fn execute_intent(
        &mut self,
        _declaration: &WorthQueryIntentDeclaration,
    ) -> Result<WorthQueryIntentExecution, WorthQueryRuntimeError> {
        Err(WorthQueryRuntimeError::MissingIntentAuthority)
    }

    fn admit_query_writeback_authority(&self) -> Result<(), WorthQueryWorkspaceError> {
        if self.state.borrow().bridge.writeback_authority().is_some() {
            Ok(())
        } else {
            Err(WorthQueryWorkspaceError::new(
                "stateful bridge fixture has no truth writeback authority",
            ))
        }
    }

    fn execute_query_writeback(
        &mut self,
        declaration: &crate::workflow::QueryWritebackDeclaration,
    ) -> Result<
        worth_runtime_bridge::facade::BridgeAdmittedWritebackExecution,
        (crate::effect_lifecycle::EffectExecutionDenialKind, String),
    > {
        crate::effect_lifecycle::execute_lowered_writeback(&self.state.borrow().bridge, declaration)
    }

    fn live_entities_for_target(
        &self,
        target: &WorthQueryLiveArtifactTarget,
    ) -> Vec<WorthQueryEntity> {
        let state = self.state.borrow();
        let Some(collection) = state.live_views.get(target) else {
            return Vec::new();
        };
        let Some(rows) = state.rows_by_collection.get(collection.as_str()) else {
            return Vec::new();
        };
        rows.iter()
            .map(|(identity, external_row)| {
                WorthQueryEntity::from_native_field_values(
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

    fn drain_live_patches_for_target(
        &mut self,
        _target: &WorthQueryLiveArtifactTarget,
    ) -> Vec<WorthQueryLivePatch> {
        Vec::new()
    }

    fn affected_live_view_targets(
        &self,
        receipt: &WorthQueryMutationReceipt,
    ) -> Vec<WorthQueryLiveArtifactTarget> {
        let state = self.state.borrow();
        let mut affected = receipt
            .deltas
            .iter()
            .flat_map(|delta| {
                state
                    .live_views
                    .iter()
                    .filter(move |(_, collection)| {
                        delta
                            .target_collection_identity()
                            .same_target_collection_as(collection)
                    })
                    .map(|(target, _)| target.clone())
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
    ) -> Result<SubscriptionActivationReceipt, WorthQueryWorkspaceError> {
        let receipt = TestSubscriptionActivation.admit_activation(view_name, activation)?;
        Ok(receipt.activation_receipt().clone())
    }

    fn admit_preview_basis(
        &self,
        label: &WorthQuerySessionLabel,
        effect_policy: WorthQueryEffectPolicy,
        authority: &WorthQueryRuntimeEvidenceAuthority,
    ) -> Result<WorthQueryPreviewBasisAdmission, WorthQueryWorkspaceError> {
        TestPreviewBasis.admit_preview_basis(label, effect_policy, authority)
    }

    fn inspect_write_receipt(
        &self,
        receipt: &WorthQueryWriteReceipt,
        authority: &WorthQueryRuntimeEvidenceAuthority,
    ) -> Result<WorthQueryRuntimeInspectionEvidence, WorthQueryWorkspaceError> {
        TestInspectorEvidence.inspect_write_receipt(receipt, authority)
    }

    fn grouped_baseline_members(
        &self,
        request: &DeclarativeLiveQueryRequest,
    ) -> Result<
        Option<Vec<crate::view_shape_live::WorthQueryGroupedBaselineMember>>,
        WorthQueryWorkspaceError,
    > {
        let DeclarativeLiveViewShape::KanbanGrouped { grouping_aspect } = request.view_shape()
        else {
            return Ok(None);
        };
        let identity_path = request
            .projection()
            .iter()
            .find(|field| field.source_field_key().native_aspect_key() == identity_aspect_key())
            .map(native_external_field_path_for_projection_field)
            .unwrap_or_else(|| native_external_field_path_for_aspect_field("identity", "id"));
        let identity_path = identity_path?;
        let grouping_path = request
            .projection()
            .iter()
            .find(|field| field.source_field_key().native_aspect_key() == *grouping_aspect)
            .map(native_external_field_path_for_projection_field)
            .unwrap_or_else(|| native_external_field_path_for_grouping_aspect(grouping_aspect));
        let grouping_path = grouping_path?;
        let members = self
            .state
            .borrow()
            .rows_by_collection
            .get(request.target())
            .into_iter()
            .flat_map(|rows| rows.iter())
            .filter_map(|(entity_identity, external_row)| {
                let member = external_row_text_at_path(external_row, &identity_path)
                    .unwrap_or_else(|| entity_identity.clone());
                let lane = external_row_text_at_path(external_row, &grouping_path)?;
                Some(
                    crate::view_shape_live::WorthQueryGroupedBaselineMember::from_authoritative_member_lane_keys(
                        member,
                        lane,
                    ),
                )
            })
            .collect::<Vec<_>>();
        Ok(Some(members))
    }
}

fn identity_aspect_key() -> AspectKey {
    AspectKey::new("identity").expect("identity aspect key must admit")
}

fn native_external_field_path_for_projection_field(
    field: &DeclarativeProjectionField,
) -> Result<CanonicalFieldPath, WorthQueryWorkspaceError> {
    native_external_field_path_for_touch(&WorthQueryAspectTouch::aspect_field_path(
        field.source_field_key().native_aspect_key(),
        CanonicalFieldPath::single(field.source_field_key().native_field_key()),
    ))
}

fn native_external_field_path_for_grouping_aspect(
    grouping_aspect: &AspectKey,
) -> Result<CanonicalFieldPath, WorthQueryWorkspaceError> {
    native_external_field_path_for_touch(&WorthQueryAspectTouch::aspect_field_path(
        grouping_aspect.clone(),
        CanonicalFieldPath::single(FieldKey::new("value").expect("value field key must admit")),
    ))
}

fn native_external_field_path_for_aspect_field(
    aspect: &str,
    field: &str,
) -> Result<CanonicalFieldPath, WorthQueryWorkspaceError> {
    native_external_field_path_for_touch(&WorthQueryAspectTouch::aspect_field_path(
        AspectKey::new(aspect).expect("stateful bridge fixture aspect key must admit"),
        CanonicalFieldPath::single(
            FieldKey::new(field).expect("stateful bridge fixture field key must admit"),
        ),
    ))
}
