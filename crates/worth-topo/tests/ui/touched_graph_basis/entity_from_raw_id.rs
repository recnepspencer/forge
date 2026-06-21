use forge_relational::facade::identity::{EntityId, PartitionId};
use topology::facade::TopologyTouchedEntity;

fn main() {
    let raw = EntityId::new(PartitionId::main(), 7, 1);
    let _entity = TopologyTouchedEntity::new(raw);
}
