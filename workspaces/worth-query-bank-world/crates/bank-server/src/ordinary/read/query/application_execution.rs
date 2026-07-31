use bank_domain::queries::{
    AccountAuthorizedUsersQuery, AccountAuthorizedUsersQueryResult, AccountAuthorizedUsersRequest,
    AccountDetailQuery, AccountDetailRequest, AccountDiscoveryQuery, AccountDiscoveryRequest,
    EstateCaseOverviewQuery, EstateCaseOverviewRequest, EstateGovernanceQuery,
    EstateGovernanceRequest, InstitutionAuditQuery, InstitutionAuditRequest, PaymentDetailQuery,
    PaymentDetailRequest, PendingPaymentsQuery, PendingPaymentsRequest,
};
use bank_domain::queries::{AccountSummaryQuery, AccountSummaryRequest};
use bank_domain::reads::{
    AccountDetail, AccountSummary, EstateCaseOverview, EstateGovernanceContext,
    InstitutionAuditView, PaymentSummary, VisibleAccount,
};
use bank_domain::schema::{
    AccountIdentity, EstateCaseIdentityField, InstitutionIdentityField, PaymentIdentityField,
    PrincipalIdentityField,
};
use worth_query_host::facade::{
    declaration::application_query::ApplicationQueryParameterSet,
    primary_graph::{WorthQueryApplicationOneShotResult, WorthQueryApplicationPreviewResult},
};

use super::BankReadyQuery;
use crate::application_query::{
    execute_one_shot, execute_preview, BankApplicationQueryDenial, BankApplicationQueryInvocation,
    BankPreviewSession,
};

impl BankReadyQuery<'_, '_, AccountSummaryRequest> {
    pub fn execute(
        self,
    ) -> Result<
        WorthQueryApplicationOneShotResult<AccountSummaryQuery, AccountSummary>,
        BankApplicationQueryDenial,
    > {
        let controls = self.controls.application_query_controls();
        execute_one_shot(
            self.runtime,
            self.principal,
            BankApplicationQueryInvocation::new(
                AccountSummaryQuery::reference(),
                AccountIdentity::reference(),
                self.query.account(),
                ApplicationQueryParameterSet::<AccountSummaryQuery>::new(),
                controls,
            ),
        )
    }
}

impl BankReadyQuery<'_, '_, AccountDiscoveryRequest> {
    pub fn execute(
        self,
    ) -> Result<
        WorthQueryApplicationOneShotResult<AccountDiscoveryQuery, VisibleAccount>,
        BankApplicationQueryDenial,
    > {
        let controls = self.controls.application_query_controls();
        execute_one_shot(
            self.runtime,
            self.principal,
            BankApplicationQueryInvocation::new(
                AccountDiscoveryQuery::reference(),
                PrincipalIdentityField::reference(),
                self.principal.principal_id(),
                ApplicationQueryParameterSet::<AccountDiscoveryQuery>::new(),
                controls,
            ),
        )
    }
}

impl BankReadyQuery<'_, '_, AccountDetailRequest> {
    pub fn execute(
        self,
    ) -> Result<
        WorthQueryApplicationOneShotResult<AccountDetailQuery, AccountDetail>,
        BankApplicationQueryDenial,
    > {
        let controls = self.controls.application_query_controls();
        execute_one_shot(
            self.runtime,
            self.principal,
            BankApplicationQueryInvocation::new(
                AccountDetailQuery::reference(),
                AccountIdentity::reference(),
                self.query.account(),
                ApplicationQueryParameterSet::<AccountDetailQuery>::new(),
                controls,
            ),
        )
    }
}

impl BankReadyQuery<'_, '_, AccountAuthorizedUsersRequest> {
    pub fn execute(
        self,
    ) -> Result<
        WorthQueryApplicationOneShotResult<
            AccountAuthorizedUsersQuery,
            AccountAuthorizedUsersQueryResult,
        >,
        BankApplicationQueryDenial,
    > {
        let controls = self.controls.application_query_controls();
        execute_one_shot(
            self.runtime,
            self.principal,
            BankApplicationQueryInvocation::new(
                AccountAuthorizedUsersQuery::reference(),
                AccountIdentity::reference(),
                self.query.account(),
                ApplicationQueryParameterSet::<AccountAuthorizedUsersQuery>::new(),
                controls,
            ),
        )
    }
}

