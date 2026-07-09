use worth_query::facade::WorthQueryEffectExpression;

fn assert_no_terminal_output_projection(expression: &WorthQueryEffectExpression) {
    let _ = expression.terminal_output_aspects_projection();
}

fn main() {}
