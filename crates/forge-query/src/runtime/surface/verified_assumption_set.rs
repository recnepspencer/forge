use crate::identity::hash_parts;
use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryVerificationReadSetBreadth {
    target_binding_count: usize,
    asserted_aspect_count: usize,
    distinct_asserted_aspect_path_count: usize,
    cleared_assertion_count: usize,
    counter_snapshot: String,
}

impl ForgeQueryVerificationReadSetBreadth {
    pub(in crate::runtime) fn new(
        target_binding_count: usize,
        asserted_aspect_count: usize,
        asserted_aspect_paths: &[String],
        cleared_assertion_count: usize,
    ) -> Self {
        let distinct_asserted_aspect_path_count =
            asserted_aspect_paths.iter().collect::<BTreeSet<_>>().len();
        let counter_snapshot = format!(
            "target_bindings={target_binding_count};asserted_aspects={asserted_aspect_count};distinct_asserted_aspect_paths={distinct_asserted_aspect_path_count};cleared_assertions={cleared_assertion_count}"
        );
        Self {
            target_binding_count,
            asserted_aspect_count,
            distinct_asserted_aspect_path_count,
            cleared_assertion_count,
            counter_snapshot,
        }
    }

    pub fn target_binding_count(&self) -> usize {
        self.target_binding_count
    }

    pub fn asserted_aspect_count(&self) -> usize {
        self.asserted_aspect_count
    }

    pub fn distinct_asserted_aspect_path_count(&self) -> usize {
        self.distinct_asserted_aspect_path_count
    }

    pub fn cleared_assertion_count(&self) -> usize {
        self.cleared_assertion_count
    }

    pub fn counter_snapshot(&self) -> &str {
        &self.counter_snapshot
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryVerifiedAssumptionSet {
    binding_digest: String,
    asserted_aspect_paths: Vec<String>,
    assumption_snapshot_token: String,
    assumption_snapshot_digest: String,
    verified_precondition_digest: String,
    verification_read_set_breadth: ForgeQueryVerificationReadSetBreadth,
    verified_assumption_digest: String,
}

impl ForgeQueryVerifiedAssumptionSet {
    pub(in crate::runtime) fn new(
        binding_digest: impl Into<String>,
        asserted_aspect_paths: Vec<String>,
        assumed_value_fragments: Vec<String>,
        cleared_assertion_count: usize,
        snapshot_token: &str,
    ) -> Self {
        let binding_digest = binding_digest.into();
        let assumption_snapshot_token = snapshot_token.to_string();
        let verification_read_set_breadth = ForgeQueryVerificationReadSetBreadth::new(
            1,
            asserted_aspect_paths.len(),
            &asserted_aspect_paths,
            cleared_assertion_count,
        );
        let assumption_snapshot_digest = hash_parts(&[
            "forge_query_existing_truth_assumption_snapshot_v1".to_string(),
            format!("binding:{binding_digest}"),
            format!("snapshot:{assumption_snapshot_token}"),
        ]);
        let verified_precondition_digest = hash_parts(
            &std::iter::once("forge_query_existing_truth_verified_precondition_v1".to_string())
                .chain(std::iter::once(format!(
                    "assumption-snapshot:{assumption_snapshot_digest}"
                )))
                .chain(assumed_value_fragments)
                .collect::<Vec<_>>(),
        );
        let verified_assumption_digest = hash_parts(&[
            "forge_query_existing_truth_verified_assumption_set_v1".to_string(),
            format!("binding:{binding_digest}"),
            format!("assumption-snapshot:{assumption_snapshot_digest}"),
            format!("precondition:{verified_precondition_digest}"),
            format!(
                "paths:{}",
                if asserted_aspect_paths.is_empty() {
                    "none".to_string()
                } else {
                    asserted_aspect_paths.join("|")
                }
            ),
            format!(
                "read-set:{}",
                verification_read_set_breadth.counter_snapshot()
            ),
        ]);
        Self {
            binding_digest,
            asserted_aspect_paths,
            assumption_snapshot_token,
            assumption_snapshot_digest,
            verified_precondition_digest,
            verification_read_set_breadth,
            verified_assumption_digest,
        }
    }

    pub fn binding_digest(&self) -> &str {
        &self.binding_digest
    }

    pub fn asserted_aspect_paths(&self) -> &[String] {
        &self.asserted_aspect_paths
    }

    pub fn assumption_snapshot_token(&self) -> &str {
        &self.assumption_snapshot_token
    }

    pub fn assumption_snapshot_digest(&self) -> &str {
        &self.assumption_snapshot_digest
    }

    pub fn verified_precondition_digest(&self) -> &str {
        &self.verified_precondition_digest
    }

    pub fn verification_read_set_breadth(&self) -> &ForgeQueryVerificationReadSetBreadth {
        &self.verification_read_set_breadth
    }

    pub fn verified_assumption_digest(&self) -> &str {
        &self.verified_assumption_digest
    }
}
