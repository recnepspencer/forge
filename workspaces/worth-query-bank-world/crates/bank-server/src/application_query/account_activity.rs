use std::num::NonZeroUsize;

use bank_domain::model::{AccountId, BankPrincipalId};
use bank_domain::queries::{
    AccountActivityLiveCause, AccountActivityQuery, AccountActivityQueryParameters,
    AccountActivityQueryResult,
};
use bank_domain::schema::{Account, AccountIdentity, BankSchema, Posting, Principal};
use worth_query_host::facade::{
    declaration::application_query::ApplicationQueryParameterSet,
    domain::WorthQueryInstalledApplicationQuery,
    primary_graph::{
        WorthQueryApplicationEntityIdentity, WorthQueryApplicationLiveControls,
        WorthQueryApplicationLiveLease, WorthQueryApplicationQueryAccessContext,
        WorthQueryApplicationQueryControls, WorthQueryApplicationQueryResumeControls,
        WorthQueryPrincipalResolutionMode,
    },
    publication::domain_computation::publish_application_result,
};

mod output;

pub use output::{
    BankAccountActivityContinuation, BankAccountActivityHistoricalResult,
    BankAccountActivityLiveOutcome, BankAccountActivityLiveUpdate, BankAccountActivityPageResult,
    BankAccountActivityQueryResult,
};

use super::{execute_one_shot, BankApplicationQueryDenial, BankApplicationQueryInvocation};
use crate::{
    BankApplicationLiveCloseOutcome, BankAuthenticatedPrincipal, BankCommitReceipt,
    BankIdentityRuntime,
};

type QueryAccountActivityLiveLease<'runtime, 'principal> = WorthQueryApplicationLiveLease<
    'runtime,
    'principal,
    BankSchema,
    AccountActivityQuery,
    AccountActivityQueryParameters,
    AccountActivityQueryResult,
    Principal,
    BankPrincipalId,
    Account,
    Posting,
    AccountActivityLiveCause,
>;

pub struct BankAccountActivityRequest<'runtime> {
    runtime: &'runtime BankIdentityRuntime,
    account: AccountId,
}

pub struct BankAccountActivityRequestForPrincipal<'runtime, 'principal> {
    runtime: &'runtime BankIdentityRuntime,
    principal: &'principal BankAuthenticatedPrincipal,
    account: AccountId,
}

pub struct BankAccountActivityLiveLease<'runtime, 'principal> {
    query: QueryAccountActivityLiveLease<'runtime, 'principal>,
}

impl BankIdentityRuntime {
    pub const fn account_activity(&self, account: AccountId) -> BankAccountActivityRequest<'_> {
        BankAccountActivityRequest {
            runtime: self,
            account,
        }
    }
}

impl<'runtime> BankAccountActivityRequest<'runtime> {
    pub const fn as_principal<'principal>(
        self,
        principal: &'principal BankAuthenticatedPrincipal,
    ) -> BankAccountActivityRequestForPrincipal<'runtime, 'principal> {
        BankAccountActivityRequestForPrincipal {
            runtime: self.runtime,
            principal,
            account: self.account,
        }
    }
}

impl<'runtime, 'principal> BankAccountActivityRequestForPrincipal<'runtime, 'principal> {
    pub fn historical(
        self,
        commit: &BankCommitReceipt,
        maximum_result_count: NonZeroUsize,
        maximum_work: NonZeroUsize,
        request: &worth_query_host::facade::admission::authenticated_principal::WorthQueryRequestScope,
    ) -> Result<BankAccountActivityHistoricalResult, BankApplicationQueryDenial> {
        let prepared = self.prepare(request)?;
        let basis = prepared
            .runtime
            .application_runtime()
            .admit_application_historical_basis(
                commit.recovery_evidence().historical_read(),
                request,
            )
            .map_err(BankApplicationQueryDenial::from_admission)?;
        let access = prepared.access();
        let plan = prepared
            .runtime
            .application_runtime()
            .admit_application_query(
                &prepared.query,
                &access,
                ApplicationQueryParameterSet::<AccountActivityQuery>::new(),
                WorthQueryApplicationQueryControls::historical(
                    basis,
                    maximum_result_count,
                    maximum_work,
                    request,
                ),
            )
            .map_err(BankApplicationQueryDenial::from_admission)?;
        let result = prepared
            .runtime
            .application_runtime()
            .execute_application_query_historical(plan)
            .map_err(BankApplicationQueryDenial::from_historical_execution)?;
        Ok(publish_application_result(result.into_admitted_disclosed()))
    }

