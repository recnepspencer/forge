use forge_query::facade::{execute_query_basis_context, HistoricalEvaluationAdmission};

fn main() {
    let admission: HistoricalEvaluationAdmission = todo!();
    let _ = execute_query_basis_context(&admission);
}
