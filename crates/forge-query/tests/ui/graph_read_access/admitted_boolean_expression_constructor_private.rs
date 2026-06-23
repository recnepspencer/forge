#![allow(unreachable_code)]

use forge_query::facade::runtime::{
    ForgeQueryAdmittedBooleanExpressionTopology, ForgeQueryAdmittedBooleanPredicateExpression,
};

fn main() {
    let _ = ForgeQueryAdmittedBooleanPredicateExpression {
        read_graph_digest: String::new(),
        topology: ForgeQueryAdmittedBooleanExpressionTopology::ConjunctiveFlat,
        branches: Vec::new(),
        counters: todo!(),
    };
}
