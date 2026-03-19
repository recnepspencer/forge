use std::collections::BTreeSet;

use smallvec::SmallVec;

use crate::schema::data::{
    AspectBinding, AspectComparator, AspectPlanCatalog, AspectPlanRevision, AspectPrecision,
    DeclaredAspect, EntityKindRegistration, KindAspectDeclarations, LoweredAspectBinding,
    LoweredAspectComparator, LoweredAspectExtractor, LoweredAspectPlan, RelationKindRegistration,
    RelationPayloadClass, RelationalSchemaRegistry, SchemaRegistryError,
};
use crate::symbols::data::InternedString;

enum RegistrationDomain {
    Entity,
    Relation { topology_only: bool },
}

pub(crate) fn canonicalize_entity_registration(
    mut registration: EntityKindRegistration,
) -> Result<EntityKindRegistration, SchemaRegistryError> {
    registration.aspect_declarations = canonicalize_declarations(
        registration.kind_id,
        registration.aspect_declarations,
        RegistrationDomain::Entity,
    )?;
    Ok(registration)
}

pub(crate) fn canonicalize_relation_registration(
    mut registration: RelationKindRegistration,
) -> Result<RelationKindRegistration, SchemaRegistryError> {
    registration.aspect_declarations = canonicalize_declarations(
        registration.kind_id,
        registration.aspect_declarations,
        RegistrationDomain::Relation {
            topology_only: registration.payload_class == RelationPayloadClass::TopologyOnlyRelation,
        },
    )?;
    Ok(registration)
}

pub(crate) fn lower_aspect_plans(registry: &RelationalSchemaRegistry) -> AspectPlanCatalog {
    let entity_plans = registry
        .entity_kinds
        .iter()
        .map(|(kind_id, registration)| {
            (
                *kind_id,
                lower_kind_plan(*kind_id, &registration.aspect_declarations),
            )
        })
        .collect();
    let relation_plans = registry
        .relation_kinds
        .iter()
        .map(|(kind_id, registration)| {
            (
                *kind_id,
                lower_kind_plan(*kind_id, &registration.aspect_declarations),
            )
        })
        .collect();
    AspectPlanCatalog {
        entity_plans,
        relation_plans,
    }
}

fn canonicalize_declarations(
    kind_id: crate::identity::data::KindId,
    declarations: KindAspectDeclarations,
    domain: RegistrationDomain,
) -> Result<KindAspectDeclarations, SchemaRegistryError> {
    let mut seen = BTreeSet::new();
    let mut aspects = declarations.aspects;
    aspects.sort_by(|left, right| left.key.cmp(&right.key));
    for aspect in &aspects {
        if !seen.insert(aspect.key.clone()) {
            return Err(SchemaRegistryError::duplicate_aspect_key(
                kind_id,
                aspect.key.clone(),
            ));
        }
        validate_declared_aspect(kind_id, aspect, &domain)?;
    }
    let plan_revision = derive_plan_revision(&aspects);
    Ok(KindAspectDeclarations {
        plan_revision,
        aspects,
    })
}

