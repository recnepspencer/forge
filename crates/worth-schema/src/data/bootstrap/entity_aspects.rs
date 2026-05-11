use forge_relational::facade::publication::AspectKey;
use forge_relational::facade::schema::{
    AspectBinding, AspectComparator, AspectPrecision, DeclaredAspect, KindAspectDeclarations,
};
use forge_relational::facade::symbols::InternedString;

use crate::data::aspects::{
    Aspect, DiagnosticsAspect, GeometryAspect, NamingAspect, TopologyAspect,
};
use crate::data::entities::{DiagnosticsEntityKind, EntityKind};

pub fn entity_aspects(kind: EntityKind) -> KindAspectDeclarations {
    KindAspectDeclarations::new(vec![
        entity_payload_aspect(domain_aspect(kind), domain_field(kind)),
        lifecycle_aspect(),
    ])
}

fn domain_aspect(kind: EntityKind) -> Aspect {
    match kind {
        EntityKind::Topology(_) => Aspect::Topology(TopologyAspect::Structure),
        EntityKind::Geometry(_) => Aspect::Geometry(GeometryAspect::Binding),
        EntityKind::Naming(_) => Aspect::Naming(NamingAspect::PersistentName),
        EntityKind::Diagnostics(DiagnosticsEntityKind::WireInterpretation)
        | EntityKind::Diagnostics(DiagnosticsEntityKind::ShellInterpretation) => {
            Aspect::Diagnostics(DiagnosticsAspect::Interpretations)
        }
    }
}

fn entity_payload_aspect(aspect: Aspect, field: &str) -> DeclaredAspect {
    DeclaredAspect {
        key: aspect.aspect_key(),
        binding: AspectBinding::EntityPayloadField {
            field: InternedString::Raw(field.to_string()),
        },
        comparator: AspectComparator::JsonScalarEquality,
        precision: AspectPrecision::Structured,
    }
}

fn domain_field(kind: EntityKind) -> &'static str {
    match kind {
        EntityKind::Topology(_) => "structure",
        EntityKind::Geometry(_) => "binding",
        EntityKind::Naming(_) => "persistent_name",
        EntityKind::Diagnostics(DiagnosticsEntityKind::WireInterpretation)
        | EntityKind::Diagnostics(DiagnosticsEntityKind::ShellInterpretation) => "interpretations",
    }
}

fn lifecycle_aspect() -> DeclaredAspect {
    DeclaredAspect {
        key: AspectKey(InternedString::Raw("lifecycle".to_string())),
        binding: AspectBinding::LifecycleTransition,
        comparator: AspectComparator::LifecycleTransitionEquality,
        precision: AspectPrecision::Structured,
    }
}
