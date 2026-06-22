use forge_query::facade::ForgeQueryEffectPayload;

fn assert_no_terminal_input_projection(payload: &ForgeQueryEffectPayload) {
    let _ = payload.terminal_input_aspects_projection();
}

fn main() {}
