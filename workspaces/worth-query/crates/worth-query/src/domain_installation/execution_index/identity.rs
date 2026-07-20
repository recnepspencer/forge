use std::collections::BTreeMap;

use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};

use super::semantic_keys::{
    InstalledDeclarationFamilySlot, InstalledDomainOwner, InstalledInvariantSlot,
};
use super::{InstalledGraphReadOperation, WorthQueryGraphReadOperationKey};

pub(super) fn execution_index_identity(
    packages: &[String],
    operations: &BTreeMap<WorthQueryGraphReadOperationKey, InstalledGraphReadOperation>,
    families: &BTreeMap<InstalledDeclarationFamilySlot, String>,
    policies: &BTreeMap<InstalledDomainOwner, Vec<String>>,
    invariants: &BTreeMap<InstalledInvariantSlot, String>,
    obligations: &[String],
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::InstalledDomainExecutionIndex)
        .field_value_sequence(WorthQueryEvidenceTag::new("package"), packages)
        .field_value_sequence(
            WorthQueryEvidenceTag::new("operation"),
            operations
                .values()
                .map(|operation| operation.registration.digest_part()),
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("family"),
            families
                .iter()
                .map(|(slot, meaning)| format!("{}:{meaning}", slot.terminal_projection())),
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("policy"),
            policies.iter().flat_map(|(owner, values)| {
                values
                    .iter()
                    .map(move |value| format!("{}:{value}", owner.as_str()))
            }),
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("invariant"),
            invariants
                .iter()
                .map(|(slot, meaning)| format!("{}:{meaning}", slot.terminal_projection())),
        )
        .field_value_sequence(WorthQueryEvidenceTag::new("obligation"), obligations)
        .seal()
}
