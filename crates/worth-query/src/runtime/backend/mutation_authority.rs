use super::writeback_effect_intent::WorthQueryBridgeWritebackEffectIntent;
use crate::evidence_identity::{
    WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};
use crate::memory_workspace::{
    WorthQueryEntityIdentity, WorthQueryMutationKind, WorthQuerySnapshotIdentity,
    WorthQueryWorkspaceError,
};
use crate::runtime::{WorthQueryBackendAdmissibleMutation, WorthQueryExistingTruthBindingFamily};

use worth_runtime_bridge::facade::{
    BridgeDiagnosticsTier, BridgeExecutionPolicyClass,
    BridgeExistingTruthBindingAuthoritativeIdentity, BridgeExistingTruthBindingBundle,
    BridgeExistingTruthBindingResolvedTargetIdentity, BridgeExistingTruthBindingTargetCollection,
    BridgeMutationAuthorityBundle, BridgePolicyDeclaration, BridgePolicyDeclarationIdentity,
    BridgeRequestKind, BridgeRouteIdentity, BridgeWritebackAuthoritativeStateBasis,
    BridgeWritebackCausalityIdentity, BridgeWritebackDeclaration,
    BridgeWritebackDeclarationIdentity, BridgeWritebackEffectClass, BridgeWritebackEffectIdentity,
    BridgeWritebackFamilyKind, BridgeWritebackFeedbackProvenance, BridgeWritebackIdempotenceClass,
    BridgeWritebackIdempotenceIdentity, BridgeWritebackNativeCausalityInputs,
    BridgeWritebackStrategyClass, RelationalBridgeSnapshotIdentityParts, RuntimeBridge,
    TruthCommitIdentity, TruthSnapshotIdentity,
};

pub(crate) fn build_bridge_authority_bundle(
    bridge: &RuntimeBridge,
    snapshot_identity: &WorthQuerySnapshotIdentity,
    mutation: &WorthQueryBackendAdmissibleMutation,
    collection: &str,
    entity_identity: &WorthQueryEntityIdentity,
    mutation_kind: WorthQueryMutationKind,
) -> Result<BridgeMutationAuthorityBundle, WorthQueryWorkspaceError> {
    let writeback_identity = writeback_identity(
        mutation,
        collection,
        entity_identity,
        &mutation_kind,
        snapshot_identity,
    );
    let writeback_bridge_identity = writeback_identity.bridge_evidence_identity();
    let policy = bridge.lower_admitted_policy(
        &bridge
            .admit_policy_declaration(BridgePolicyDeclaration::new(
                BridgePolicyDeclarationIdentity::from_bridge_evidence(&writeback_bridge_identity),
                BridgeRequestKind::Authoritative,
                BridgeExecutionPolicyClass::DeterministicCanonical,
                BridgeDiagnosticsTier::Standard,
                true,
                true,
            ))
            .map_err(|error| WorthQueryWorkspaceError::new(format!("{error:?}")))?,
    );
    let contract = bridge
        .admit_writeback_declaration(
            BridgeWritebackDeclaration::writeback_capable(
                BridgeWritebackDeclarationIdentity::from_bridge_evidence(
                    &writeback_bridge_identity,
                ),
                BridgeRequestKind::Authoritative,
                BridgeWritebackFamilyKind::AspectReconciliation,
                BridgeWritebackEffectClass::AspectReconciliation,
                BridgeWritebackStrategyClass::AspectReconciliationCommit,
                BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
            ),
            &policy,
        )
        .map_err(|error| WorthQueryWorkspaceError::new(format!("{error:?}")))?;
    let Some(snapshot_parts) = snapshot_identity.relational_parts() else {
        return Err(WorthQueryWorkspaceError::new(
            "bridge writeback authority requires a relational snapshot identity",
        ));
    };
    let evaluation_snapshot = RelationalBridgeSnapshotIdentityParts::new(
        snapshot_parts.snapshot_id(),
        snapshot_parts.version_id().saturating_sub(1),
    );
    let causality = BridgeWritebackNativeCausalityInputs::new(
        BridgeWritebackCausalityIdentity::from_bridge_evidence(&writeback_bridge_identity),
        TruthCommitIdentity::from_relational_commit_id(snapshot_parts.version_id()),
        BridgeRouteIdentity::from_bridge_evidence(&writeback_bridge_identity),
        TruthSnapshotIdentity::from_relational_snapshot(evaluation_snapshot),
        TruthSnapshotIdentity::from_relational_snapshot(snapshot_parts),
    );
    let effect = bridge.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::from_bridge_evidence(&writeback_bridge_identity),
        WorthQueryBridgeWritebackEffectIntent::from_admitted_mutation(
            BridgeWritebackEffectClass::AspectReconciliation,
            mutation,
        )?
        .into_bridge_intent(),
    );
    let authoritative_state_basis = BridgeWritebackAuthoritativeStateBasis::from_effect(&effect);
    let feedback = BridgeWritebackFeedbackProvenance::new(&effect);
    let idempotence = bridge.classify_writeback_idempotence(
        &effect,
        &policy,
        &authoritative_state_basis,
        BridgeWritebackIdempotenceIdentity::from_bridge_evidence(&writeback_bridge_identity),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let (outcome, _) = bridge
        .execute_writeback_authority(&contract, &effect, &idempotence)
        .map_err(|error| WorthQueryWorkspaceError::new(format!("{error:?}")))?;
    let execution_record = bridge
        .diagnostics()
        .last_writeback_execution_record()
        .ok_or_else(|| {
            WorthQueryWorkspaceError::new("bridge writeback did not retain an execution record")
        })?;
    Ok(attach_existing_truth_binding(
        BridgeMutationAuthorityBundle::from_writeback_artifacts(
            &causality,
            &effect,
            &feedback,
            &execution_record,
            Some(&outcome),
        ),
        mutation,
    )?)
}

