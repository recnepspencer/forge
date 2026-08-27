use super::*;

#[test]
fn pair_table_is_total_and_symmetric() {
    for left in UiAllocationStreamFamily::ALL {
        for right in UiAllocationStreamFamily::ALL {
            assert_eq!(pair_contract(left, right), pair_contract(right, left));
        }
    }
}

#[test]
fn terminal_commit_cannot_absorb_preserve_every_input_semantics() {
    let mut payload_counters = crate::evidence::UiAllocationStreamPolicyPayloadCounters::default();
    assert_eq!(
        resolve_stream_families(
            &[
                UiAllocationStreamFamily::TextInput,
                UiAllocationStreamFamily::DurableResize,
            ],
            &mut payload_counters,
        ),
        UiAllocationStreamCommitDecision::Denied(
            UiAllocationStreamCompositionDenial::IllegalFamilyPair {
                left: UiAllocationStreamFamily::TextInput,
                right: UiAllocationStreamFamily::DurableResize,
            },
        )
    );
}

#[test]
fn illegal_collapse_pair_denies_in_both_caller_orders() {
    assert_eq!(
        pair_contract(
            UiAllocationStreamFamily::DurableResize,
            UiAllocationStreamFamily::TextInput,
        ),
        Err(UiAllocationStreamCompositionDenial::IllegalFamilyPair {
            left: UiAllocationStreamFamily::TextInput,
            right: UiAllocationStreamFamily::DurableResize,
        }),
    );
}

#[test]
fn viewport_commit_lane_is_exclusive_across_the_complete_family_table() {
    for other in UiAllocationStreamFamily::ALL {
        let result = pair_contract(UiAllocationStreamFamily::ViewportObservation, other);
        if other == UiAllocationStreamFamily::ViewportObservation {
            let contract = result.expect("viewport self-composition remains admitted");
            assert_eq!(
                contract.resolved().commit_lane(),
                UiAllocationResolvedCommitLane::ViewportDerived
            );
        } else {
            assert!(matches!(
                result,
                Err(UiAllocationStreamCompositionDenial::IllegalFamilyPair { .. })
            ));
        }
    }
}

#[test]
fn commit_lane_participates_in_canonical_policy_identity() {
    let ordinary = resolved_family_policy(UiAllocationStreamFamily::HostMeasurementReplacement);
    let viewport_lane = UiResolvedAllocationStreamPolicy {
        commit_lane: UiAllocationResolvedCommitLane::ViewportDerived,
        ..ordinary
    };

    assert_ne!(ordinary, viewport_lane);
    assert_ne!(
        ordinary.mix_canonical_identity(0xcbf29ce484222325),
        viewport_lane.mix_canonical_identity(0xcbf29ce484222325),
    );
}

#[test]
fn canonical_n_way_resolution_retains_each_pair_verdict() {
    let mut payload_counters = crate::evidence::UiAllocationStreamPolicyPayloadCounters::default();
    let decision = resolve_stream_families(
        &[
            UiAllocationStreamFamily::ResizePreview,
            UiAllocationStreamFamily::QueryProjection,
            UiAllocationStreamFamily::TextInput,
        ],
        &mut payload_counters,
    );
    let UiAllocationStreamCommitDecision::Commit(receipt) = decision else {
        panic!("mixed frame must resolve");
    };
    assert_eq!(
        receipt.families(),
        &[
            UiAllocationStreamFamily::TextInput,
            UiAllocationStreamFamily::QueryProjection,
            UiAllocationStreamFamily::ResizePreview,
        ]
    );
    assert_eq!(receipt.intermediate.len(), 3);
    assert_eq!(receipt.branches.len(), 2);
    assert_eq!(receipt.policy.budget().ingress_window(), 16);
    assert_eq!(receipt.policy.budget().max_durable_mutations(), 16);
}

#[test]
fn input_budget_is_a_reachable_typed_denial() {
    let entries = [UiAllocationStreamFamily::TextInput; 17];
    let mut payload_counters = crate::evidence::UiAllocationStreamPolicyPayloadCounters::default();
    assert_eq!(
        resolve_stream_families(&entries, &mut payload_counters),
        UiAllocationStreamCommitDecision::Denied(
            UiAllocationStreamCompositionDenial::InputBudgetExceeded {
                admitted: 17,
                allowed: 16,
            },
        )
    );
}

