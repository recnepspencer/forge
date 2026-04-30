use super::*;
use crate::declarative_live::{
    declare_writeback_from_live_session, DeclarativeWritebackChange, DeclarativeWritebackIntent,
    DeclarativeWritebackValue,
};
use crate::live::BridgeFieldDelta;
use crate::view_shape_live::execute_live_view_shape_change;
use forge_runtime_bridge::facade::{
    BridgeDiagnosticsTier, BridgeExecutionPolicyClass, BridgeExistingTruthBindingBundle,
    BridgePolicyDeclaration, BridgePolicyDeclarationIdentity, BridgeRequestKind,
    BridgeWritebackCausalityBasis, BridgeWritebackCausalityIdentity, BridgeWritebackEffectIdentity,
    BridgeWritebackFeedbackProvenance, BridgeWritebackIdempotenceClass,
    BridgeWritebackIdempotenceIdentity,
};

impl ForgeQueryMutationKind {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Updated => "updated",
            Self::Deleted => "deleted",
        }
    }
}

impl ForgeQueryMemoryApp {
    pub(super) fn execute_query_writeback(
        &self,
        collection: &str,
        kind: ForgeQueryMutationKind,
        aspect_paths: Vec<String>,
        payload: &Value,
        operation: ForgeQueryPendingOperation,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let session = self
            .live_views
            .values()
            .find(|view| view.session.request().target() == collection)
            .ok_or_else(|| {
                ForgeQueryWorkspaceError::new(format!(
                    "no live query session declared for writeback collection `{collection}`"
                ))
            })?;
        let changes = if aspect_paths.is_empty() {
            vec![DeclarativeWritebackChange::new(
                "mutation",
                kind.as_str(),
                DeclarativeWritebackValue::StructuredJson(payload.to_string()),
            )]
        } else {
            aspect_paths
                .iter()
                .map(|path| {
                    let (aspect, field) = split_aspect_path(path);
                    DeclarativeWritebackChange::new(
                        aspect,
                        field,
                        DeclarativeWritebackValue::StructuredJson(payload.to_string()),
                    )
                })
                .collect()
        };
        let artifact = declare_writeback_from_live_session(
            &session.session,
            DeclarativeWritebackIntent::new(changes),
        )
        .map_err(|error| ForgeQueryWorkspaceError::new(format!("{error:?}")))?;
        let policy_contract = self
            .bridge
            .admit_policy_declaration(BridgePolicyDeclaration::new(
                BridgePolicyDeclarationIdentity::new(format!(
                    "policy:forge-query-memory:{}:{}",
                    collection,
                    artifact.artifact_digest()
                )),
                BridgeRequestKind::Authoritative,
                BridgeExecutionPolicyClass::DeterministicCanonical,
                BridgeDiagnosticsTier::Standard,
                true,
                true,
            ))
            .map_err(|error| ForgeQueryWorkspaceError::new(format!("{error:?}")))?;
        let lowered_policy = self.bridge.lower_admitted_policy(&policy_contract);
        let contract = self
            .bridge
            .admit_writeback_declaration(
                artifact.declaration().bridge_declaration().clone(),
                &lowered_policy,
            )
            .map_err(|error| ForgeQueryWorkspaceError::new(format!("{error:?}")))?;
        let effect_digest = crate::identity::hash_parts(&[
            format!("collection:{collection}"),
            format!("kind:{}", kind.as_str()),
            format!("payload:{payload}"),
            format!("intent:{}", artifact.intent_digest()),
        ]);
        let causality = BridgeWritebackCausalityBasis::new(
            BridgeWritebackCausalityIdentity::new(format!("causality:{effect_digest}")),
            format!("truth-trigger:{effect_digest}"),
            "route:forge-query-memory",
            artifact.live_view_basis_digest(),
            self.snapshot_token(),
        );
        let effect = self.bridge.lower_writeback_effect(
            &contract,
            &causality,
            BridgeWritebackEffectIdentity::new(format!("effect:{effect_digest}")),
            format!("effect:{effect_digest}"),
        );
        let feedback_provenance = BridgeWritebackFeedbackProvenance::new(&effect);
        let idempotence = self.bridge.classify_writeback_idempotence(
            &effect,
            &lowered_policy,
            self.snapshot_token(),
            BridgeWritebackIdempotenceIdentity::new(format!("idempotence:{effect_digest}")),
            BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
        );
        let effect_key = format!("effect:{effect_digest}");
        {
            let mut state = self.authority_state.lock().map_err(|_| {
                ForgeQueryWorkspaceError::new("query memory authority lock poisoned")
            })?;
            state.pending.insert(
                effect_key,
                ForgeQueryPendingWriteback {
                    collection: collection.to_string(),
                    kind,
                    aspect_paths,
                    operation: operation.clone(),
                },
            );
        }
        let (outcome, truth_receipt) = self
            .bridge
            .execute_writeback_authority(&contract, &effect, &idempotence)
            .map_err(|error| ForgeQueryWorkspaceError::new(format!("{error:?}")))?;
        let execution_record = self
            .bridge
            .diagnostics()
            .last_writeback_execution_record()
            .ok_or_else(|| {
                ForgeQueryWorkspaceError::new(
                    "writeback authority did not retain a diagnostics execution record",
                )
            })?;
        let bridge_authority = BridgeMutationAuthorityBundle::from_writeback_artifacts(
            &causality,
            &effect,
            &feedback_provenance,
            &execution_record,
            Some(&outcome),
        );
        let bridge_authority = attach_existing_truth_binding(bridge_authority, &operation);
        self.authority_state
            .lock()
            .map_err(|_| ForgeQueryWorkspaceError::new("query memory authority lock poisoned"))?
            .completed
            .remove(truth_receipt.authoritative_artifact_digest())
            .map(|mut receipt| {
                receipt.bridge_authority = Some(bridge_authority);
                receipt
            })
            .ok_or_else(|| {
                ForgeQueryWorkspaceError::new(format!(
                    "writeback authority did not publish receipt `{}`",
                    truth_receipt.authoritative_artifact_digest()
                ))
            })
    }

