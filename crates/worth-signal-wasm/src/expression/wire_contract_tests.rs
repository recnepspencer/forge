use serde_json::json;

use super::model::Expr;

#[test]
fn conditional_expression_uses_the_public_camel_case_wire_contract() {
    let expression: Expr = serde_json::from_value(json!({
        "kind": "if",
        "condition": { "kind": "value", "value": true },
        "thenExpr": { "kind": "value", "value": "accepted" },
        "elseExpr": { "kind": "value", "value": "rejected" }
    }))
    .expect("the published conditional expression should deserialize");

    let serialized = serde_json::to_value(expression)
        .expect("a conditional expression should serialize for graph publication");

    assert_eq!(serialized["thenExpr"]["value"], "accepted");
    assert_eq!(serialized["elseExpr"]["value"], "rejected");
    assert!(serialized.get("then_expr").is_none());
    assert!(serialized.get("else_expr").is_none());
}
