use forge_query::facade::ForgeQueryEffectDelivery;

fn assert_no_neutral_path_alias(delivery: &ForgeQueryEffectDelivery) {
    let _ = delivery.aspect_paths();
}

fn main() {}
