use topology::facade::{
    TopologyPrimitiveConstructionQueryAdmittedHandoff,
    TopologyPrimitiveConstructionQueryHandoff,
};

fn requires_admitted(_: TopologyPrimitiveConstructionQueryAdmittedHandoff) {}

fn main() {
    let raw: Option<TopologyPrimitiveConstructionQueryHandoff> = None;
    if let Some(raw) = raw {
        requires_admitted(raw);
    }
}
