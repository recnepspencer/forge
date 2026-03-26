use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use crate::identity::data::KindId;
use crate::publication::patch::data::AspectKey;
use crate::schema::data::PayloadSchemaDeclaration;
use crate::symbols::data::InternedString;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct AspectPlanRevision(pub u128);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KindAspectDeclarations {
    pub plan_revision: AspectPlanRevision,
    pub aspects: Vec<DeclaredAspect>,
    pub payload_schema: Option<PayloadSchemaDeclaration>,
}

impl KindAspectDeclarations {
    pub fn new(aspects: Vec<DeclaredAspect>) -> Self {
        Self {
            plan_revision: AspectPlanRevision(0),
            aspects,
            payload_schema: None,
        }
    }

    pub fn with_payload_schema(mut self, payload_schema: PayloadSchemaDeclaration) -> Self {
        self.payload_schema = Some(payload_schema);
        self
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
#[non_exhaustive]
pub enum AspectBinding {
    EntityPayloadField { field: InternedString },
    RelationPayloadField { field: InternedString },
    RelationSourceEndpoint,
    RelationTargetEndpoint,
    LifecycleTransition,
    OpaqueWholePayload,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AspectComparator {
    JsonScalarEquality,
    EndpointIdentityEquality,
    LifecycleTransitionEquality,
    OpaquePayloadByteEquality,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
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
    pub binding_kind: LoweredExecutableAspectBindingKind,
    pub precision: AspectPrecision,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum LoweredAspectExtractor {
    EntityJsonField { field: InternedString },
    RelationJsonField { field: InternedString },
    RelationSourceEndpoint,
    RelationTargetEndpoint,
    LifecycleTransition,
    OpaqueWholePayloadBytes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum LoweredAspectComparator {
    JsonScalarEquality,
    EndpointIdentityEquality,
    LifecycleTransitionEquality,
    OpaquePayloadByteEquality,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum LoweredExecutableAspectBindingKind {
    EntityJsonScalarField { field: InternedString },
    RelationJsonScalarField { field: InternedString },
    RelationSourceEndpointIdentity,
    RelationTargetEndpointIdentity,
    LifecycleTransitionEquality,
    OpaqueWholePayloadBytes,
}

impl LoweredAspectBinding {
    pub fn extractor(&self) -> LoweredAspectExtractor {
        match &self.binding_kind {
            LoweredExecutableAspectBindingKind::EntityJsonScalarField { field } => {
                LoweredAspectExtractor::EntityJsonField {
                    field: field.clone(),
                }
            }
            LoweredExecutableAspectBindingKind::RelationJsonScalarField { field } => {
                LoweredAspectExtractor::RelationJsonField {
                    field: field.clone(),
                }
            }
            LoweredExecutableAspectBindingKind::RelationSourceEndpointIdentity => {
                LoweredAspectExtractor::RelationSourceEndpoint
            }
            LoweredExecutableAspectBindingKind::RelationTargetEndpointIdentity => {
                LoweredAspectExtractor::RelationTargetEndpoint
            }
            LoweredExecutableAspectBindingKind::LifecycleTransitionEquality => {
                LoweredAspectExtractor::LifecycleTransition
            }
            LoweredExecutableAspectBindingKind::OpaqueWholePayloadBytes => {
                LoweredAspectExtractor::OpaqueWholePayloadBytes
            }
        }
    }

    pub fn comparator(&self) -> LoweredAspectComparator {
        match self.binding_kind {
            LoweredExecutableAspectBindingKind::EntityJsonScalarField { .. }
            | LoweredExecutableAspectBindingKind::RelationJsonScalarField { .. } => {
                LoweredAspectComparator::JsonScalarEquality
            }
            LoweredExecutableAspectBindingKind::RelationSourceEndpointIdentity
            | LoweredExecutableAspectBindingKind::RelationTargetEndpointIdentity => {
                LoweredAspectComparator::EndpointIdentityEquality
            }
            LoweredExecutableAspectBindingKind::LifecycleTransitionEquality => {
                LoweredAspectComparator::LifecycleTransitionEquality
            }
            LoweredExecutableAspectBindingKind::OpaqueWholePayloadBytes => {
                LoweredAspectComparator::OpaquePayloadByteEquality
            }
        }
    }
}
