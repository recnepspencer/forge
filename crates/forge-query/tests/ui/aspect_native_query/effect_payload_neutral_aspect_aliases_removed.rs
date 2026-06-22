use forge_query::facade::ForgeQueryEffectPayload;

fn assert_no_neutral_payload_aspect_aliases(payload: &ForgeQueryEffectPayload) {
    let _ = payload.input_aspects();
    let _ = payload.output_aspects();
    let _ = payload.changed_aspects();
}

fn main() {}
