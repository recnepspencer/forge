use worth_query::facade::policy::ExecutionPlanBundle;
use worth_query::facade::read::WorthQueryReadRequest;

fn promote(request: WorthQueryReadRequest) -> ExecutionPlanBundle {
    request.into()
}

fn main() {}
