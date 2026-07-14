use super::*;

impl UiAllocationFrameDispatcher {
    pub(in crate::runtime::allocation_frame_dispatch) fn submit(
        &mut self,
        ingress: UiAdmittedAllocationStreamIngress,
    ) -> UiAllocationFrameSubmissionTransition {
        let ingress_key = ingress.key();
        let descriptor = ingress.descriptor();
        if matches!(self.state, UiAllocationFrameDispatcherState::Paused(_)) {
            return match self.open_epoch_for_submission(ingress_key) {
                Err(outcome) => UiAllocationFrameSubmissionTransition::denied(outcome),
                Ok(_) => unreachable!("paused dispatcher cannot open an epoch"),
            };
        }
        let (identity_assignment, identity_slots_scanned) = self
            .retry_state
            .assignment_for_ingress_identity(descriptor.clone());
        self.counters.record_identity_lookup(identity_slots_scanned);
        self.counters
            .record_retry_ledger_work(identity_slots_scanned, 0);
        if let Some(assignment) = identity_assignment {
            if assignment.descriptor() == descriptor {
                self.counters.record_duplicate();
                let outcome = match assignment.sequence() {
                    Some(sequence) => UiAllocationFrameSubmissionOutcome::duplicate_assigned(
                        &self.seal_authority,
                        ingress_key,
                        assignment.epoch(),
                        sequence,
                        self.counters,
                    ),
                    None => UiAllocationFrameSubmissionOutcome::duplicate_pending(
                        &self.seal_authority,
                        ingress_key,
                        assignment.epoch(),
                        self.counters,
                    ),
                };
                return UiAllocationFrameSubmissionTransition::new(outcome, None);
            }
            return self.denied_submission(
                ingress_key,
                UiAllocationFrameSubmissionDenial::ConflictingIdentity,
            );
        }
        let (position_assignment, position_slots_scanned) = self
            .retry_state
            .assignment_for_source_position(descriptor.clone());
        self.counters.record_sequence_lookup(position_slots_scanned);
        self.counters
            .record_retry_ledger_work(position_slots_scanned, 0);
        if position_assignment.is_some() {
            return self.denied_submission(
                ingress_key,
                UiAllocationFrameSubmissionDenial::ConflictingSourceOrder,
            );
        }
        let (is_retired, retired_comparisons) = self.retry_state.is_retired(descriptor.clone());
        self.counters
            .record_retry_ledger_work(retired_comparisons, 0);
        if is_retired {
            return self.denied_submission(
                ingress_key,
                UiAllocationFrameSubmissionDenial::RetryWindowExpired,
            );
        }
        let (epoch, epoch_assignment) = match self.open_epoch_for_submission(ingress_key.clone()) {
            Ok(transition) => transition,
            Err(outcome) => return UiAllocationFrameSubmissionTransition::denied(outcome),
        };
        let (epoch_comparisons, epoch_writes) = self.retry_state.begin_epoch(epoch);
        self.counters
            .record_retry_ledger_work(epoch_comparisons, epoch_writes);
        let (can_track, domain_comparisons) = self.retry_state.can_track(descriptor.clone());
        self.counters
            .record_retry_ledger_work(domain_comparisons, 0);
        if !can_track {
            let outcome = self
                .denied_submission(
                    ingress_key,
                    UiAllocationFrameSubmissionDenial::SourceDomainCapacityExhausted,
                )
                .into_outcome();
            return UiAllocationFrameSubmissionTransition::new(outcome, epoch_assignment);
        }
        let successor_target = matches!(
            self.state,
            UiAllocationFrameDispatcherState::Closing { .. }
                | UiAllocationFrameDispatcherState::Sealed(_)
        );
        let target_mailbox = if successor_target {
            &mut self.successor_mailbox
        } else {
            &mut self.mailbox
        };
        if target_mailbox.is_full() {
            self.counters.record_backpressure();
            return UiAllocationFrameSubmissionTransition::backpressured(
                UiAllocationFrameSubmissionOutcome::backpressured(
                    &self.seal_authority,
                    ingress_key,
                    target_mailbox.capacity(),
                    epoch,
                    self.counters,
                ),
                epoch_assignment,
                ingress,
            );
        }
        let insert_work = target_mailbox.insert(ingress);
        self.counters
            .record_mailbox_insert(insert_work.comparisons, insert_work.canonical_writes);
        let (record_comparisons, record_writes) = self.retry_state.record(descriptor, epoch);
        self.counters
            .record_retry_ledger_work(record_comparisons, record_writes);
        self.counters.record_accepted(target_mailbox.len());
        let outcome = if successor_target {
            UiAllocationFrameSubmissionOutcome::late_ingress(
                &self.seal_authority,
                ingress_key,
                epoch,
                self.counters,
            )
        } else {
            UiAllocationFrameSubmissionOutcome::queued(
                &self.seal_authority,
                ingress_key,
                epoch,
                self.counters,
            )
        };
        UiAllocationFrameSubmissionTransition::new(outcome, epoch_assignment)
    }

