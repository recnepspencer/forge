use forge_query::facade::ForgeQueryEffectDelivery;

fn assert_no_terminal_path_projection(delivery: &ForgeQueryEffectDelivery) {
    let _ = delivery.terminal_aspect_paths_projection();
}

fn main() {}