fn validate_declared_aspect(
    kind_id: crate::identity::data::KindId,
    aspect: &DeclaredAspect,
    domain: &RegistrationDomain,
) -> Result<(), SchemaRegistryError> {
    validate_raw_interned_string(kind_id, "aspect key", &aspect.key.0)?;
    match &aspect.binding {
        AspectBinding::EntityPayloadField { field } => {
            if !matches!(domain, RegistrationDomain::Entity) {
                return Err(SchemaRegistryError::invalid_aspect_declaration(
                    kind_id,
                    "entity payload field bindings are valid only for entity kinds",
                ));
            }
            validate_raw_interned_string(kind_id, "entity payload field", field)?;
            match aspect.comparator {
                AspectComparator::JsonScalarEquality => {}
                _ => {
                    return Err(SchemaRegistryError::invalid_aspect_declaration(
                        kind_id,
                        "entity payload fields require JsonScalarEquality",
                    ));
                }
            }
        }
        AspectBinding::RelationPayloadField { field } => {
            let RegistrationDomain::Relation { topology_only } = domain else {
                return Err(SchemaRegistryError::invalid_aspect_declaration(
                    kind_id,
                    "relation payload field bindings are valid only for relation kinds",
                ));
            };
            validate_raw_interned_string(kind_id, "relation payload field", field)?;
            if *topology_only {
                return Err(SchemaRegistryError::invalid_aspect_declaration(
                    kind_id,
                    "topology-only relations cannot declare relation payload field aspects",
                ));
            }
            match aspect.comparator {
                AspectComparator::JsonScalarEquality => {}
                _ => {
                    return Err(SchemaRegistryError::invalid_aspect_declaration(
                        kind_id,
                        "relation payload fields require JsonScalarEquality",
                    ));
                }
            }
        }
        AspectBinding::RelationSourceEndpoint | AspectBinding::RelationTargetEndpoint => {
            if !matches!(domain, RegistrationDomain::Relation { .. }) {
                return Err(SchemaRegistryError::invalid_aspect_declaration(
                    kind_id,
                    "relation endpoint bindings are valid only for relation kinds",
                ));
            }
            match aspect.comparator {
                AspectComparator::EndpointIdentityEquality => {}
                _ => {
                    return Err(SchemaRegistryError::invalid_aspect_declaration(
                        kind_id,
                        "relation endpoint bindings require EndpointIdentityEquality",
                    ));
                }
            }
        }
        AspectBinding::LifecycleTransition => match aspect.comparator {
            AspectComparator::LifecycleTransitionEquality => {}
            _ => {
                return Err(SchemaRegistryError::invalid_aspect_declaration(
                    kind_id,
                    "lifecycle bindings require LifecycleTransitionEquality",
                ));
            }
        },
        AspectBinding::OpaqueWholePayload => {
            match aspect.comparator {
                AspectComparator::OpaquePayloadByteEquality => {}
                _ => {
                    return Err(SchemaRegistryError::invalid_aspect_declaration(
                        kind_id,
                        "opaque whole-payload bindings require OpaquePayloadByteEquality",
                    ));
                }
            }
            if aspect.precision != AspectPrecision::Opaque {
                return Err(SchemaRegistryError::invalid_aspect_declaration(
                    kind_id,
                    "opaque whole-payload bindings must declare Opaque precision",
                ));
            }
        }
    }
    Ok(())
}

fn validate_raw_interned_string(
    kind_id: crate::identity::data::KindId,
    field_name: &str,
    value: &InternedString,
) -> Result<(), SchemaRegistryError> {
    match value {
        InternedString::Raw(raw) if raw.trim().is_empty() => {
            Err(SchemaRegistryError::invalid_aspect_declaration(
                kind_id,
                format!("{field_name} must not be empty"),
            ))
        }
        InternedString::Raw(_) => Ok(()),
        InternedString::Symbol(_) => Err(SchemaRegistryError::invalid_aspect_declaration(
            kind_id,
            format!("{field_name} must use raw string declarations"),
        )),
    }
}

fn lower_kind_plan(
    kind_id: crate::identity::data::KindId,
    declarations: &KindAspectDeclarations,
) -> LoweredAspectPlan {
    let executable_bindings = declarations
        .aspects
        .iter()
        .map(lower_binding)
        .collect::<SmallVec<[LoweredAspectBinding; 8]>>();
    LoweredAspectPlan {
        kind_id,
        plan_revision: declarations.plan_revision,
        executable_bindings,
    }
}