    pub fn execute(
        self,
        controls: WorthQueryApplicationQueryControls<'_, BankSchema>,
    ) -> Result<BankAccountActivityQueryResult, BankApplicationQueryDenial> {
        execute_one_shot(
            self.runtime,
            self.principal,
            BankApplicationQueryInvocation::new(
                AccountActivityQuery::reference(),
                AccountIdentity::reference(),
                self.account,
                ApplicationQueryParameterSet::<AccountActivityQuery>::new(),
                controls,
            ),
        )
    }

    pub fn page(
        self,
        controls: WorthQueryApplicationQueryControls<'_, BankSchema>,
    ) -> Result<BankAccountActivityPageResult, BankApplicationQueryDenial> {
        let prepared = self.prepare(controls.request_scope())?;
        let access = prepared.access();
        let plan = prepared
            .runtime
            .application_runtime()
            .admit_application_query(
                &prepared.query,
                &access,
                ApplicationQueryParameterSet::<AccountActivityQuery>::new(),
                controls,
            )
            .map_err(BankApplicationQueryDenial::from_admission)?;
        let page = prepared
            .runtime
            .application_runtime()
            .execute_application_query_continuation_page(plan)
            .map_err(BankApplicationQueryDenial::from_continuation_execution)?;
        Ok(output::publish_page(page))
    }

    pub fn resume(
        self,
        continuation: BankAccountActivityContinuation,
        controls: WorthQueryApplicationQueryResumeControls<'_>,
    ) -> Result<BankAccountActivityPageResult, BankApplicationQueryDenial> {
        let prepared = self.prepare(controls.request_scope())?;
        continuation.resume(prepared, controls)
    }

    pub fn subscribe(
        self,
        controls: WorthQueryApplicationLiveControls,
    ) -> Result<BankAccountActivityLiveLease<'runtime, 'principal>, BankApplicationQueryDenial>
    {
        let prepared = self.prepare(controls.request())?;
        let query = prepared
            .runtime
            .application_runtime()
            .open_application_query_live::<
                AccountActivityQuery,
                AccountActivityQueryParameters,
                AccountActivityQueryResult,
                Principal,
                BankPrincipalId,
                Account,
                Posting,
                AccountActivityLiveCause,
            >(
                prepared.query,
                prepared.principal.query(),
                prepared.scope,
                ApplicationQueryParameterSet::<AccountActivityQuery>::new(),
                controls,
            )
            .map_err(BankApplicationQueryDenial::from_live_open)?;
        Ok(BankAccountActivityLiveLease { query })
    }

    fn prepare(
        self,
        request: &worth_query_host::facade::admission::authenticated_principal::WorthQueryRequestScope,
    ) -> Result<PreparedAccountActivity<'runtime, 'principal>, BankApplicationQueryDenial> {
        let application = self.runtime.application_runtime();
        let query = application
            .installed_schema()
            .application_query(AccountActivityQuery::reference())
            .map_err(BankApplicationQueryDenial::from_installation)?;
        let scope = application
            .resolve_entity(
                AccountIdentity::reference(),
                self.account,
                request,
                WorthQueryPrincipalResolutionMode::Ordinary,
            )
            .map_err(BankApplicationQueryDenial::from_scope_resolution)?;
        Ok(PreparedAccountActivity {
            runtime: self.runtime,
            principal: self.principal,
            query,
            scope,
        })
    }
}

impl BankAccountActivityLiveLease<'_, '_> {
    pub fn buffered_cause_count(&self) -> usize {
        self.query.buffered_cause_count()
    }

    pub fn poll(&mut self) -> BankAccountActivityLiveOutcome {
        output::publish_live_outcome(self.query.poll())
    }

    pub fn close(self) -> BankApplicationLiveCloseOutcome {
        output::publish_close(self.query.close())
    }
}

struct PreparedAccountActivity<'runtime, 'principal> {
    runtime: &'runtime BankIdentityRuntime,
    principal: &'principal BankAuthenticatedPrincipal,
    query: WorthQueryInstalledApplicationQuery<
        BankSchema,
        AccountActivityQuery,
        AccountActivityQueryParameters,
        AccountActivityQueryResult,
        Account,
    >,
    scope: WorthQueryApplicationEntityIdentity<BankSchema, Account>,
}

impl PreparedAccountActivity<'_, '_> {
    fn access(
        &self,
    ) -> WorthQueryApplicationQueryAccessContext<'_, BankSchema, Principal, BankPrincipalId, Account>
    {
        WorthQueryApplicationQueryAccessContext::new(self.principal.query(), &self.scope)
    }
}
