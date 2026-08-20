use super::completion_semantic_changes::completion_semantic_changes;
use super::*;

impl WorthUiPresentationAsyncOwner {
    pub fn admit_presented(
        &mut self,
        receipt: &WorthUiPresentationPendingReceipt,
        completion: WorthUiPresentationValidatedCompletion,
    ) -> Result<WorthUiPresentationPresentedReceipt, WorthUiPresentationSettlementDenial> {
        let (authority, receipt_authority, completion_nonce, payload_byte_len) =
            completion.into_parts();
        if !correspondence::is_correspondence_authority(
            &self.correspondence_authority,
            &receipt.authority,
        ) {
            return Err(WorthUiPresentationSettlementDenial::ForeignPendingReceiptAuthority);
        }
        if !correspondence::is_correspondence_authority(&self.correspondence_authority, &authority)
        {
            return Err(WorthUiPresentationSettlementDenial::ForeignCompletionAuthority);
        }
        if !correspondence::is_correspondence_authority(&receipt.authority, &receipt_authority) {
            return Err(WorthUiPresentationSettlementDenial::CompletionReceiptMismatch);
        }
        if completion_nonce != receipt.nonce {
            return Err(WorthUiPresentationSettlementDenial::CompletionReceiptMismatch);
        }
        let key = PresentationAdmissionKey {
            attempt: receipt.attempt,
            binding: receipt.binding,
        };
        if self
            .superseded_pending
            .get(&key)
            .is_some_and(|pending| pending.nonce == receipt.nonce)
        {
            return self.finish_superseded(key, true);
        }
        if let Some(completed) = self
            .superseded_awaiting_completion
            .remove(&(key, receipt.nonce))
        {
            self.record_transition(
                WorthUiPresentationTransitionKind::StaleCompletionRejected,
                key,
            );
            return Ok(completed);
        }
        if self
            .current
            .values()
            .any(|(current_key, nonce, _)| *current_key == key && *nonce == receipt.nonce)
        {
            self.record_transition(
                WorthUiPresentationTransitionKind::DuplicateCompletionRejected,
                key,
            );
            return Err(WorthUiPresentationSettlementDenial::InvalidPendingReceipt);
        }
        if self
            .pending
            .get(&key)
            .is_some_and(|pending| pending.nonce == receipt.nonce)
        {
            debug_assert!(self
                .pending
                .get(&key)
                .is_some_and(pending_progress::pending_admission_complete));
            let pending = self
                .pending
                .remove(&key)
                .expect("validated pending receipt remains retained");
            self.settling.insert(key, pending);
        }
        let mut pending = self
            .settling
            .remove(&key)
            .filter(|pending| pending.nonce == receipt.nonce)
            .ok_or(WorthUiPresentationSettlementDenial::InvalidPendingReceipt)?;
        if let Err(denial) = self.advance_settlement(&mut pending, payload_byte_len) {
            self.settling.insert(key, pending);
            return Err(denial);
        }
        let observation = pending
            .settlement
            .completion
            .expect("completed settlement retains Query observation");
        let mut frontiers = std::mem::take(&mut pending.pending_frontiers);
        frontiers.append(&mut pending.settlement.frontiers);
        let frontiers = frontiers.into_boxed_slice();
        self.retained
            .insert(pending.lineage, pending.transition.successor().clone());
        self.current
            .insert(pending.lineage, (key, pending.nonce, pending.admission));
        self.record_transition(
            if pending.reconstructing_unresolved_predecessor {
                WorthUiPresentationTransitionKind::ReconstructionCurrent
            } else {
                WorthUiPresentationTransitionKind::Completed
            },
            key,
        );
        Ok(WorthUiPresentationPresentedReceipt {
            frontiers,
            observation,
            predecessor_observation: pending.settlement.predecessor_observation,
        })
    }

    fn advance_settlement(
        &mut self,
        pending: &mut PendingPresentationAdmission,
        payload_byte_len: u64,
    ) -> Result<(), WorthUiPresentationSettlementDenial> {
        let changes = completion_semantic_changes(pending.admission.basis());
        while let Some(change) = changes.get(pending.settlement.publication_index).copied() {
            let publication = self
                .registry
                .publication_for_admission(&pending.admission, change)
                .map_err(|_| {
                    WorthUiPresentationSettlementDenial::Progress(
                        WorthUiPresentationSettlementStop::SemanticExecution,
                    )
                })?;
            let semantic = self
                .registry
                .publish_and_execute_publication(
                    &mut self.workspace,
                    &pending.admission,
                    &publication,
                )
                .map_err(|_| {
                    WorthUiPresentationSettlementDenial::Progress(
                        WorthUiPresentationSettlementStop::SemanticExecution,
                    )
                })?;
            pending
                .settlement
                .frontiers
                .push(semantic_frontier_observation(change, &semantic));
            pending.settlement.publication_index += 1;
        }
        if pending.settlement.completion.is_none() {
            if pending.settlement.completion_progress.is_none() {
                pending.settlement.completion_progress = Some(
                    pending
                        .admission
                        .begin_owner_validated_completion(&mut self.workspace, payload_byte_len)
                        .map_err(|_| {
                            WorthUiPresentationSettlementDenial::Progress(
                                WorthUiPresentationSettlementStop::QueryCompletion,
                            )
                        })?,
                );
            }
            let completion = pending
                .admission
                .resume_completion(
                    &mut self.workspace,
                    pending
                        .settlement
                        .completion_progress
                        .as_mut()
                        .expect("committed completion retains resumable progress"),
                )
                .map_err(|_| {
                    WorthUiPresentationSettlementDenial::Progress(
                        WorthUiPresentationSettlementStop::QueryCompletion,
                    )
                })?;
            pending.settlement.completion = Some(completion.observation());
        }
        if pending.reconstructing_unresolved_predecessor {
            self.supersede_pending_lineage(pending.lineage, &pending.admission)
                .map_err(reconstruction_predecessor_stop)?;
        }
        self.retire_current_predecessor(pending)?;
        self.retire_superseded_predecessors(pending.lineage)
    }