    pub(super) fn deliver_live_patches(&mut self, receipt: &ForgeQueryMutationReceipt) {
        for delta in &receipt.deltas {
            let change = bridge_change_from_delta(delta);
            for (view_name, view) in self.live_views.iter_mut() {
                if view.session.request().target() != delta.collection {
                    continue;
                }
                let Ok(execution) =
                    execute_live_view_shape_change(view.session.live_view(), &change)
                else {
                    continue;
                };
                view.session
                    .advance_live_view(execution.next_live_view().clone());
                view.patches.push(ForgeQueryLivePatch {
                    view_name: view_name.clone(),
                    commit_identity: receipt.commit_identity.clone(),
                    entity_identity: delta.entity_identity.clone(),
                    mutation_kind: delta.kind.clone(),
                    aspect_paths: delta.aspect_paths.clone(),
                    envelope: execution.patch_envelope().clone(),
                });
            }
        }
    }
}

fn attach_existing_truth_binding(
    bridge_authority: BridgeMutationAuthorityBundle,
    operation: &ForgeQueryPendingOperation,
) -> BridgeMutationAuthorityBundle {
    let binding = match operation {
        ForgeQueryPendingOperation::Update {
            existing_truth_binding,
            ..
        }
        | ForgeQueryPendingOperation::Delete {
            existing_truth_binding,
            ..
        } => existing_truth_binding.as_ref(),
        ForgeQueryPendingOperation::Insert { .. } => None,
    };

    binding.map_or(bridge_authority.clone(), |binding| {
        let bundle = match binding.family() {
            crate::runtime::ForgeQueryExistingTruthBindingFamily::DirectEntityIdentity => {
                BridgeExistingTruthBindingBundle::direct_entity(
                    binding.authoritative_identity(),
                    binding.resolved_entity_identity(),
                    binding.target_collection(),
                )
            }
            crate::runtime::ForgeQueryExistingTruthBindingFamily::DirectRelationIdentity => {
                BridgeExistingTruthBindingBundle::direct_relation(
                    binding.authoritative_identity(),
                    binding.resolved_relation_identity(),
                    binding.target_collection(),
                )
            }
        };
        bridge_authority.with_existing_truth_binding(bundle)
    })
}

fn bridge_change_from_delta(delta: &ForgeQueryMutationDelta) -> BridgeChangeSummary {
    let mut change = match delta.kind {
        ForgeQueryMutationKind::Created => {
            BridgeChangeSummary::default().with_membership_transition(false, true)
        }
        ForgeQueryMutationKind::Updated => BridgeChangeSummary::default(),
        ForgeQueryMutationKind::Deleted => {
            BridgeChangeSummary::default().with_membership_transition(true, false)
        }
    };
    for path in &delta.aspect_paths {
        let (aspect, field) = split_aspect_path(path);
        change = change.with_field_delta(BridgeFieldDelta::new(
            aspect.to_string(),
            field.to_string(),
            None::<String>,
            None::<String>,
        ));
    }
    change
}

fn split_aspect_path(path: &str) -> (&str, &str) {
    path.split_once('.').unwrap_or((path, "value"))
}
