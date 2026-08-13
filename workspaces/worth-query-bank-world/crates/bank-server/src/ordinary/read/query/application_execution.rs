use bank_domain::queries::{
    AccountAuthorizedUsersQuery, AccountAuthorizedUsersQueryResult, AccountAuthorizedUsersRequest,
    AccountDetailQuery, AccountDetailRequest, AccountDiscoveryQuery, AccountDiscoveryRequest,
    EstateCaseOverviewQuery, EstateCaseOverviewRequest, EstateCustomerDisclosure,
    EstateCustomerDisclosureQuery, EstateCustomerDisclosureRequest, EstateEmergencyAccountDetails,
    EstateEmergencyAccountDetailsQuery, EstateEmergencyAccountDetailsRequest,
    InstitutionAuditQuery, InstitutionAuditRequest, PaymentDetailQuery, PaymentDetailRequest,
    PendingPaymentsQuery, PendingPaymentsRequest,
};
use bank_domain::queries::{AccountSummaryQuery, AccountSummaryRequest};
use bank_domain::reads::{
    AccountDetail, AccountSummary, EstateCaseOverview, InstitutionAuditView, PaymentSummary,
    VisibleAccount,
};
use bank_domain::schema::{
    AccountIdentity, EstateCaseIdentityField, InstitutionIdentityField, PaymentIdentityField,
    PrincipalIdentityField,
};
use worth_query_host::facade::{
    declaration::application_query::ApplicationQueryParameterSet,
    publication::domain_computation::WorthQueryPublishedApplicationResult,
};

use super::BankReadyQuery;
use crate::application_query::{
    execute_estate_customer_disclosure, execute_estate_emergency_account_details, execute_one_shot,
    execute_preview, BankAdmittedEstateEmergencyAccountDetailsHistorical,
    BankAdmittedEstateEmergencyAccountDetailsPreview, BankApplicationQueryDenial,
    BankApplicationQueryInvocation, BankEstateEmergencyAccountDetailsAdmission, BankPreviewSession,
};
use crate::BankApprovedEstateElevation;

impl BankReadyQuery<'_, '_, AccountSummaryRequest> {
    pub fn execute(
        self,
    ) -> Result<
        WorthQueryPublishedApplicationResult<AccountSummaryQuery, AccountSummary>,
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
        WorthQueryPublishedApplicationResult<AccountDiscoveryQuery, VisibleAccount>,
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
        WorthQueryPublishedApplicationResult<AccountDetailQuery, AccountDetail>,
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
        WorthQueryPublishedApplicationResult<
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
        WorthQueryPublishedApplicationResult<PaymentDetailQuery, PaymentSummary>,
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
        WorthQueryPublishedApplicationResult<PendingPaymentsQuery, PaymentSummary>,
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
        WorthQueryPublishedApplicationResult<EstateCaseOverviewQuery, EstateCaseOverview>,
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
        WorthQueryPublishedApplicationResult<EstateCaseOverviewQuery, EstateCaseOverview>,
        BankApplicationQueryDenial,
    > {
        let application = self.runtime.application_runtime();
        let controls = session.admit_controls(
            application,
            self.controls.maximum_result_count(),
            self.controls.maximum_work(),
            self.controls.request(),
        )?;
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

impl BankReadyQuery<'_, '_, EstateCustomerDisclosureRequest> {
    pub fn execute(
        self,
    ) -> Result<
        WorthQueryPublishedApplicationResult<
            EstateCustomerDisclosureQuery,
            EstateCustomerDisclosure,
        >,
        BankApplicationQueryDenial,
    > {
        execute_estate_customer_disclosure(
            self.runtime,
            self.principal,
            self.query,
            self.controls.application_query_controls(),
        )
    }
}

impl BankReadyQuery<'_, '_, EstateEmergencyAccountDetailsRequest> {
    pub fn execute_with_approved_elevation(
        self,
        approved: &BankApprovedEstateElevation,
    ) -> Result<
        WorthQueryPublishedApplicationResult<
            EstateEmergencyAccountDetailsQuery,
            EstateEmergencyAccountDetails,
        >,
        BankApplicationQueryDenial,
    > {
        execute_estate_emergency_account_details(
            self.runtime,
            self.principal,
            self.query,
            approved,
            &self.controls,
        )
    }

    pub fn admit_historical_with_approved_elevation<Output>(
        self,
        approved: &BankApprovedEstateElevation,
        after_admission: impl for<'admitted> FnOnce(
            BankAdmittedEstateEmergencyAccountDetailsHistorical<'admitted>,
        )
            -> Result<Output, BankApplicationQueryDenial>,
    ) -> Result<Output, BankApplicationQueryDenial> {
        BankEstateEmergencyAccountDetailsAdmission::new(
            self.runtime,
            self.principal,
            self.query,
            approved,
            &self.controls,
        )
        .historical(after_admission)
    }

    pub fn admit_preview_with_approved_elevation<Output>(
        self,
        approved: &BankApprovedEstateElevation,
        session: &BankPreviewSession,
        after_admission: impl for<'admitted> FnOnce(
            BankAdmittedEstateEmergencyAccountDetailsPreview<'admitted>,
        )
            -> Result<Output, BankApplicationQueryDenial>,
    ) -> Result<Output, BankApplicationQueryDenial> {
        BankEstateEmergencyAccountDetailsAdmission::new(
            self.runtime,
            self.principal,
            self.query,
            approved,
            &self.controls,
        )
        .preview(session, after_admission)
    }
}

impl BankReadyQuery<'_, '_, InstitutionAuditRequest> {
    pub fn execute(
        self,
    ) -> Result<
        WorthQueryPublishedApplicationResult<InstitutionAuditQuery, InstitutionAuditView>,
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
