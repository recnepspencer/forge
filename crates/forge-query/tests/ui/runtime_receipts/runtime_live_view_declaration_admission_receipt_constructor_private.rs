use forge_query::facade::{DeclarativeLiveQueryRequest, DeclarativeLiveViewShape};
use forge_query::facade::LiveViewDeclarationAdmissionReceipt;

fn main() {
    let request = DeclarativeLiveQueryRequest::new("Task", DeclarativeLiveViewShape::table());
    let _ = LiveViewDeclarationAdmissionReceipt::from_request("tasks.table", &request);
}
