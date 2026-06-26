use forge_query::facade::ForgeQueryEffectExpression;

fn assert_no_terminal_input_projection(expression: &ForgeQueryEffectExpression) {
    let _ = expression.terminal_input_aspects_projection();
}

fn main() {}
