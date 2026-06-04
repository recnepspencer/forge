use forge_runtime_bridge::facade::{
    BridgeContinuityAuthoritativeIdentity, BridgeContinuityMutationBundle,
    BridgeContinuityOutcomeClass, BridgeContinuityResolvedTargetIdentity,
    BridgeContinuityTargetCollection, BridgeNamingMutationBundle,
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
    _basis_binding_digest: Option<&str>,
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
    let prior_authoritative_identity =
        continuity_authoritative_identity(intent.prior_authoritative_identity())?;
    let resolved_target_entity_identity = match resolved_target_entity_identity {
        Some(identity) => Some(continuity_resolved_target_identity(identity)?),
        None => None,
    };
    let target_collection = match target_collection {
        Some(collection) => Some(continuity_target_collection(collection)?),
        None => None,
    };

    match intent.family() {
        ForgeQueryContinuityMutationFamily::RebindExistingTarget => {
            BridgeContinuityMutationBundle::rebind_existing_target(
                outcome_class,
                prior_authoritative_identity,
                Some(continuity_authoritative_identity(
                    intent.successor_authoritative_identity(),
                )?),
                resolved_target_entity_identity,
                target_collection,
            )
            .ok()
        }
        ForgeQueryContinuityMutationFamily::SplitExistingTarget => Some(
            BridgeContinuityMutationBundle::split_existing_target(
                outcome_class,
                prior_authoritative_identity,
                intent
                    .successor_authoritative_identities()
                    .iter()
                    .map(|identity| continuity_authoritative_identity(identity))
                    .collect::<Option<Vec<_>>>()?,
                resolved_target_entity_identity,
                target_collection,
            )
            .expect("validated split continuity intent should lower into bridge bundle"),
        ),
    }
}

fn continuity_authoritative_identity(value: &str) -> Option<BridgeContinuityAuthoritativeIdentity> {
    BridgeContinuityAuthoritativeIdentity::new(value).ok()
}

fn continuity_resolved_target_identity(
    value: &str,
) -> Option<BridgeContinuityResolvedTargetIdentity> {
    BridgeContinuityResolvedTargetIdentity::new(value).ok()
}

fn continuity_target_collection(value: &str) -> Option<BridgeContinuityTargetCollection> {
    BridgeContinuityTargetCollection::new(value).ok()
}

#[cfg(test)]
mod tests {
    use super::bridge_continuity_mutation_bundle;
    use crate::runtime::{
        ForgeQueryContinuityMutationEvidence, ForgeQueryContinuityMutationIntent,
    };

    #[test]
    fn bridge_lowered_continuity_uses_bridge_native_digest_basis() {
        let intent = ForgeQueryContinuityMutationIntent::rebind_merge_successor(
            "authority:task-1",
            "authority:task-1-successor",
        )
        .expect("continuity intent should build");

        let lowered =
            bridge_continuity_mutation_bundle(&intent, None, Some("entity:task-1"), Some("Task"))
                .expect("bridge continuity bundle should lower");

        let bridge_evidence = ForgeQueryContinuityMutationEvidence::from_bridge(&lowered);

        assert_eq!(bridge_evidence.family(), intent.family());
        assert_eq!(
            bridge_evidence.prior_authoritative_identity(),
            intent.prior_authoritative_identity()
        );
        assert_eq!(
            bridge_evidence.successor_authoritative_identity(),
            Some(intent.successor_authoritative_identity())
        );
        assert_eq!(
            bridge_evidence.resolved_target_entity_identity(),
            Some("entity:task-1")
        );
        assert_eq!(bridge_evidence.target_collection(), Some("Task"));
        assert!(bridge_evidence
            .lineage_digest()
            .starts_with("bridge-continuity-mutation-lineage:sha256:"));
        assert!(bridge_evidence
            .continuity_resolution_digest()
            .starts_with("bridge-continuity-mutation-resolution:sha256:"));
    }
}
