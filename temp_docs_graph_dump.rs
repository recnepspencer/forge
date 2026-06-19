use worth_kernel::docs_closeout::current_worth_docs_graph;
fn main() {
    let graph = current_worth_docs_graph().unwrap();
    for edge in graph.edges() {
        if edge.from_path() == "crates/worth-kernel/docs/features/primitive-construction.md" {
            println!("{} -> {}", edge.from_path(), edge.to_path());
        }
    }
}