impl BankReadyQuery<'_, '_, PaymentDetailRequest> {
    pub fn execute(
        self,
    ) -> Result<
        WorthQueryApplicationOneShotResult<PaymentDetailQuery, PaymentSummary>,
        BankApplicationQueryDenial,
    > {
        let controls = self.controls.application_query_controls();
        execute_one_shot(
            self.runtime,
            self.principal,
            BankApplicationQueryInvocation::new(
                PaymentDetailQuery::reference(),
                PaymentIdentityField::reference(),
                self.query.payment(),
                ApplicationQueryParameterSet::<PaymentDetailQuery>::new(),
                controls,
            ),
        )
    }
}

impl BankReadyQuery<'_, '_, PendingPaymentsRequest> {
    pub fn execute(
        self,
    ) -> Result<
        WorthQueryApplicationOneShotResult<PendingPaymentsQuery, PaymentSummary>,
        BankApplicationQueryDenial,
    > {
        let controls = self.controls.application_query_controls();
        execute_one_shot(
            self.runtime,
            self.principal,
            BankApplicationQueryInvocation::new(
                PendingPaymentsQuery::reference(),
                PrincipalIdentityField::reference(),
                self.principal.principal_id(),
                ApplicationQueryParameterSet::<PendingPaymentsQuery>::new(),
                controls,
            ),
        )
    }
}

impl BankReadyQuery<'_, '_, EstateCaseOverviewRequest> {
    pub fn execute(
        self,
    ) -> Result<
        WorthQueryApplicationOneShotResult<EstateCaseOverviewQuery, EstateCaseOverview>,
        BankApplicationQueryDenial,
    > {
        let controls = self.controls.application_query_controls();
        execute_one_shot(
            self.runtime,
            self.principal,
            BankApplicationQueryInvocation::new(
                EstateCaseOverviewQuery::reference(),
                EstateCaseIdentityField::reference(),
                self.query.estate(),
                ApplicationQueryParameterSet::<EstateCaseOverviewQuery>::new(),
                controls,
            ),
        )
    }

    pub fn preview(
        self,
        session: &BankPreviewSession,
    ) -> Result<
        WorthQueryApplicationPreviewResult<EstateCaseOverviewQuery, EstateCaseOverview>,
        BankApplicationQueryDenial,
    > {
        let application = self.runtime.application_runtime();
        let basis = application
            .admit_application_preview_basis(session, self.controls.request())
            .map_err(BankApplicationQueryDenial::Admission)?;
        let controls = self.controls.application_query_preview_controls(basis);
        execute_preview(
            self.runtime,
            self.principal,
            BankApplicationQueryInvocation::new(
                EstateCaseOverviewQuery::reference(),
                EstateCaseIdentityField::reference(),
                self.query.estate(),
                ApplicationQueryParameterSet::<EstateCaseOverviewQuery>::new(),
                controls,
            ),
        )
    }
}

impl BankReadyQuery<'_, '_, EstateGovernanceRequest> {
    pub fn execute(
        self,
    ) -> Result<
        WorthQueryApplicationOneShotResult<EstateGovernanceQuery, EstateGovernanceContext>,
        BankApplicationQueryDenial,
    > {
        let controls = self.controls.application_query_controls();
        execute_one_shot(
            self.runtime,
            self.principal,
            BankApplicationQueryInvocation::new(
                EstateGovernanceQuery::reference(),
                EstateCaseIdentityField::reference(),
                self.query.estate(),
                ApplicationQueryParameterSet::<EstateGovernanceQuery>::new(),
                controls,
            ),
        )
    }
}

impl BankReadyQuery<'_, '_, InstitutionAuditRequest> {
    pub fn execute(
        self,
    ) -> Result<
        WorthQueryApplicationOneShotResult<InstitutionAuditQuery, InstitutionAuditView>,
        BankApplicationQueryDenial,
    > {
        let controls = self.controls.application_query_controls();
        execute_one_shot(
            self.runtime,
            self.principal,
            BankApplicationQueryInvocation::new(
                InstitutionAuditQuery::reference(),
                InstitutionIdentityField::reference(),
                self.query.institution(),
                ApplicationQueryParameterSet::<InstitutionAuditQuery>::new(),
                controls,
            ),
        )
    }
}
