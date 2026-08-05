use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationProjection, WorthQueryApplicationProjectionDenial,
    WorthQueryApplicationProjectionRow,
};

use crate::schema::BankSchema;

use super::{
    selectors::{
        access_expires_at, access_id, access_issued_at, access_reason, access_review,
        access_status, estate_accesses, estate_id, review_id, review_status,
    },
    EstateEmergencyAccessActivity, EstateEmergencyAccessActivityItem,
    EstateEmergencyAccessActivityQuery,
};

type Row<'row> =
    WorthQueryApplicationProjectionRow<'row, BankSchema, EstateEmergencyAccessActivityQuery>;

impl WorthQueryApplicationProjection<BankSchema, EstateEmergencyAccessActivityQuery>
    for EstateEmergencyAccessActivity
{
    fn project(row: &Row<'_>) -> Result<Self, WorthQueryApplicationProjectionDenial> {
        let accesses = row
            .many(estate_accesses())?
            .iter()
            .map(|access| project_access(&access))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self::from_projection(row.field(estate_id())?, accesses))
    }
}

fn project_access(
    access: &Row<'_>,
) -> Result<EstateEmergencyAccessActivityItem, WorthQueryApplicationProjectionDenial> {
    let review = access.one(access_review())?;
    Ok(EstateEmergencyAccessActivityItem {
        access: access.field(access_id())?,
        reason: access.field(access_reason())?,
        status: access.field(access_status())?,
        issued_at: access.field(access_issued_at())?,
        expires_at: access.field(access_expires_at())?,
        review: review.field(review_id())?,
        review_status: review.field(review_status())?,
    })
}
