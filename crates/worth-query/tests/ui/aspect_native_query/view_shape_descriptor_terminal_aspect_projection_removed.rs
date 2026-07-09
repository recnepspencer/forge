use worth_query::facade::ViewShapeDescriptor;

fn assert_no_terminal_descriptor_aspect_projection(descriptor: &ViewShapeDescriptor) {
    let _ = descriptor.terminal_focused_aspect_projection();
    let _ = descriptor.terminal_grouping_aspect_projection();
}

fn main() {}
