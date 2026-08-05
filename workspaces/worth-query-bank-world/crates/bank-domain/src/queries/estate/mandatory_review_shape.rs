use worth_query_decl::facade::application_query::{
    ApplicationQueryResultShapeBuilder, TypedApplicationQueryResultShape,
};

use crate::schema::{BankSchema, EstateCase, MandatoryReview, Principal};

use super::{
    mandatory_review::EstateMandatoryReviewQuery,
    mandatory_review_projection::EstateMandatoryReviewResult, mandatory_review_selectors::*,
};

pub(super) fn mandatory_review_shape() -> TypedApplicationQueryResultShape<
    BankSchema,
    EstateMandatoryReviewQuery,
    EstateCase,
    EstateMandatoryReviewResult,
> {
    let reviewer = ApplicationQueryResultShapeBuilder::<
        BankSchema,
        EstateMandatoryReviewQuery,
        Principal,
        (),
    >::new(Principal::reference())
    .field(review_principal_identity());
    let review = ApplicationQueryResultShapeBuilder::<
        BankSchema,
        EstateMandatoryReviewQuery,
        MandatoryReview,
        (),
    >::new(MandatoryReview::reference())
    .field(review_identity())
    .field(review_kind())
    .field(review_status())
    .relation(review_principal(), reviewer);
    ApplicationQueryResultShapeBuilder::new(EstateCase::reference())
        .field(estate_identity())
        .relation(estate_reviews(), review)
        .build()
}
