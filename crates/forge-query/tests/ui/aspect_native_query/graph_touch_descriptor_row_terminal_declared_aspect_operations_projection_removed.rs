use forge_query::facade::ForgeQueryGraphTouchDescriptorRow;

fn assert_no_terminal_declared_operation_projection(row: &ForgeQueryGraphTouchDescriptorRow) {
    let _ = row.terminal_declared_aspect_operations_projection();
}

fn main() {}
