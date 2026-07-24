mod conditional_nodes;
mod input_and_graph_contracts;
mod lifecycle_and_support_contracts;
mod workflow_contract;

use sha2::{Digest, Sha256};

use crate::canonical_hash_encoding::hash_text_field;

use super::{WorthQueryDomainOperationIdentity, WorthQueryDomainOperationSemanticClosure};

pub(super) fn canonical_operation_identity(
    identity: &WorthQueryDomainOperationIdentity,
    semantics: &WorthQueryDomainOperationSemanticClosure,
) -> String {
    let mut hasher = Sha256::new();
    hash_text_field(&mut hasher, "operation-name", identity.name());
    hash_text_field(
        &mut hasher,
        "operation-version",
        &identity.version().to_string(),
    );
    input_and_graph_contracts::hash_input_and_graph_contracts(&mut hasher, semantics);
    hash_text_field(
        &mut hasher,
        "operation-evidence",
        &semantics.evidence.canonical_token(),
    );
    workflow_contract::hash_workflow_contract(&mut hasher, &semantics.workflow);
    lifecycle_and_support_contracts::hash_lifecycle_and_support_contracts(&mut hasher, semantics);
    format!("{:x}", hasher.finalize())
}

pub(super) fn hash_sequence<'a>(
    hasher: &mut Sha256,
    tag: &'static str,
    values: impl IntoIterator<Item = &'a str>,
) {
    for value in values {
        hash_text_field(hasher, tag, value);
    }
}

pub(super) fn bool_name(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}
