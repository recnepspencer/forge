use forge_relational::facade::identity::EntityId;

use crate::data::topology_view::{
    TopologyBody, TopologyFace, TopologyHalfEdge, TopologyLoop, TopologyLump, TopologyModel,
    TopologyRegion, TopologyShell, TopologyWire,
};

pub trait HasEntityId {
    fn entity_id(&self) -> EntityId;
}

impl HasEntityId for TopologyModel {
    fn entity_id(&self) -> EntityId {
        self.entity_id
    }
}

impl HasEntityId for TopologyBody {
    fn entity_id(&self) -> EntityId {
        self.entity_id
    }
}

impl HasEntityId for TopologyLump {
    fn entity_id(&self) -> EntityId {
        self.entity_id
    }
}

impl HasEntityId for TopologyRegion {
    fn entity_id(&self) -> EntityId {
        self.entity_id
    }
}

impl HasEntityId for TopologyShell {
    fn entity_id(&self) -> EntityId {
        self.entity_id
    }
}

impl HasEntityId for TopologyFace {
    fn entity_id(&self) -> EntityId {
        self.entity_id
    }
}

impl HasEntityId for TopologyLoop {
    fn entity_id(&self) -> EntityId {
        self.entity_id
    }
}

impl HasEntityId for TopologyHalfEdge {
    fn entity_id(&self) -> EntityId {
        self.entity_id
    }
}

impl HasEntityId for TopologyWire {
    fn entity_id(&self) -> EntityId {
        self.entity_id
    }
}
