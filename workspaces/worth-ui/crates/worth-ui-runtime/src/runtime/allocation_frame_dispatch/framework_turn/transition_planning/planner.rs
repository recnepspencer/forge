use super::{
    UiDeniedAllocationExecutionPlan, UiDragResizeExecutionPlan, UiDurableResizeExecutionPlan,
    UiFrameworkTransitionFamilyPlan, UiFrameworkTransitionPlanningCounters,
    UiFrameworkTransitionPlanningDenial, UiOrdinaryAllocationExecutionPlan,
    UiPlannedFrameworkTransition, UiViewportResizeExecutionPlan,
};

pub(in crate::runtime::allocation_frame_dispatch::framework_turn) enum UiFrameworkTransitionPlanningDisposition
{
    Planned(Box<UiPlannedFrameworkTransition>),
    FrameResolutionDenied(crate::runtime::UiAllocationFrameRejection),
    InvalidationNarrowingDenied(crate::runtime::UiAllocationInvalidationNarrowingRejection),
    ReplanSelectionDenied(crate::graph::UiReplanLocalityDenial),
    PlanningDenied(UiFrameworkTransitionPlanningDenial),
    DispatchDenied {
        denial: crate::runtime::UiAllocationFrameDispatchDenial,
        counters: crate::runtime::UiAllocationFrameDispatcherCounters,
    },
}

pub(in crate::runtime::allocation_frame_dispatch::framework_turn) fn plan_framework_transition(
    turn: super::super::UiAllocationFrameTurnOutcome,
    active_generation: &crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
    active_frame_epoch: crate::runtime::WorthUiRuntimeFrameEpoch,
    source_order_ledger: &crate::runtime::stream_policy::UiAllocationSourceOrderLedger,
    receipt_ledger: &crate::runtime::allocation_receipt::UiAllocationReceiptLedger,
    invalidation_authority: &std::cell::RefCell<
        crate::runtime::invalidation_narrowing::UiAllocationInvalidationAuthority,
    >,
) -> UiFrameworkTransitionPlanningDisposition {
    match turn {
        super::super::UiAllocationFrameTurnOutcome::NoAdmittedIngress { .. } => {
            UiFrameworkTransitionPlanningDisposition::Planned(Box::new(
                UiPlannedFrameworkTransition::no_ingress(
                    active_generation.clone(),
                    active_frame_epoch,
                ),
            ))
        }
        super::super::UiAllocationFrameTurnOutcome::Denied { denial, counters } => {
            UiFrameworkTransitionPlanningDisposition::DispatchDenied { denial, counters }
        }
        super::super::UiAllocationFrameTurnOutcome::SealedFrameReady {
            sealed_frame,
            frame_epoch_assignment,
        } => plan_sealed_frame(
            super::super::UiPendingAllocationFrameHandoff::unchanged(*sealed_frame),
            frame_epoch_assignment,
            active_generation,
            active_frame_epoch,
            source_order_ledger,
            receipt_ledger,
            invalidation_authority,
        ),
    }
}

fn plan_sealed_frame(
    handoff: super::super::UiPendingAllocationFrameHandoff,
    frame_epoch_assignment: crate::runtime::allocation_frame_dispatch::UiAllocationFrameEpochAssignment,
    active_generation: &crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
    active_frame_epoch: crate::runtime::WorthUiRuntimeFrameEpoch,
    source_order_ledger: &crate::runtime::stream_policy::UiAllocationSourceOrderLedger,
    receipt_ledger: &crate::runtime::allocation_receipt::UiAllocationReceiptLedger,
    invalidation_authority: &std::cell::RefCell<
        crate::runtime::invalidation_narrowing::UiAllocationInvalidationAuthority,
    >,
) -> UiFrameworkTransitionPlanningDisposition {
    let (resolved, source_order_transition) =
        match crate::runtime::stream_policy::consume_pending_frame(handoff, source_order_ledger) {
            crate::runtime::stream_policy::UiAllocationFrameConsumptionDisposition::Accepted {
                plan,
                source_order_transition,
            } => (plan, source_order_transition),
            crate::runtime::stream_policy::UiAllocationFrameConsumptionDisposition::Rejected(
                rejection,
            ) => return UiFrameworkTransitionPlanningDisposition::FrameResolutionDenied(rejection),
        };
    let narrowed = match crate::runtime::invalidation_narrowing::narrow_resolved_frame(
        resolved,
        invalidation_authority,
    ) {
        crate::runtime::invalidation_narrowing::UiAllocationInvalidationNarrowingDisposition::Accepted(
            plan,
        ) => plan,
        crate::runtime::invalidation_narrowing::UiAllocationInvalidationNarrowingDisposition::Rejected(
            rejection,
        ) => {
            return UiFrameworkTransitionPlanningDisposition::InvalidationNarrowingDenied(
                rejection,
            )
        }
    };
    if frame_epoch_assignment.epoch() != narrowed.frame_epoch() {
        return UiFrameworkTransitionPlanningDisposition::PlanningDenied(
            UiFrameworkTransitionPlanningDenial::FrameEpochAssignmentMismatch,
        );
    }
    let selection = {
        let authority = invalidation_authority.borrow();
        match crate::graph::select_replan_neighborhoods(&narrowed, &authority) {
            Ok(selection) => selection,
            Err(denial) => {
                return UiFrameworkTransitionPlanningDisposition::ReplanSelectionDenied(denial)
            }
        }
    };
    let counters = UiFrameworkTransitionPlanningCounters::from_planned_frame(&narrowed, &selection);
    let family = match classify_family(
        narrowed,
        selection,
        receipt_ledger,
        &invalidation_authority.borrow(),
    ) {
        Ok(family) => family,
        Err(denial) => return UiFrameworkTransitionPlanningDisposition::PlanningDenied(denial),
    };
    UiFrameworkTransitionPlanningDisposition::Planned(Box::new(
        UiPlannedFrameworkTransition::admitted_frame(
            active_generation.clone(),
            active_frame_epoch,
            frame_epoch_assignment,
            source_order_transition,
            counters,
            family,
        ),
    ))
}

