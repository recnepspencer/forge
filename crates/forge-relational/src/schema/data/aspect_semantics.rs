use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use crate::identity::data::KindId;
use crate::publication::patch::data::AspectKey;
use crate::symbols::data::InternedString;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct AspectPlanRevision(pub u128);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KindAspectDeclarations {
    pub plan_revision: AspectPlanRevision,
    pub aspects: Vec<DeclaredAspect>,
}

impl KindAspectDeclarations {
    pub fn new(aspects: Vec<DeclaredAspect>) -> Self {
        Self {
            plan_revision: AspectPlanRevision(0),
            aspects,
        }
    }
}

impl Default for KindAspectDeclarations {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeclaredAspect {
    pub key: AspectKey,
    pub binding: AspectBinding,
    pub comparator: AspectComparator,
    pub precision: AspectPrecision,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AspectBinding {
    EntityPayloadField { field: InternedString },
    RelationPayloadField { field: InternedString },
    RelationSourceEndpoint,
    RelationTargetEndpoint,
    LifecycleTransition,
    OpaqueWholePayload,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AspectComparator {
    JsonScalarEquality,
    EndpointIdentityEquality,
    LifecycleTransitionEquality,
    OpaquePayloadByteEquality,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AspectPrecision {
    Structured,
    Opaque,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AspectPlanCatalog {
    pub entity_plans: BTreeMap<KindId, LoweredAspectPlan>,
    pub relation_plans: BTreeMap<KindId, LoweredAspectPlan>,
}

impl AspectPlanCatalog {
    pub fn empty() -> Self {
        Self {
            entity_plans: BTreeMap::new(),
            relation_plans: BTreeMap::new(),
        }
    }
}

impl Default for AspectPlanCatalog {
    fn default() -> Self {
        Self::empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoweredAspectPlan {
    pub kind_id: KindId,
    pub plan_revision: AspectPlanRevision,
    pub executable_bindings: SmallVec<[LoweredAspectBinding; 8]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoweredAspectBinding {
    pub aspect_key: AspectKey,
    pub extractor: LoweredAspectExtractor,
    pub comparator: LoweredAspectComparator,
    pub precision: AspectPrecision,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoweredAspectExtractor {
    EntityJsonField { field: InternedString },
    RelationJsonField { field: InternedString },
    RelationSourceEndpoint,
    RelationTargetEndpoint,
    LifecycleTransition,
    OpaqueWholePayloadBytes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoweredAspectComparator {
    JsonScalarEquality,
    EndpointIdentityEquality,
    LifecycleTransitionEquality,
    OpaquePayloadByteEquality,
}