#[test]
fn viewport_budget_matches_the_complete_bounded_source_courtroom() {
    let admitted = [UiAllocationStreamFamily::ViewportObservation; 64];
    let mut admitted_counters = crate::evidence::UiAllocationStreamPolicyPayloadCounters::default();
    let UiAllocationStreamCommitDecision::Commit(receipt) =
        resolve_stream_families(&admitted, &mut admitted_counters)
    else {
        panic!("every source in the bounded allocation courtroom must be admissible");
    };
    assert_eq!(receipt.policy.budget().ingress_window(), 64);
    assert_eq!(receipt.policy.budget().max_committed_receipts(), 64);
    assert_eq!(receipt.policy.budget().max_invalidation_targets(), 128);

    let denied = [UiAllocationStreamFamily::ViewportObservation; 65];
    let mut denied_counters = crate::evidence::UiAllocationStreamPolicyPayloadCounters::default();
    assert_eq!(
        resolve_stream_families(&denied, &mut denied_counters),
        UiAllocationStreamCommitDecision::Denied(
            UiAllocationStreamCompositionDenial::InputBudgetExceeded {
                admitted: 65,
                allowed: 64,
            },
        )
    );
}

#[test]
fn preview_and_terminal_resize_preserve_a_distinct_composed_lane() {
    let mut counters = crate::evidence::UiAllocationStreamPolicyPayloadCounters::default();
    let UiAllocationStreamCommitDecision::Commit(receipt) = resolve_stream_families(
        &[
            UiAllocationStreamFamily::ResizePreview,
            UiAllocationStreamFamily::DurableResize,
        ],
        &mut counters,
    ) else {
        panic!("preview plus terminal resize must compose");
    };
    assert_eq!(
        receipt.policy.commit_lane(),
        UiAllocationResolvedCommitLane::DragResize
    );
    assert_eq!(receipt.policy.budget().max_durable_mutations(), 1);
    assert_eq!(receipt.policy.budget().max_committed_receipts(), 1);
    assert_eq!(receipt.policy.budget().max_invalidation_targets(), 8);
}

#[test]
fn declared_policy_join_is_associative_for_every_family_triple() {
    for left in UiAllocationStreamFamily::ALL {
        for middle in UiAllocationStreamFamily::ALL {
            for right in UiAllocationStreamFamily::ALL {
                let viewport_count = [left, middle, right]
                    .into_iter()
                    .filter(|family| *family == UiAllocationStreamFamily::ViewportObservation)
                    .count();
                if viewport_count != 0 && viewport_count != 3 {
                    continue;
                }
                let left = resolved_family_policy(left);
                let middle = resolved_family_policy(middle);
                let right = resolved_family_policy(right);
                assert_eq!(
                    join_contract_policies(join_contract_policies(left, middle), right),
                    join_contract_policies(left, join_contract_policies(middle, right)),
                );
            }
        }
    }
}

#[test]
fn n_way_contract_resolution_is_permutation_invariant() {
    let permutations = [
        [
            UiAllocationStreamFamily::TextInput,
            UiAllocationStreamFamily::QueryProjection,
            UiAllocationStreamFamily::ResizePreview,
        ],
        [
            UiAllocationStreamFamily::TextInput,
            UiAllocationStreamFamily::ResizePreview,
            UiAllocationStreamFamily::QueryProjection,
        ],
        [
            UiAllocationStreamFamily::QueryProjection,
            UiAllocationStreamFamily::TextInput,
            UiAllocationStreamFamily::ResizePreview,
        ],
        [
            UiAllocationStreamFamily::QueryProjection,
            UiAllocationStreamFamily::ResizePreview,
            UiAllocationStreamFamily::TextInput,
        ],
        [
            UiAllocationStreamFamily::ResizePreview,
            UiAllocationStreamFamily::TextInput,
            UiAllocationStreamFamily::QueryProjection,
        ],
        [
            UiAllocationStreamFamily::ResizePreview,
            UiAllocationStreamFamily::QueryProjection,
            UiAllocationStreamFamily::TextInput,
        ],
    ];
    let mut expected = None;
    for permutation in permutations {
        let mut counters = crate::evidence::UiAllocationStreamPolicyPayloadCounters::default();
        let decision = resolve_stream_families(&permutation, &mut counters);
        let UiAllocationStreamCommitDecision::Commit(receipt) = decision else {
            panic!("declared three-family contract must commit");
        };
        if let Some(expected) = expected {
            assert_eq!(receipt.policy, expected);
        } else {
            expected = Some(receipt.policy);
        }
    }
}

