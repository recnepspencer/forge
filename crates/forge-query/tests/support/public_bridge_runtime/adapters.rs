use std::cell::RefCell;
use std::rc::Rc;

use forge_query::facade::{
    DeclarativeLiveQueryRequest, ForgeQueryAspectValue, ForgeQueryBasisAdmissionEvidenceRow,
    ForgeQueryEffectPolicy, ForgeQueryEntity, ForgeQueryExistingTruthAssertionDenial,
    ForgeQueryExistingTruthAssertionDenialKind, ForgeQueryExistingTruthProbeDenial,
    ForgeQueryExistingTruthProbeDenialKind, ForgeQueryExistingTruthProbeRequest,
    ForgeQueryExistingTruthTargetBinding, ForgeQueryLivePatch, ForgeQueryLiveViewHandle,
    ForgeQueryMutationDelta, ForgeQueryMutationKind, ForgeQueryMutationReceipt,
    ForgeQueryPreviewBasisAdmission, ForgeQueryRuntimeEvidenceAuthority,
    ForgeQueryRuntimeInspectionEvidence, ForgeQueryRuntimeInspectorEvidenceAdapter,
    ForgeQueryRuntimePreviewBasisAdapter, ForgeQueryRuntimeSchemaAdapter,
    ForgeQueryRuntimeSignalSinkAdapter, ForgeQueryRuntimeSnapshotIdentityAdapter,
    ForgeQueryRuntimeSourceAdapter, ForgeQueryRuntimeSubscriptionActivationAdapter,
    ForgeQueryRuntimeWriteAuthorityAdapter, ForgeQuerySessionLabel, ForgeQuerySnapshotIdentity,
    ForgeQueryVerifiedExistingTruthAssertion, ForgeQueryWorkspaceError, ForgeQueryWriteCommand,
    ForgeQueryWriteReceipt, LiveViewDeclarationAdmissionBoundaryReceipt, QuerySchemaView,
    SignalInvalidationBoundaryReceipt, SubscriptionActivationBoundaryReceipt,
    SubscriptionActivationInput, WriteAuthorityExecutionReceipt,
};
use forge_query::facade::{ForgeQueryCommitIdentity, ForgeQueryEntityIdentity};
use forge_relational::facade::runtime::RelationalRuntime;
use forge_runtime_bridge::facade::{RelationalBridgeSnapshotIdentityParts, RuntimeBridge};
use serde_json::Value;

use super::external_row::{apply_aspects_to_external_row, external_row_from_aspects};
use super::state::PublicBridgeRuntimeState;

type SharedRuntimeState = Rc<RefCell<PublicBridgeRuntimeState>>;

pub(super) struct PublicSchemaAdapter;

impl ForgeQueryRuntimeSchemaAdapter for PublicSchemaAdapter {
    fn admit_live_view(
        &self,
        name: &str,
        request: &DeclarativeLiveQueryRequest,
        _schema_view: &QuerySchemaView,
    ) -> Result<LiveViewDeclarationAdmissionBoundaryReceipt, ForgeQueryWorkspaceError> {
        let receipt = self.build_live_view_declaration_admission_receipt(name, request);
        Ok(self.build_live_view_declaration_boundary_receipt(name, request, receipt))
    }
}

pub(super) struct PublicSourceAdapter {
    state: SharedRuntimeState,
}

impl PublicSourceAdapter {
    pub(super) fn new(state: SharedRuntimeState) -> Self {
        Self { state }
    }
}

impl ForgeQueryRuntimeSourceAdapter for PublicSourceAdapter {
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
                    ForgeQueryEntityIdentity::authored_command(identity),
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
            .deltas()
            .iter()
            .flat_map(|delta| {
                state
                    .live_views
                    .iter()
                    .filter(move |(_, collection)| *collection == delta.collection())
                    .map(|(name, _)| name.clone())
            })
            .collect::<Vec<_>>();
        affected.sort();
        affected.dedup();
        affected
    }
}

pub(super) struct PublicWriteAuthorityAdapter {
    state: SharedRuntimeState,
}

impl PublicWriteAuthorityAdapter {
    pub(super) fn new(state: SharedRuntimeState) -> Self {
        Self { state }
    }
}

