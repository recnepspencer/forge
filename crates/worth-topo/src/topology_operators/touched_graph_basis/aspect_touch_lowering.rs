use forge_query::facade::ForgeQueryAspectTouch;
use schema::facade::platform::aspects::{
    Aspect, DiagnosticsAspect, GeometryAspect, LineageAspect, NamingAspect, TopologyAspect,
};

use super::TopologyTouchedAspect;

pub(crate) fn query_aspect_touch(aspect: TopologyTouchedAspect) -> ForgeQueryAspectTouch {
    ForgeQueryAspectTouch::whole_aspect(schema_aspect_for_touched_aspect(aspect).aspect_key())
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
