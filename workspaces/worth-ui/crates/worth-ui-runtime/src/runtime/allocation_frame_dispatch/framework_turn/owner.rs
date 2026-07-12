use crate::runtime::launch::runtime_instance::WorthUiRuntime;
use crate::runtime::UiAllocationFrameGatewayOutcome;

use super::UiAllocationFrameTurnOutcome;
use super::WorthUiFrameworkTurnCompletion;
use super::WorthUiFrameworkTurnExecution;
/// The single ordinary production turn that may collect allocation sources.
///
/// Source callbacks receive this borrowed capability, never the runtime or the
/// dispatcher. When the callback returns, `WorthUiRuntime::execute_framework_turn`
/// closes and pumps exactly once before yielding an activation boundary.
pub struct WorthUiFrameworkTurn<'runtime> {
    pub(super) runtime: &'runtime mut WorthUiRuntime,
}

pub struct WorthUiHostMeasurementTurnSource<'turn> {
    pub(super) runtime: &'turn mut WorthUiRuntime,
}

pub struct WorthUiQueryProjectionTurnSource<'turn> {
    pub(super) runtime: &'turn mut WorthUiRuntime,
}

pub struct WorthUiInteractionTurnSource<'turn> {
    pub(super) runtime: &'turn mut WorthUiRuntime,
}

pub struct WorthUiResizePreviewTurnSource<'turn> {
    pub(super) runtime: &'turn mut WorthUiRuntime,
}

pub struct WorthUiScrollOffsetTurnSource<'turn> {
    pub(super) runtime: &'turn mut WorthUiRuntime,
}

pub struct WorthUiDurableResizeTurnSource<'turn> {
    pub(super) runtime: &'turn mut WorthUiRuntime,
}

