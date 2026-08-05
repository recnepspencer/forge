use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationProjection, WorthQueryApplicationProjectionDenial,
    WorthQueryApplicationProjectionRow,
};

use crate::reads::{
    EstateAccountView, EstateAssignmentView, EstateCaseOverview, EstateCaseOverviewProjection,
    EstateDeathNoticeView, EstateLegalAuthorityView, EstateReviewView,
};
use crate::schema::BankSchema;

use super::overview::EstateCaseOverviewQuery;
use super::overview_fields::*;
use super::overview_relations::*;

impl WorthQueryApplicationProjection<BankSchema, EstateCaseOverviewQuery> for EstateCaseOverview {
    fn project(
        row: &WorthQueryApplicationProjectionRow<'_, BankSchema, EstateCaseOverviewQuery>,
    ) -> Result<Self, WorthQueryApplicationProjectionDenial> {
        Ok(Self::from_projection(EstateCaseOverviewProjection {
            id: row.field(estate_identity())?,
            stage: row.field(estate_stage())?,
            status: row.field(estate_status())?,
            branch: row.one(estate_branch())?.field(branch_identity())?,
            account: project_account(&row.one(estate_account())?)?,
            death_notice: project_notice(&row.one(estate_notice())?)?,
            deceased: row.one(estate_deceased())?.field(deceased_identity())?,
            executors: row
                .many(estate_executors())?
                .iter()
                .map(|actor| actor.field(executor_identity()))
                .collect::<Result<Vec<_>, _>>()?,
            beneficiaries: row
                .many(estate_beneficiaries())?
                .iter()
                .map(|actor| actor.field(beneficiary_identity()))
                .collect::<Result<Vec<_>, _>>()?,
            assignments: row
                .many(estate_assignments())?
                .iter()
                .map(|assignment| project_assignment(&assignment))
                .collect::<Result<Vec<_>, _>>()?,
            legal_authorities: row
                .many(estate_authorities())?
                .iter()
                .map(|authority| project_authority(&authority))
                .collect::<Result<Vec<_>, _>>()?,
            reviews: row
                .many(estate_reviews())?
                .iter()
                .map(|review| project_review(&review))
                .collect::<Result<Vec<_>, _>>()?,
        }))
    }
}

fn project_account(
    row: &WorthQueryApplicationProjectionRow<'_, BankSchema, EstateCaseOverviewQuery>,
) -> Result<EstateAccountView, WorthQueryApplicationProjectionDenial> {
    Ok(EstateAccountView::from_projection(
        row.field(account_identity())?,
        row.field(account_name())?,
        row.field(account_status())?,
    ))
}

fn project_notice(
    row: &WorthQueryApplicationProjectionRow<'_, BankSchema, EstateCaseOverviewQuery>,
) -> Result<EstateDeathNoticeView, WorthQueryApplicationProjectionDenial> {
    Ok(EstateDeathNoticeView::from_projection(
        row.field(notice_identity())?,
        row.field(notice_status())?,
    ))
}

fn project_assignment(
    row: &WorthQueryApplicationProjectionRow<'_, BankSchema, EstateCaseOverviewQuery>,
) -> Result<EstateAssignmentView, WorthQueryApplicationProjectionDenial> {
    Ok(EstateAssignmentView::from_projection(
        row.field(assignment_identity())?,
        row.one(assignment_principal())?
            .field(assignment_principal_identity())?,
        row.field(assignment_role())?,
    ))
}

fn project_authority(
    row: &WorthQueryApplicationProjectionRow<'_, BankSchema, EstateCaseOverviewQuery>,
) -> Result<EstateLegalAuthorityView, WorthQueryApplicationProjectionDenial> {
    Ok(EstateLegalAuthorityView::from_projection(
        row.field(authority_identity())?,
        row.one(authority_holder())?
            .field(authority_holder_identity())?,
        row.field(authority_kind())?,
        row.field(authority_recognized())?,
    ))
}

fn project_review(
    row: &WorthQueryApplicationProjectionRow<'_, BankSchema, EstateCaseOverviewQuery>,
) -> Result<EstateReviewView, WorthQueryApplicationProjectionDenial> {
    let reviewer = row
        .optional(review_principal())?
        .map(|principal| principal.field(review_principal_identity()))
        .transpose()?;
    Ok(EstateReviewView::from_projection(
        row.field(review_identity())?,
        row.field(review_kind())?,
        row.field(review_status())?,
        reviewer,
    ))
}
