use std::marker::PhantomData;

use super::WorthQueryApplicationQueryAccessReceipt;

/// Query-admitted consumer shape. Construction is private to completed query
/// lanes, after governed projection has replaced every protected slot with a
/// typed disclosed-or-omitted value.
///
/// A consumer holding a terminal receipt still cannot construct an admitted
/// result and bypass governed projection:
///
/// ```compile_fail
/// use worth_query_execution::facade::primary_graph::{
///     WorthQueryAdmittedDisclosedApplicationResult,
///     WorthQueryApplicationQueryAccessReceipt,
/// };
///
/// fn counterfeit(
///     receipt: WorthQueryApplicationQueryAccessReceipt,
/// ) -> WorthQueryAdmittedDisclosedApplicationResult<(), ()> {
///     WorthQueryAdmittedDisclosedApplicationResult::new(vec![()], receipt)
/// }
/// ```
pub struct WorthQueryAdmittedDisclosedApplicationResult<Query, QueryResult> {
    rows: Vec<QueryResult>,
    receipt: WorthQueryApplicationQueryAccessReceipt,
    _query: PhantomData<fn() -> Query>,
}

impl<Query, QueryResult> WorthQueryAdmittedDisclosedApplicationResult<Query, QueryResult> {
    pub(super) fn new(
        rows: Vec<QueryResult>,
        receipt: WorthQueryApplicationQueryAccessReceipt,
    ) -> Self {
        Self {
            rows,
            receipt,
            _query: PhantomData,
        }
    }

    pub fn rows(&self) -> &[QueryResult] {
        &self.rows
    }

    pub const fn receipt(&self) -> &WorthQueryApplicationQueryAccessReceipt {
        &self.receipt
    }
}
