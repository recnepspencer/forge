use forge_relational::facade::schema::{AspectBinding, DeclaredAspect, KindAspectDeclarations};

use crate::data::aspects::{
    aspect_key, entity_reference_contract, relation_domain_aspect as relation_kind_domain_aspect,
    scalar_string_contract, Aspect,
};
use crate::data::relations::RelationKind;

pub fn relation_aspects(kind: RelationKind) -> KindAspectDeclarations {
    KindAspectDeclarations::new(vec![
        relation_domain_aspect(relation_kind_domain_aspect(kind)),
        lifecycle_aspect(),
        relation_source_aspect(),
        relation_target_aspect(),
    ])
}

fn relation_domain_aspect(aspect: Aspect) -> DeclaredAspect {
    DeclaredAspect {
        binding: AspectBinding::RelationTargetEndpoint,
        contract: entity_reference_contract(aspect.aspect_key().as_str()),
    }
}

fn lifecycle_aspect() -> DeclaredAspect {
    DeclaredAspect {
        binding: AspectBinding::LifecycleTransition,
        contract: scalar_string_contract(aspect_key("lifecycle").as_str()),
    }
}

fn relation_source_aspect() -> DeclaredAspect {
    DeclaredAspect {
        binding: AspectBinding::RelationSourceEndpoint,
        contract: entity_reference_contract(aspect_key("source").as_str()),
    }
}

fn relation_target_aspect() -> DeclaredAspect {
    DeclaredAspect {
        binding: AspectBinding::RelationTargetEndpoint,
        contract: entity_reference_contract(aspect_key("target").as_str()),
    }
}
