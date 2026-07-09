use worth_query::facade::WorthQueryEffectPayload;

fn assert_no_terminal_changed_projection(payload: &WorthQueryEffectPayload) {
    let _ = payload.terminal_changed_aspects_projection();
}

fn main() {}
