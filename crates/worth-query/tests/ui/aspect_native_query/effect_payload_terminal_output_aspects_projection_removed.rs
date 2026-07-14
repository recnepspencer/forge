use worth_query::facade::runtime::WorthQueryEffectPayload;

fn assert_no_terminal_output_projection(payload: &WorthQueryEffectPayload) {
    let _ = payload.terminal_output_aspects_projection();
}

fn main() {}
