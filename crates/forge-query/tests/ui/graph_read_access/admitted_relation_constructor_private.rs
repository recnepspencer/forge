use forge_query::facade::runtime::{
    ForgeQueryAdmittedGraphReadRelation, ForgeQueryAdmittedGraphReadRelationDirection,
};

fn main() {
    let _ = ForgeQueryAdmittedGraphReadRelation {
        relation: "manager".to_string(),
        direction: ForgeQueryAdmittedGraphReadRelationDirection::Forward,
        depth: 1,
    };
}
