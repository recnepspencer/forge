use forge_query::facade::{AdmittedPreviewWorkflowFoundation, QueryBasisResultBundle};

fn takes_bundle(_: QueryBasisResultBundle) {}

fn main() {
    let foundation: AdmittedPreviewWorkflowFoundation = unsafe { std::mem::zeroed() };
    takes_bundle(foundation);
}
