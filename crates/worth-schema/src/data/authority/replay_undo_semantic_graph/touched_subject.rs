use serde::Serialize;

use crate::data::authority::touched_graph_basis::{
    WorthTopologyTouchedAspect, WorthTopologyTouchedScope,
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ReplayUndoSemanticGraphTouchedSubject {
    TopologyEntity {
        entity_identity: String,
    },
    TopologyRelation {
        relation_identity: String,
    },
    TopologyRelationKind {
        relation_kind: String,
    },
    TopologyAspect {
        aspect: WorthTopologyTouchedAspect,
    },
    TopologyScope {
        scope: WorthTopologyTouchedScope,
    },
    SpatialAuthorityStage {
        evidence_stage: String,
        evidence_identity: String,
    },
}

impl ReplayUndoSemanticGraphTouchedSubject {
    pub fn digest_part(&self) -> String {
        match self {
            Self::TopologyEntity { entity_identity } => {
                format!("topology-entity:{entity_identity}")
            }
            Self::TopologyRelation { relation_identity } => {
                format!("topology-relation:{relation_identity}")
            }
            Self::TopologyRelationKind { relation_kind } => {
                format!("topology-relation-kind:{relation_kind}")
            }
            Self::TopologyAspect { aspect } => format!("topology-aspect:{}", aspect.as_str()),
            Self::TopologyScope { scope } => format!("topology-scope:{}", scope.as_str()),
            Self::SpatialAuthorityStage {
                evidence_stage,
                evidence_identity,
            } => format!("spatial-stage:{evidence_stage}:{evidence_identity}"),
        }
    }
}
