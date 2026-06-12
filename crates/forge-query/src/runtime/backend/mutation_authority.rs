use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::memory_workspace::{
    ForgeQueryEntityIdentity, ForgeQueryMutationKind, ForgeQuerySnapshotIdentity,
    ForgeQueryWorkspaceError,
};
use crate::runtime::{ForgeQueryExistingTruthBindingFamily, ForgeQueryWriteCommand};

use forge_runtime_bridge::facade::{
    BridgeDiagnosticsTier, BridgeExecutionPolicyClass,
    BridgeExistingTruthBindingAuthoritativeIdentity, BridgeExistingTruthBindingBundle,
    BridgeExistingTruthBindingResolvedTargetIdentity, BridgeExistingTruthBindingTargetCollection,
    BridgeMutationAuthorityBundle, BridgePolicyDeclaration, BridgePolicyDeclarationIdentity,
    BridgeRequestKind, BridgeRouteIdentity, BridgeWritebackAuthoritativeStateBasis,
    BridgeWritebackCausalityIdentity, BridgeWritebackDeclaration,
    BridgeWritebackDeclarationIdentity, BridgeWritebackEffectClass, BridgeWritebackEffectIdentity,
    BridgeWritebackEffectIntent, BridgeWritebackFamilyKind, BridgeWritebackFeedbackProvenance,
    BridgeWritebackIdempotenceClass, BridgeWritebackIdempotenceIdentity,
    BridgeWritebackNativeCausalityInputs, BridgeWritebackStrategyClass,
    RelationalBridgeSnapshotIdentityParts, RuntimeBridge, TruthCommitIdentity,
    TruthSnapshotIdentity,
};

pub(crate) fn build_bridge_authority_bundle(
    bridge: &RuntimeBridge,
    snapshot_identity: &ForgeQuerySnapshotIdentity,
    command: &ForgeQueryWriteCommand,
    collection: &str,
    entity_identity: &ForgeQueryEntityIdentity,
    mutation_kind: ForgeQueryMutationKind,
) -> Result<BridgeMutationAuthorityBundle, ForgeQueryWorkspaceError> {
    let writeback_identity = writeback_identity(
        command,
        collection,
        entity_identity,
        &mutation_kind,
        snapshot_identity,
    );
    let policy = bridge.lower_admitted_policy(
        &bridge
            .admit_policy_declaration(BridgePolicyDeclaration::new(
                BridgePolicyDeclarationIdentity::from_external_authority_evidence(
                    &writeback_identity,
                ),
                BridgeRequestKind::Authoritative,
                BridgeExecutionPolicyClass::DeterministicCanonical,
                BridgeDiagnosticsTier::Standard,
                true,
                true,
            ))
            .map_err(|error| ForgeQueryWorkspaceError::new(format!("{error:?}")))?,
    );
    let contract = bridge
        .admit_writeback_declaration(
            BridgeWritebackDeclaration::writeback_capable(
                BridgeWritebackDeclarationIdentity::from_external_authority_evidence(
                    &writeback_identity,
                ),
                BridgeRequestKind::Authoritative,
                BridgeWritebackFamilyKind::AspectReconciliation,
                BridgeWritebackEffectClass::AspectReconciliation,
                BridgeWritebackStrategyClass::AspectReconciliationCommit,
                BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
            ),
            &policy,
        )
        .map_err(|error| ForgeQueryWorkspaceError::new(format!("{error:?}")))?;
    let Some(snapshot_parts) = snapshot_identity.relational_parts() else {
        return Err(ForgeQueryWorkspaceError::new(
            "bridge writeback authority requires a relational snapshot identity",
        ));
    };
    let evaluation_snapshot = RelationalBridgeSnapshotIdentityParts::new(
        snapshot_parts.snapshot_id(),
        snapshot_parts.version_id().saturating_sub(1),
    );
    let causality = BridgeWritebackNativeCausalityInputs::new(
        BridgeWritebackCausalityIdentity::from_external_authority_evidence(&writeback_identity),
        TruthCommitIdentity::from_relational_commit_id(snapshot_parts.version_id()),
        BridgeRouteIdentity::from_external_authority_evidence(&writeback_identity),
        TruthSnapshotIdentity::from_relational_snapshot(evaluation_snapshot),
        TruthSnapshotIdentity::from_relational_snapshot(snapshot_parts),
    );
    let effect = bridge.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::from_external_authority_evidence(&writeback_identity),
        writeback_effect_intent(
            BridgeWritebackEffectClass::AspectReconciliation,
            &writeback_identity,
        ),
    );
    let authoritative_state_basis = BridgeWritebackAuthoritativeStateBasis::from_effect(&effect);
    let feedback = BridgeWritebackFeedbackProvenance::new(&effect);
    let idempotence = bridge.classify_writeback_idempotence(
        &effect,
        &policy,
        &authoritative_state_basis,
        BridgeWritebackIdempotenceIdentity::from_external_authority_evidence(&writeback_identity),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let (outcome, _) = bridge
        .execute_writeback_authority(&contract, &effect, &idempotence)
        .map_err(|error| ForgeQueryWorkspaceError::new(format!("{error:?}")))?;
    let execution_record = bridge
        .diagnostics()
        .last_writeback_execution_record()
        .ok_or_else(|| {
            ForgeQueryWorkspaceError::new("bridge writeback did not retain an execution record")
        })?;
    Ok(attach_existing_truth_binding(
        BridgeMutationAuthorityBundle::from_writeback_artifacts(
            &causality,
            &effect,
            &feedback,
            &execution_record,
            Some(&outcome),
        ),
        command,
    )?)
}