    fn retire_current_predecessor(
        &mut self,
        pending: &mut PendingPresentationAdmission,
    ) -> Result<(), WorthUiPresentationSettlementDenial> {
        let Some((prior_key, prior_nonce, prior)) = self.current.remove(&pending.lineage) else {
            return Ok(());
        };
        if let Err(denial) = self.advance_current_predecessor(pending, &prior) {
            self.current
                .insert(pending.lineage, (prior_key, prior_nonce, prior));
            return Err(denial);
        }
        self.active_keys.remove(&prior_key);
        Ok(())
    }

    fn advance_current_predecessor(
        &mut self,
        pending: &mut PendingPresentationAdmission,
        prior: &WorthUiPresentationRuntimeAdmission,
    ) -> Result<(), WorthUiPresentationSettlementDenial> {
        if !pending.settlement.predecessor_superseded {
            prior
                .admit_supersession(&mut self.workspace, &pending.admission)
                .map_err(|_| {
                    WorthUiPresentationSettlementDenial::Progress(
                        WorthUiPresentationSettlementStop::QuerySupersession,
                    )
                })?;
            pending.settlement.predecessor_superseded = true;
        }
        if pending.settlement.predecessor_observation.is_none() {
            let observation = prior.observation(&self.workspace).map_err(|_| {
                WorthUiPresentationSettlementDenial::Progress(
                    WorthUiPresentationSettlementStop::QueryObservation,
                )
            })?;
            if observation.posture() != WorthUiPresentationAsyncPosture::Superseded {
                return Err(WorthUiPresentationSettlementDenial::Progress(
                    WorthUiPresentationSettlementStop::UnexpectedQueryPosture,
                ));
            }
            pending.settlement.predecessor_observation = Some(observation);
        }
        if !pending.settlement.predecessor_semantic_retired {
            self.registry
                .retire(&mut self.workspace, prior)
                .map_err(|_| {
                    WorthUiPresentationSettlementDenial::Progress(
                        WorthUiPresentationSettlementStop::SemanticRetirement,
                    )
                })?;
            pending.settlement.predecessor_semantic_retired = true;
        }
        if !pending.settlement.predecessor_query_closed {
            prior
                .close_query_live_view(&mut self.workspace)
                .map_err(|_| {
                    WorthUiPresentationSettlementDenial::Progress(
                        WorthUiPresentationSettlementStop::QueryClose,
                    )
                })?;
            pending.settlement.predecessor_query_closed = true;
        }
        Ok(())
    }
}

fn reconstruction_predecessor_stop(
    stop: WorthUiPresentationAdmissionStop,
) -> WorthUiPresentationSettlementDenial {
    let stop = match stop {
        WorthUiPresentationAdmissionStop::RuntimeObservation => {
            WorthUiPresentationSettlementStop::QueryObservation
        }
        WorthUiPresentationAdmissionStop::UnexpectedSupersessionPosture => {
            WorthUiPresentationSettlementStop::UnexpectedQueryPosture
        }
        WorthUiPresentationAdmissionStop::SemanticRetirement => {
            WorthUiPresentationSettlementStop::SemanticRetirement
        }
        _ => WorthUiPresentationSettlementStop::QuerySupersession,
    };
    WorthUiPresentationSettlementDenial::Progress(stop)
}

pub(super) fn semantic_frontier_observation(
    change: WorthUiPresentationSemanticChange,
    execution: &super::super::semantic_registry::WorthUiPresentationSemanticExecution,
) -> WorthUiPresentationSemanticFrontierObservation {
    WorthUiPresentationSemanticFrontierObservation {
        change,
        subscribers: execution.subscribers().into(),
        source_deliveries: u32::try_from(execution.deliveries().len()).unwrap_or(u32::MAX),
        outcomes: execution
            .query_observations()
            .iter()
            .map(WorthUiPresentationSemanticQueryObservation::outcome)
            .collect::<Vec<_>>()
            .into_boxed_slice(),
        performed: execution
            .query_observations()
            .iter()
            .map(|observation| *observation.performed_signal_invalidation())
            .collect::<Vec<_>>()
            .into_boxed_slice(),
        scope_rejections: execution.scope_rejections(),
    }
}
