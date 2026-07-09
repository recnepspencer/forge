use worth_query::facade::WorthQueryGraphTouchDescriptorRow;

fn assert_no_terminal_declared_operation_projection(row: &WorthQueryGraphTouchDescriptorRow) {
    let _ = row.terminal_declared_aspect_operations_projection();
}

fn main() {}
