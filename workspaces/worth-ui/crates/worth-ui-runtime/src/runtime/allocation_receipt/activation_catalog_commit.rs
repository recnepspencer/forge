use super::{
    ledger_state::UiAllocationCatalogLedgerTransition, UiAllocationReceiptLedger,
    UiAllocationReplanTransactionCounters, UiAllocationReuseVerdict, UiCommittedAllocationReplan,
};
use crate::evidence::UiAllocationNeighborhoodScope;

impl UiAllocationReceiptLedger {
    pub(crate) fn seal_activation_catalog(
        &self,
        catalog: crate::runtime::invalidation_narrowing::UiAllocationActivationCatalog,
        frame_epoch: crate::runtime::UiAllocationFrameEpoch,
        reconciliation: &crate::runtime::WorthUiDurableStateReconciliationPlan,
    ) -> Result<
        crate::runtime::UiCommittedAllocationActivationAttempt,
        super::UiAllocationReceiptCommitOutcome,
    > {
        let candidates = catalog.candidates_for_commit();
        for (ordinal, candidate) in candidates.iter().enumerate() {
            let ordinal = u16::try_from(ordinal).map_err(|_| {
                super::UiAllocationReceiptCommitOutcome::Denied(
                    super::UiAllocationReceiptCommitDenial::CatalogBindingCardinalityMismatch,
                )
            })?;
            let admission = candidate.replan_admission_opt().ok_or_else(|| {
                super::UiAllocationReceiptCommitOutcome::Denied(
                    super::UiAllocationReceiptCommitDenial::CatalogActivationAuthority(
                        super::UiCommittedAllocationCatalogActivationDenial::MissingReplanAdmission {
                            ordinal,
                        },
                    ),
                )
            })?;
            admission.committed_scroll_sources().map_err(|denial| {
                super::UiAllocationReceiptCommitOutcome::Denied(
                    super::UiAllocationReceiptCommitDenial::CatalogActivationAuthority(
                        super::UiCommittedAllocationCatalogActivationDenial::ScrollAuthority {
                            ordinal,
                            denial,
                        },
                    ),
                )
            })?;
        }
        let state = self.state.borrow();
        let transaction_generation = state.checked_transaction_generation();
        let transaction = super::UiAllocationReplanTransaction::for_replacement_activation(
            candidates,
            state.runtime_generation,
            transaction_generation.unwrap_or(state.next_transaction_generation),
            frame_epoch,
        )
        .map_err(|_| {
            super::UiAllocationReceiptCommitOutcome::Denied(
                super::UiAllocationReceiptCommitDenial::CandidatePlanningDenied(
                    super::UiAllocationReceiptDenialReport::candidate_planning_denied(
                        candidates
                            .first()
                            .expect("catalog admission proves non-empty"),
                    ),
                ),
            )
        })?;
        let replay_key = transaction.idempotency_key();
        if let Some(previous) = state
            .completed_transactions
            .get(&replay_key)
            .and_then(|bucket| {
                bucket
                    .iter()
                    .find(|completed| completed.transaction().same_idempotency_basis(&transaction))
            })
            .cloned()
        {
            let predecessor = state.clone();
            let mut successor = predecessor.clone();
            let durable = reconciliation.allocation_durable_semantic_state();
            let replacement = successor.durable_semantic_state.as_ref() != Some(&durable);
            successor.truth_revision = successor
                .checked_truth_successor(0, false, replacement)
                .map_err(|denial| {
                    super::UiAllocationReceiptCommitOutcome::Denied(
                        super::UiAllocationReceiptCommitDenial::AuthorityCounterExhausted(denial),
                    )
                })?;
            if replacement {
                successor.durable_semantic_state = Some(durable);
            }
            let bindings =
                super::UiCommittedAllocationCatalogBindings::seal(candidates, previous.receipts())
                    .map_err(super::UiAllocationReceiptCommitOutcome::Denied)?;
            let activation =
                super::UiCommittedAllocationCatalogActivation::seal(candidates, &bindings)
                    .map_err(|denial| {
                        super::UiAllocationReceiptCommitOutcome::Denied(
                            super::UiAllocationReceiptCommitDenial::CatalogActivationAuthority(
                                denial,
                            ),
                        )
                    })?;
            let committed = previous.clone();
            return Ok(crate::runtime::UiCommittedAllocationActivationAttempt::new(
                catalog,
                UiAllocationCatalogLedgerTransition {
                    predecessor,
                    successor,
                    outcome: previous,
                    durable_reconciliation: reconciliation.clone(),
                },
                committed,
                activation,
            ));
        }
        transaction_generation.map_err(|denial| {
            super::UiAllocationReceiptCommitOutcome::Denied(
                super::UiAllocationReceiptCommitDenial::AuthorityCounterExhausted(denial),
            )
        })?;
        let mut verdicts = Vec::with_capacity(candidates.len());
        for candidate in candidates {
            let scope = UiAllocationNeighborhoodScope::from_neighborhood(
                candidate.allocation_neighborhood(),
            );
            verdicts.push(super::receipt_commit::admit_allocation_receipt_candidate(
                candidate,
                state.committed_by_scope.get(&scope),
            )?);
        }
        drop(state);
        let receipts = candidates
            .iter()
            .cloned()
            .zip(verdicts)
            .map(|(candidate, verdict)| {
                super::receipt_commit::commit_admitted_allocation_receipt(
                    candidate,
                    verdict,
                    transaction.clone(),
                )
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let predecessor = self.state.borrow().clone();
        let mut successor = predecessor.clone();
        let durable = reconciliation.allocation_durable_semantic_state();
        let replacement = successor.durable_semantic_state.as_ref() != Some(&durable);
        successor.truth_revision = successor
            .checked_truth_successor(receipts.len(), false, replacement)
            .map_err(|denial| {
                super::UiAllocationReceiptCommitOutcome::Denied(
                    super::UiAllocationReceiptCommitDenial::AuthorityCounterExhausted(denial),
                )
            })?;
        if replacement {
            successor.durable_semantic_state = Some(durable);
        }
        for receipt in &receipts {
            successor.committed_by_scope.insert(
                UiAllocationNeighborhoodScope::from_neighborhood(
                    receipt.committed_allocation().allocation_neighborhood(),
                ),
                receipt.clone(),
            );
        }
        let mut counters = UiAllocationReplanTransactionCounters::preflight(receipts.len())
            .map_err(|()| {
                super::UiAllocationReceiptCommitOutcome::Denied(
                    super::UiAllocationReceiptCommitDenial::EvidenceCounterExhausted,
                )
            })?;
        for verdict in candidates.iter().map(|candidate| {
            let scope = UiAllocationNeighborhoodScope::from_neighborhood(
                candidate.allocation_neighborhood(),
            );
            predecessor.committed_by_scope.get(&scope).map_or(
                UiAllocationReuseVerdict::NewCommit,
                |previous| {
                    super::receipt_commit::evaluate_allocation_receipt_reuse(candidate, previous)
                },
            )
        }) {
            let counted = match verdict {
                UiAllocationReuseVerdict::FullReuse => counters.reused(),
                UiAllocationReuseVerdict::NewCommit => counters.replanned(),
                _ => unreachable!("catalog preflight admitted only publishable verdicts"),
            };
            counted.map_err(|()| {
                super::UiAllocationReceiptCommitOutcome::Denied(
                    super::UiAllocationReceiptCommitDenial::EvidenceCounterExhausted,
                )
            })?;
        }
        counters.committed(receipts.len()).map_err(|()| {
            super::UiAllocationReceiptCommitOutcome::Denied(
                super::UiAllocationReceiptCommitDenial::EvidenceCounterExhausted,
            )
        })?;
        let receipts = receipts.into_vec();
        let catalog_bindings =
            super::UiCommittedAllocationCatalogBindings::seal(candidates, &receipts)
                .map_err(|denial| super::UiAllocationReceiptCommitOutcome::Denied(denial))?;
        let outcome = UiCommittedAllocationReplan::new(
            transaction.clone(),
            receipts,
            counters,
            catalog_bindings,
        )
        .map_err(|()| {
            super::UiAllocationReceiptCommitOutcome::Denied(
                super::UiAllocationReceiptCommitDenial::EvidenceCounterExhausted,
            )
        })?;
        let activation = super::UiCommittedAllocationCatalogActivation::seal(
            candidates,
            outcome.catalog_bindings(),
        )
        .map_err(|denial| {
            super::UiAllocationReceiptCommitOutcome::Denied(
                super::UiAllocationReceiptCommitDenial::CatalogActivationAuthority(denial),
            )
        })?;
        successor.next_transaction_generation = transaction.transaction_generation();
        successor.latest_frame_epoch = Some(frame_epoch);
        successor
            .completed_transactions
            .entry(replay_key)
            .or_default()
            .push(outcome.clone());
        let committed = outcome.clone();
        Ok(crate::runtime::UiCommittedAllocationActivationAttempt::new(
            catalog,
            UiAllocationCatalogLedgerTransition {
                predecessor,
                successor,
                outcome,
                durable_reconciliation: reconciliation.clone(),
            },
            committed,
            activation,
        ))
    }
}
