use forge_query::facade::ForgeQueryEffectPayload;

fn main() {
    let _ = ForgeQueryEffectPayload {
        condition: None,
        input_aspects: Vec::new(),
        output_aspects: Vec::new(),
        changed_aspects: Vec::new(),
    };
}
