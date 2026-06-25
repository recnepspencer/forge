use forge_foundational::facade::{
    AspectKey, AspectValue, CanonicalFieldPath, FieldKey, InternedString,
};
use forge_query::facade::{ForgeQueryAspectTouch, ForgeQueryAuthoredAspectValue};
use schema::facade::platform::aspects::{
    Aspect, DiagnosticsAspect, GeometryAspect, LineageAspect, NamingAspect, TopologyAspect,
};

use crate::topology_operators::TopologyTouchedAspect;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthTopologyNativeCarrierBoundaryError {
    EmptyFieldPath,
    InvalidFieldSegment(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyNativeFieldPath {
    canonical: CanonicalFieldPath,
    digest_part: String,
}

impl WorthTopologyNativeFieldPath {
    pub fn from_segments(
        segments: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<Self, WorthTopologyNativeCarrierBoundaryError> {
        let fields = segments
            .into_iter()
            .map(|segment| {
                let segment = segment.into();
                FieldKey::new(segment.clone())
                    .ok_or(WorthTopologyNativeCarrierBoundaryError::InvalidFieldSegment(segment))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let canonical = CanonicalFieldPath::new(fields)
            .ok_or(WorthTopologyNativeCarrierBoundaryError::EmptyFieldPath)?;
        Ok(Self::from_canonical(canonical))
    }

    pub fn single(
        segment: impl Into<String>,
    ) -> Result<Self, WorthTopologyNativeCarrierBoundaryError> {
        Self::from_segments([segment])
    }

    fn from_canonical(canonical: CanonicalFieldPath) -> Self {
        let digest_part = canonical
            .fields()
            .iter()
            .map(|field| field.as_str())
            .collect::<Vec<_>>()
            .join(".");
        Self {
            canonical,
            digest_part,
        }
    }

    pub fn canonical(&self) -> &CanonicalFieldPath {
        &self.canonical
    }

    pub fn digest_part(&self) -> &str {
        &self.digest_part
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyNativeAspectField {
    aspect: Aspect,
    field_path: WorthTopologyNativeFieldPath,
    digest_part: String,
}

impl WorthTopologyNativeAspectField {
    pub fn from_touched_aspect_and_field_path(
        aspect: TopologyTouchedAspect,
        field_path: WorthTopologyNativeFieldPath,
    ) -> Self {
        Self::from_schema_aspect_and_field_path(
            schema_aspect_for_touched_aspect(aspect),
            field_path,
        )
    }

    pub fn from_schema_aspect_and_field_path(
        aspect: Aspect,
        field_path: WorthTopologyNativeFieldPath,
    ) -> Self {
        let digest_part = format!(
            "{}.{}",
            aspect.aspect_key().as_str(),
            field_path.digest_part()
        );
        Self {
            aspect,
            field_path,
            digest_part,
        }
    }

    pub fn aspect_key(&self) -> AspectKey {
        self.aspect.aspect_key()
    }

    pub fn field_path(&self) -> &WorthTopologyNativeFieldPath {
        &self.field_path
    }

    pub fn query_aspect_touch(&self) -> ForgeQueryAspectTouch {
        ForgeQueryAspectTouch::aspect_field_path(
            self.aspect_key(),
            self.field_path.canonical.clone(),
        )
    }

    pub fn digest_part(&self) -> &str {
        &self.digest_part
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyNativeAspectValue {
    foundational: AspectValue,
    digest_part: String,
}

impl WorthTopologyNativeAspectValue {
    pub fn string(value: impl Into<String>) -> Self {
        let value = value.into();
        Self {
            foundational: AspectValue::String(value.clone().into()),
            digest_part: format!("string:{value}"),
        }
    }

    pub fn int64(value: i64) -> Self {
        Self {
            foundational: AspectValue::Int64(value),
            digest_part: format!("int64:{value}"),
        }
    }

    pub fn bool(value: bool) -> Self {
        Self {
            foundational: AspectValue::Bool(value),
            digest_part: format!("bool:{value}"),
        }
    }

    pub fn null() -> Self {
        Self {
            foundational: AspectValue::Null,
            digest_part: "null".to_string(),
        }
    }

    pub fn foundational(&self) -> &AspectValue {
        &self.foundational
    }

    pub fn query_authored_value(&self) -> ForgeQueryAuthoredAspectValue {
        match &self.foundational {
            AspectValue::String(value) => ForgeQueryAuthoredAspectValue::string(match value {
                InternedString::Raw(value) => value.clone(),
                InternedString::Symbol(symbol) => format!("symbol:{}", symbol.0),
            }),
            AspectValue::Int64(value) => ForgeQueryAuthoredAspectValue::int64(*value),
            AspectValue::Bool(value) => ForgeQueryAuthoredAspectValue::bool(*value),
            AspectValue::Null => ForgeQueryAuthoredAspectValue::null(),
            _ => ForgeQueryAuthoredAspectValue::string(self.digest_part.as_str()),
        }
    }

    pub fn digest_part(&self) -> &str {
        &self.digest_part
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyNativeSetAspectInput {
    aspect_field: WorthTopologyNativeAspectField,
    value: WorthTopologyNativeAspectValue,
    digest: String,
}

impl WorthTopologyNativeSetAspectInput {
    pub fn new(
        aspect_field: WorthTopologyNativeAspectField,
        value: WorthTopologyNativeAspectValue,
    ) -> Self {
        let digest = format!(
            "worth-topo-native-set-aspect-input-v1|{}|{}",
            aspect_field.digest_part(),
            value.digest_part()
        );
        Self {
            aspect_field,
            value,
            digest,
        }
    }

    pub fn query_aspect_touch(&self) -> ForgeQueryAspectTouch {
        self.aspect_field.query_aspect_touch()
    }

    pub fn query_authored_value(&self) -> ForgeQueryAuthoredAspectValue {
        self.value.query_authored_value()
    }

    pub fn foundational_value(&self) -> &AspectValue {
        self.value.foundational()
    }

    pub fn aspect_field(&self) -> &WorthTopologyNativeAspectField {
        &self.aspect_field
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

fn schema_aspect_for_touched_aspect(aspect: TopologyTouchedAspect) -> Aspect {
    match aspect {
        TopologyTouchedAspect::TopologyStructure => Aspect::Topology(TopologyAspect::Structure),
        TopologyTouchedAspect::TopologyOwnership => Aspect::Topology(TopologyAspect::Ownership),
        TopologyTouchedAspect::TopologyBoundary => Aspect::Topology(TopologyAspect::Boundary),
        TopologyTouchedAspect::TopologyRadial => Aspect::Topology(TopologyAspect::Radial),
        TopologyTouchedAspect::GeometryBinding => Aspect::Geometry(GeometryAspect::Binding),
        TopologyTouchedAspect::GeometryEmbedding => Aspect::Geometry(GeometryAspect::Embedding),
        TopologyTouchedAspect::GeometryProvenance => Aspect::Geometry(GeometryAspect::Provenance),
        TopologyTouchedAspect::GeometryApproximation => {
            Aspect::Geometry(GeometryAspect::Approximation)
        }
        TopologyTouchedAspect::GeometryUvAnchoring => Aspect::Geometry(GeometryAspect::UvAnchoring),
        TopologyTouchedAspect::GeometryCarrier => Aspect::Geometry(GeometryAspect::Carrier),
        TopologyTouchedAspect::GeometryPrecision => Aspect::Geometry(GeometryAspect::Precision),
        TopologyTouchedAspect::GeometryFallback => Aspect::Geometry(GeometryAspect::Fallback),
        TopologyTouchedAspect::LineageProvenance => Aspect::Lineage(LineageAspect::Provenance),
        TopologyTouchedAspect::NamingPersistentName => Aspect::Naming(NamingAspect::PersistentName),
        TopologyTouchedAspect::DiagnosticsDecisions => {
            Aspect::Diagnostics(DiagnosticsAspect::Decisions)
        }
        TopologyTouchedAspect::DiagnosticsInterpretations => {
            Aspect::Diagnostics(DiagnosticsAspect::Interpretations)
        }
    }
}
