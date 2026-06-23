use forge_query::facade::ForgeQueryEffectExpression;

fn assert_no_terminal_output_projection(expression: &ForgeQueryEffectExpression) {
    let _ = expression.terminal_output_aspects_projection();
}

fn main() {}
