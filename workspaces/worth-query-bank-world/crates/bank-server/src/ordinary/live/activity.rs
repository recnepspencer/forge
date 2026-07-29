use bank_domain::model::ReadOutcome;
use bank_domain::schema::{
    Account, AccountActivityEffect, ActivityEvent, BankSchema, ReadAccountActivity,
    ReadAccountActivityOperation,
};
use worth_query_host::facade::primary_graph::{
    WorthQueryEntityResolutionDenialKind, WorthQueryLiveDeliveryOutcome, WorthQueryLiveEffectLease,
    WorthQueryOperationAuthorizationDenialKind,
};

use super::{
    BankActivityLiveOutcome, BankActivityLiveUpdate, BankLiveControls, BankLiveOpenDenial,
};
use crate::ordinary::{map_read_admission_denial, queries, BankQueryForPrincipal};
use crate::{BankAuthenticatedPrincipal, BankIdentityRuntime};

type QueryActivityLease<'runtime> = WorthQueryLiveEffectLease<
    'runtime,
    BankSchema,
    ReadAccountActivityOperation,
    ReadAccountActivity,
    Account,
    AccountActivityEffect,
    ActivityEvent,
>;

/// Bank-owned activity lease. Its private Query lease and principal binding
/// cannot be assembled by a transport adapter.
///
/// ```compile_fail
/// use bank_server::BankActivityLiveLease;
///
/// let _ = BankActivityLiveLease {
///     runtime: todo!(),
///     principal: todo!(),
///     account: todo!(),
///     controls: todo!(),
///     query: todo!(),
/// };
/// ```
pub struct BankActivityLiveLease<'runtime, 'principal> {
    runtime: &'runtime BankIdentityRuntime,
    principal: &'principal BankAuthenticatedPrincipal,
    account: bank_domain::model::AccountId,
    controls: BankLiveControls,
    query: QueryActivityLease<'runtime>,
    pending: Option<BankPendingActivityCause>,
}

struct BankPendingActivityCause {
    commit_id: u64,
    event: ActivityEvent,
}

impl<'runtime, 'principal> BankQueryForPrincipal<'runtime, 'principal, queries::AccountActivity> {
    pub fn subscribe(
        self,
        controls: BankLiveControls,
    ) -> Result<BankActivityLiveLease<'runtime, 'principal>, BankLiveOpenDenial> {
        let account = self.query_account();
        let admission = self
            .runtime()
            .authorize_account_activity(self.principal(), account, controls.read().request())
            .map_err(|denial| BankLiveOpenDenial::Admission(map_read_admission_denial(denial)))?;
        let query = self
            .runtime()
            .application_runtime()
            .open_live_effect_lease_matching(
                admission.query(),
                AccountActivityEffect::reference(),
                controls.delivery().clone(),
                move |event| event.account == account,
            )
            .map_err(|denial| BankLiveOpenDenial::Delivery(denial.kind()))?;
        Ok(BankActivityLiveLease {
            runtime: self.runtime(),
            principal: self.principal(),
            account,
            controls,
            query,
            pending: None,
        })
    }
}

impl BankActivityLiveLease<'_, '_> {
    pub fn buffered_update_count(&self) -> usize {
        self.query
            .buffered_cause_count()
            .saturating_add(usize::from(self.pending.is_some()))
    }

    pub fn poll(&mut self) -> BankActivityLiveOutcome {
        if self.pending.is_some() {
            self.project_pending_cause()
        } else {
            self.acquire_and_project_cause()
        }
    }

    fn acquire_and_project_cause(&mut self) -> BankActivityLiveOutcome {
        let admission = match self.runtime.authorize_account_activity(
            self.principal,
            self.account,
            self.controls.read().request(),
        ) {
            Ok(admission) => admission,
            Err(denial) => return admission_failure(map_read_admission_denial(denial)),
        };
        let cause = match self.query.poll(admission.query()) {
            WorthQueryLiveDeliveryOutcome::Delivered(cause) => cause,
            WorthQueryLiveDeliveryOutcome::Pending => return BankActivityLiveOutcome::Pending,
            WorthQueryLiveDeliveryOutcome::Overflow(overflow) => {
                return BankActivityLiveOutcome::Overflow(overflow)
            }
            WorthQueryLiveDeliveryOutcome::AuthorizationDenied(kind) => {
                return BankActivityLiveOutcome::AuthorizationRevoked(
                    crate::BankReadDenial::Authorization(kind),
                )
            }
            WorthQueryLiveDeliveryOutcome::ScopeMismatch => {
                return BankActivityLiveOutcome::Unavailable
            }
            WorthQueryLiveDeliveryOutcome::Cancelled => return BankActivityLiveOutcome::Cancelled,
            WorthQueryLiveDeliveryOutcome::DeadlineExceeded => {
                return BankActivityLiveOutcome::DeadlineExceeded
            }
            WorthQueryLiveDeliveryOutcome::Closed => return BankActivityLiveOutcome::Closed,
            WorthQueryLiveDeliveryOutcome::Unavailable => {
                return BankActivityLiveOutcome::Unavailable
            }
        };
        self.pending = Some(BankPendingActivityCause {
            commit_id: cause.commit_ordinal(),
            event: cause.into_payload(),
        });
        self.project_pending_cause()
    }

