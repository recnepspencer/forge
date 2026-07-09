use std::sync::Arc;

use super::{BridgeContinuityAuthoritativeIdentity, BridgeContinuityMutationFamily};
use crate::continuity::BridgeContinuityOutcomeClass;

pub(super) fn derive_optional_binding_basis_digest(
    prior_authoritative_identity: &str,
    resolved_target_entity_identity: Option<&str>,
    target_collection: Option<&str>,
) -> Option<Arc<str>> {
    if resolved_target_entity_identity.is_none() && target_collection.is_none() {
        return None;
    }

    Some(derive_continuity_mutation_digest(
        "bridge-continuity-mutation-binding-basis",
        [
            format!("prior-authority:{prior_authoritative_identity}"),
            format!(
                "resolved-target:{}",
                resolved_target_entity_identity.unwrap_or("none")
            ),
            format!("target-collection:{}", target_collection.unwrap_or("none")),
        ],
    ))
}

pub(super) fn derive_lineage_digest(
    family: BridgeContinuityMutationFamily,
    outcome_class: BridgeContinuityOutcomeClass,
    prior_authoritative_identity: &str,
    successor_authoritative_identities: &[BridgeContinuityAuthoritativeIdentity],
) -> Arc<str> {
    derive_continuity_mutation_digest(
        "bridge-continuity-mutation-lineage",
        [
            format!("family:{family:?}"),
            format!("outcome:{outcome_class:?}"),
            format!("prior-authority:{prior_authoritative_identity}"),
            format!(
                "successor-authorities:{}",
                format_successor_authoritative_identities(successor_authoritative_identities)
            ),
        ],
    )
}

pub(super) fn derive_resolution_digest(
    family: BridgeContinuityMutationFamily,
    outcome_class: BridgeContinuityOutcomeClass,
    prior_authoritative_identity: &str,
    successor_authoritative_identities: &[BridgeContinuityAuthoritativeIdentity],
    basis_binding_digest: Option<&Arc<str>>,
    resolved_target_entity_identity: Option<&str>,
    target_collection: Option<&str>,
) -> Arc<str> {
    derive_continuity_mutation_digest(
        "bridge-continuity-mutation-resolution",
        [
            format!("family:{family:?}"),
            format!("outcome:{outcome_class:?}"),
            format!("prior-authority:{prior_authoritative_identity}"),
            format!(
                "successor-authorities:{}",
                format_successor_authoritative_identities(successor_authoritative_identities)
            ),
            format!(
                "binding-basis:{}",
                basis_binding_digest
                    .map(|value| value.as_ref())
                    .unwrap_or("none")
            ),
            format!(
                "resolved-target:{}",
                resolved_target_entity_identity.unwrap_or("none")
            ),
            format!("target-collection:{}", target_collection.unwrap_or("none")),
        ],
    )
}

fn format_successor_authoritative_identities(
    successors: &[BridgeContinuityAuthoritativeIdentity],
) -> String {
    if successors.is_empty() {
        return "none".to_string();
    }
    successors
        .iter()
        .map(BridgeContinuityAuthoritativeIdentity::as_str)
        .collect::<Vec<_>>()
        .join("|")
}

fn derive_continuity_mutation_digest(
    digest_family: &'static str,
    entries: impl IntoIterator<Item = String>,
) -> Arc<str> {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(digest_family.as_bytes());
    for entry in entries {
        let entry_bytes = entry.as_bytes();
        hasher.update(entry_bytes.len().to_le_bytes());
        hasher.update(entry_bytes);
    }
    let digest = hasher.finalize();
    Arc::from(format!("{digest_family}:sha256:{digest:x}"))
}
