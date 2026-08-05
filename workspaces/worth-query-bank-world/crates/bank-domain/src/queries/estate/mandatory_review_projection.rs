use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationProjection, WorthQueryApplicationProjectionDenial,
    WorthQueryApplicationProjectionRow,
};

use crate::{estate::EstateCaseId, reads::EstateReviewView, schema::BankSchema};

use super::{mandatory_review::EstateMandatoryReviewQuery, mandatory_review_selectors::*};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EstateMandatoryReviewResult {
    estate: EstateCaseId,
    reviews: Vec<EstateReviewView>,
}

impl EstateMandatoryReviewResult {
    pub const fn estate(&self) -> EstateCaseId {
        self.estate
    }

    pub fn reviews(&self) -> &[EstateReviewView] {
        &self.reviews
    }
}

impl WorthQueryApplicationProjection<BankSchema, EstateMandatoryReviewQuery>
    for EstateMandatoryReviewResult
{
    fn project(
        row: &WorthQueryApplicationProjectionRow<'_, BankSchema, EstateMandatoryReviewQuery>,
    ) -> Result<Self, WorthQueryApplicationProjectionDenial> {
        let reviews = row
            .many(estate_reviews())?
            .iter()
            .map(|review| {
                let reviewer = review
                    .optional(review_principal())?
                    .map(|principal| principal.field(review_principal_identity()))
                    .transpose()?;
                Ok(EstateReviewView::from_projection(
                    review.field(review_identity())?,
                    review.field(review_kind())?,
                    review.field(review_status())?,
                    reviewer,
                ))
            })
            .collect::<Result<Vec<_>, WorthQueryApplicationProjectionDenial>>()?;
        Ok(Self {
            estate: row.field(estate_identity())?,
            reviews,
        })
    }
}
