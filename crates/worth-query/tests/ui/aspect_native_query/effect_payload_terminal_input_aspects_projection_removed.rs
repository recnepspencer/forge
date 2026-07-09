use worth_query::facade::WorthQueryEffectPayload;

fn assert_no_terminal_input_projection(payload: &WorthQueryEffectPayload) {
    let _ = payload.terminal_input_aspects_projection();
}

fn main() {}