    fn denied_submission(
        &mut self,
        ingress_key: crate::runtime::UiAllocationFrameIngressKey,
        denial: UiAllocationFrameSubmissionDenial,
    ) -> UiAllocationFrameSubmissionTransition {
        self.counters.record_terminal_denial();
        UiAllocationFrameSubmissionTransition::denied(UiAllocationFrameSubmissionOutcome::denied(
            &self.seal_authority,
            ingress_key,
            denial,
            self.counters,
        ))
    }

    fn open_epoch_for_submission(
        &mut self,
        ingress_key: crate::runtime::UiAllocationFrameIngressKey,
    ) -> Result<
        (
            UiAllocationFrameEpoch,
            Option<UiAllocationFrameEpochAssignment>,
        ),
        UiAllocationFrameSubmissionOutcome,
    > {
        match self.state {
            UiAllocationFrameDispatcherState::Open(epoch) => {
                self.open_existing_epoch(ingress_key, epoch)
            }
            UiAllocationFrameDispatcherState::Dispatched(epoch) => {
                self.open_successor_epoch(ingress_key, epoch)
            }
            UiAllocationFrameDispatcherState::Closing { next_epoch, .. } => {
                self.counters.record_late();
                Ok((next_epoch, None))
            }
            UiAllocationFrameDispatcherState::Sealed(epoch) => {
                self.counters.record_late();
                match epoch.checked_next() {
                    Some(next_epoch) => Ok((next_epoch, None)),
                    None => Err(self.epoch_exhausted(ingress_key)),
                }
            }
            UiAllocationFrameDispatcherState::Paused(reason) => {
                let denial = match reason {
                    UiAllocationFramePauseReason::Replacement => {
                        UiAllocationFrameSubmissionDenial::ReplacementPaused
                    }
                    UiAllocationFramePauseReason::Shutdown => {
                        UiAllocationFrameSubmissionDenial::Shutdown
                    }
                    UiAllocationFramePauseReason::EpochExhausted => {
                        UiAllocationFrameSubmissionDenial::EpochExhausted
                    }
                };
                Err(self.denied_submission(ingress_key, denial).into_outcome())
            }
        }
    }

    fn open_existing_epoch(
        &mut self,
        ingress_key: crate::runtime::UiAllocationFrameIngressKey,
        epoch: UiAllocationFrameEpoch,
    ) -> Result<
        (
            UiAllocationFrameEpoch,
            Option<UiAllocationFrameEpochAssignment>,
        ),
        UiAllocationFrameSubmissionOutcome,
    > {
        if epoch.checked_next().is_none() {
            return Err(self.epoch_exhausted(ingress_key));
        }
        Ok((epoch, None))
    }

    fn open_successor_epoch(
        &mut self,
        ingress_key: crate::runtime::UiAllocationFrameIngressKey,
        epoch: UiAllocationFrameEpoch,
    ) -> Result<
        (
            UiAllocationFrameEpoch,
            Option<UiAllocationFrameEpochAssignment>,
        ),
        UiAllocationFrameSubmissionOutcome,
    > {
        let Some(next_epoch) = epoch.checked_next() else {
            return Err(self.epoch_exhausted(ingress_key));
        };
        if next_epoch.checked_next().is_none() {
            return Err(self.epoch_exhausted(ingress_key));
        }
        self.state = UiAllocationFrameDispatcherState::Open(next_epoch);
        if !self.successor_mailbox.is_empty() {
            std::mem::swap(&mut self.mailbox, &mut self.successor_mailbox);
        }
        Ok((
            next_epoch,
            Some(UiAllocationFrameEpochAssignment::from_linearization(
                next_epoch,
            )),
        ))
    }

    fn epoch_exhausted(
        &mut self,
        ingress_key: crate::runtime::UiAllocationFrameIngressKey,
    ) -> UiAllocationFrameSubmissionOutcome {
        self.state =
            UiAllocationFrameDispatcherState::Paused(UiAllocationFramePauseReason::EpochExhausted);
        self.denied_submission(
            ingress_key,
            UiAllocationFrameSubmissionDenial::EpochExhausted,
        )
        .into_outcome()
    }
}
