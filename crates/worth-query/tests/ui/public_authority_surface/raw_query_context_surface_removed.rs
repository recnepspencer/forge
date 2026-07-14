use worth_query::facade::{bind_query_basis_context, QueryBasisContextBinding, QueryBasisContextRequest};

fn main() {
    let _ = QueryBasisContextRequest::current_branch_head();
    let _bind = bind_query_basis_context;
    let _binding: Option<QueryBasisContextBinding> = None;
}
