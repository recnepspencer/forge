use worth_query::facade::runtime::WorthQueryEffectDelivery;

fn assert_no_neutral_path_alias(delivery: &WorthQueryEffectDelivery) {
    let _ = delivery.aspect_paths();
}

fn main() {}
