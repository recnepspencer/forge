use forge_relational::facade::publication::AspectKey;
use forge_relational::facade::schema::{
    AspectBinding, AspectComparator, AspectPrecision, DeclaredAspect, KindAspectDeclarations,
};
use forge_relational::facade::symbols::InternedString;

use crate::data::aspects::{
    WorthAspect, WorthDiagnosticsAspect, WorthGeometryAspect, WorthNamingAspect,
    WorthTopologyAspect,
};
use crate::data::entities::{WorthDiagnosticsEntityKind, WorthEntityKind};

pub fn entity_aspects(kind: WorthEntityKind) -> KindAspectDeclarations {
    KindAspectDeclarations::new(vec![
        entity_payload_aspect(domain_aspect(kind), domain_field(kind)),
        lifecycle_aspect(),
    ])
}

fn domain_aspect(kind: WorthEntityKind) -> WorthAspect {
    match kind {
        WorthEntityKind::Topology(_) => WorthAspect::Topology(WorthTopologyAspect::Structure),
        WorthEntityKind::Geometry(_) => WorthAspect::Geometry(WorthGeometryAspect::Binding),
        WorthEntityKind::Naming(_) => WorthAspect::Naming(WorthNamingAspect::PersistentName),
        WorthEntityKind::Diagnostics(WorthDiagnosticsEntityKind::WireInterpretation)
        | WorthEntityKind::Diagnostics(WorthDiagnosticsEntityKind::ShellInterpretation) => {
            WorthAspect::Diagnostics(WorthDiagnosticsAspect::Interpretations)
        }
    }
}

fn entity_payload_aspect(aspect: WorthAspect, field: &str) -> DeclaredAspect {
    DeclaredAspect {
        key: aspect.aspect_key(),
        binding: AspectBinding::EntityPayloadField {
            field: InternedString::Raw(field.to_string()),
        },
        comparator: AspectComparator::JsonScalarEquality,
        precision: AspectPrecision::Structured,
    }
}

fn domain_field(kind: WorthEntityKind) -> &'static str {
    match kind {
        WorthEntityKind::Topology(_) => "structure",
        WorthEntityKind::Geometry(_) => "binding",
        WorthEntityKind::Naming(_) => "persistent_name",
        WorthEntityKind::Diagnostics(WorthDiagnosticsEntityKind::WireInterpretation)
        | WorthEntityKind::Diagnostics(WorthDiagnosticsEntityKind::ShellInterpretation) => {
            "interpretations"
        }
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
