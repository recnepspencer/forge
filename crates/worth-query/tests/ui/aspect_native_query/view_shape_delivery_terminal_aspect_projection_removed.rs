use worth_query::facade::runtime::ViewShapeDeliveryMetadata;

fn assert_no_terminal_delivery_aspect_projection(metadata: &ViewShapeDeliveryMetadata) {
    let _ = metadata.terminal_focus_aspect_projection();
    let _ = metadata.terminal_grouping_aspect_projection();
}

fn main() {}