#[test]
fn single_family_performs_no_pair_or_policy_join_work() {
    let mut payload = crate::evidence::UiAllocationStreamPolicyPayloadCounters::default();
    let UiAllocationStreamCommitDecision::Preview(receipt) =
        resolve_stream_families(&[UiAllocationStreamFamily::ResizePreview], &mut payload)
    else {
        panic!("resize preview resolves as preview");
    };
    assert_eq!(receipt.counters.pair_contract_evaluations(), 0);
    assert_eq!(receipt.counters.pair_policy_joins(), 0);
    assert_eq!(receipt.counters.n_way_policy_joins(), 0);
    assert_eq!(receipt.counters.branch_policy_joins(), 0);
}

#[test]
fn mixed_family_work_counters_name_each_operation() {
    let mut payload = crate::evidence::UiAllocationStreamPolicyPayloadCounters::default();
    let UiAllocationStreamCommitDecision::Commit(receipt) = resolve_stream_families(
        &[
            UiAllocationStreamFamily::TextInput,
            UiAllocationStreamFamily::QueryProjection,
            UiAllocationStreamFamily::ResizePreview,
        ],
        &mut payload,
    ) else {
        panic!("mixed family frame resolves");
    };
    assert_eq!(receipt.counters.pair_contract_evaluations(), 3);
    assert_eq!(receipt.counters.pair_policy_joins(), 3);
    assert_eq!(receipt.counters.n_way_policy_joins(), 2);
    assert_eq!(receipt.counters.branch_policy_joins(), 0);
}

#[test]
fn illegal_pair_counts_evaluation_without_a_policy_join() {
    let mut payload = crate::evidence::UiAllocationStreamPolicyPayloadCounters::default();
    assert!(matches!(
        resolve_stream_families(
            &[
                UiAllocationStreamFamily::TextInput,
                UiAllocationStreamFamily::DurableResize,
            ],
            &mut payload,
        ),
        UiAllocationStreamCommitDecision::Denied(
            UiAllocationStreamCompositionDenial::IllegalFamilyPair { .. }
        )
    ));
    assert_eq!(payload.pair_contract_evaluations(), 1);
    assert_eq!(payload.pair_policy_joins(), 0);
    assert_eq!(payload.n_way_policy_joins(), 0);
    assert_eq!(payload.branch_policy_joins(), 0);
}

#[test]
fn maximum_non_viewport_family_set_has_exact_quadratic_contract_work() {
    let families = [
        UiAllocationStreamFamily::TextInput,
        UiAllocationStreamFamily::QueryProjection,
        UiAllocationStreamFamily::HostMeasurementReplacement,
        UiAllocationStreamFamily::ResizePreview,
        UiAllocationStreamFamily::ScrollExtentObservation,
        UiAllocationStreamFamily::PortalAnchorObservation,
    ];
    let mut payload = crate::evidence::UiAllocationStreamPolicyPayloadCounters::default();
    let UiAllocationStreamCommitDecision::Commit(receipt) =
        resolve_stream_families(&families, &mut payload)
    else {
        panic!("maximum compatible family set resolves");
    };
    assert_eq!(receipt.counters.pair_contract_evaluations(), 15);
    assert_eq!(receipt.counters.pair_policy_joins(), 15);
    assert_eq!(receipt.counters.n_way_policy_joins(), 14);
    assert_eq!(receipt.counters.branch_policy_joins(), 9);
}
