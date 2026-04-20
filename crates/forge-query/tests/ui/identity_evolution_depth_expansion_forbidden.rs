use forge_query::facade::LineageTraversalDescriptor;

fn main() {
    let _: fn(String, usize) -> LineageTraversalDescriptor =
        LineageTraversalDescriptor::with_max_depth;
}
