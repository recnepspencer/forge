use worth_query::facade::runtime::WorthQueryEffectTrigger;

fn assert_no_terminal_aspects_projection(trigger: &WorthQueryEffectTrigger) {
    let _ = trigger.terminal_aspects_projection();
}

fn main() {}
