use worth_query_execution::facade::primary_graph::WorthQueryAdmittedDisclosedApplicationResult;

mod inspection;
mod receipt;

pub use inspection::WorthQueryApplicationQueryPublicationInspection;
pub use receipt::WorthQueryApplicationQueryPublicationReceipt;

/// Publication-owned result whose input was already governed before domain
/// projection. Publication performs no field-policy decision or redaction.
pub struct WorthQueryPublishedApplicationResult<Query, QueryResult> {
    admitted: WorthQueryAdmittedDisclosedApplicationResult<Query, QueryResult>,
    receipt: WorthQueryApplicationQueryPublicationReceipt,
}

/// Accepts only Query's admitted disclosed shape.
///
/// Raw application values cannot enter publication:
///
/// ```compile_fail
/// use worth_query_publication::facade::domain_computation::publish_application_result;
///
/// let raw = vec!["protected".to_string()];
/// let _ = publish_application_result::<(), _>(raw);
/// ```
///
/// A descriptive Foundational mask is not a publication result:
///
/// ```compile_fail
/// use worth_foundational::facade::{AspectMask, ProjectionMask};
/// use worth_query_publication::facade::domain_computation::publish_application_result;
///
/// let mask = AspectMask::<ProjectionMask>::whole_aspect();
/// let _ = publish_application_result::<(), ()>(mask);
/// ```
pub fn publish_application_result<Query, QueryResult>(
    admitted: WorthQueryAdmittedDisclosedApplicationResult<Query, QueryResult>,
) -> WorthQueryPublishedApplicationResult<Query, QueryResult> {
    let receipt = WorthQueryApplicationQueryPublicationReceipt::from_terminal(admitted.receipt());
    WorthQueryPublishedApplicationResult { admitted, receipt }
}

impl<Query, QueryResult> WorthQueryPublishedApplicationResult<Query, QueryResult> {
    pub fn rows(&self) -> &[QueryResult] {
        self.admitted.rows()
    }

    pub const fn receipt(&self) -> &WorthQueryApplicationQueryPublicationReceipt {
        &self.receipt
    }
}
