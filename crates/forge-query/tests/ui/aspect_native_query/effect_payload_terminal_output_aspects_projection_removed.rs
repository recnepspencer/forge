use forge_query::facade::ForgeQueryEffectPayload;

fn assert_no_terminal_output_projection(payload: &ForgeQueryEffectPayload) {
    let _ = payload.terminal_output_aspects_projection();
}

fn main() {}
