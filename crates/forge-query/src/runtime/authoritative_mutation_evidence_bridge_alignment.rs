use forge_runtime_bridge::facade::{
    BridgeAuthoritativeMutationEvidenceCloseout, BridgeAuthoritativeMutationEvidenceSupport,
};

use super::ForgeQueryAuthoritativeMutationEvidenceSupport;

pub(super) fn assert_bridge_support_alignment(
    query_support: &ForgeQueryAuthoritativeMutationEvidenceSupport,
    bridge_support: &BridgeAuthoritativeMutationEvidenceSupport,
    bridge_closeout: &BridgeAuthoritativeMutationEvidenceCloseout,
) {
    let mut failures = Vec::new();

    for section in [
        "declared-resolved-target-evidence",
        "batch-session-causality-provenance",
        "existing-truth-binding",
        "same-batch-symbolic-target-reference",
        "naming-mutation-evidence",
        "continuity-mutation-evidence",
        "replay-safe-request-receipt-digests",
    ] {
        if !bridge_support
            .carry_forward_sections()
            .iter()
            .any(|bridge_section| bridge_section == section)
        {
            failures.push(format!("missing carry-forward section `{section}`"));
        }
    }

    for family in query_support.existing_truth_binding_families() {
        if !bridge_support
            .existing_truth_binding_families()
            .iter()
            .any(|bridge_family| bridge_family == family)
        {
            failures.push(format!("missing existing-truth binding family `{family}`"));
        }
    }
    for family in query_support.symbolic_target_reference_families() {
        if !bridge_support
            .symbolic_target_reference_families()
            .iter()
            .any(|bridge_family| bridge_family == family)
        {
            failures.push(format!("missing symbolic target family `{family}`"));
        }
    }
    for family in query_support.naming_mutation_families() {
        if !bridge_support
            .naming_mutation_families()
            .iter()
            .any(|bridge_family| bridge_family == family)
        {
            failures.push(format!("missing naming family `{family}`"));
        }
    }
    for family in query_support.continuity_mutation_families() {
        if !bridge_support
            .continuity_mutation_families()
            .iter()
            .any(|bridge_family| bridge_family == family)
        {
            failures.push(format!("missing continuity family `{family}`"));
        }
    }

    for section in [
        "aggregate_existing_truth_binding_digest",
        "aggregate_symbolic_target_reference_digest",
        "aggregate_naming_mutation_digest",
        "aggregate_continuity_mutation_digest",
        "aggregate_causality_digest",
        "aggregate_provenance_digest",
    ] {
        if !bridge_support
            .aggregate_evidence_sections()
            .iter()
            .any(|bridge_section| bridge_section == section)
        {
            failures.push(format!("missing aggregate evidence section `{section}`"));
        }
    }

    if !bridge_closeout
        .must_not_assume_yet()
        .iter()
        .any(|line| line.contains("existing-truth binding") && line.contains("fail-closed"))
    {
        failures.push(
            "bridge closeout does not fail-close unsupported existing-truth binding families"
                .to_string(),
        );
    }

    assert!(
        failures.is_empty(),
        "bridge/query authoritative mutation evidence drifted: {}",
        failures.join(", ")
    );
}
