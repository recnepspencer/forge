use forge_relational::facade::identity::EntityId;

use crate::data::topology_view::{
    WorthTopologyBody, WorthTopologyFace, WorthTopologyHalfEdge, WorthTopologyLoop,
    WorthTopologyLump, WorthTopologyModel, WorthTopologyRegion, WorthTopologyShell,
    WorthTopologyWire,
};

pub trait HasEntityId {
    fn entity_id(&self) -> EntityId;
}

impl HasEntityId for WorthTopologyModel {
    fn entity_id(&self) -> EntityId {
        self.entity_id
    }
}

impl HasEntityId for WorthTopologyBody {
    fn entity_id(&self) -> EntityId {
        self.entity_id
    }
}

impl HasEntityId for WorthTopologyLump {
    fn entity_id(&self) -> EntityId {
        self.entity_id
    }
}

impl HasEntityId for WorthTopologyRegion {
    fn entity_id(&self) -> EntityId {
        self.entity_id
    }
}

impl HasEntityId for WorthTopologyShell {
    fn entity_id(&self) -> EntityId {
        self.entity_id
    }
}

impl HasEntityId for WorthTopologyFace {
    fn entity_id(&self) -> EntityId {
        self.entity_id
    }
}

impl HasEntityId for WorthTopologyLoop {
    fn entity_id(&self) -> EntityId {
        self.entity_id
    }
}

impl HasEntityId for WorthTopologyHalfEdge {
    fn entity_id(&self) -> EntityId {
        self.entity_id
    }
}

impl HasEntityId for WorthTopologyWire {
    fn entity_id(&self) -> EntityId {
        self.entity_id
    }
}
