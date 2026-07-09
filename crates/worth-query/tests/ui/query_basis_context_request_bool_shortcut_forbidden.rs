use worth_query::facade::QueryBasisContextRequest;

fn main() {
    let _ = QueryBasisContextRequest::current_branch_head(true);
}
