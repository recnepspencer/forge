//! Worth aspect catalogs for platform bootstrap and descriptor assembly.
//!
//! This module is the authority for Worth-specific aspect identity and lower
//! publication contract data. It is not the ordinary Query lifecycle entry
//! surface; downstream runtime work should enter through `forge-query`.

mod diagnostics;
mod domain_bindings;
mod geometry;
mod lineage;
mod naming;
mod topology;

use forge_foundational::facade::{aspects, AspectIdentity, AspectKey, FieldKey, ScalarAspectType};
use serde::{Deserialize, Serialize};

pub use diagnostics::DiagnosticsAspect;
pub use domain_bindings::{entity_domain_aspect, entity_domain_field, relation_domain_aspect};
pub use geometry::GeometryAspect;
pub use lineage::LineageAspect;
pub use naming::NamingAspect;
pub use topology::TopologyAspect;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Aspect {
    Topology(TopologyAspect),
    Geometry(GeometryAspect),
    Lineage(LineageAspect),
    Naming(NamingAspect),
    Diagnostics(DiagnosticsAspect),
}

impl Aspect {
    pub const ALL: [Self; 15] = [
        Self::Topology(TopologyAspect::Structure),
        Self::Topology(TopologyAspect::Ownership),
        Self::Topology(TopologyAspect::Boundary),
        Self::Topology(TopologyAspect::Radial),
        Self::Geometry(GeometryAspect::Binding),
        Self::Geometry(GeometryAspect::Embedding),
        Self::Geometry(GeometryAspect::Provenance),
        Self::Geometry(GeometryAspect::Approximation),
        Self::Geometry(GeometryAspect::UvAnchoring),
        Self::Geometry(GeometryAspect::Carrier),
        Self::Geometry(GeometryAspect::Precision),
        Self::Geometry(GeometryAspect::Fallback),
        Self::Lineage(LineageAspect::Provenance),
        Self::Naming(NamingAspect::PersistentName),
        Self::Diagnostics(DiagnosticsAspect::Decisions),
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Topology(aspect) => aspect.as_str(),
            Self::Geometry(aspect) => aspect.as_str(),
            Self::Lineage(aspect) => aspect.as_str(),
            Self::Naming(aspect) => aspect.as_str(),
            Self::Diagnostics(aspect) => aspect.as_str(),
        }
    }

    pub fn aspect_key(self) -> AspectKey {
        aspect_key(self.as_str())
    }
}

pub fn aspect_key(label: &str) -> AspectKey {
    AspectKey::new(label).expect("worth aspect key must be foundational")
}

pub fn field_key(label: &str) -> FieldKey {
    FieldKey::new(label).expect("worth aspect field must be foundational")
}

pub fn scalar_string_contract(label: &str) -> forge_foundational::AspectContract {
    aspects()
        .contract()
        .for_key(aspect_key(label))
        .identified_by(AspectIdentity(stable_contract_identity(label)))
        .at_revision(aspects().vocabulary().revision(1))
        .scalar(ScalarAspectType::String)
}

pub fn entity_reference_contract(label: &str) -> forge_foundational::AspectContract {
    aspects()
        .contract()
        .for_key(aspect_key(label))
        .identified_by(AspectIdentity(stable_contract_identity(label)))
        .at_revision(aspects().vocabulary().revision(1))
        .reference_entity()
}

fn stable_contract_identity(label: &str) -> u64 {
    let mut hash = 14695981039346656037_u64;
    for byte in label.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(1099511628211_u64);
    }
    hash
}
