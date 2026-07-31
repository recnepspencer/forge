use std::marker::PhantomData;

use worth_query_admission::facade::application_query::WorthQueryApplicationQueryLane;
use worth_query_declaration::facade::application_schema::ApplicationSchema;

use super::{
    bounded_lane::{execute_bounded_lane, WorthQueryBoundedLaneDenial},
    WorthQueryAdmittedApplicationQueryPlan, WorthQueryApplicationProjection,
    WorthQueryApplicationQueryAccessReceipt,
};
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;

pub struct WorthQueryApplicationHistoricalResult<Query, QueryResult> {
    rows: Vec<QueryResult>,
    receipt: WorthQueryApplicationQueryAccessReceipt,
    _query: PhantomData<fn() -> Query>,
}

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    pub fn execute_application_query_historical<
        Query,
        Parameters,
        QueryResult,
        Principal,
        PrincipalIdentity,
        Scope,
    >(
        &self,
        plan: WorthQueryAdmittedApplicationQueryPlan<
            '_,
            Schema,
            Query,
            Parameters,
            QueryResult,
            Principal,
            PrincipalIdentity,
            Scope,
        >,
    ) -> Result<
        WorthQueryApplicationHistoricalResult<Query, QueryResult>,
        WorthQueryBoundedLaneDenial,
    >
    where
        QueryResult: WorthQueryApplicationProjection<Schema, Query>,
    {
        let result = execute_bounded_lane(self, plan, WorthQueryApplicationQueryLane::Historical)?;
        let (rows, receipt) = result.into_parts();
        Ok(WorthQueryApplicationHistoricalResult {
            rows,
            receipt,
            _query: PhantomData,
        })
    }
}

impl<Query, QueryResult> WorthQueryApplicationHistoricalResult<Query, QueryResult> {
    pub fn rows(&self) -> &[QueryResult] {
        &self.rows
    }

    pub const fn receipt(&self) -> &WorthQueryApplicationQueryAccessReceipt {
        &self.receipt
    }

    pub fn into_rows(self) -> Vec<QueryResult> {
        self.rows
    }

    pub fn into_admitted_disclosed(
        self,
    ) -> super::WorthQueryAdmittedDisclosedApplicationResult<Query, QueryResult> {
        super::WorthQueryAdmittedDisclosedApplicationResult::new(self.rows, self.receipt)
    }
}
