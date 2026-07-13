use worth_query::facade::runtime::{WorthQueryAdmittedGraphReadRelation, WorthQueryAdmittedGraphReadRelationDirection};

fn main() {
    let _ = WorthQueryAdmittedGraphReadRelation {
        relation: "manager".to_string(),
        direction: WorthQueryAdmittedGraphReadRelationDirection::Forward,
        depth: 1,
    };
}