fn writeback_identity(
    mutation: &WorthQueryBackendAdmissibleMutation,
    collection: &str,
    entity_identity: &WorthQueryEntityIdentity,
    mutation_kind: &WorthQueryMutationKind,
    snapshot_identity: &WorthQuerySnapshotIdentity,
) -> WorthQueryEvidenceIdentity {
    let snapshot_evidence_identity = snapshot_identity.evidence_identity();
    let entity_evidence_identity = entity_identity.evidence_identity();
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::RuntimeBridgeWritebackAuthority)
        .field_value(WorthQueryEvidenceTag::new("collection"), collection)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("entity"),
            &entity_evidence_identity,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("snapshot_identity"),
            &snapshot_evidence_identity,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("mutation_family"),
            mutation.mutation_family().as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("mutation_kind"),
            mutation_kind_label(mutation_kind),
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("declared_aspect_operations"),
            mutation
                .declared_aspect_operations()
                .into_iter()
                .map(|operation| {
                    format!(
                        "{}:{}",
                        operation.kind(),
                        operation.aspect_touch().admitted_touch_digest_part()
                    )
                }),
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("aspect_values"),
            mutation
                .admitted_aspect_values()
                .iter()
                .map(|aspect| aspect.terminal_digest_material()),
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("asserted_aspect_values"),
            mutation
                .asserted_admitted_aspect_values()
                .iter()
                .map(|aspect| aspect.terminal_digest_material()),
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("mutation_metadata"),
            mutation
                .mutation_metadata()
                .entries()
                .map(|(key, value)| format!("{}={}", key.as_str(), value.terminal_digest_text())),
        )
        .seal()
}

fn mutation_kind_label(mutation_kind: &WorthQueryMutationKind) -> &'static str {
    match mutation_kind {
        WorthQueryMutationKind::Created => "created",
        WorthQueryMutationKind::Updated => "updated",
        WorthQueryMutationKind::Deleted => "deleted",
    }
}

fn attach_existing_truth_binding(
    bridge_authority: BridgeMutationAuthorityBundle,
    mutation: &WorthQueryBackendAdmissibleMutation,
) -> Result<BridgeMutationAuthorityBundle, WorthQueryWorkspaceError> {
    let Some(binding) = mutation.existing_truth_binding() else {
        return Ok(bridge_authority);
    };
    let authoritative_identity =
        BridgeExistingTruthBindingAuthoritativeIdentity::from_bridge_evidence(
            &binding
                .authoritative_identity()
                .evidence_identity()
                .bridge_evidence_identity(),
        );
    let target_collection = binding
        .target_collection_identity()
        .map(|collection| BridgeExistingTruthBindingTargetCollection::new(collection.as_str()));
    let bundle = match binding.family() {
        WorthQueryExistingTruthBindingFamily::DirectEntityIdentity => {
            let resolved_entity_identity = bridge_existing_truth_resolved_target_identity(
                binding.resolved_entity_identity(),
                "bridge existing-truth entity binding requires a relational resolved entity identity",
            )?;
            BridgeExistingTruthBindingBundle::direct_entity(
                authoritative_identity,
                resolved_entity_identity,
                target_collection,
            )
        }
        WorthQueryExistingTruthBindingFamily::DirectRelationIdentity => {
            let resolved_relation_identity = bridge_existing_truth_resolved_target_identity(
                binding.resolved_relation_identity(),
                "bridge existing-truth relation binding requires a relational resolved relation identity",
            )?;
            BridgeExistingTruthBindingBundle::direct_relation(
                authoritative_identity,
                resolved_relation_identity,
                target_collection,
            )
        }
    };
    Ok(bridge_authority.with_existing_truth_binding(bundle))
}

fn bridge_existing_truth_resolved_target_identity(
    resolved_target_identity: &WorthQueryEntityIdentity,
    message: &'static str,
) -> Result<BridgeExistingTruthBindingResolvedTargetIdentity, WorthQueryWorkspaceError> {
    resolved_target_identity
        .relational_record_parts()
        .map(BridgeExistingTruthBindingResolvedTargetIdentity::from_relational_record)
        .ok_or_else(|| WorthQueryWorkspaceError::new(message))
}
