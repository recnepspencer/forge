use worth_query::facade::foundation::LineageTraversalDescriptor;

fn main() {
    let _: fn(String, usize) -> LineageTraversalDescriptor =
        LineageTraversalDescriptor::with_max_depth;
}
