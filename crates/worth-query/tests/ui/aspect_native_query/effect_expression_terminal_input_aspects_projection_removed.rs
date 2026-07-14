use worth_query::facade::runtime::WorthQueryEffectExpression;

fn assert_no_terminal_input_projection(expression: &WorthQueryEffectExpression) {
    let _ = expression.terminal_input_aspects_projection();
}

fn main() {}
