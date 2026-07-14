use worth_query::facade::runtime::WorthQueryEffectDelivery;

fn assert_no_terminal_path_projection(delivery: &WorthQueryEffectDelivery) {
    let _ = delivery.terminal_aspect_paths_projection();
}

fn main() {}
