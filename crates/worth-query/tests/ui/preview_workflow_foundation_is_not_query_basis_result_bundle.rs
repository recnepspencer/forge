use worth_query::facade::policy::{AdmittedPreviewWorkflowFoundation, QueryBasisResultBundle};

fn takes_bundle(_: QueryBasisResultBundle) {}

fn main() {
    let foundation: AdmittedPreviewWorkflowFoundation = unsafe { std::mem::zeroed() };
    takes_bundle(foundation);
}