fn writeback_effect_intent(
    effect_class: BridgeWritebackEffectClass,
    writeback_identity: &ForgeQueryEvidenceIdentity,
) -> BridgeWritebackEffectIntent {
    BridgeWritebackEffectIntent::validated_scalar_patch(
        effect_class,
        forge_foundational::facade::AspectKey::new("forge.query.writeback")
            .expect("valid writeback effect aspect key"),
        forge_foundational::facade::AspectValue::String(
            writeback_identity.as_str().to_string().into(),
        ),
    )
    .expect("bridge writeback effect intent should validate")
}

fn writeback_identity(
    command: &ForgeQueryWriteCommand,
    collection: &str,
    entity_identity: &ForgeQueryEntityIdentity,
    mutation_kind: &ForgeQueryMutationKind,
    snapshot_identity: &ForgeQuerySnapshotIdentity,
) -> ForgeQueryEvidenceIdentity {
    let snapshot_evidence_identity = snapshot_identity.evidence_identity();
    let entity_evidence_identity = entity_identity.evidence_identity();
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::RuntimeBridgeWritebackAuthority)
        .field_value(ForgeQueryEvidenceTag::new("collection"), collection)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("entity"),
            &entity_evidence_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("snapshot_identity"),
            &snapshot_evidence_identity,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("mutation_family"),
            command.mutation_family().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("mutation_kind"),
            mutation_kind_label(mutation_kind),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("declared_aspect_operations"),
            command
                .declared_aspect_operations()
                .into_iter()
                .map(|operation| format!("{}:{}", operation.kind(), operation.aspect_path())),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("aspect_values"),
            command
                .aspect_values()
                .iter()
                .map(|aspect| format!("{}={}", aspect.aspect_path(), aspect.value())),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("asserted_aspect_values"),
            command
                .asserted_aspect_values()
                .iter()
                .map(|aspect| format!("{}={}", aspect.aspect_path(), aspect.value())),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("mutation_metadata"),
            command
                .mutation_metadata()
                .entries()
                .iter()
                .map(|(key, value)| format!("{key}={value}")),
        )
        .seal()
}

fn mutation_kind_label(mutation_kind: &ForgeQueryMutationKind) -> &'static str {
    match mutation_kind {
        ForgeQueryMutationKind::Created => "created",
        ForgeQueryMutationKind::Updated => "updated",
        ForgeQueryMutationKind::Deleted => "deleted",
    }
}

fn attach_existing_truth_binding(
    bridge_authority: BridgeMutationAuthorityBundle,
    command: &ForgeQueryWriteCommand,
) -> Result<BridgeMutationAuthorityBundle, ForgeQueryWorkspaceError> {
    let Some(binding) = command.existing_truth_binding() else {
        return Ok(bridge_authority);
    };
    let authoritative_identity =
        BridgeExistingTruthBindingAuthoritativeIdentity::from_external_authority_evidence(
            binding.authoritative_identity().evidence_identity(),
        );
    let target_collection = binding
        .target_collection()
        .map(BridgeExistingTruthBindingTargetCollection::new);
    let bundle = match binding.family() {
        ForgeQueryExistingTruthBindingFamily::DirectEntityIdentity => {
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
        ForgeQueryExistingTruthBindingFamily::DirectRelationIdentity => {
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
    resolved_target_identity: &ForgeQueryEntityIdentity,
    message: &'static str,
) -> Result<BridgeExistingTruthBindingResolvedTargetIdentity, ForgeQueryWorkspaceError> {
    resolved_target_identity
        .relational_record_parts()
        .map(BridgeExistingTruthBindingResolvedTargetIdentity::from_relational_record)
        .ok_or_else(|| ForgeQueryWorkspaceError::new(message))
}