    fn project_pending_cause(&mut self) -> BankActivityLiveOutcome {
        let pending = self
            .pending
            .as_ref()
            .expect("poll acquires one pending cause before projection");
        let read = self
            .runtime
            .query(queries::account_activity_cause(
                pending.event.account,
                pending.event.journal,
                pending.event.journal_sequence,
            ))
            .as_principal(self.principal)
            .controls(self.controls.read().clone())
            .execute();
        resolve_pending_projection(&mut self.pending, read)
    }

    pub fn close(self) {
        self.query.close();
    }
}

fn resolve_pending_projection(
    pending: &mut Option<BankPendingActivityCause>,
    read: crate::BankReadOutcome<bank_domain::reads::AccountActivityItem>,
) -> BankActivityLiveOutcome {
    match read {
        ReadOutcome::Delivered(activity) => {
            let committed = pending
                .take()
                .expect("delivered projection consumes its exact cause");
            BankActivityLiveOutcome::Delivered(BankActivityLiveUpdate::new(
                committed.commit_id,
                activity,
            ))
        }
        ReadOutcome::Denied(denial)
            if matches!(
                denial,
                crate::BankReadDenial::Scope(_) | crate::BankReadDenial::Authorization(_)
            ) =>
        {
            *pending = None;
            admission_failure(denial)
        }
        ReadOutcome::Denied(_)
        | ReadOutcome::Absent
        | ReadOutcome::Stale
        | ReadOutcome::Unavailable => BankActivityLiveOutcome::Unavailable,
        ReadOutcome::Cancelled => BankActivityLiveOutcome::Cancelled,
        ReadOutcome::DeadlineExceeded => BankActivityLiveOutcome::DeadlineExceeded,
    }
}

fn admission_failure(denial: crate::BankReadDenial) -> BankActivityLiveOutcome {
    match denial {
        crate::BankReadDenial::Scope(WorthQueryEntityResolutionDenialKind::Cancelled) => {
            BankActivityLiveOutcome::Cancelled
        }
        crate::BankReadDenial::Scope(WorthQueryEntityResolutionDenialKind::DeadlineExceeded) => {
            BankActivityLiveOutcome::DeadlineExceeded
        }
        crate::BankReadDenial::Authorization(
            WorthQueryOperationAuthorizationDenialKind::Cancelled,
        ) => BankActivityLiveOutcome::Cancelled,
        crate::BankReadDenial::Authorization(
            WorthQueryOperationAuthorizationDenialKind::DeadlineExceeded,
        ) => BankActivityLiveOutcome::DeadlineExceeded,
        denial => BankActivityLiveOutcome::AuthorizationRevoked(denial),
    }
}

#[cfg(test)]
mod tests {
    use bank_domain::model::{AccountId, JournalEntryId, ReadOutcome};
    use bank_domain::schema::ActivityEvent;

    use super::{resolve_pending_projection, BankActivityLiveOutcome, BankPendingActivityCause};

    #[test]
    fn unavailable_projection_retains_the_exact_pending_cause() {
        let mut pending = Some(BankPendingActivityCause {
            commit_id: 41,
            event: ActivityEvent {
                account: AccountId::new(7).unwrap(),
                journal: JournalEntryId::new(9).unwrap(),
                journal_sequence: 13,
            },
        });

        let outcome = resolve_pending_projection(&mut pending, ReadOutcome::Unavailable);

        assert!(matches!(outcome, BankActivityLiveOutcome::Unavailable));
        let retained = pending.expect("transient projection failure must retain the cause");
        assert_eq!(retained.commit_id, 41);
        assert_eq!(retained.event.journal_sequence, 13);
    }
}
