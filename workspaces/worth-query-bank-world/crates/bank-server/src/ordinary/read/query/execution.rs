mod activity_cause;

use bank_domain::model::ReadOutcome;
use bank_domain::reads::{
    AccountActivityItem, AccountDetail, AccountSummary, AuthorizedAccountUser, PaymentSummary,
    VisibleAccount,
};
use bank_domain::schema::*;
use worth_query_host::facade::admission::authenticated_principal::WorthQueryRequestInterruption;
use worth_query_host::facade::domain::ApplicationSchema;
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationOperationInvariantProjectionReader, WorthQueryInvariantEntityIdentity,
    WorthQueryOperationProjectionDenialKind, WorthQueryOrdinaryReadBatch,
};

use super::{queries, BankReadyQuery};
use crate::bank_projection::{
    project_account_activity_page_read, project_account_activity_read, project_account_detail_read,
    project_account_discovery_read, project_account_summary_read, project_account_users_read,
    project_institution_audit_read, project_payment_read, project_pending_payments_read,
};
use crate::ordinary::read::{
    BankActivityCursor, BankActivityCursorDenial, BankActivityPage, BankProjectedActivityPage,
    BankReadControls, BankReadDenial, BankReadOutcome, BankReadProjectedBatch, BankReadResult,
};
use crate::{
    BankAdmittedOperation, BankAuthenticatedPrincipal, BankIdentityRuntime,
    BankOperationAdmissionError, BankProjectionDenial,
};

trait ExecutableBankRead {
    type Output;

    fn execute(
        self,
        runtime: &BankIdentityRuntime,
        principal: &BankAuthenticatedPrincipal,
        controls: &BankReadControls,
    ) -> BankReadOutcome<Self::Output>;
}

macro_rules! expose_execute {
    ($query:ty, $output:ty) => {
        impl BankReadyQuery<'_, '_, $query> {
            pub fn execute(self) -> BankReadOutcome<$output> {
                self.query
                    .execute(self.runtime, self.principal, &self.controls)
            }
        }
    };
}

expose_execute!(queries::AccountDiscovery, Vec<VisibleAccount>);
expose_execute!(queries::AccountSummary, AccountSummary);
expose_execute!(queries::AccountDetail, AccountDetail);
expose_execute!(queries::AccountAuthorizedUsers, Vec<AuthorizedAccountUser>);
expose_execute!(queries::AccountActivity, Vec<AccountActivityItem>);
expose_execute!(queries::AccountActivityPage, BankActivityPage);
expose_execute!(queries::PendingPayments, Vec<PaymentSummary>);
expose_execute!(queries::Payment, PaymentSummary);
expose_execute!(queries::InstitutionAudit, Vec<AccountActivityItem>);

impl ExecutableBankRead for queries::AccountDiscovery {
    type Output = Vec<VisibleAccount>;

    fn execute(
        self,
        runtime: &BankIdentityRuntime,
        principal: &BankAuthenticatedPrincipal,
        controls: &BankReadControls,
    ) -> BankReadOutcome<Self::Output> {
        execute_read(
            runtime,
            controls,
            runtime.authorize_account_discovery(principal, controls.request()),
            |reader, root| {
                project_account_discovery_read(
                    reader,
                    root,
                    principal.principal_id(),
                    controls.maximum_results(),
                )
            },
        )
    }
}

macro_rules! account_query {
    ($query:ty, $output:ty, $authorize:ident, $project:ident $(, $limit:ident)?) => {
        impl ExecutableBankRead for $query {
            type Output = $output;

            fn execute(
                self,
                runtime: &BankIdentityRuntime,
                principal: &BankAuthenticatedPrincipal,
                controls: &BankReadControls,
            ) -> BankReadOutcome<Self::Output> {
                execute_read(
                    runtime,
                    controls,
                    runtime.$authorize(principal, self.account, controls.request()),
                    |reader, root| {
                        $project(
                            reader,
                            root,
                            self.account
                            $(, {
                                let _ = stringify!($limit);
                                controls.maximum_results()
                            })?
                        )
                    },
                )
            }
        }
    };
}

account_query!(
    queries::AccountSummary,
    AccountSummary,
    authorize_account_summary,
    project_account_summary_read
);

impl ExecutableBankRead for queries::AccountActivityPage {
    type Output = BankActivityPage;

    fn execute(
        self,
        runtime: &BankIdentityRuntime,
        principal: &BankAuthenticatedPrincipal,
        controls: &BankReadControls,
    ) -> BankReadOutcome<Self::Output> {
        if self
            .cursor
            .is_some_and(|cursor| cursor.account() != self.account)
        {
            return ReadOutcome::Denied(BankReadDenial::ActivityCursor(
                BankActivityCursorDenial::ForeignAccount,
            ));
        }
        let offset = self.cursor.map_or(0, BankActivityCursor::offset);
        let projected: BankReadOutcome<BankProjectedActivityPage> = execute_read(
            runtime,
            controls,
            runtime.authorize_account_activity(principal, self.account, controls.request()),
            |reader, root| {
                project_account_activity_page_read(
                    reader,
                    root,
                    self.account,
                    offset,
                    controls.maximum_results(),
                )
            },
        );
        let delivered = match projected {
            ReadOutcome::Delivered(delivered) => delivered,
            ReadOutcome::Absent => return ReadOutcome::Absent,
            ReadOutcome::Denied(denial) => return ReadOutcome::Denied(denial),
            ReadOutcome::Stale => return ReadOutcome::Stale,
            ReadOutcome::Cancelled => return ReadOutcome::Cancelled,
            ReadOutcome::DeadlineExceeded => return ReadOutcome::DeadlineExceeded,
            ReadOutcome::Unavailable => return ReadOutcome::Unavailable,
        };
        let version = delivered.metadata().version();
        if let Some(cursor) = self.cursor {
            if cursor.version() != version {
                return ReadOutcome::Denied(BankReadDenial::ActivityCursor(
                    BankActivityCursorDenial::StaleVersion {
                        expected: cursor.version(),
                        actual: version,
                    },
                ));
            }
        }
        ReadOutcome::Delivered(delivered.map_output(|page, metadata| {
            let next = page
                .next_offset
                .map(|offset| BankActivityCursor::new(self.account, metadata.version(), offset));
            BankActivityPage::new(page.entries, next)
        }))
    }
}
account_query!(
    queries::AccountDetail,
    AccountDetail,
    authorize_account_detail,
    project_account_detail_read
);
account_query!(
    queries::AccountAuthorizedUsers,
    Vec<AuthorizedAccountUser>,
    authorize_account_users,
    project_account_users_read,
    limit
);
account_query!(
    queries::AccountActivity,
    Vec<AccountActivityItem>,
    authorize_account_activity,
    project_account_activity_read,
    limit
);

