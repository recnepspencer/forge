use worth_query::facade::foundation::HistoricalEvaluationAdmission;
use worth_query::facade::policy::execute_query_basis_context;

fn main() {
    let admission: HistoricalEvaluationAdmission = todo!();
    let _ = execute_query_basis_context(&admission);
}
