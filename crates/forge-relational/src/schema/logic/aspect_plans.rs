use std::collections::BTreeSet;

use smallvec::SmallVec;

use crate::schema::data::{
    AspectBinding, AspectComparator, AspectPlanCatalog, AspectPlanRevision, AspectPrecision,
    CardinalityContractDeclaration, DeclaredAspect, EndpointDeletionIntegrityDeclaration,
    EndpointKindContractDeclaration, EntityKindRegistration, KindAspectDeclarations,
    LoweredAspectBinding, LoweredAspectComparator, LoweredAspectExtractor, LoweredAspectPlan,
    LoweredCardinalityContract, LoweredEndpointDeletionIntegrityContract,
    LoweredEndpointKindContract, LoweredRelationIntegrityPlan, LoweredSymmetryContract,
    LoweredUniquenessContract, RelationIntegrityDeclarations, RelationIntegrityPlanCatalog,
    RelationIntegrityPlanRevision, RelationKindRegistration, RelationPayloadClass,
    RelationalSchemaRegistry, SchemaRegistryError, SymmetryContractDeclaration,
    UniquenessContractDeclaration,
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
    registration.relation_integrity =
        canonicalize_relation_integrity(registration.kind_id, registration.relation_integrity)?;
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

pub(crate) fn lower_relation_integrity_plans(
    registry: &RelationalSchemaRegistry,
) -> RelationIntegrityPlanCatalog {
    let relation_plans = registry
        .relation_kinds
        .iter()
        .map(|(kind_id, registration)| {
            (
                *kind_id,
                lower_relation_integrity_plan(*kind_id, &registration.relation_integrity),
            )
        })
        .collect();
    RelationIntegrityPlanCatalog { relation_plans }
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

fn canonicalize_relation_integrity(
    kind_id: crate::identity::data::KindId,
    declarations: RelationIntegrityDeclarations,
) -> Result<RelationIntegrityDeclarations, SchemaRegistryError> {
    let mut seen_contract_ids = BTreeSet::new();
    let mut endpoint_kind_contracts = declarations.endpoint_kind_contracts;
    let mut cardinality_contracts = declarations.cardinality_contracts;
    let mut uniqueness_contracts = declarations.uniqueness_contracts;
    let mut symmetry_contracts = declarations.symmetry_contracts;
    let mut endpoint_deletion_integrity_contracts =
        declarations.endpoint_deletion_integrity_contracts;

    endpoint_kind_contracts.sort_by(|left, right| left.contract_id.cmp(&right.contract_id));
    cardinality_contracts.sort_by(|left, right| left.contract_id.cmp(&right.contract_id));
    uniqueness_contracts.sort_by(|left, right| left.contract_id.cmp(&right.contract_id));
    symmetry_contracts.sort_by(|left, right| left.contract_id.cmp(&right.contract_id));
    endpoint_deletion_integrity_contracts
        .sort_by(|left, right| left.contract_id.cmp(&right.contract_id));

    for declaration in &endpoint_kind_contracts {
        validate_contract_id(kind_id, &declaration.contract_id, &mut seen_contract_ids)?;
        validate_endpoint_kind_contract(kind_id, declaration)?;
    }
    for declaration in &cardinality_contracts {
        validate_contract_id(kind_id, &declaration.contract_id, &mut seen_contract_ids)?;
        validate_cardinality_contract(kind_id, declaration)?;
    }
    for declaration in &uniqueness_contracts {
        validate_contract_id(kind_id, &declaration.contract_id, &mut seen_contract_ids)?;
    }
    for declaration in &symmetry_contracts {
        validate_contract_id(kind_id, &declaration.contract_id, &mut seen_contract_ids)?;
    }
    for declaration in &endpoint_deletion_integrity_contracts {
        validate_contract_id(kind_id, &declaration.contract_id, &mut seen_contract_ids)?;
    }

    Ok(RelationIntegrityDeclarations {
        plan_revision: derive_relation_integrity_plan_revision(
            &endpoint_kind_contracts,
            &cardinality_contracts,
            &uniqueness_contracts,
            &symmetry_contracts,
            &endpoint_deletion_integrity_contracts,
        ),
        endpoint_kind_contracts,
        cardinality_contracts,
        uniqueness_contracts,
        symmetry_contracts,
        endpoint_deletion_integrity_contracts,
    })
}

fn validate_contract_id(
    kind_id: crate::identity::data::KindId,
    contract_id: &str,
    seen_contract_ids: &mut BTreeSet<String>,
) -> Result<(), SchemaRegistryError> {
    if contract_id.trim().is_empty() {
        return Err(SchemaRegistryError::invalid_relation_integrity_declaration(
            kind_id,
            "relation contract id must not be empty",
        ));
    }
    if !seen_contract_ids.insert(contract_id.to_string()) {
        return Err(SchemaRegistryError::duplicate_relation_contract_id(
            kind_id,
            contract_id,
        ));
    }
    Ok(())
}

fn validate_endpoint_kind_contract(
    kind_id: crate::identity::data::KindId,
    declaration: &EndpointKindContractDeclaration,
) -> Result<(), SchemaRegistryError> {
    if declaration.allowed_source_kinds.is_empty() {
        return Err(SchemaRegistryError::invalid_relation_integrity_declaration(
            kind_id,
            format!(
                "endpoint kind contract '{}' must declare at least one allowed source kind",
                declaration.contract_id
            ),
        ));
    }
    if declaration.allowed_target_kinds.is_empty() {
        return Err(SchemaRegistryError::invalid_relation_integrity_declaration(
            kind_id,
            format!(
                "endpoint kind contract '{}' must declare at least one allowed target kind",
                declaration.contract_id
            ),
        ));
    }
    Ok(())
}

fn validate_cardinality_contract(
    kind_id: crate::identity::data::KindId,
    declaration: &CardinalityContractDeclaration,
) -> Result<(), SchemaRegistryError> {
    if declaration.source_max.is_none()
        && declaration.target_max.is_none()
        && declaration.pair_max.is_none()
    {
        return Err(SchemaRegistryError::invalid_relation_integrity_declaration(
            kind_id,
            format!(
                "cardinality contract '{}' must declare at least one bound",
                declaration.contract_id
            ),
        ));
    }
    if declaration.pair_max == Some(0) {
        return Err(SchemaRegistryError::invalid_relation_integrity_declaration(
            kind_id,
            format!(
                "cardinality contract '{}' must not declare pair_max = 0",
                declaration.contract_id
            ),
        ));
    }
    Ok(())
}

fn lower_relation_integrity_plan(
    kind_id: crate::identity::data::KindId,
    declarations: &RelationIntegrityDeclarations,
) -> LoweredRelationIntegrityPlan {
    LoweredRelationIntegrityPlan {
        kind_id,
        plan_revision: declarations.plan_revision,
        endpoint_kind_contracts: declarations
            .endpoint_kind_contracts
            .iter()
            .map(|declaration| LoweredEndpointKindContract {
                contract_id: declaration.contract_id.clone(),
                relation_kind_id: kind_id,
                allowed_source_kinds: declaration.allowed_source_kinds.clone(),
                allowed_target_kinds: declaration.allowed_target_kinds.clone(),
                self_edges_allowed: declaration.self_edges_allowed,
                cross_context_policy: declaration.cross_context_policy,
                plan_revision: declarations.plan_revision,
            })
            .collect(),
        cardinality_contracts: declarations
            .cardinality_contracts
            .iter()
            .map(|declaration| LoweredCardinalityContract {
                contract_id: declaration.contract_id.clone(),
                relation_kind_id: kind_id,
                source_max: declaration.source_max,
                target_max: declaration.target_max,
                pair_max: declaration.pair_max,
                plan_revision: declarations.plan_revision,
            })
            .collect(),
        uniqueness_contracts: declarations
            .uniqueness_contracts
            .iter()
            .map(|declaration| LoweredUniquenessContract {
                contract_id: declaration.contract_id.clone(),
                relation_kind_id: kind_id,
                scope: declaration.scope,
                plan_revision: declarations.plan_revision,
            })
            .collect(),
        symmetry_contracts: declarations
            .symmetry_contracts
            .iter()
            .map(|declaration| LoweredSymmetryContract {
                contract_id: declaration.contract_id.clone(),
                relation_kind_id: kind_id,
                mode: declaration.mode,
                plan_revision: declarations.plan_revision,
            })
            .collect(),
        endpoint_deletion_integrity_contracts: declarations
            .endpoint_deletion_integrity_contracts
            .iter()
            .map(|declaration| LoweredEndpointDeletionIntegrityContract {
                contract_id: declaration.contract_id.clone(),
                relation_kind_id: kind_id,
                mode: declaration.mode,
                plan_revision: declarations.plan_revision,
            })
            .collect(),
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

fn derive_relation_integrity_plan_revision(
    endpoint_kind_contracts: &[EndpointKindContractDeclaration],
    cardinality_contracts: &[CardinalityContractDeclaration],
    uniqueness_contracts: &[UniquenessContractDeclaration],
    symmetry_contracts: &[SymmetryContractDeclaration],
    endpoint_deletion_integrity_contracts: &[EndpointDeletionIntegrityDeclaration],
) -> RelationIntegrityPlanRevision {
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

    fn mix_kind_ids(hash: &mut u128, kinds: &[crate::identity::data::KindId]) {
        for kind in kinds {
            mix_bytes(hash, &kind.0.to_le_bytes());
        }
    }

    let mut hash = FNV_OFFSET;
    for declaration in endpoint_kind_contracts {
        mix_bytes(&mut hash, b"endpoint_kind");
        mix_string(&mut hash, &declaration.contract_id);
        mix_kind_ids(&mut hash, &declaration.allowed_source_kinds);
        mix_kind_ids(&mut hash, &declaration.allowed_target_kinds);
        mix_bytes(&mut hash, &[u8::from(declaration.self_edges_allowed)]);
        mix_bytes(
            &mut hash,
            match declaration.cross_context_policy {
                crate::config::data::CrossContextPolicy::AllowExplicit => b"allow_explicit",
                crate::config::data::CrossContextPolicy::SchemaControlled => b"schema_controlled",
                crate::config::data::CrossContextPolicy::Forbid => b"forbid",
            },
        );
    }
    for declaration in cardinality_contracts {
        mix_bytes(&mut hash, b"cardinality");
        mix_string(&mut hash, &declaration.contract_id);
        mix_bytes(
            &mut hash,
            &declaration.source_max.unwrap_or(usize::MAX).to_le_bytes(),
        );
        mix_bytes(
            &mut hash,
            &declaration.target_max.unwrap_or(usize::MAX).to_le_bytes(),
        );
        mix_bytes(
            &mut hash,
            &declaration.pair_max.unwrap_or(usize::MAX).to_le_bytes(),
        );
    }
    for declaration in uniqueness_contracts {
        mix_bytes(&mut hash, b"uniqueness");
        mix_string(&mut hash, &declaration.contract_id);
        mix_bytes(
            &mut hash,
            match declaration.scope {
                crate::schema::data::UniquenessScope::DirectedSemanticEdge => b"directed",
                crate::schema::data::UniquenessScope::NormalizedSymmetricEdge => b"normalized",
            },
        );
    }
    for declaration in symmetry_contracts {
        mix_bytes(&mut hash, b"symmetry");
        mix_string(&mut hash, &declaration.contract_id);
        mix_bytes(
            &mut hash,
            match declaration.mode {
                crate::schema::data::SymmetryMode::CanonicalUndirected => b"canonical_undirected",
                crate::schema::data::SymmetryMode::PairedInverseRequired => {
                    b"paired_inverse_required"
                }
                crate::schema::data::SymmetryMode::InverseProhibited => b"inverse_prohibited",
                crate::schema::data::SymmetryMode::PairedTwinRequired => b"paired_twin_required",
            },
        );
    }
    for declaration in endpoint_deletion_integrity_contracts {
        mix_bytes(&mut hash, b"endpoint_delete");
        mix_string(&mut hash, &declaration.contract_id);
        mix_bytes(
            &mut hash,
            match declaration.mode {
                crate::schema::data::EndpointDeletionIntegrityMode::RejectDeleteWithLiveRelations => {
                    b"reject_delete_with_live_relations"
                }
                crate::schema::data::EndpointDeletionIntegrityMode::RequireRelationDeletionInSameCommit => {
                    b"require_relation_deletion_in_same_commit"
                }
                crate::schema::data::EndpointDeletionIntegrityMode::RequireRelationRetirement => {
                    b"require_relation_retirement"
                }
            },
        );
    }
    RelationIntegrityPlanRevision(hash)
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
