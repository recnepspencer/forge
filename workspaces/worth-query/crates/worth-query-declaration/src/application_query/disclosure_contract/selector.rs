use crate::portable_identity::WorthQueryPortableTypeIdentity;
use worth_foundational::facade::{AspectMask, DiagnosticMask, ProjectionMask};

use super::super::{
    ApplicationQueryCardinality, ApplicationQueryResultSlotKey,
    ApplicationQueryResultTraversalDirection,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ApplicationQueryDisclosureSelector {
    InternalField {
        entity: &'static str,
        aspect: &'static str,
        field: &'static str,
        projection_mask: AspectMask<ProjectionMask>,
        diagnostic_mask: AspectMask<DiagnosticMask>,
    },
    Field {
        slot_key: ApplicationQueryResultSlotKey,
        query_type: WorthQueryPortableTypeIdentity,
        slot_type: WorthQueryPortableTypeIdentity,
        entity: &'static str,
        aspect: &'static str,
        field: &'static str,
        output_name: &'static str,
        projection_mask: AspectMask<ProjectionMask>,
        diagnostic_mask: AspectMask<DiagnosticMask>,
    },
    Relation {
        slot_key: ApplicationQueryResultSlotKey,
        query_type: WorthQueryPortableTypeIdentity,
        slot_type: WorthQueryPortableTypeIdentity,
        relation: &'static str,
        from: &'static str,
        to: &'static str,
        direction: ApplicationQueryResultTraversalDirection,
        cardinality: ApplicationQueryCardinality,
        output_name: &'static str,
    },
}

impl ApplicationQueryDisclosureSelector {
    pub const fn result_slot_key(&self) -> Option<ApplicationQueryResultSlotKey> {
        match self {
            Self::Field { slot_key, .. } | Self::Relation { slot_key, .. } => Some(*slot_key),
            Self::InternalField { .. } => None,
        }
    }

    pub const fn is_internal_computation(&self) -> bool {
        matches!(self, Self::InternalField { .. })
    }

    pub const fn slot_type(&self) -> &'static str {
        match self {
            Self::Field { slot_type, .. } | Self::Relation { slot_type, .. } => slot_type.as_str(),
            Self::InternalField { .. } => "internal-computation",
        }
    }

    pub const fn query_type(&self) -> &'static str {
        match self {
            Self::Field { query_type, .. } | Self::Relation { query_type, .. } => {
                query_type.as_str()
            }
            Self::InternalField { .. } => "internal-computation",
        }
    }

    pub const fn output_name(&self) -> &'static str {
        match self {
            Self::Field { output_name, .. } | Self::Relation { output_name, .. } => output_name,
            Self::InternalField { .. } => "internal-computation",
        }
    }

    pub const fn field_contract(&self) -> Option<(&'static str, &'static str, &'static str)> {
        match self {
            Self::InternalField {
                entity,
                aspect,
                field,
                ..
            }
            | Self::Field {
                entity,
                aspect,
                field,
                ..
            } => Some((entity, aspect, field)),
            Self::Relation { .. } => None,
        }
    }

    pub const fn relation_contract(
        &self,
    ) -> Option<(
        &'static str,
        &'static str,
        &'static str,
        ApplicationQueryResultTraversalDirection,
        ApplicationQueryCardinality,
    )> {
        match self {
            Self::Relation {
                relation,
                from,
                to,
                direction,
                cardinality,
                ..
            } => Some((relation, from, to, *direction, *cardinality)),
            Self::Field { .. } | Self::InternalField { .. } => None,
        }
    }

    pub const fn projection_mask(&self) -> Option<&AspectMask<ProjectionMask>> {
        match self {
            Self::InternalField {
                projection_mask, ..
            }
            | Self::Field {
                projection_mask, ..
            } => Some(projection_mask),
            Self::Relation { .. } => None,
        }
    }

    pub const fn diagnostic_mask(&self) -> Option<&AspectMask<DiagnosticMask>> {
        match self {
            Self::InternalField {
                diagnostic_mask, ..
            }
            | Self::Field {
                diagnostic_mask, ..
            } => Some(diagnostic_mask),
            Self::Relation { .. } => None,
        }
    }
}
