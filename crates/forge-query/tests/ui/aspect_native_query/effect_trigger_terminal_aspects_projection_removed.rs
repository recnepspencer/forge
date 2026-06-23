use forge_query::facade::ForgeQueryEffectTrigger;

fn assert_no_terminal_aspects_projection(trigger: &ForgeQueryEffectTrigger) {
    let _ = trigger.terminal_aspects_projection();
}

fn main() {}