fn lower_binding(aspect: &DeclaredAspect) -> LoweredAspectBinding {
    LoweredAspectBinding {
        aspect_key: aspect.key.clone(),
        extractor: match &aspect.binding {
            AspectBinding::EntityPayloadField { field } => {
                LoweredAspectExtractor::EntityJsonField {
                    field: field.clone(),
                }
            }
            AspectBinding::RelationPayloadField { field } => {
                LoweredAspectExtractor::RelationJsonField {
                    field: field.clone(),
                }
            }
            AspectBinding::RelationSourceEndpoint => LoweredAspectExtractor::RelationSourceEndpoint,
            AspectBinding::RelationTargetEndpoint => LoweredAspectExtractor::RelationTargetEndpoint,
            AspectBinding::LifecycleTransition => LoweredAspectExtractor::LifecycleTransition,
            AspectBinding::OpaqueWholePayload => LoweredAspectExtractor::OpaqueWholePayloadBytes,
        },
        comparator: match aspect.comparator {
            AspectComparator::JsonScalarEquality => LoweredAspectComparator::JsonScalarEquality,
            AspectComparator::EndpointIdentityEquality => {
                LoweredAspectComparator::EndpointIdentityEquality
            }
            AspectComparator::LifecycleTransitionEquality => {
                LoweredAspectComparator::LifecycleTransitionEquality
            }
            AspectComparator::OpaquePayloadByteEquality => {
                LoweredAspectComparator::OpaquePayloadByteEquality
            }
        },
        precision: aspect.precision,
    }
}

fn derive_plan_revision(aspects: &[DeclaredAspect]) -> AspectPlanRevision {
    const FNV_OFFSET: u128 = 0x6c62272e07bb014262b821756295c58d;
    const FNV_PRIME: u128 = 0x0000000001000000000000000000013B;

    fn mix_bytes(hash: &mut u128, bytes: &[u8]) {
        for byte in bytes {
            *hash ^= *byte as u128;
            *hash = hash.wrapping_mul(FNV_PRIME);
        }
        *hash ^= 0xff_u128;
        *hash = hash.wrapping_mul(FNV_PRIME);
    }

    fn mix_string(hash: &mut u128, value: &str) {
        mix_bytes(hash, value.as_bytes());
    }

    fn mix_interned_string(hash: &mut u128, value: &InternedString) {
        match value {
            InternedString::Raw(raw) => {
                mix_bytes(hash, b"raw");
                mix_string(hash, raw);
            }
            InternedString::Symbol(symbol) => {
                mix_bytes(hash, b"symbol");
                mix_bytes(hash, &symbol.0.to_le_bytes());
            }
        }
    }

    let mut hash = FNV_OFFSET;
    for aspect in aspects {
        mix_interned_string(&mut hash, &aspect.key.0);
        match &aspect.binding {
            AspectBinding::EntityPayloadField { field } => {
                mix_bytes(&mut hash, b"entity_field");
                mix_interned_string(&mut hash, field);
            }
            AspectBinding::RelationPayloadField { field } => {
                mix_bytes(&mut hash, b"relation_field");
                mix_interned_string(&mut hash, field);
            }
            AspectBinding::RelationSourceEndpoint => mix_bytes(&mut hash, b"source_endpoint"),
            AspectBinding::RelationTargetEndpoint => mix_bytes(&mut hash, b"target_endpoint"),
            AspectBinding::LifecycleTransition => mix_bytes(&mut hash, b"lifecycle"),
            AspectBinding::OpaqueWholePayload => mix_bytes(&mut hash, b"opaque"),
        }
        mix_bytes(
            &mut hash,
            match aspect.comparator {
                AspectComparator::JsonScalarEquality => b"json_scalar",
                AspectComparator::EndpointIdentityEquality => b"endpoint_identity",
                AspectComparator::LifecycleTransitionEquality => b"lifecycle_transition",
                AspectComparator::OpaquePayloadByteEquality => b"opaque_payload_bytes",
            },
        );
        mix_bytes(
            &mut hash,
            match aspect.precision {
                AspectPrecision::Structured => b"structured",
                AspectPrecision::Opaque => b"opaque",
            },
        );
    }
    AspectPlanRevision(hash)
}
