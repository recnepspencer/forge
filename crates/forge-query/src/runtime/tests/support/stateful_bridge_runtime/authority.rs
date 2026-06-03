use super::super::*;
use crate::identity::hash_parts;
use crate::memory_workspace::ForgeQueryMutationKind;

use forge_runtime_bridge::facade::{
    BridgeDiagnosticsTier, BridgeExecutionPolicyClass, BridgeExistingTruthBindingBundle,
    BridgeMutationAuthorityBundle, BridgePolicyDeclaration, BridgePolicyDeclarationIdentity,
    BridgeRequestKind, BridgeWritebackAuthoritativeStateBasis, BridgeWritebackCausalityBasis,
    BridgeWritebackCausalityEvidence, BridgeWritebackCausalityIdentity, BridgeWritebackDeclaration,
    BridgeWritebackDeclarationIdentity, BridgeWritebackEffectClass, BridgeWritebackEffectIdentity,
    BridgeWritebackEffectIntent, BridgeWritebackFamilyKind, BridgeWritebackFeedbackProvenance,
    BridgeWritebackIdempotenceClass, BridgeWritebackIdempotenceIdentity,
    BridgeWritebackStrategyClass, RuntimeBridge,
};

pub(super) fn build_bridge_authority_bundle(
    bridge: &RuntimeBridge,
    snapshot_token: &str,
    command: &ForgeQueryWriteCommand,
    collection: &str,
    entity_identity: &str,
    mutation_kind: ForgeQueryMutationKind,
) -> Result<BridgeMutationAuthorityBundle, ForgeQueryWorkspaceError> {
    let writeback_digest = writeback_digest(command, collection, entity_identity, &mutation_kind);
    let policy = bridge.lower_admitted_policy(
        &bridge
            .admit_policy_declaration(BridgePolicyDeclaration::new(
                BridgePolicyDeclarationIdentity::new(format!("policy:{writeback_digest}")),
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
                BridgeWritebackDeclarationIdentity::new(format!(
                    "stateful-bridge:{writeback_digest}"
                )),
                BridgeRequestKind::Authoritative,
                BridgeWritebackFamilyKind::AspectReconciliation,
                BridgeWritebackEffectClass::AspectReconciliation,
                BridgeWritebackStrategyClass::AspectReconciliationCommit,
                BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
            ),
            &policy,
        )
        .map_err(|error| ForgeQueryWorkspaceError::new(format!("{error:?}")))?;
    let causality = BridgeWritebackCausalityBasis::from_evidence(
        BridgeWritebackCausalityIdentity::new(format!("causality:{writeback_digest}")),
        BridgeWritebackCausalityEvidence::from_native_bases(
            format!("truth-trigger:{writeback_digest}"),
            "route:forge-query-stateful-bridge",
            format!("evaluation:{collection}:{entity_identity}"),
            snapshot_token.to_string(),
        ),
    );
    let effect = bridge.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new(format!("effect:{writeback_digest}")),
        writeback_effect_intent(
            BridgeWritebackEffectClass::AspectReconciliation,
            &writeback_digest,
        ),
    );
    let authoritative_state_basis = BridgeWritebackAuthoritativeStateBasis::from_effect(&effect);
    let feedback = BridgeWritebackFeedbackProvenance::new(&effect);
    let idempotence = bridge.classify_writeback_idempotence(
        &effect,
        &policy,
        &authoritative_state_basis,
        BridgeWritebackIdempotenceIdentity::new(format!("idempotence:{writeback_digest}")),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let (outcome, _) = bridge
        .execute_writeback_authority(&contract, &effect, &idempotence)
        .map_err(|error| ForgeQueryWorkspaceError::new(format!("{error:?}")))?;
    let execution_record = bridge
        .diagnostics()
        .last_writeback_execution_record()
        .ok_or_else(|| {
            ForgeQueryWorkspaceError::new(
                "stateful bridge writeback did not retain an execution record",
            )
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
    ))
}

fn writeback_effect_intent(
    effect_class: BridgeWritebackEffectClass,
    writeback_digest: &str,
) -> BridgeWritebackEffectIntent {
    BridgeWritebackEffectIntent::validated_scalar_patch(
        effect_class,
        forge_foundational::facade::AspectKey::new("forge.query.writeback")
            .expect("valid writeback effect aspect key"),
        forge_foundational::facade::AspectValue::String(writeback_digest.to_string().into()),
    )
    .expect("stateful bridge writeback effect intent should validate")
}

fn writeback_digest(
    command: &ForgeQueryWriteCommand,
    collection: &str,
    entity_identity: &str,
    mutation_kind: &ForgeQueryMutationKind,
) -> String {
    hash_parts(
        &std::iter::once("forge-query-stateful-bridge-writeback-v1".to_string())
            .chain(std::iter::once(format!("collection:{collection}")))
            .chain(std::iter::once(format!("entity:{entity_identity}")))
            .chain(std::iter::once(format!(
                "mutation-family:{:?}",
                command.mutation_family()
            )))
            .chain(std::iter::once(format!("mutation-kind:{mutation_kind:?}")))
            .chain(
                command
                    .declared_aspect_operations()
                    .into_iter()
                    .map(|operation| {
                        format!("aspect:{}:{}", operation.kind(), operation.aspect_path())
                    }),
            )
            .chain(
                command
                    .aspect_values()
                    .iter()
                    .map(|aspect| format!("value:{}={}", aspect.aspect_path(), aspect.value())),
            )
            .chain(
                command
                    .asserted_aspect_values()
                    .iter()
                    .map(|aspect| format!("asserted:{}={}", aspect.aspect_path(), aspect.value())),
            )
            .chain(
                command
                    .mutation_metadata()
                    .entries()
                    .iter()
                    .map(|(key, value)| format!("metadata:{key}={value}")),
            )
            .collect::<Vec<_>>(),
    )
}

fn attach_existing_truth_binding(
    bridge_authority: BridgeMutationAuthorityBundle,
    command: &ForgeQueryWriteCommand,
) -> BridgeMutationAuthorityBundle {
    let Some(binding) = command.existing_truth_binding() else {
        return bridge_authority;
    };
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
}
