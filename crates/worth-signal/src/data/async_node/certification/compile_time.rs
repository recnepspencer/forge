use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::data::error::SignalError;

use super::{canonical_digest, ASYNC_NODE_COMPILE_TIME_BOUNDARY_PROOF_SCHEMA_VERSION};

pub const REQUIRED_ASYNC_NODE_COMPILE_TIME_FIXTURES: [&str; 5] = [
    "validated_async_node_capability_declaration_fields_are_private",
    "lowered_async_node_capability_bundle_fields_are_private",
    "async_capable_node_fields_are_private",
    "async_node_request_intent_constructor_is_private",
    "async_node_revalidation_intent_constructor_is_private",
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AsyncNodeCompileTimeBoundaryProof {
    fixture_labels: Vec<String>,
    proof_digest: String,
}

impl AsyncNodeCompileTimeBoundaryProof {
    pub fn fixture_labels(&self) -> &[String] {
        &self.fixture_labels
    }

    pub fn proof_digest(&self) -> &str {
        &self.proof_digest
    }
}

pub fn async_node_compile_time_boundary_proof<I, S>(
    fixture_labels: I,
) -> Result<AsyncNodeCompileTimeBoundaryProof, SignalError>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let labels = fixture_labels
        .into_iter()
        .map(Into::into)
        .collect::<BTreeSet<_>>();
    let missing = REQUIRED_ASYNC_NODE_COMPILE_TIME_FIXTURES
        .iter()
        .filter(|label| !labels.contains(**label))
        .copied()
        .collect::<Vec<_>>();
    if !missing.is_empty() {
        return Err(SignalError::invalid_input(format!(
            "missing required async-node compile-time fixtures: {}",
            missing.join(", ")
        )));
    }
    let fixture_labels = labels.into_iter().collect::<Vec<_>>();
    Ok(AsyncNodeCompileTimeBoundaryProof {
        proof_digest: canonical_digest(&(
            ASYNC_NODE_COMPILE_TIME_BOUNDARY_PROOF_SCHEMA_VERSION,
            &fixture_labels,
        )),
        fixture_labels,
    })
}
