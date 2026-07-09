use worth_query::facade::WorthQueryEffectPayload;

fn main() {
    let _ = WorthQueryEffectPayload {
        condition: None,
        input_aspects: Vec::new(),
        output_aspects: Vec::new(),
        changed_aspects: Vec::new(),
    };
}
