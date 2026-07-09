#![allow(unreachable_code)]

use worth_query::facade::runtime::{
    WorthQueryAdmittedBooleanExpressionTopology, WorthQueryAdmittedBooleanPredicateExpression,
};

fn main() {
    let _ = WorthQueryAdmittedBooleanPredicateExpression {
        read_graph_digest: String::new(),
        topology: WorthQueryAdmittedBooleanExpressionTopology::ConjunctiveFlat,
        branches: Vec::new(),
        counters: todo!(),
    };
}
