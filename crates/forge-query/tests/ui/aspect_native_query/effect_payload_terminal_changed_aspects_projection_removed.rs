use forge_query::facade::ForgeQueryEffectPayload;

fn assert_no_terminal_changed_projection(payload: &ForgeQueryEffectPayload) {
    let _ = payload.terminal_changed_aspects_projection();
}

fn main() {}
