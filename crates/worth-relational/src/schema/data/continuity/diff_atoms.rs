use std::sync::Arc;

use serde::{Deserialize, Serialize};

use worth_foundational::FieldKey;

use crate::identity::data::KindId;
use crate::schema::data::{SchemaId, SchemaVersionId};

use super::{
    default_boundary_visibility_for_subscriber_impact, FreeFormSchemaDiffIntent,
    HistoricalInterpretationSensitivity, SchemaPublicationImpact, SchemaStratum,
    SchemaSubscriberImpact, SubscriberBoundaryVisibility,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SchemaElementKind {
    Schema,
    EntityKind,
    RelationKind,
    Field,
    RelationEndpoint,
    EnumDomain,
    PrecisionContract,
    InvariantContract,
    ProjectionContract,
    SubscriberContract,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SchemaElementRef {
    pub kind: SchemaElementKind,
    pub schema_id: SchemaId,
    pub schema_version_id: SchemaVersionId,
    pub kind_id: Option<KindId>,
    pub element_name: Arc<str>,
}

impl SchemaElementRef {
    pub fn new(
        kind: SchemaElementKind,
        schema_id: SchemaId,
        schema_version_id: SchemaVersionId,
        kind_id: Option<KindId>,
        element_name: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            kind,
            schema_id,
            schema_version_id,
            kind_id,
            element_name: element_name.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SchemaDiffDetail {
    AddedField {
        field: FieldKey,
        required: bool,
        default_expression: Option<Arc<str>>,
    },
    RemovedField {
        field: FieldKey,
    },
    TypeChanged {
        field: FieldKey,
        from_type: Arc<str>,
        to_type: Arc<str>,
    },
    EnumDomainExpanded {
        field: FieldKey,
        added_variants: Vec<Arc<str>>,
    },
    InvariantContractChanged {
        contract_name: Arc<str>,
    },
    ProjectionContractChanged {
        projection_name: Arc<str>,
    },
    SubscriberContractChanged {
        contract_name: Arc<str>,
    },
    FreeText {
        detail: Arc<str>,
        declared_intent: FreeFormSchemaDiffIntent,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaDiffAtom {
    pub element: SchemaElementRef,
    pub strata: Vec<SchemaStratum>,
    pub publication_impact: SchemaPublicationImpact,
    pub subscriber_impact: SchemaSubscriberImpact,
    pub boundary_visibility: SubscriberBoundaryVisibility,
    pub historical_interpretation: HistoricalInterpretationSensitivity,
    pub detail: SchemaDiffDetail,
}

impl SchemaDiffAtom {
    pub fn new(
        element: SchemaElementRef,
        strata: Vec<SchemaStratum>,
        publication_impact: SchemaPublicationImpact,
        subscriber_impact: SchemaSubscriberImpact,
        historical_interpretation: HistoricalInterpretationSensitivity,
        detail: SchemaDiffDetail,
    ) -> Self {
        Self {
            element,
            strata,
            publication_impact,
            subscriber_impact,
            boundary_visibility: default_boundary_visibility_for_subscriber_impact(
                subscriber_impact,
            ),
            historical_interpretation,
            detail,
        }
    }

    pub fn with_boundary_visibility_proof(
        mut self,
        boundary_visibility: SubscriberBoundaryVisibility,
    ) -> Self {
        self.boundary_visibility = boundary_visibility;
        self
    }
}