impl ForgeQueryRuntimeWriteAuthorityAdapter for PublicWriteAuthorityAdapter {
    fn write(
        &mut self,
        _bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut RelationalRuntime>,
        command: ForgeQueryWriteCommand,
    ) -> Result<WriteAuthorityExecutionReceipt, ForgeQueryWorkspaceError> {
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
                        .get(&identity.to_string())
                        .cloned()
                })
            })
            .ok_or_else(|| {
                ForgeQueryWorkspaceError::new("public bridge write could not resolve collection")
            })?;
        let entity_identity = match command.mutation_family() {
            forge_query::facade::ForgeQueryMutationFamily::Insert => {
                state.next_entity_identity += 1;
                ForgeQueryEntityIdentity::authored_command(format!(
                    "public-bridge-entity-{}",
                    state.next_entity_identity
                ))
            }
            _ => command
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
                        "public bridge write could not resolve target entity identity",
                    )
                })?,
        };
        let mutation_kind = apply_command(&mut state, &command, &collection, &entity_identity)?;
        state.next_commit_identity += 1;
        state.next_snapshot_token += 1;
        let commit_identity =
            ForgeQueryCommitIdentity::from_relational_commit_id(state.next_commit_identity as u64);
        let snapshot_identity = public_snapshot_identity(state.next_snapshot_token as u64);
        let bridge_authority = self.build_bridge_mutation_authority_bundle(
            _bridge,
            &snapshot_identity,
            &command,
            &collection,
            &entity_identity,
            mutation_kind.clone(),
        )?;
        let receipt = ForgeQueryMutationReceipt::from_bridge_authoritative_parts(
            commit_identity,
            snapshot_identity,
            vec![ForgeQueryMutationDelta::new(
                collection,
                entity_identity,
                mutation_kind,
                command.declared_aspect_paths(),
            )],
            bridge_authority,
        );
        Ok(self.build_write_authority_execution_receipt(&command, receipt))
    }
}

fn apply_command(
    state: &mut PublicBridgeRuntimeState,
    command: &ForgeQueryWriteCommand,
    collection: &str,
    entity_identity: &ForgeQueryEntityIdentity,
) -> Result<ForgeQueryMutationKind, ForgeQueryWorkspaceError> {
    let entity_identity_key = entity_identity.to_string();
    match command.mutation_family() {
        forge_query::facade::ForgeQueryMutationFamily::Insert => {
            let external_row = external_row_from_aspects(command.aspect_values())?;
            state
                .rows_by_collection
                .entry(collection.to_string())
                .or_default()
                .insert(entity_identity_key.clone(), external_row);
            state
                .collection_by_identity
                .insert(entity_identity_key.clone(), collection.to_string());
            if let Some(reference) = command.symbolic_target_reference() {
                state
                    .identity_by_symbol
                    .insert(reference.symbol().to_string(), entity_identity.clone());
            }
            Ok(ForgeQueryMutationKind::Created)
        }
        forge_query::facade::ForgeQueryMutationFamily::Update
        | forge_query::facade::ForgeQueryMutationFamily::Assertion => {
            let row = state
                .rows_by_collection
                .entry(collection.to_string())
                .or_default()
                .get_mut(&entity_identity_key)
                .ok_or_else(|| {
                    ForgeQueryWorkspaceError::new(format!(
                        "public bridge update could not find `{entity_identity_key}` in `{collection}`"
                    ))
                })?;
            apply_aspects_to_external_row(row, command.aspect_values())?;
            Ok(ForgeQueryMutationKind::Updated)
        }
        forge_query::facade::ForgeQueryMutationFamily::Delete => {
            if let Some(rows) = state.rows_by_collection.get_mut(collection) {
                rows.remove(&entity_identity_key);
            }
            state.collection_by_identity.remove(&entity_identity_key);
            state.identity_by_symbol.retain(|_, resolved_identity| {
                resolved_identity.evidence_identity() != entity_identity.evidence_identity()
            });
            Ok(ForgeQueryMutationKind::Deleted)
        }
    }
}

pub(super) struct PublicSnapshotIdentityAdapter {
    state: SharedRuntimeState,
}

impl PublicSnapshotIdentityAdapter {
    pub(super) fn new(state: SharedRuntimeState) -> Self {
        Self { state }
    }
}

impl ForgeQueryRuntimeSnapshotIdentityAdapter for PublicSnapshotIdentityAdapter {
    fn current_snapshot_identity(&self) -> ForgeQuerySnapshotIdentity {
        let state = self.state.borrow();
        public_snapshot_identity(state.next_snapshot_token as u64)
    }
}

fn public_snapshot_identity(position: u64) -> ForgeQuerySnapshotIdentity {
    ForgeQuerySnapshotIdentity::from_relational_snapshot(
        RelationalBridgeSnapshotIdentityParts::new(1, position),
    )
}

pub(super) struct PublicExistingTruthVerificationAdapter {
    state: SharedRuntimeState,
}

impl PublicExistingTruthVerificationAdapter {
    pub(super) fn new(state: SharedRuntimeState) -> Self {
        Self { state }
    }
}