/// ```compile_fail
/// use worth_ui_runtime::facade::runtime_handoff::WorthUiRuntime;
///
/// fn caller_cannot_pump_runtime(runtime: &mut WorthUiRuntime) {
///     runtime.run_allocation_frame_turn();
/// }
/// ```
/// Production owner of source capabilities and framework turns. Source
/// capabilities enqueue through the dispatcher and cannot close it.
/// The framework boundary advances only through `WorthUiRuntimeFrameworkLoop`.
impl WorthUiRuntime {
    /// Run one ordinary framework turn and close/pump it exactly once.
    ///
    /// The borrowed turn exposes source-family admission only. It cannot close,
    /// pump, classify policy, acknowledge a handoff, or commit a receipt.
    ///
    /// ```compile_fail
    /// use worth_ui_runtime::facade::runtime_handoff::WorthUiRuntime;
    ///
    /// fn caller_cannot_clock_allocation_turn(runtime: &mut WorthUiRuntime) {
    ///     runtime.framework_turn(|_| {});
    /// }
    /// ```
    pub fn execute_framework_turn(
        &mut self,
        collect_sources: impl FnOnce(&mut WorthUiFrameworkTurn<'_>),
    ) -> WorthUiFrameworkTurnCompletion<'_> {
        if self.pending_narrowed_allocation_frame.is_some() {
            return WorthUiFrameworkTurnCompletion::Phase6Backpressured;
        }
        if self.pending_allocation_frame_handoff.is_some() {
            return WorthUiFrameworkTurnCompletion::UnacceptedFrameBackpressured;
        }
        let collection = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            collect_sources(&mut WorthUiFrameworkTurn { runtime: self });
        }));
        let completion = self.close_allocation_ingress_at_framework_boundary();
        if let Err(payload) = collection {
            std::panic::resume_unwind(payload);
        }
        completion
    }

    fn close_allocation_ingress_at_framework_boundary(
        &mut self,
    ) -> WorthUiFrameworkTurnCompletion<'_> {
        let outcome = self.advance_allocation_frame_at_framework_boundary();
        match outcome {
            UiAllocationFrameTurnOutcome::DownstreamBackpressured { sealed_frame } => {
                self.pending_allocation_frame_handoff = Some(
                    super::UiPendingAllocationFrameHandoff::unchanged(sealed_frame),
                );
                match crate::runtime::stream_policy::consume_pending_frame(
                    &mut self.pending_allocation_frame_handoff,
                    &mut self.allocation_source_order_ledger,
                ) {
                    crate::runtime::stream_policy::UiAllocationFrameConsumptionDisposition::Accepted(plan) =>
                        match crate::runtime::invalidation_narrowing::narrow_resolved_frame(
                            plan,
                            &self.allocation_invalidation_index,
                        ) {
                            crate::runtime::invalidation_narrowing::UiAllocationInvalidationNarrowingDisposition::Accepted(plan) => {
                                let mut authority = self.allocation_invalidation_index.borrow_mut();
                                match crate::runtime::viewport_resize::UiResolvedAllocationCommitPlan::classify(&plan) {
                                    crate::runtime::viewport_resize::UiResolvedAllocationCommitPlan::Viewport(viewport_plan) => {
                                    let basis = match crate::runtime::UiViewportResizeCommitBasis::select(
                                        viewport_plan,
                                        &authority,
                                    ) {
                                        Ok(basis) => basis,
                                        Err(denial) => return WorthUiFrameworkTurnCompletion::AllocationReplanSelectionDenied { denial },
                                    };
                                    let outcome = crate::runtime::UiViewportResizeOutcome::resolve(
                                        basis,
                                        |basis| super::allocation_transaction::commit_viewport(
                                            &self.allocation_receipt_ledger,
                                            &mut authority,
                                            basis,
                                        ),
                                    );
                                    return match outcome {
                                        Ok(outcome) => {
                                            WorthUiFrameworkTurnCompletion::ViewportResizeResolved { outcome }
                                        },
                                        Err(denial) => WorthUiFrameworkTurnCompletion::ViewportResizeDenied { denial },
                                    };
                                    }
                                    crate::runtime::viewport_resize::UiResolvedAllocationCommitPlan::Ordinary => {
                                        let selection = match crate::graph::select_replan_neighborhoods(&plan, &authority) {
                                            Ok(selection) => selection,
                                            Err(denial) => return WorthUiFrameworkTurnCompletion::AllocationReplanSelectionDenied { denial },
                                        };
                                        let transaction = super::allocation_transaction::commit_selected(
                                            &self.allocation_receipt_ledger,
                                            &mut authority,
                                            &selection,
                                        );
                                        WorthUiFrameworkTurnCompletion::AllocationInvalidationsNarrowed {
                                            plan,
                                            selection,
                                            transaction,
                                        }
                                    }
                                    crate::runtime::viewport_resize::UiResolvedAllocationCommitPlan::ResizePreview(preview_plan) => {
                                        let selection = match crate::graph::select_replan_neighborhoods(preview_plan, &authority) {
                                            Ok(selection) => selection,
                                            Err(denial) => return WorthUiFrameworkTurnCompletion::AllocationReplanSelectionDenied { denial },
                                        };
                                        match crate::runtime::UiResizePreviewOutcome::from_selection(preview_plan, &selection) {
                                            Ok(outcome) => WorthUiFrameworkTurnCompletion::ResizePreviewPublished {
                                                pending: super::WorthUiPendingPreviewPaint::new(
                                                    crate::host::seal_preview_paint_input(outcome),
                                                    crate::runtime::allocation_receipt::UiPreviewPaintIsolationPort::new(
                                                        &self.allocation_receipt_ledger,
                                                    ),
                                                ),
                                            },
                                            Err(denial) => WorthUiFrameworkTurnCompletion::AllocationInvalidationsNarrowed {
                                                plan,
                                                selection,
                                                transaction: crate::runtime::UiAllocationReplanTransactionOutcome::Denied(denial),
                                            },
                                        }
                                    }
                                    crate::runtime::viewport_resize::UiResolvedAllocationCommitPlan::DurableResize(durable_plan) => {
                                        let selection = match crate::graph::select_replan_neighborhoods(durable_plan, &authority) {
                                            Ok(selection) => selection,
                                            Err(denial) => return WorthUiFrameworkTurnCompletion::AllocationReplanSelectionDenied { denial },
                                        };
                                        let extent = durable_plan.durable_resize_extent().expect("durable policy carries extent");
                                        let identity = durable_plan.durable_resize_identity_digest().expect("durable policy carries identity");
                                        let (transaction, durable_state, mutated) =
                                            super::allocation_transaction::commit_durable_resize(
                                                &self.allocation_receipt_ledger,
                                                &mut authority,
                                                &selection,
                                                identity,
                                                extent,
                                            );
                                        match transaction {
                                            crate::runtime::UiAllocationReplanTransactionOutcome::Committed(committed) => {
                                                WorthUiFrameworkTurnCompletion::DurableResizeCommitted { outcome: crate::runtime::UiDurableResizeCommitOutcome::new(extent, committed, durable_state.expect("committed resize owns activated semantic state"), mutated, false), selection }
                                            }
                                            crate::runtime::UiAllocationReplanTransactionOutcome::Replayed(committed) => {
                                                let extent = durable_plan.durable_resize_extent().expect("durable policy carries extent");
                                                WorthUiFrameworkTurnCompletion::DurableResizeCommitted { outcome: crate::runtime::UiDurableResizeCommitOutcome::new(extent, committed, durable_state.expect("replayed resize owns activated semantic state"), false, true), selection }
                                            }
                                            crate::runtime::UiAllocationReplanTransactionOutcome::Denied(_) => WorthUiFrameworkTurnCompletion::AllocationInvalidationsNarrowed { plan, selection, transaction },
                                        }
                                    }
                                    crate::runtime::viewport_resize::UiResolvedAllocationCommitPlan::DragResize(drag_plan) => {
                                        let selection = match crate::graph::select_replan_neighborhoods(drag_plan, &authority) {
                                            Ok(selection) => selection,
                                            Err(denial) => return WorthUiFrameworkTurnCompletion::AllocationReplanSelectionDenied { denial },
                                        };
                                        let preview = match crate::runtime::UiResizePreviewOutcome::from_selection(drag_plan, &selection) {
                                            Ok(preview) => preview,
                                            Err(denial) => return WorthUiFrameworkTurnCompletion::AllocationInvalidationsNarrowed {
                                                plan,
                                                selection,
                                                transaction: crate::runtime::UiAllocationReplanTransactionOutcome::Denied(denial),
                                            },
                                        };
                                        let extent = drag_plan.durable_resize_extent().expect("drag-resize policy carries terminal extent");
                                        let identity = drag_plan.durable_resize_identity_digest().expect("drag-resize policy carries durable identity");
                                        drop(authority);
                                        WorthUiFrameworkTurnCompletion::DragResizePreviewPending {
                                            preview: super::WorthUiPendingPreviewPaint::new(
                                                crate::host::seal_preview_paint_input(preview),
                                                crate::runtime::allocation_receipt::UiPreviewPaintIsolationPort::new(
                                                    &self.allocation_receipt_ledger,
                                                ),
                                            ),
                                            durable: super::WorthUiPendingDurableResize::new(
                                                super::UiPendingDurableResizeCommitPort::new(
                                                    &self.allocation_receipt_ledger,
                                                    &self.allocation_invalidation_index,
                                                    &selection,
                                                    identity,
                                                    extent,
                                                ),
                                                selection,
                                                identity,
                                                extent,
                                            ),
                                        }
                                    }
                                }
                            }
                            crate::runtime::invalidation_narrowing::UiAllocationInvalidationNarrowingDisposition::Rejected(rejection) => {
                                WorthUiFrameworkTurnCompletion::AllocationInvalidationNarrowingDenied { rejection }
                            }
                        },
                    crate::runtime::stream_policy::UiAllocationFrameConsumptionDisposition::Rejected(rejection) => {
                        WorthUiFrameworkTurnCompletion::AllocationFrameResolutionDenied {
                            rejection,
                        }
                    }
                }
            }
            UiAllocationFrameTurnOutcome::NoAdmittedIngress { .. } => {
                let boundary =
                    crate::runtime::WorthUiFrameBoundary::safe_to_activate(self.frame_epoch());
                WorthUiFrameworkTurnCompletion::ReadyToExecute {
                    execution: WorthUiFrameworkTurnExecution {
                        _runtime: self,
                        boundary,
                    },
                }
            }
            UiAllocationFrameTurnOutcome::Denied { denial, counters } => {
                WorthUiFrameworkTurnCompletion::Denied { denial, counters }
            }
        }
    }

    pub(in crate::runtime::allocation_frame_dispatch) fn host_measurement_collector(
        &self,
    ) -> crate::host::WorthUiHostMeasurementCollector {
        crate::host::WorthUiHostMeasurementCollector::new(std::rc::Rc::clone(
            &self.host_measurement_source,
        ))
    }

    pub(super) fn collect_and_submit_host_measurement<
        A: worth_ui_host_contract::WorthUiMeasurementHostAdapter,
    >(
        &self,
        adapter: &A,
        identity: worth_ui_host_contract::UiMeasurementRequestIdentity,
        evidence_family: worth_ui_host_contract::UiMeasurementEvidenceFamily,
        need: crate::host::UiHostMeasurementNeed,
        capability_report: &worth_ui_host_contract::WorthUiHostCapabilityReport,
        evidence_generation: worth_ui_inspection::UiEvidenceAuthorityGeneration,
        normalization_context: crate::host::UiHostMeasurementNormalizationContext,
    ) -> Result<UiAllocationFrameGatewayOutcome, crate::host::UiHostMeasurementEvidenceDenial> {
        let admitted = self.host_measurement_collector().collect_admitted(
            adapter,
            identity,
            evidence_family,
            need,
            capability_report,
            evidence_generation,
            normalization_context,
        )?;
        Ok(self
            .host_measurement_submission()
            .submit_admitted_host_measurement(admitted))
    }

    pub(super) fn admit_and_submit_query_projection(
        &mut self,
        prerequisites: worth_ui_query_binding::WorthUiQueryPrerequisiteEvidence,
        authority: worth_query::facade::ProjectionAuthorityOutcome,
    ) -> Result<
        UiAllocationFrameGatewayOutcome,
        worth_ui_query_binding::WorthUiQueryMeasurementFactSettlementDenial,
    > {
        let settlement = self
            .query_binding
            .allocation_admission()
            .admit(prerequisites, authority)?;
        Ok(self
            .query_projection_submission()
            .submit_query_projection_settlement(settlement))
    }

    pub(super) fn admit_and_submit_interaction(
        &mut self,
        target: crate::graph::UiGraphNodeIdentity,
        state: crate::runtime::WorthUiTransientInteractionState,
    ) -> Result<
        UiAllocationFrameGatewayOutcome,
        crate::runtime::WorthUiTransientInteractionAdmissionDenial,
    > {
        let admitted = self.transient_interaction_admission.admit(target, state)?;
        Ok(self
            .interaction_submission()
            .submit_admitted_transient_interaction(admitted))
    }

    pub(super) fn admit_and_submit_resize_preview(
        &mut self,
        sample: crate::runtime::UiResizePreviewSample,
    ) -> Result<
        UiAllocationFrameGatewayOutcome,
        crate::runtime::WorthUiTransientInteractionAdmissionDenial,
    > {
        let admitted = self
            .transient_interaction_admission
            .admit_resize_preview(sample)?;
        Ok(self
            .interaction_submission()
            .submit_admitted_transient_interaction(admitted))
    }

    pub(super) fn admit_and_submit_durable_resize(
        &mut self,
        input: crate::runtime::UiDurableResizeCommitIntent,
    ) -> Result<
        UiAllocationFrameGatewayOutcome,
        crate::runtime::WorthUiDurableResizeSourceAdmissionDenial,
    > {
        let admitted = self.durable_resize_source.admit(input)?;
        Ok(self
            .durable_resize_submission()
            .submit_admitted_durable_resize(admitted))
    }

    pub(in crate::runtime::allocation_frame_dispatch) fn host_measurement_submission(
        &self,
    ) -> super::super::gateway::WorthUiHostMeasurementSubmission {
        super::super::gateway::WorthUiHostMeasurementSubmission::new(
            self.allocation_frame_scheduler.mailbox(),
        )
    }

    pub(in crate::runtime::allocation_frame_dispatch) fn query_projection_submission(
        &self,
    ) -> super::super::gateway::WorthUiQueryProjectionSubmission {
        super::super::gateway::WorthUiQueryProjectionSubmission::new(
            self.allocation_frame_scheduler.mailbox(),
        )
    }

    pub(in crate::runtime::allocation_frame_dispatch) fn interaction_submission(
        &self,
    ) -> super::super::gateway::WorthUiInteractionSubmission {
        super::super::gateway::WorthUiInteractionSubmission::new(
            self.allocation_frame_scheduler.mailbox(),
        )
    }

    pub(in crate::runtime::allocation_frame_dispatch) fn durable_resize_submission(
        &self,
    ) -> super::super::gateway::WorthUiDurableResizeSubmission {
        super::super::gateway::WorthUiDurableResizeSubmission::new(
            self.allocation_frame_scheduler.mailbox(),
        )
    }

    fn advance_allocation_frame_at_framework_boundary(&mut self) -> UiAllocationFrameTurnOutcome {
        let (outcome, epoch_assignment) = self.allocation_frame_scheduler.run_turn();
        if let Some(assignment) = epoch_assignment {
            self.active
                .apply_allocation_frame_epoch_assignment(assignment);
        }
        outcome
    }
}
