use worth_query::facade::WorthQueryEffectPayload;

fn assert_no_neutral_payload_aspect_aliases(payload: &WorthQueryEffectPayload) {
    let _ = payload.input_aspects();
    let _ = payload.output_aspects();
    let _ = payload.changed_aspects();
}

fn main() {}