impl forge_query::facade::ForgeQueryRuntimeExistingTruthVerificationAdapter
    for PublicExistingTruthVerificationAdapter
{
    fn verify_existing_truth_assertion(
        &self,
        binding: &ForgeQueryExistingTruthTargetBinding,
        aspects: &[ForgeQueryAspectValue],
    ) -> Result<ForgeQueryVerifiedExistingTruthAssertion, ForgeQueryExistingTruthAssertionDenial>
    {
        let state = self.state.borrow();
        for aspect in aspects {
            let key = existing_truth_key(binding, aspect.aspect_path());
            let Some(found) = state.existing_truth_values.get(&key) else {
                return Err(ForgeQueryExistingTruthAssertionDenial::new(
                    binding,
                    ForgeQueryExistingTruthAssertionDenialKind::MissingAssertedAspect,
                    Some(aspect.aspect_path().to_string()),
                    Some(aspect.value().to_string()),
                    None,
                    "public bridge verification state did not contain the asserted aspect",
                ));
            };
            if found != aspect.value() {
                return Err(ForgeQueryExistingTruthAssertionDenial::new(
                    binding,
                    ForgeQueryExistingTruthAssertionDenialKind::AssertedValueMismatch,
                    Some(aspect.aspect_path().to_string()),
                    Some(aspect.value().to_string()),
                    Some(found.to_string()),
                    "public bridge verification state did not match the asserted value",
                ));
            }
        }
        ForgeQueryVerifiedExistingTruthAssertion::from_snapshot_identity(
            binding,
            aspects,
            &ForgeQuerySnapshotIdentity::from_external_authority_label(
                "public-bridge-existing-truth-snapshot",
            ),
        )
        .map_err(|error| {
            ForgeQueryExistingTruthAssertionDenial::new(
                binding,
                ForgeQueryExistingTruthAssertionDenialKind::MissingAssertedAspect,
                None,
                None,
                None,
                error.to_string(),
            )
        })
    }

    fn probe_existing_truth(
        &self,
        request: &ForgeQueryExistingTruthProbeRequest,
    ) -> Result<Vec<(String, Value)>, ForgeQueryExistingTruthProbeDenial> {
        let state = self.state.borrow();
        let mut fields = Vec::with_capacity(request.aspect_paths().len());
        for aspect_path in request.aspect_paths() {
            let key = existing_truth_key(request.binding(), aspect_path);
            let Some(value) = state.existing_truth_values.get(&key) else {
                return Err(ForgeQueryExistingTruthProbeDenial::new(
                    request.binding(),
                    ForgeQueryExistingTruthProbeDenialKind::MissingProbedAspect,
                    Some(aspect_path.to_string()),
                    "public bridge verification state did not contain the probed aspect",
                ));
            };
            fields.push((aspect_path.clone(), value.clone()));
        }
        Ok(fields)
    }
}

fn existing_truth_key(
    binding: &ForgeQueryExistingTruthTargetBinding,
    aspect_path: &str,
) -> (String, String, String) {
    (
        binding.binding_digest(),
        binding.target_collection().unwrap_or("none").to_string(),
        aspect_path.to_string(),
    )
}

pub(super) struct PublicSignalSinkAdapter;

impl ForgeQueryRuntimeSignalSinkAdapter for PublicSignalSinkAdapter {
    fn route_write_receipt(
        &mut self,
        receipt: &ForgeQueryMutationReceipt,
    ) -> Result<SignalInvalidationBoundaryReceipt, ForgeQueryWorkspaceError> {
        let routed = self.build_signal_invalidation_routing_receipt(receipt)?;
        self.build_signal_invalidation_boundary_receipt(receipt, routed)
    }
}

pub(super) struct PublicSubscriptionActivationAdapter;

impl ForgeQueryRuntimeSubscriptionActivationAdapter for PublicSubscriptionActivationAdapter {
    fn support_evidence(&self) -> String {
        "public-graph-subscription-activation".to_string()
    }

    fn admit_activation(
        &mut self,
        view_name: &str,
        activation: &SubscriptionActivationInput,
    ) -> Result<SubscriptionActivationBoundaryReceipt, ForgeQueryWorkspaceError> {
        let receipt = self.build_subscription_activation_receipt(view_name, activation);
        Ok(self.build_subscription_activation_boundary_receipt(view_name, activation, receipt))
    }
}

pub(super) struct PublicPreviewBasisAdapter;

impl ForgeQueryRuntimePreviewBasisAdapter for PublicPreviewBasisAdapter {
    fn admit_preview_basis(
        &self,
        label: &ForgeQuerySessionLabel,
        effect_policy: ForgeQueryEffectPolicy,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryPreviewBasisAdmission, ForgeQueryWorkspaceError> {
        Ok(ForgeQueryPreviewBasisAdmission::new(
            authority,
            label.clone(),
            effect_policy,
            ForgeQueryBasisAdmissionEvidenceRow::rows_from_values(["public-graph-preview-basis"]),
        ))
    }
}

pub(super) struct PublicInspectorEvidenceAdapter;

impl ForgeQueryRuntimeInspectorEvidenceAdapter for PublicInspectorEvidenceAdapter {
    fn inspect_write_receipt(
        &self,
        receipt: &ForgeQueryWriteReceipt,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryRuntimeInspectionEvidence, ForgeQueryWorkspaceError> {
        Ok(ForgeQueryRuntimeInspectionEvidence::new(
            authority,
            "public-graph-write-receipt",
            receipt.authority_lane(),
            ["public-graph-inspector-evidence"],
        ))
    }
}
