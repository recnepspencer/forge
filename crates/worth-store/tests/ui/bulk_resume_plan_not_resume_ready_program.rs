use worth_store::{BulkResumeCompatibilityPlan, ResumeReadyBulkProgram};

fn main() {
    require_resume_ready(bulk_plan());
}

fn require_resume_ready(_: ResumeReadyBulkProgram) {}

fn bulk_plan() -> BulkResumeCompatibilityPlan {
    panic!("compile-fail fixture")
}