impl ExecutableBankRead for queries::PendingPayments {
    type Output = Vec<PaymentSummary>;

    fn execute(
        self,
        runtime: &BankIdentityRuntime,
        principal: &BankAuthenticatedPrincipal,
        controls: &BankReadControls,
    ) -> BankReadOutcome<Self::Output> {
        execute_read(
            runtime,
            controls,
            runtime.authorize_pending_payments(principal, controls.request()),
            |reader, root| {
                project_pending_payments_read(
                    reader,
                    root,
                    principal.principal_id(),
                    controls.maximum_results(),
                )
            },
        )
    }
}

impl ExecutableBankRead for queries::Payment {
    type Output = PaymentSummary;

    fn execute(
        self,
        runtime: &BankIdentityRuntime,
        principal: &BankAuthenticatedPrincipal,
        controls: &BankReadControls,
    ) -> BankReadOutcome<Self::Output> {
        execute_read(
            runtime,
            controls,
            runtime.authorize_payment(principal, self.payment, controls.request()),
            |reader, root| project_payment_read(reader, root, self.payment),
        )
    }
}

impl ExecutableBankRead for queries::InstitutionAudit {
    type Output = Vec<AccountActivityItem>;

    fn execute(
        self,
        runtime: &BankIdentityRuntime,
        principal: &BankAuthenticatedPrincipal,
        controls: &BankReadControls,
    ) -> BankReadOutcome<Self::Output> {
        execute_read(
            runtime,
            controls,
            runtime.authorize_institution_audit(principal, self.institution, controls.request()),
            |reader, root| {
                project_institution_audit_read(
                    reader,
                    root,
                    self.institution,
                    controls.maximum_results(),
                )
            },
        )
    }
}

fn execute_read<Operation, Input, Scope, ScopeIdentity, Output>(
    runtime: &BankIdentityRuntime,
    controls: &BankReadControls,
    admission: Result<
        BankAdmittedOperation<Operation, Input, Scope, ScopeIdentity>,
        BankOperationAdmissionError,
    >,
    projection: impl FnOnce(
        &mut WorthQueryApplicationOperationInvariantProjectionReader<'_, '_, BankSchema, Operation>,
        &WorthQueryInvariantEntityIdentity<BankSchema, Scope>,
    ) -> Result<BankReadProjectedBatch<Output>, BankProjectionDenial>,
) -> BankReadOutcome<Output>
where
    BankSchema: ApplicationSchema,
    ScopeIdentity: Copy,
{
    if let Some(interruption) = controls.request().interruption() {
        return match interruption {
            WorthQueryRequestInterruption::Cancelled => ReadOutcome::Cancelled,
            WorthQueryRequestInterruption::DeadlineExceeded => ReadOutcome::DeadlineExceeded,
        };
    }
    let admission = match admission {
        Ok(admission) => admission,
        Err(denial) => return ReadOutcome::Denied(map_admission_denial(denial)),
    };
    let projected = runtime.invariant_projection().read_admitted_operation(
        admission.query(),
        |reader, root| match projection(reader, root) {
            Ok(batch) => {
                let (output, count, truncated) = batch.into_parts();
                if truncated {
                    WorthQueryOrdinaryReadBatch::truncated(Ok(output), count)
                } else {
                    WorthQueryOrdinaryReadBatch::complete(Ok(output), count)
                }
            }
            Err(denial) => WorthQueryOrdinaryReadBatch::complete(Err(denial), 0),
        },
    );
    let projected = match projected {
        Ok(projected) => projected,
        Err(denial) => {
            return ReadOutcome::Denied(match denial.kind() {
                WorthQueryOperationProjectionDenialKind::Authorization(kind) => {
                    BankReadDenial::Authorization(kind)
                }
                WorthQueryOperationProjectionDenialKind::WorkBudgetExceeded => {
                    BankReadDenial::ProjectionWorkBudgetExceeded
                }
            })
        }
    };
    let (output, metadata) = projected.into_parts();
    match output {
        Ok(output) => ReadOutcome::Delivered(BankReadResult::new(output, metadata.into())),
        Err(denial) => ReadOutcome::Denied(BankReadDenial::Projection(denial)),
    }
}

pub(crate) fn map_admission_denial(denial: BankOperationAdmissionError) -> BankReadDenial {
    match denial {
        BankOperationAdmissionError::ScopeResolution(denial) => {
            BankReadDenial::Scope(denial.kind())
        }
        BankOperationAdmissionError::OperationInstallation(denial) => {
            BankReadDenial::Installation(denial.kind())
        }
        BankOperationAdmissionError::Authorization(denial) => {
            BankReadDenial::Authorization(denial.kind())
        }
    }
}
