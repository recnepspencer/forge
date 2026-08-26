mod conditional_nodes;
mod input_and_graph_contracts;
mod lifecycle_and_support_contracts;
mod workflow_contract;

use sha2::{Digest, Sha256};

use crate::canonical_hash_encoding::{
    hash_text_field, CanonicalHashByteCounter, CanonicalHashSink,
};

use super::{WorthQueryDomainOperationIdentity, WorthQueryDomainOperationSemanticClosure};

pub(super) fn canonical_operation_identity(
    identity: &WorthQueryDomainOperationIdentity,
    semantics: &WorthQueryDomainOperationSemanticClosure,
) -> String {
    let mut hasher = Sha256::new();
    append_operation_identity(&mut hasher, identity, semantics);
    format!("{:x}", hasher.finalize())
}

pub(crate) fn canonical_operation_encoded_bytes(
    identity: &WorthQueryDomainOperationIdentity,
    semantics: &WorthQueryDomainOperationSemanticClosure,
) -> u64 {
    let mut counter = CanonicalHashByteCounter::default();
    append_operation_identity(&mut counter, identity, semantics);
    counter.bytes()
}

fn append_operation_identity(
    hasher: &mut impl CanonicalHashSink,
    identity: &WorthQueryDomainOperationIdentity,
    semantics: &WorthQueryDomainOperationSemanticClosure,
) {
    hash_text_field(hasher, "operation-name", identity.name());
    hash_text_field(hasher, "operation-version", &identity.version().to_string());
    input_and_graph_contracts::hash_input_and_graph_contracts(hasher, semantics);
    for family in semantics.decision_facts.required_families() {
        hash_text_field(hasher, "decision-fact-family", &family.canonical_token());
    }
    hash_text_field(
        hasher,
        "operation-evidence",
        &semantics.evidence.canonical_token(),
    );
    workflow_contract::hash_workflow_contract(hasher, &semantics.workflow);
    hash_text_field(
        hasher,
        "operation-resources",
        &semantics.resources.canonical_token(),
    );
    lifecycle_and_support_contracts::hash_lifecycle_and_support_contracts(hasher, semantics);
}

pub(super) fn hash_sequence<'a>(
    hasher: &mut impl CanonicalHashSink,
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
