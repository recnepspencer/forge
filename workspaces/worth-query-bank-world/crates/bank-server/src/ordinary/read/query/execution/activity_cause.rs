use bank_domain::model::ReadOutcome;
use bank_domain::reads::AccountActivityItem;

use super::{execute_read, queries, BankReadyQuery, ExecutableBankRead};
use crate::bank_projection::project_account_activity_cause_read;
use crate::ordinary::read::BankReadOutcome;
use crate::{BankAuthenticatedPrincipal, BankIdentityRuntime};

impl BankReadyQuery<'_, '_, queries::AccountActivityCause> {
    pub(crate) fn execute(self) -> BankReadOutcome<AccountActivityItem> {
        self.query
            .execute(self.runtime, self.principal, &self.controls)
    }
}

impl ExecutableBankRead for queries::AccountActivityCause {
    type Output = AccountActivityItem;

    fn execute(
        self,
        runtime: &BankIdentityRuntime,
        principal: &BankAuthenticatedPrincipal,
        controls: &crate::BankReadControls,
    ) -> BankReadOutcome<Self::Output> {
        let projected = execute_read(
            runtime,
            controls,
            runtime.authorize_account_activity(principal, self.account, controls.request()),
            |reader, root| {
                project_account_activity_cause_read(
                    reader,
                    root,
                    self.account,
                    self.journal,
                    self.journal_sequence,
                )
            },
        );
        match projected {
            ReadOutcome::Delivered(projected) => {
                let metadata = projected.metadata();
                match projected.into_output() {
                    Some(activity) => {
                        ReadOutcome::Delivered(crate::BankReadResult::new(activity, metadata))
                    }
                    None => ReadOutcome::Absent,
                }
            }
            ReadOutcome::Absent => ReadOutcome::Absent,
            ReadOutcome::Denied(denial) => ReadOutcome::Denied(denial),
            ReadOutcome::Stale => ReadOutcome::Stale,
            ReadOutcome::Cancelled => ReadOutcome::Cancelled,
            ReadOutcome::DeadlineExceeded => ReadOutcome::DeadlineExceeded,
            ReadOutcome::Unavailable => ReadOutcome::Unavailable,
        }
    }
}
