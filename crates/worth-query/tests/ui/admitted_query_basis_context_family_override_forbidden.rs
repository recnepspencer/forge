use worth_query::facade::{AdmittedQueryBasisContext, QueryBasisContextRequest};

fn main() {
    let mut context: AdmittedQueryBasisContext = todo!();
    context.binding.request = QueryBasisContextRequest::current_branch_head();
}
