use forge_runtime_bridge::facade::{
    BridgeContinuityMutationBundle, BridgeContinuityOutcomeClass, BridgeNamingMutationBundle,
};

use super::{
    ForgeQueryContinuityMutationFamily, ForgeQueryContinuityMutationIntent,
    ForgeQueryContinuityMutationOutcomeClass, ForgeQueryNamingMutationFamily,
    ForgeQueryNamingMutationIntent,
};

pub(super) fn bridge_naming_mutation_bundle(
    intent: &ForgeQueryNamingMutationIntent,
    resolved_target_entity_identity: Option<&str>,
    target_collection: Option<&str>,
) -> Option<BridgeNamingMutationBundle> {
    match intent.family() {
        ForgeQueryNamingMutationFamily::AttachNewTarget => {
            resolved_target_entity_identity.map(|resolved_target_entity_identity| {
                BridgeNamingMutationBundle::attach_new_target(
                    intent.attachment_identity(),
                    resolved_target_entity_identity,
                    target_collection,
                )
            })
        }
        ForgeQueryNamingMutationFamily::AttachExistingTarget => resolved_target_entity_identity
            .and_then(|resolved_target_entity_identity| {
                intent
                    .target_authoritative_identity()
                    .map(|target_authoritative_identity| {
                        BridgeNamingMutationBundle::attach_existing_target(
                            intent.attachment_identity(),
                            target_authoritative_identity,
                            resolved_target_entity_identity,
                            target_collection,
                        )
                    })
            }),
        ForgeQueryNamingMutationFamily::RebindTarget => {
            resolved_target_entity_identity.and_then(|resolved_target_entity_identity| {
                intent
                    .prior_authoritative_identity()
                    .and_then(|prior_authoritative_identity| {
                        intent.target_authoritative_identity().map(
                            |target_authoritative_identity| {
                                BridgeNamingMutationBundle::rebind_target(
                                    intent.attachment_identity(),
                                    prior_authoritative_identity,
                                    target_authoritative_identity,
                                    resolved_target_entity_identity,
                                    target_collection,
                                )
                            },
                        )
                    })
            })
        }
        ForgeQueryNamingMutationFamily::Remove => {
            intent
                .prior_authoritative_identity()
                .map(|prior_authoritative_identity| {
                    BridgeNamingMutationBundle::remove(
                        intent.attachment_identity(),
                        prior_authoritative_identity,
                        resolved_target_entity_identity,
                        target_collection,
                    )
                })
        }
    }
}

pub(super) fn bridge_continuity_mutation_bundle(
    intent: &ForgeQueryContinuityMutationIntent,
    basis_binding_digest: Option<&str>,
    resolved_target_entity_identity: Option<&str>,
    target_collection: Option<&str>,
) -> Option<BridgeContinuityMutationBundle> {
    let outcome_class = match intent.outcome_class() {
        ForgeQueryContinuityMutationOutcomeClass::ContinuesAsSingleSuccessor => {
            BridgeContinuityOutcomeClass::ContinuesAsSingleSuccessor
        }
        ForgeQueryContinuityMutationOutcomeClass::ContinuesAsSplitSuccessors => {
            BridgeContinuityOutcomeClass::ContinuesAsSplitSuccessors
        }
        ForgeQueryContinuityMutationOutcomeClass::ContinuesViaTruthLoweredCanonicalMergeSuccessor => {
            BridgeContinuityOutcomeClass::ContinuesViaTruthLoweredCanonicalMergeSuccessor
        }
    };
    let lineage_digest = crate::identity::hash_parts(&[
        "forge-query-continuity-lineage-v1".to_string(),
        format!("family:{}", intent.family().as_str()),
        format!("outcome:{}", intent.outcome_class().as_str()),
        format!("prior:{}", intent.prior_authoritative_identity()),
        format!(
            "successors:{}",
            intent.successor_authoritative_identities().join("|")
        ),
        format!("basis-binding:{}", basis_binding_digest.unwrap_or("none")),
    ]);
    let continuity_resolution_digest = crate::identity::hash_parts(&[
        "forge-query-continuity-resolution-v1".to_string(),
        format!("lineage:{lineage_digest}"),
        format!(
            "successors:{}",
            intent.successor_authoritative_identities().join("|")
        ),
        format!("basis-binding:{}", basis_binding_digest.unwrap_or("none")),
        format!(
            "resolved:{}",
            resolved_target_entity_identity.unwrap_or("none")
        ),
        format!("collection:{}", target_collection.unwrap_or("none")),
    ]);

    match intent.family() {
        ForgeQueryContinuityMutationFamily::RebindExistingTarget => {
            Some(BridgeContinuityMutationBundle::rebind_existing_target(
                outcome_class,
                intent.prior_authoritative_identity(),
                Some(intent.successor_authoritative_identity()),
                basis_binding_digest,
                resolved_target_entity_identity,
                target_collection,
                lineage_digest,
                continuity_resolution_digest,
            ))
        }
        ForgeQueryContinuityMutationFamily::SplitExistingTarget => Some(
            BridgeContinuityMutationBundle::split_existing_target(
                outcome_class,
                intent.prior_authoritative_identity(),
                intent.successor_authoritative_identities().iter().cloned(),
                basis_binding_digest,
                resolved_target_entity_identity,
                target_collection,
                lineage_digest,
                continuity_resolution_digest,
            )
            .expect("validated split continuity intent should lower into bridge bundle"),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::bridge_continuity_mutation_bundle;
    use crate::runtime::{
        ForgeQueryContinuityMutationEvidence, ForgeQueryContinuityMutationIntent,
    };

    #[test]
    fn bridge_lowered_continuity_matches_query_intent_digest_shape() {
        let intent = ForgeQueryContinuityMutationIntent::rebind_merge_successor(
            "authority:task-1",
            "authority:task-1-successor",
        )
        .expect("continuity intent should build");

        let lowered = bridge_continuity_mutation_bundle(
            &intent,
            Some("binding:sha256:task-1"),
            Some("entity:task-1"),
            Some("Task"),
        )
        .expect("bridge continuity bundle should lower");

        let bridge_evidence = ForgeQueryContinuityMutationEvidence::from_bridge(&lowered);
        let intent_evidence = ForgeQueryContinuityMutationEvidence::from_intent(
            &intent,
            Some("binding:sha256:task-1"),
            Some("entity:task-1"),
            Some("Task"),
        );

        assert_eq!(
            bridge_evidence.lineage_digest(),
            intent_evidence.lineage_digest()
        );
        assert_eq!(
            bridge_evidence.continuity_resolution_digest(),
            intent_evidence.continuity_resolution_digest()
        );
    }
}