fn classify_family(
    plan: crate::runtime::UiNarrowedAllocationFramePlan,
    selection: crate::graph::UiAdmittedReplanNeighborhoodSet,
    receipt_ledger: &crate::runtime::allocation_receipt::UiAllocationReceiptLedger,
    invalidation_authority: &crate::runtime::invalidation_narrowing::UiAllocationInvalidationAuthority,
) -> Result<UiFrameworkTransitionFamilyPlan, UiFrameworkTransitionPlanningDenial> {
    use crate::runtime::stream_policy::UiAllocationResolvedCommitLane;

    Ok(match plan.policy().commit_lane() {
        UiAllocationResolvedCommitLane::Ordinary => {
            let transaction = super::super::allocation_transaction::prepare_selected(
                receipt_ledger,
                invalidation_authority,
                &selection,
            );
            UiFrameworkTransitionFamilyPlan::Ordinary(UiOrdinaryAllocationExecutionPlan {
                plan,
                selection,
                transaction,
            })
        }
        UiAllocationResolvedCommitLane::ViewportDerived => {
            let basis = match crate::runtime::UiViewportResizeCommitBasis::admit(plan, selection) {
                Ok(basis) => basis,
                Err(denial) => return Ok(UiFrameworkTransitionFamilyPlan::ViewportDenied(denial)),
            };
            let transaction = super::super::allocation_transaction::prepare_viewport(
                receipt_ledger,
                invalidation_authority,
                basis,
            );
            UiFrameworkTransitionFamilyPlan::Viewport(UiViewportResizeExecutionPlan { transaction })
        }
        UiAllocationResolvedCommitLane::ResizePreview => {
            match crate::runtime::UiResizePreviewOutcome::from_selection(&plan, &selection) {
                Ok(outcome) => UiFrameworkTransitionFamilyPlan::ResizePreview(outcome),
                Err(denial) => UiFrameworkTransitionFamilyPlan::AllocationDenied(
                    UiDeniedAllocationExecutionPlan {
                        plan,
                        selection,
                        denial,
                    },
                ),
            }
        }
        UiAllocationResolvedCommitLane::DurableResize => {
            let identity_digest = plan
                .durable_resize_identity_digest()
                .ok_or(UiFrameworkTransitionPlanningDenial::DurableResizeIdentityMissing)?;
            let extent = plan
                .durable_resize_extent()
                .ok_or(UiFrameworkTransitionPlanningDenial::DurableResizeExtentMissing)?;
            let previous_extent = receipt_ledger.durable_semantic_state().and_then(|state| {
                state
                    .committed_resize(identity_digest)
                    .map(|basis| basis.extent())
            });
            let (transaction, requested_mutation) =
                super::super::allocation_transaction::prepare_pending_durable_resize(
                    receipt_ledger,
                    invalidation_authority,
                    &selection,
                    identity_digest,
                    extent,
                );
            UiFrameworkTransitionFamilyPlan::DurableResize(UiDurableResizeExecutionPlan {
                plan,
                selection,
                transaction,
                extent,
                previous_extent,
                requested_mutation,
            })
        }
        UiAllocationResolvedCommitLane::DragResize => {
            let identity_digest = plan
                .durable_resize_identity_digest()
                .ok_or(UiFrameworkTransitionPlanningDenial::DragResizeIdentityMissing)?;
            let extent = plan
                .durable_resize_extent()
                .ok_or(UiFrameworkTransitionPlanningDenial::DragResizeExtentMissing)?;
            let preview =
                match crate::runtime::UiResizePreviewOutcome::from_selection(&plan, &selection) {
                    Ok(preview) => preview,
                    Err(denial) => {
                        return Ok(UiFrameworkTransitionFamilyPlan::AllocationDenied(
                            UiDeniedAllocationExecutionPlan {
                                plan,
                                selection,
                                denial,
                            },
                        ))
                    }
                };
            let (transaction, _) =
                super::super::allocation_transaction::prepare_pending_durable_resize(
                    receipt_ledger,
                    invalidation_authority,
                    &selection,
                    identity_digest,
                    extent,
                );
            UiFrameworkTransitionFamilyPlan::DragResize(UiDragResizeExecutionPlan {
                preview,
                selection,
                transaction,
                identity_digest,
                extent,
            })
        }
    })
}
