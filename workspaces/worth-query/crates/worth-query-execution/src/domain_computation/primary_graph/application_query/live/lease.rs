use std::marker::PhantomData;
use std::rc::Rc;

use worth_foundational::facade::AspectValue;
use worth_query_admission::facade::authenticated_principal::WorthQueryRequestInterruption;
use worth_query_declaration::facade::{
    application_query::{ApplicationQueryLiveCauseBinding, ApplicationQueryParameterSet},
    application_schema::{ApplicationSchema, TypedApplicationValue},
};
use worth_query_installation::facade::WorthQueryInstalledApplicationQuery;
use worth_runtime_bridge::facade::BridgeExecutionBasisTerminalDisposition;

mod lifecycle;
mod open;
mod validation;

use super::{
    controls::WorthQueryApplicationLiveControls,
    outcome::{
        WorthQueryApplicationLiveCauseDenialKind, WorthQueryApplicationLiveCloseOutcome,
        WorthQueryApplicationLiveOutcome, WorthQueryApplicationLiveOverflow,
        WorthQueryApplicationLiveUpdate,
    },
    projection::{finalize_live_projection, WorthQueryLiveProjectionFinalizationDenial},
};
use crate::domain_computation::{
    managed_run::WorthQueryManagedLowerExecutionBasis,
    primary_graph::{
        application_query::{
            authorized_read::{execute_authorized_read, WorthQueryAuthorizedApplicationReadDenial},
            read_execution::{read_live_target, WorthQueryApplicationReadExecutionDenialKind},
            WorthQueryApplicationProjection, WorthQueryApplicationQueryAccessContext,
            WorthQueryApplicationQueryAdmissionDenial,
            WorthQueryApplicationQueryAdmissionDenialKind, WorthQueryApplicationQueryControls,
        },
        live_delivery::{WorthQueryLiveCauseFillPosture, WorthQueryLiveCauseQueue},
        WorthQueryApplicationEntityIdentity, WorthQueryAuthenticatedPrincipal,
        WorthQueryPrimaryGraphApplicationRuntime,
    },
};

pub struct WorthQueryApplicationLiveLease<
    'runtime,
    'principal,
    Schema,
    Query,
    Parameters,
    QueryResult,
    Principal,
    PrincipalIdentity,
    Scope,
    Target,
    Binding,
> where
    Binding: ApplicationQueryLiveCauseBinding<Schema, Query, Scope, Target>,
{
    runtime: &'runtime WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    query: WorthQueryInstalledApplicationQuery<Schema, Query, Parameters, QueryResult, Scope>,
    principal: &'principal WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
    scope: WorthQueryApplicationEntityIdentity<Schema, Scope>,
    parameters: ApplicationQueryParameterSet<Query>,
    controls: WorthQueryApplicationLiveControls,
    scope_identity: AspectValue,
    basis: Option<WorthQueryManagedLowerExecutionBasis>,
    queue: WorthQueryLiveCauseQueue<Binding::Payload>,
    _target: PhantomData<fn() -> (Target, Binding)>,
    _thread_affinity: PhantomData<Rc<()>>,
}

impl<
        Schema,
        Query,
        Parameters,
        QueryResult,
        Principal,
        PrincipalIdentity,
        Scope,
        Target,
        Binding,
    >
    WorthQueryApplicationLiveLease<
        '_,
        '_,
        Schema,
        Query,
        Parameters,
        QueryResult,
        Principal,
        PrincipalIdentity,
        Scope,
        Target,
        Binding,
    >
