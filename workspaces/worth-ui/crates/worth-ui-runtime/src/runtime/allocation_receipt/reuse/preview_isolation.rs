#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiPreviewPaintIsolationReceipt {
    frame_epoch: crate::runtime::UiAllocationFrameEpoch,
    before: super::UiAllocationTruthRevision,
    after: super::UiAllocationTruthRevision,
    delta: super::UiAllocationTruthDelta,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiPreviewPaintIsolationViolation {
    frame_epoch: crate::runtime::UiAllocationFrameEpoch,
    before: super::UiAllocationTruthRevision,
    after: super::UiAllocationTruthRevision,
    delta: Option<super::UiAllocationTruthDelta>,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiPreviewPaintIsolationOutcome {
    Verified(UiPreviewPaintIsolationReceipt),
    Violated(UiPreviewPaintIsolationViolation),
}

#[derive(Debug)]
pub(crate) struct UiPreviewPaintIsolationPort<'runtime> {
    ledger: &'runtime super::UiAllocationReceiptLedger,
}
impl<'runtime> UiPreviewPaintIsolationPort<'runtime> {
    pub(crate) fn new(ledger: &'runtime super::UiAllocationReceiptLedger) -> Self {
        Self { ledger }
    }
    pub(crate) fn capture(&self) -> super::UiAllocationTruthRevision {
        self.ledger.truth_revision()
    }
    pub(crate) fn seal(
        self,
        frame_epoch: crate::runtime::UiAllocationFrameEpoch,
        before: super::UiAllocationTruthRevision,
        after: super::UiAllocationTruthRevision,
    ) -> UiPreviewPaintIsolationOutcome {
        let delta = after.delta_since(before);
        if before == after && delta.is_some_and(super::UiAllocationTruthDelta::is_zero) {
            UiPreviewPaintIsolationOutcome::Verified(UiPreviewPaintIsolationReceipt {
                frame_epoch,
                before,
                after,
                delta: delta.expect("verified delta exists"),
            })
        } else {
            UiPreviewPaintIsolationOutcome::Violated(UiPreviewPaintIsolationViolation {
                frame_epoch,
                before,
                after,
                delta,
            })
        }
    }
}
impl UiPreviewPaintIsolationReceipt {
    pub fn frame_epoch(self) -> crate::runtime::UiAllocationFrameEpoch {
        self.frame_epoch
    }
    pub fn before(self) -> super::UiAllocationTruthRevision {
        self.before
    }
    pub fn after(self) -> super::UiAllocationTruthRevision {
        self.after
    }
    pub fn delta(self) -> super::UiAllocationTruthDelta {
        self.delta
    }
    pub fn durable_mutations(self) -> u64 {
        self.delta.durable_resize_mutations()
    }
    pub fn committed_receipts(self) -> u64 {
        self.delta.committed_receipt_publications()
    }
}
impl UiPreviewPaintIsolationViolation {
    pub fn frame_epoch(self) -> crate::runtime::UiAllocationFrameEpoch {
        self.frame_epoch
    }
    pub fn before(self) -> super::UiAllocationTruthRevision {
        self.before
    }
    pub fn after(self) -> super::UiAllocationTruthRevision {
        self.after
    }
    pub fn delta(self) -> Option<super::UiAllocationTruthDelta> {
        self.delta
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_scope_receipt_publication_cannot_verify_as_isolated() {
        let (runtime, _, _, candidate) = crate::runtime::tests::production_catalog_activation_test_support::runtime_with_production_catalog_activation();
        let ledger = &runtime.allocation_receipt_ledger;
        let port = UiPreviewPaintIsolationPort::new(ledger);
        let before = port.capture();
        ledger
            .commit_non_portal_receipt_law_candidate(
                crate::runtime::allocation_receipt::UiNonPortalReceiptLawCandidate::admit(
                    candidate,
                )
                .expect("preview isolation fixture is non-portal"),
            )
            .expect("real receipt publication commits");
        let after = port.capture();
        let UiPreviewPaintIsolationOutcome::Violated(violation) = port.seal(
            crate::runtime::UiAllocationFrameEpoch::for_test(1),
            before,
            after,
        ) else {
            panic!("receipt publication must violate preview isolation")
        };
        let delta = violation.delta().expect("monotonic delta exists");
        assert_eq!(delta.committed_receipt_publications(), 1);
        assert_eq!(delta.durable_resize_mutations(), 0);
    }

    #[test]
    fn unchanged_revision_reports_exact_zero_effects() {
        let ledger = super::super::UiAllocationReceiptLedger::for_runtime_generation(1);
        let port = UiPreviewPaintIsolationPort::new(&ledger);
        let revision = port.capture();
        let UiPreviewPaintIsolationOutcome::Verified(receipt) = port.seal(
            crate::runtime::UiAllocationFrameEpoch::for_test(1),
            revision,
            revision,
        ) else {
            panic!("unchanged truth must verify")
        };
        assert_eq!(receipt.delta().committed_receipt_publications(), 0);
        assert_eq!(receipt.delta().durable_resize_mutations(), 0);
        assert_eq!(receipt.delta().durable_state_replacements(), 0);
    }

    #[test]
    fn real_receipt_commit_denies_atomically_at_coherent_exhaustion() {
        use super::super::receipt_ledger_test_support::UiAllocationAuthorityExhaustionScenario;
        let (runtime, _, _, candidate) = crate::runtime::tests::production_catalog_activation_test_support::runtime_with_production_catalog_activation();
        let ledger = &runtime.allocation_receipt_ledger;
        let predecessor = ledger.position_exhaustion_for_test(
            UiAllocationAuthorityExhaustionScenario::TruthRevision { remaining: 0 },
        );
        let denial = ledger
            .commit_non_portal_receipt_law_candidate(
                crate::runtime::allocation_receipt::UiNonPortalReceiptLawCandidate::admit(
                    candidate,
                )
                .expect("exhaustion fixture is non-portal"),
            )
            .expect_err("exhausted receipt publication denies");
        let super::super::UiAllocationReceiptCommitDenial::AuthorityCounterExhausted(exhaustion) =
            denial
        else {
            panic!("typed exhaustion required")
        };
        assert_eq!(
            exhaustion.counter(),
            super::super::UiAllocationAuthorityCounter::TruthRevision
        );
        assert_eq!(exhaustion.increment(), 1);
        assert_eq!(ledger.ledger_state_for_test(), predecessor);
    }

    #[test]
    fn ordinary_replan_propagates_transaction_generation_exhaustion() {
        use super::super::receipt_ledger_test_support::UiAllocationAuthorityExhaustionScenario;
        let (mut runtime, root, _, _) = crate::runtime::tests::production_catalog_activation_test_support::runtime_with_production_catalog_activation();
        let predecessor = runtime
            .allocation_receipt_ledger
            .position_exhaustion_for_test(
                UiAllocationAuthorityExhaustionScenario::TransactionGeneration,
            );
        let completion = runtime.execute_framework_turn(|turn| {
            turn.interaction(|source| {
                source
                    .admit_and_submit(
                        root,
                        crate::runtime::WorthUiTransientInteractionState::TextInput,
                    )
                    .unwrap();
            });
        });
        let Some(crate::runtime::UiAllocationReplanTransactionOutcome::Denied(
            crate::runtime::UiAllocationReplanTransactionCommitDenial::AuthorityCounterExhausted(
                exhaustion,
            ),
        )) = completion.replan_transaction()
        else {
            panic!("ordinary path must retain typed exhaustion")
        };
        assert_eq!(
            exhaustion.counter(),
            super::super::UiAllocationAuthorityCounter::TransactionGeneration
        );
        assert_eq!(exhaustion.increment(), 1);
        assert_eq!(
            runtime.allocation_receipt_ledger.ledger_state_for_test(),
            predecessor
        );
    }

    fn assert_durable_truth_exhaustion(remaining: u64) {
        use super::super::receipt_ledger_test_support::UiAllocationAuthorityExhaustionScenario;
        let (mut runtime, _, input) = crate::runtime::tests::production_catalog_activation_test_support::runtime_with_durable_resize_catalog();
        let predecessor = runtime
            .allocation_receipt_ledger
            .position_exhaustion_for_test(UiAllocationAuthorityExhaustionScenario::TruthRevision {
                remaining,
            });
        let extent = crate::runtime::UiResizeLogicalExtent::try_from_logical_pixels(377.0).unwrap();
        let completion = runtime.execute_framework_turn(|turn| {
            turn.durable_resize(|source| {
                source
                    .admit_and_submit(crate::runtime::UiDurableResizeCommitIntent::terminal(
                        input, extent,
                    ))
                    .unwrap();
            });
        });
        let Some(crate::runtime::UiAllocationReplanTransactionOutcome::Denied(
            crate::runtime::UiAllocationReplanTransactionCommitDenial::AuthorityCounterExhausted(
                exhaustion,
            ),
        )) = completion.replan_transaction()
        else {
            panic!("ordinary durable path must expose typed component exhaustion")
        };
        assert_eq!(
            exhaustion.counter(),
            super::super::UiAllocationAuthorityCounter::TruthRevision
        );
        assert_eq!(exhaustion.increment(), 2);
        assert_eq!(
            runtime.allocation_receipt_ledger.ledger_state_for_test(),
            predecessor
        );
    }

    #[test]
    fn durable_transition_denies_atomically_when_aggregate_capacity_is_too_small() {
        assert_durable_truth_exhaustion(1);
    }

    #[test]
    fn durable_transition_uses_exact_remaining_aggregate_capacity() {
        let (mut runtime, _, input) = crate::runtime::tests::production_catalog_activation_test_support::runtime_with_durable_resize_catalog();
        runtime
            .allocation_receipt_ledger
            .position_exhaustion_for_test(
                super::super::receipt_ledger_test_support::UiAllocationAuthorityExhaustionScenario::TruthRevision {
                    remaining: 2,
                },
            );
        let before = runtime.allocation_receipt_ledger.truth_revision();
        assert!(before.invariant_holds());
        let extent = crate::runtime::UiResizeLogicalExtent::try_from_logical_pixels(388.0).unwrap();
        let completion = runtime.execute_framework_turn(|turn| {
            turn.durable_resize(|source| {
                source
                    .admit_and_submit(crate::runtime::UiDurableResizeCommitIntent::terminal(
                        input, extent,
                    ))
                    .unwrap();
            });
        });
        let outcome = completion
            .durable_resize_outcome()
            .expect("exact aggregate capacity admits receipt plus durable mutation");
        assert_eq!(outcome.counters().committed_receipts(), 1);
        assert_eq!(outcome.counters().durable_mutations(), 1);
        let after = runtime.allocation_receipt_ledger.truth_revision();
        assert_eq!(after.revision(), u64::MAX);
        assert_eq!(after.revision() - before.revision(), 2);
        assert!(after.invariant_holds());
    }

    #[test]
    fn real_durable_mutation_and_restoration_both_violate_preview_isolation() {
        let (mut runtime, _, input) = crate::runtime::tests::production_catalog_activation_test_support::runtime_with_durable_resize_catalog();
        for pixels in [410.0, 420.0, 410.0] {
            let before_port = UiPreviewPaintIsolationPort::new(&runtime.allocation_receipt_ledger);
            let before = before_port.capture();
            let extent =
                crate::runtime::UiResizeLogicalExtent::try_from_logical_pixels(pixels).unwrap();
            let completion = runtime.execute_framework_turn(|turn| {
                turn.durable_resize(|source| {
                    source
                        .admit_and_submit(crate::runtime::UiDurableResizeCommitIntent::terminal(
                            input.clone(),
                            extent,
                        ))
                        .unwrap();
                });
            });
            assert!(completion.durable_resize_outcome().is_some());
            let port = UiPreviewPaintIsolationPort::new(&runtime.allocation_receipt_ledger);
            let after = port.capture();
            let UiPreviewPaintIsolationOutcome::Violated(violation) = port.seal(
                crate::runtime::UiAllocationFrameEpoch::for_test(1),
                before,
                after,
            ) else {
                panic!("every real durable write, including restoration, must be observed")
            };
            let delta = violation.delta().unwrap();
            assert_eq!(delta.committed_receipt_publications(), 1);
            assert_eq!(delta.durable_resize_mutations(), 1);
        }
    }

    #[test]
    fn retained_replay_remains_available_at_generation_exhaustion() {
        use super::super::receipt_ledger_test_support::UiAllocationAuthorityExhaustionScenario;
        let (mut runtime, root, _, _) = crate::runtime::tests::production_catalog_activation_test_support::runtime_with_production_catalog_activation();
        let first = runtime.execute_framework_turn(|turn| {
            turn.interaction(|source| {
                source
                    .admit_and_submit(
                        root,
                        crate::runtime::WorthUiTransientInteractionState::TextInput,
                    )
                    .unwrap();
            });
        });
        let selection = first.replan_selection().unwrap().clone();
        assert!(matches!(
            first.replan_transaction(),
            Some(crate::runtime::UiAllocationReplanTransactionOutcome::Committed(_))
        ));
        let predecessor = runtime
            .allocation_receipt_ledger
            .position_exhaustion_for_test(
                UiAllocationAuthorityExhaustionScenario::TransactionGeneration,
            );
        let replay = runtime.replay_admitted_transaction_for_test(&selection);
        assert!(matches!(
            replay,
            crate::runtime::UiAllocationReplanTransactionOutcome::Replayed(_)
        ));
        assert_eq!(
            runtime.allocation_receipt_ledger.ledger_state_for_test(),
            predecessor
        );
    }
}
