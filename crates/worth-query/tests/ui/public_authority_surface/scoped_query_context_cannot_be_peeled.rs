use worth_query::facade::policy::ScopedQueryBasisContext;

fn peel(context: &ScopedQueryBasisContext) {
    let _ = context.context();
}

fn main() {}
