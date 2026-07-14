use worth_query::facade::runtime::QueryResultBindingProof;

fn assert_no_terminal_binding_projection(binding: &QueryResultBindingProof) {
    let _ = binding.terminal_binding_aspect_projection();
}

fn main() {}
