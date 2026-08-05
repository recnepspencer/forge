use bank_domain::queries::{
    EstateGovernanceQuery, EstateGovernanceRequest, EstateLegalComplianceQuery,
    EstateLegalComplianceRequest, EstateLegalComplianceResult, EstateMandatoryReviewQuery,
    EstateMandatoryReviewRequest, EstateMandatoryReviewResult,
};
use bank_domain::reads::EstateGovernanceContext;
use worth_query_host::facade::primary_graph::WorthQueryApplicationOneShotResult;

use super::BankReadyQuery;
use crate::application_query::{
    execute_estate_governance, execute_estate_legal_compliance, execute_estate_mandatory_review,
    BankApplicationQueryDenial,
};

impl BankReadyQuery<'_, '_, EstateGovernanceRequest> {
    pub fn execute(
        self,
    ) -> Result<
        WorthQueryApplicationOneShotResult<EstateGovernanceQuery, EstateGovernanceContext>,
        BankApplicationQueryDenial,
    > {
        execute_estate_governance(
            self.runtime,
            self.principal,
            self.query,
            self.controls.application_query_controls(),
        )
    }
}

impl BankReadyQuery<'_, '_, EstateLegalComplianceRequest> {
    pub fn execute(
        self,
    ) -> Result<
        WorthQueryApplicationOneShotResult<EstateLegalComplianceQuery, EstateLegalComplianceResult>,
        BankApplicationQueryDenial,
    > {
        execute_estate_legal_compliance(
            self.runtime,
            self.principal,
            self.query,
            self.controls.application_query_controls(),
        )
    }
}

impl BankReadyQuery<'_, '_, EstateMandatoryReviewRequest> {
    pub fn execute(
        self,
    ) -> Result<
        WorthQueryApplicationOneShotResult<EstateMandatoryReviewQuery, EstateMandatoryReviewResult>,
        BankApplicationQueryDenial,
    > {
        execute_estate_mandatory_review(
            self.runtime,
            self.principal,
            self.query,
            self.controls.application_query_controls(),
        )
    }
}
