use worth_query::facade::foundation::{DeclarativeLiveQueryRequest, DeclarativeLiveViewShape};
use worth_query::facade::runtime::LiveViewDeclarationAdmissionReceipt;

fn main() {
    let request = DeclarativeLiveQueryRequest::new("Task", DeclarativeLiveViewShape::table());
    let _ = LiveViewDeclarationAdmissionReceipt::from_request("tasks.table", &request);
}
