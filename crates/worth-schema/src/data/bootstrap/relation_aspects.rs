use forge_relational::facade::schema::{
    AspectBinding, DeclaredAspectContractBinding, KindAspectContractDeclarations,
};

use crate::data::aspects::{
    aspect_key, entity_reference_contract, relation_domain_aspect as relation_kind_domain_aspect,
    scalar_string_contract, Aspect,
};
use crate::data::relations::{NamingRelationKind, RelationKind};

pub fn relation_aspects(kind: RelationKind) -> KindAspectContractDeclarations {
    let mut declarations = vec![lifecycle_aspect(), relation_source_aspect(), relation_target_aspect()];
    match kind {
        RelationKind::Naming(NamingRelationKind::PersistentNameTargetsEntity) => {
            declarations.push(naming_source_identity_aspect());
            declarations.push(naming_target_identity_aspect());
        }
        _ => declarations.push(relation_domain_aspect(relation_kind_domain_aspect(kind))),
    }
    KindAspectContractDeclarations::new(declarations)
}

fn relation_domain_aspect(aspect: Aspect) -> DeclaredAspectContractBinding {
    DeclaredAspectContractBinding {
        binding: AspectBinding::RelationTargetEndpoint,
        contract: entity_reference_contract(aspect.aspect_key().as_str()),
    }
}

fn lifecycle_aspect() -> DeclaredAspectContractBinding {
    DeclaredAspectContractBinding {
        binding: AspectBinding::LifecycleTransition,
        contract: scalar_string_contract(aspect_key("lifecycle").as_str()),
    }
}

fn relation_source_aspect() -> DeclaredAspectContractBinding {
    DeclaredAspectContractBinding {
        binding: AspectBinding::RelationSourceEndpoint,
        contract: entity_reference_contract(aspect_key("source").as_str()),
    }
}

fn relation_target_aspect() -> DeclaredAspectContractBinding {
    DeclaredAspectContractBinding {
        binding: AspectBinding::RelationTargetEndpoint,
        contract: entity_reference_contract(aspect_key("target").as_str()),
    }
}

fn naming_source_identity_aspect() -> DeclaredAspectContractBinding {
    DeclaredAspectContractBinding {
        binding: AspectBinding::RelationSourceEndpoint,
        contract: entity_reference_contract(aspect_key("naming.source_identity").as_str()),
    }
}

fn naming_target_identity_aspect() -> DeclaredAspectContractBinding {
    DeclaredAspectContractBinding {
        binding: AspectBinding::RelationTargetEndpoint,
        contract: entity_reference_contract(aspect_key("naming.target_identity").as_str()),
    }
}
