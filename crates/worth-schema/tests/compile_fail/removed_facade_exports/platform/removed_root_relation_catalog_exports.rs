use schema::facade::{RelationKind, TopologyRelationKind};

fn main() {
    let _ = RelationKind::Topology(TopologyRelationKind::FaceOuterLoop);
}