where
    Schema: ApplicationSchema,
    QueryResult: WorthQueryApplicationProjection<Schema, Query>,
    Binding: ApplicationQueryLiveCauseBinding<Schema, Query, Scope, Target>,
{
    pub fn buffered_cause_count(&self) -> usize {
        self.queue.buffered_cause_count()
    }

    pub fn poll(&mut self) -> WorthQueryApplicationLiveOutcome<Query, QueryResult> {
        if self.basis.is_none() {
            return WorthQueryApplicationLiveOutcome::Closed;
        }
        if let Some(interruption) = self.controls.request().interruption() {
            if !self.terminate(BridgeExecutionBasisTerminalDisposition::Cancelled) {
                return WorthQueryApplicationLiveOutcome::Unavailable;
            }
            return match interruption {
                WorthQueryRequestInterruption::Cancelled => {
                    WorthQueryApplicationLiveOutcome::Cancelled
                }
                WorthQueryRequestInterruption::DeadlineExceeded => {
                    WorthQueryApplicationLiveOutcome::DeadlineExceeded
                }
            };
        }
        let fill = {
            let Some(basis) = self.basis.as_mut() else {
                return WorthQueryApplicationLiveOutcome::Closed;
            };
            let expected_scope = &self.scope_identity;
            self.queue.fill(
                &self.runtime.primary_provider.live_delivery,
                basis,
                Binding::effect(),
                self.controls.buffer_capacity(),
                |payload| {
                    Binding::scope_identity(payload).into_foundational_value() == *expected_scope
                },
            )
        };
        let terminal = match fill {
            WorthQueryLiveCauseFillPosture::Pending => WorthQueryApplicationLiveOutcome::Pending,
            WorthQueryLiveCauseFillPosture::Overflow(missed) => {
                WorthQueryApplicationLiveOutcome::Overflow(WorthQueryApplicationLiveOverflow::new(
                    missed,
                ))
            }
            WorthQueryLiveCauseFillPosture::Closed => WorthQueryApplicationLiveOutcome::Closed,
            WorthQueryLiveCauseFillPosture::Unavailable => {
                WorthQueryApplicationLiveOutcome::Unavailable
            }
        };
        let Some((commit_id, payload)) = self.queue.front() else {
            return match terminal {
                WorthQueryApplicationLiveOutcome::Overflow(overflow) => {
                    if self.terminate(BridgeExecutionBasisTerminalDisposition::Cancelled) {
                        WorthQueryApplicationLiveOutcome::Overflow(overflow)
                    } else {
                        WorthQueryApplicationLiveOutcome::Unavailable
                    }
                }
                WorthQueryApplicationLiveOutcome::Closed => {
                    if self.terminate(BridgeExecutionBasisTerminalDisposition::Completed) {
                        WorthQueryApplicationLiveOutcome::Closed
                    } else {
                        WorthQueryApplicationLiveOutcome::Unavailable
                    }
                }
                outcome => outcome,
            };
        };
        let target_identity = Binding::target_identity(payload).into_foundational_value();
        self.project_front(commit_id, target_identity)
    }

    fn project_front(
        &mut self,
        commit_id: worth_relational::facade::history::CommitId,
        target_identity: AspectValue,
    ) -> WorthQueryApplicationLiveOutcome<Query, QueryResult> {
        let access = WorthQueryApplicationQueryAccessContext::new(self.principal, &self.scope);
        let controls = WorthQueryApplicationQueryControls::current_live(
            self.controls.maximum_materialized_record_count(),
            self.controls.maximum_work_per_delivery(),
            self.controls.request(),
        );
        let plan = match self.runtime.admit_application_query(
            &self.query,
            &access,
            self.parameters.clone(),
            controls,
        ) {
            Ok(plan) => plan,
            Err(denial) => return self.handle_admission_denial(denial),
        };
        let Some(graph) = self.runtime.runtime.primary_graph() else {
            let _ = plan.basis.release();
            return WorthQueryApplicationLiveOutcome::Unavailable;
        };
        let result_buffer = self.runtime.result_buffers.reserve(
            plan.graph_read_plan
                .budget_check()
                .max_inline_result_bytes(),
        );
        let (raw, authorization_work) =
            match execute_authorized_read(self.runtime, graph, &plan, |runtime, graph, plan| {
                read_live_target(runtime, graph, plan, target_identity, result_buffer)
            }) {
                Ok(raw) => raw,
                Err(denial) => {
                    let released = plan.basis.release().released();
                    if !released {
                        return WorthQueryApplicationLiveOutcome::Unavailable;
                    }
                    return self.handle_read_denial(denial);
                }
            };
        match finalize_live_projection(plan, raw, authorization_work) {
            Ok((result, receipt)) => {
                if !self.acknowledge_front() {
                    return WorthQueryApplicationLiveOutcome::Unavailable;
                }
                WorthQueryApplicationLiveOutcome::Delivered(WorthQueryApplicationLiveUpdate::new(
                    commit_id, result, receipt,
                ))
            }
            Err(WorthQueryLiveProjectionFinalizationDenial::BasisRelease) => {
                WorthQueryApplicationLiveOutcome::Unavailable
            }
            Err(WorthQueryLiveProjectionFinalizationDenial::ResultShape) => self
                .acknowledge_cause_denial(
                    WorthQueryApplicationLiveCauseDenialKind::ResultShapeUnavailable,
                ),
            Err(WorthQueryLiveProjectionFinalizationDenial::Projection(kind)) => {
                if self.acknowledge_front() {
                    WorthQueryApplicationLiveOutcome::ProjectionDenied(kind)
                } else {
                    WorthQueryApplicationLiveOutcome::Unavailable
                }
            }
        }
    }

    fn handle_admission_denial(
        &mut self,
        denial: WorthQueryApplicationQueryAdmissionDenial,
    ) -> WorthQueryApplicationLiveOutcome<Query, QueryResult> {
        match denial.kind() {
            WorthQueryApplicationQueryAdmissionDenialKind::Cancelled => {
                if self.terminate(BridgeExecutionBasisTerminalDisposition::Cancelled) {
                    WorthQueryApplicationLiveOutcome::Cancelled
                } else {
                    WorthQueryApplicationLiveOutcome::Unavailable
                }
            }
            WorthQueryApplicationQueryAdmissionDenialKind::DeadlineExceeded => {
                if self.terminate(BridgeExecutionBasisTerminalDisposition::Cancelled) {
                    WorthQueryApplicationLiveOutcome::DeadlineExceeded
                } else {
                    WorthQueryApplicationLiveOutcome::Unavailable
                }
            }
            WorthQueryApplicationQueryAdmissionDenialKind::Authorization(kind) => self
                .acknowledge_and_terminate(WorthQueryApplicationLiveOutcome::AuthorizationDenied(
                    kind,
                )),
            WorthQueryApplicationQueryAdmissionDenialKind::StalePrincipal
            | WorthQueryApplicationQueryAdmissionDenialKind::ForeignPrincipal => {
                self.acknowledge_and_terminate(WorthQueryApplicationLiveOutcome::StalePrincipal)
            }
            WorthQueryApplicationQueryAdmissionDenialKind::StaleScope
            | WorthQueryApplicationQueryAdmissionDenialKind::ForeignScope
            | WorthQueryApplicationQueryAdmissionDenialKind::ScopeTypeMismatch => {
                self.acknowledge_and_terminate(WorthQueryApplicationLiveOutcome::StaleScope)
            }
            _ => WorthQueryApplicationLiveOutcome::Unavailable,
        }
    }

    fn handle_read_denial(
        &mut self,
        denial: WorthQueryAuthorizedApplicationReadDenial,
    ) -> WorthQueryApplicationLiveOutcome<Query, QueryResult> {
        match denial {
            WorthQueryAuthorizedApplicationReadDenial::StaleScope
            | WorthQueryAuthorizedApplicationReadDenial::StaleBasisScope(_) => {
                self.acknowledge_and_terminate(WorthQueryApplicationLiveOutcome::StaleScope)
            }
            WorthQueryAuthorizedApplicationReadDenial::Authorization(kind, _) => self
                .acknowledge_and_terminate(WorthQueryApplicationLiveOutcome::AuthorizationDenied(
                    kind,
                )),
            WorthQueryAuthorizedApplicationReadDenial::Read(read) => match read.kind() {
                WorthQueryApplicationReadExecutionDenialKind::TargetIdentityNotFound
                | WorthQueryApplicationReadExecutionDenialKind::TargetIdentityLookupOverflow => {
                    self.acknowledge_cause_denial(
                        WorthQueryApplicationLiveCauseDenialKind::TargetIdentityUnavailable,
                    )
                }
                WorthQueryApplicationReadExecutionDenialKind::TraversalUnavailable => self
                    .acknowledge_cause_denial(
                        WorthQueryApplicationLiveCauseDenialKind::TargetOutsideScope,
                    ),
                WorthQueryApplicationReadExecutionDenialKind::CardinalityMismatch
                | WorthQueryApplicationReadExecutionDenialKind::ProjectionUnavailable => self
                    .acknowledge_cause_denial(
                        WorthQueryApplicationLiveCauseDenialKind::ResultShapeUnavailable,
                    ),
                _ => WorthQueryApplicationLiveOutcome::Unavailable,
            },
        }
    }

    fn acknowledge_cause_denial(
        &mut self,
        kind: WorthQueryApplicationLiveCauseDenialKind,
    ) -> WorthQueryApplicationLiveOutcome<Query, QueryResult> {
        self.acknowledge_and_terminate(WorthQueryApplicationLiveOutcome::CauseDenied(kind))
    }

    fn acknowledge_front(&mut self) -> bool {
        self.basis
            .as_mut()
            .is_some_and(|basis| self.queue.acknowledge_front(basis).is_ok())
    }

    fn acknowledge_and_terminate(
        &mut self,
        outcome: WorthQueryApplicationLiveOutcome<Query, QueryResult>,
    ) -> WorthQueryApplicationLiveOutcome<Query, QueryResult> {
        if self.acknowledge_front()
            && self.terminate(BridgeExecutionBasisTerminalDisposition::Cancelled)
        {
            outcome
        } else {
            WorthQueryApplicationLiveOutcome::Unavailable
        }
    }

    pub fn close(mut self) -> WorthQueryApplicationLiveCloseOutcome {
        if self.terminate(BridgeExecutionBasisTerminalDisposition::Completed) {
            WorthQueryApplicationLiveCloseOutcome::Completed
        } else {
            WorthQueryApplicationLiveCloseOutcome::Unavailable
        }
    }
}

#[cfg(test)]
#[path = "lease/delivery_tests.rs"]
mod delivery_tests;
