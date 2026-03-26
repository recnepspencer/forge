use std::collections::BTreeSet;

use smallvec::SmallVec;

use crate::merge::data::{
    AspectMergePolicyDeclaration, AspectMergePolicyKind, IdentityBasisDeclaration,
    IdentityBasisKind, IdentityBasisScope,
};
use crate::schema::data::{
    derive_relation_integrity_plan_revision, AllowedCycleClass, AspectBinding, AspectComparator,
    AspectPlanCatalog, AspectPlanRevision, AspectPrecision, CardinalityContractDeclaration,
    ConnectivityMinimumEnforcement, ContractId, DeclaredAspect, DirectedTraversalKind,
    EndpointKindContractDeclaration, EntityKindRegistration, KindAspectDeclarations,
    LoweredAspectBinding, LoweredAspectPlan, LoweredCardinalityMaximumContract,
    LoweredCardinalityMinimumContract, LoweredEndpointDeletionIntegrityContract,
    LoweredEndpointKindContract, LoweredExecutableAspectBindingKind,
    LoweredPartitionIsolationContract, LoweredRelationIntegrityPlan, LoweredSymmetryContract,
    LoweredUniquenessContract, MinimumCardinalityEnforcement, PairMinimumSemantics,
    PartitionIsolationMode, PayloadFieldConstraintDeclaration, PayloadSchemaDeclaration,
    RelationIntegrityDeclarations, RelationIntegrityPlanCatalog, RelationKindRegistration,
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
                lower_relation_integrity_plan(
                    *kind_id,
                    &registration.relation_integrity,
                    registration.cascade_delete_policy,
                ),
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
    let mut identity_declarations =
        canonicalize_identity_declarations(kind_id, declarations.identity_declarations, &aspects, &domain)?;
    let merge_policy_declarations = canonicalize_merge_policy_declarations(
        kind_id,
        declarations.merge_policy_declarations,
        &aspects,
    )?;
    let payload_schema = declarations
        .payload_schema
        .map(|payload_schema| canonicalize_payload_schema(kind_id, payload_schema))
        .transpose()?;
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
    identity_declarations.sort();
    let plan_revision = derive_plan_revision(
        &aspects,
        &identity_declarations,
        &merge_policy_declarations,
    );
    Ok(KindAspectDeclarations {
        plan_revision,
        aspects,
        identity_declarations,
        merge_policy_declarations,
        payload_schema,
    })
}

fn canonicalize_identity_declarations(
    kind_id: crate::identity::data::KindId,
    mut declarations: Vec<IdentityBasisDeclaration>,
    aspects: &[DeclaredAspect],
    domain: &RegistrationDomain,
) -> Result<Vec<IdentityBasisDeclaration>, SchemaRegistryError> {
    if declarations.is_empty() {
        declarations = default_identity_declarations(kind_id, domain);
    }
    declarations.sort();
    let mut seen = BTreeSet::new();
    let aspect_keys = aspects
        .iter()
        .map(|aspect| aspect.key.clone())
        .collect::<BTreeSet<_>>();
    for declaration in &declarations {
        if !seen.insert(declaration.clone()) {
            return Err(SchemaRegistryError::invalid_aspect_declaration(
                kind_id,
                format!("duplicate identity basis declaration for scope {:?}", declaration.scope),
            ));
        }
        match &declaration.scope {
            IdentityBasisScope::EntityKind(scope_kind) => {
                if *scope_kind != kind_id || !matches!(domain, RegistrationDomain::Entity) {
                    return Err(SchemaRegistryError::invalid_aspect_declaration(
                        kind_id,
                        "entity identity declarations must target the registering entity kind",
                    ));
                }
            }
            IdentityBasisScope::RelationKind(scope_kind) => {
                if *scope_kind != kind_id || !matches!(domain, RegistrationDomain::Relation { .. }) {
                    return Err(SchemaRegistryError::invalid_aspect_declaration(
                        kind_id,
                        "relation identity declarations must target the registering relation kind",
                    ));
                }
            }
            IdentityBasisScope::AspectKey(aspect_key) => {
                if !aspect_keys.contains(aspect_key) {
                    return Err(SchemaRegistryError::invalid_aspect_declaration(
                        kind_id,
                        format!(
                            "identity basis declaration references undeclared aspect key {:?}",
                            aspect_key
                        ),
                    ));
                }
                if !matches!(declaration.basis, IdentityBasisKind::DeclaredKeySet(_)) {
                    return Err(SchemaRegistryError::invalid_aspect_declaration(
                        kind_id,
                        "aspect-key-scoped identity declarations must use DeclaredKeySet",
                    ));
                }
            }
        }
        if matches!(domain, RegistrationDomain::Relation { .. })
            && matches!(declaration.basis, IdentityBasisKind::LineageIdentity)
        {
            return Err(SchemaRegistryError::invalid_aspect_declaration(
                kind_id,
                "relation kinds cannot currently declare LineageIdentity",
            ));
        }
        if let IdentityBasisKind::DeclaredKeySet(keys) = &declaration.basis {
            if keys.is_empty() {
                return Err(SchemaRegistryError::invalid_aspect_declaration(
                    kind_id,
                    "DeclaredKeySet identity declarations must contain at least one aspect key",
                ));
            }
            for key in keys.iter() {
                if !aspect_keys.contains(key) {
                    return Err(SchemaRegistryError::invalid_aspect_declaration(
                        kind_id,
                        format!(
                            "DeclaredKeySet identity declaration references undeclared aspect key {:?}",
                            key
                        ),
                    ));
                }
            }
        }
        match &declaration.basis {
            IdentityBasisKind::StorageIdentity
            | IdentityBasisKind::LineageIdentity
            | IdentityBasisKind::DeclaredKeySet(_) => {}
            IdentityBasisKind::StructuralFingerprint => {
                return Err(SchemaRegistryError::invalid_aspect_declaration(
                    kind_id,
                    "merge planning does not yet support StructuralFingerprint identity declarations",
                ));
            }
            IdentityBasisKind::Custom(_) => {
                return Err(SchemaRegistryError::invalid_aspect_declaration(
                    kind_id,
                    "merge planning does not yet support custom identity declarations",
                ));
            }
        }
    }
    Ok(declarations)
}

fn default_identity_declarations(
    kind_id: crate::identity::data::KindId,
    domain: &RegistrationDomain,
) -> Vec<IdentityBasisDeclaration> {
    match domain {
        RegistrationDomain::Entity => vec![
            IdentityBasisDeclaration {
                scope: IdentityBasisScope::EntityKind(kind_id),
                basis: IdentityBasisKind::StorageIdentity,
            },
            IdentityBasisDeclaration {
                scope: IdentityBasisScope::EntityKind(kind_id),
                basis: IdentityBasisKind::LineageIdentity,
            },
        ],
        RegistrationDomain::Relation { .. } => vec![IdentityBasisDeclaration {
            scope: IdentityBasisScope::RelationKind(kind_id),
            basis: IdentityBasisKind::StorageIdentity,
        }],
    }
}

fn canonicalize_merge_policy_declarations(
    kind_id: crate::identity::data::KindId,
    mut declarations: Vec<AspectMergePolicyDeclaration>,
    aspects: &[DeclaredAspect],
) -> Result<Vec<AspectMergePolicyDeclaration>, SchemaRegistryError> {
    declarations.sort_by(|left, right| left.aspect_key.cmp(&right.aspect_key));
    let aspect_keys = aspects
        .iter()
        .map(|aspect| aspect.key.clone())
        .collect::<BTreeSet<_>>();
    let mut seen = BTreeSet::new();
    for declaration in &declarations {
        if !aspect_keys.contains(&declaration.aspect_key) {
            return Err(SchemaRegistryError::invalid_aspect_declaration(
                kind_id,
                format!(
                    "merge policy declaration references undeclared aspect key {:?}",
                    declaration.aspect_key
                ),
            ));
        }
        if !seen.insert(declaration.aspect_key.clone()) {
            return Err(SchemaRegistryError::invalid_aspect_declaration(
                kind_id,
                format!(
                    "duplicate merge policy declaration for aspect key {:?}",
                    declaration.aspect_key
                ),
            ));
        }
        match declaration.policy {
            AspectMergePolicyKind::FailOnConflict | AspectMergePolicyKind::PreferRicher => {}
            AspectMergePolicyKind::LastWriterWins => {
                return Err(SchemaRegistryError::invalid_aspect_declaration(
                    kind_id,
                    "merge planning does not yet support LastWriterWins declarations",
                ));
            }
            AspectMergePolicyKind::MonotonicCounter => {
                return Err(SchemaRegistryError::invalid_aspect_declaration(
                    kind_id,
                    "merge planning does not yet support MonotonicCounter declarations",
                ));
            }
            AspectMergePolicyKind::AdditiveSet => {
                return Err(SchemaRegistryError::invalid_aspect_declaration(
                    kind_id,
                    "merge planning does not yet support AdditiveSet declarations",
                ));
            }
            AspectMergePolicyKind::Custom(_) => {
                return Err(SchemaRegistryError::invalid_aspect_declaration(
                    kind_id,
                    "merge planning does not yet support custom merge policy declarations",
                ));
            }
        }
    }
    Ok(declarations)
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

fn canonicalize_payload_schema(
    kind_id: crate::identity::data::KindId,
    mut payload_schema: PayloadSchemaDeclaration,
) -> Result<PayloadSchemaDeclaration, SchemaRegistryError> {
    if payload_schema.contract_id.trim().is_empty() {
        return Err(SchemaRegistryError::invalid_aspect_declaration(
            kind_id,
            "payload schema contract id must not be empty",
        ));
    }
    payload_schema
        .field_constraints
        .sort_by(|left, right| left.field().cmp(right.field()).then(left.cmp(right)));
    for constraint in &payload_schema.field_constraints {
        if constraint.field().trim().is_empty() {
            return Err(SchemaRegistryError::invalid_aspect_declaration(
                kind_id,
                "payload schema field names must not be empty",
            ));
        }
        match constraint {
            PayloadFieldConstraintDeclaration::Required { .. }
            | PayloadFieldConstraintDeclaration::Type { .. } => {}
        }
    }
    Ok(payload_schema)
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
    let mut acyclicity_contracts = declarations.acyclicity_contracts;
    let mut partition_isolation_contracts = declarations.partition_isolation_contracts;
    let mut connectivity_minimum_contracts = declarations.connectivity_minimum_contracts;

    endpoint_kind_contracts.sort_by(|left, right| left.contract_id.cmp(&right.contract_id));
    cardinality_contracts.sort_by(|left, right| left.contract_id.cmp(&right.contract_id));
    uniqueness_contracts.sort_by(|left, right| left.contract_id.cmp(&right.contract_id));
    symmetry_contracts.sort_by(|left, right| left.contract_id.cmp(&right.contract_id));
    endpoint_deletion_integrity_contracts
        .sort_by(|left, right| left.contract_id.cmp(&right.contract_id));
    acyclicity_contracts.sort_by(|left, right| left.contract_id.cmp(&right.contract_id));
    partition_isolation_contracts.sort_by(|left, right| left.contract_id.cmp(&right.contract_id));
    connectivity_minimum_contracts.sort_by(|left, right| left.contract_id.cmp(&right.contract_id));
    for declaration in &mut endpoint_kind_contracts {
        declaration.allowed_source_kinds.sort();
        declaration.allowed_source_kinds.dedup();
        declaration.allowed_target_kinds.sort();
        declaration.allowed_target_kinds.dedup();
    }

    for declaration in &endpoint_kind_contracts {
        validate_contract_id(kind_id, &declaration.contract_id, &mut seen_contract_ids)?;
        validate_endpoint_kind_contract(kind_id, declaration)?;
    }
    for declaration in &cardinality_contracts {
        validate_contract_id(kind_id, &declaration.contract_id, &mut seen_contract_ids)?;
        validate_cardinality_contract(kind_id, declaration, &endpoint_kind_contracts)?;
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
    for declaration in &acyclicity_contracts {
        validate_contract_id(kind_id, &declaration.contract_id, &mut seen_contract_ids)?;
        match declaration.traversal_direction {
            DirectedTraversalKind::SourceToTarget => {}
        }
        match declaration.allowed_cycle_class {
            AllowedCycleClass::NoCycles => {}
        }
    }
    for declaration in &partition_isolation_contracts {
        validate_contract_id(kind_id, &declaration.contract_id, &mut seen_contract_ids)?;
        match declaration.isolation_mode {
            PartitionIsolationMode::SamePartitionEndpoints => {}
        }
    }
    for declaration in &mut connectivity_minimum_contracts {
        validate_contract_id(kind_id, &declaration.contract_id, &mut seen_contract_ids)?;
        declaration.source_kind_ids.sort();
        declaration.source_kind_ids.dedup();
        declaration.target_kind_ids.sort();
        declaration.target_kind_ids.dedup();
        if declaration.source_kind_ids.is_empty() || declaration.target_kind_ids.is_empty() {
            return Err(SchemaRegistryError::invalid_relation_integrity_declaration(
                kind_id,
                format!(
                    "connectivity minimum contract '{}' must declare source and target kind domains",
                    declaration.contract_id
                ),
            ));
        }
        if declaration.minimum_reachable_targets == 0 {
            return Err(SchemaRegistryError::invalid_relation_integrity_declaration(
                kind_id,
                format!(
                    "connectivity minimum contract '{}' must require at least one reachable target",
                    declaration.contract_id
                ),
            ));
        }
        match declaration.enforcement_boundary {
            ConnectivityMinimumEnforcement::SnapshotPublication => {}
        }
    }

    Ok(RelationIntegrityDeclarations {
        plan_revision: derive_relation_integrity_plan_revision(
            &endpoint_kind_contracts,
            &cardinality_contracts,
            &uniqueness_contracts,
            &symmetry_contracts,
            &endpoint_deletion_integrity_contracts,
            &acyclicity_contracts,
            &partition_isolation_contracts,
            &connectivity_minimum_contracts,
        ),
        endpoint_kind_contracts,
        cardinality_contracts,
        uniqueness_contracts,
        symmetry_contracts,
        endpoint_deletion_integrity_contracts,
        acyclicity_contracts,
        partition_isolation_contracts,
        connectivity_minimum_contracts,
    })
}

fn validate_contract_id(
    kind_id: crate::identity::data::KindId,
    contract_id: &ContractId,
    seen_contract_ids: &mut BTreeSet<ContractId>,
) -> Result<(), SchemaRegistryError> {
    if contract_id.trim().is_empty() {
        return Err(SchemaRegistryError::invalid_relation_integrity_declaration(
            kind_id,
            "relation contract id must not be empty",
        ));
    }
    if !seen_contract_ids.insert(contract_id.clone()) {
        return Err(SchemaRegistryError::duplicate_relation_contract_id(
            kind_id,
            contract_id.clone(),
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
    endpoint_kind_contracts: &[EndpointKindContractDeclaration],
) -> Result<(), SchemaRegistryError> {
    if declaration.source_max.is_none()
        && declaration.target_max.is_none()
        && declaration.pair_max.is_none()
        && declaration.source_min.is_none()
        && declaration.target_min.is_none()
        && declaration.pair_min.is_none()
    {
        return Err(SchemaRegistryError::invalid_relation_integrity_declaration(
            kind_id,
            format!(
                "cardinality contract '{}' must declare at least one bound",
                declaration.contract_id
            ),
        ));
    }
    if declaration.source_min == Some(0)
        || declaration.target_min == Some(0)
        || declaration.pair_min == Some(0)
    {
        return Err(SchemaRegistryError::invalid_relation_integrity_declaration(
            kind_id,
            format!(
                "cardinality contract '{}' minimums must be greater than zero",
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
    if declaration.pair_min.is_some() {
        match declaration.pair_min_semantics {
            PairMinimumSemantics::ObservedDirectedPairs => {}
        }
    }
    for (minimum, maximum, label) in [
        (declaration.source_min, declaration.source_max, "source"),
        (declaration.target_min, declaration.target_max, "target"),
        (declaration.pair_min, declaration.pair_max, "pair"),
    ] {
        if let (Some(minimum), Some(maximum)) = (minimum, maximum) {
            if minimum > maximum {
                return Err(SchemaRegistryError::invalid_relation_integrity_declaration(
                    kind_id,
                    format!(
                        "cardinality contract '{}' declares {label}_min > {label}_max",
                        declaration.contract_id
                    ),
                ));
            }
        }
    }
    match declaration.minimum_enforcement {
        MinimumCardinalityEnforcement::CommitBoundary
        | MinimumCardinalityEnforcement::CertificationBoundary => {}
    }
    if (declaration.source_min.is_some() || declaration.target_min.is_some())
        && endpoint_kind_contracts.is_empty()
    {
        return Err(SchemaRegistryError::invalid_relation_integrity_declaration(
            kind_id,
            format!(
                "cardinality contract '{}' requires endpoint kind contracts to define minimum candidate domains",
                declaration.contract_id
            ),
        ));
    }
    Ok(())
}

fn lower_relation_integrity_plan(
    kind_id: crate::identity::data::KindId,
    declarations: &RelationIntegrityDeclarations,
    cascade_delete_policy: crate::config::data::CascadeDeletePolicy,
) -> LoweredRelationIntegrityPlan {
    let candidate_source_kinds = declarations
        .endpoint_kind_contracts
        .iter()
        .flat_map(|declaration| declaration.allowed_source_kinds.iter().copied())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let candidate_target_kinds = declarations
        .endpoint_kind_contracts
        .iter()
        .flat_map(|declaration| declaration.allowed_target_kinds.iter().copied())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
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
        cardinality_maximum_contracts: declarations
            .cardinality_contracts
            .iter()
            .filter(|declaration| {
                declaration.source_max.is_some()
                    || declaration.target_max.is_some()
                    || declaration.pair_max.is_some()
            })
            .map(|declaration| LoweredCardinalityMaximumContract {
                contract_id: declaration.contract_id.clone(),
                relation_kind_id: kind_id,
                source_max: declaration.source_max,
                target_max: declaration.target_max,
                pair_max: declaration.pair_max,
                plan_revision: declarations.plan_revision,
            })
            .collect(),
        cardinality_minimum_contracts: declarations
            .cardinality_contracts
            .iter()
            .filter(|declaration| {
                declaration.source_min.is_some()
                    || declaration.target_min.is_some()
                    || declaration.pair_min.is_some()
            })
            .map(|declaration| LoweredCardinalityMinimumContract {
                contract_id: declaration.contract_id.clone(),
                relation_kind_id: kind_id,
                source_min: declaration.source_min,
                target_min: declaration.target_min,
                pair_min: declaration.pair_min,
                pair_min_semantics: declaration.pair_min_semantics,
                candidate_source_kinds: candidate_source_kinds.clone(),
                candidate_target_kinds: candidate_target_kinds.clone(),
                minimum_enforcement: declaration.minimum_enforcement,
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
                cascade_delete_policy,
                plan_revision: declarations.plan_revision,
            })
            .collect(),
        acyclicity_contracts: declarations
            .acyclicity_contracts
            .iter()
            .map(
                |declaration| crate::schema::data::LoweredAcyclicityContract {
                    contract_id: declaration.contract_id.clone(),
                    relation_kind_id: kind_id,
                    traversal_direction: declaration.traversal_direction,
                    allowed_cycle_class: declaration.allowed_cycle_class,
                    plan_revision: declarations.plan_revision,
                },
            )
            .collect(),
        partition_isolation_contracts: declarations
            .partition_isolation_contracts
            .iter()
            .map(|declaration| LoweredPartitionIsolationContract {
                contract_id: declaration.contract_id.clone(),
                relation_kind_id: kind_id,
                isolation_mode: declaration.isolation_mode,
                plan_revision: declarations.plan_revision,
            })
            .collect(),
        connectivity_minimum_contracts: declarations
            .connectivity_minimum_contracts
            .iter()
            .map(
                |declaration| crate::schema::data::LoweredConnectivityMinimumContract {
                    contract_id: declaration.contract_id.clone(),
                    source_kind_ids: declaration.source_kind_ids.clone(),
                    relation_kind_id: kind_id,
                    target_kind_ids: declaration.target_kind_ids.clone(),
                    minimum_reachable_targets: declaration.minimum_reachable_targets,
                    enforcement_boundary: declaration.enforcement_boundary,
                    plan_revision: declarations.plan_revision,
                },
            )
            .collect(),
    }
}

fn lower_binding(aspect: &DeclaredAspect) -> LoweredAspectBinding {
    LoweredAspectBinding {
        aspect_key: aspect.key.clone(),
        binding_kind: match (&aspect.binding, aspect.comparator) {
            (AspectBinding::EntityPayloadField { field }, AspectComparator::JsonScalarEquality) => {
                LoweredExecutableAspectBindingKind::EntityJsonScalarField {
                    field: field.clone(),
                }
            }
            (
                AspectBinding::RelationPayloadField { field },
                AspectComparator::JsonScalarEquality,
            ) => LoweredExecutableAspectBindingKind::RelationJsonScalarField {
                field: field.clone(),
            },
            (AspectBinding::RelationSourceEndpoint, AspectComparator::EndpointIdentityEquality) => {
                LoweredExecutableAspectBindingKind::RelationSourceEndpointIdentity
            }
            (AspectBinding::RelationTargetEndpoint, AspectComparator::EndpointIdentityEquality) => {
                LoweredExecutableAspectBindingKind::RelationTargetEndpointIdentity
            }
            (AspectBinding::LifecycleTransition, AspectComparator::LifecycleTransitionEquality) => {
                LoweredExecutableAspectBindingKind::LifecycleTransitionEquality
            }
            (AspectBinding::OpaqueWholePayload, AspectComparator::OpaquePayloadByteEquality) => {
                LoweredExecutableAspectBindingKind::OpaqueWholePayloadBytes
            }
            _ => unreachable!("aspect declarations are canonicalized before lowering"),
        },
        precision: aspect.precision,
    }
}

fn derive_plan_revision(
    aspects: &[DeclaredAspect],
    identity_declarations: &[IdentityBasisDeclaration],
    merge_policy_declarations: &[AspectMergePolicyDeclaration],
) -> AspectPlanRevision {
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
    for declaration in identity_declarations {
        match &declaration.scope {
            IdentityBasisScope::EntityKind(kind_id) => {
                mix_bytes(&mut hash, b"identity_scope_entity_kind");
                mix_bytes(&mut hash, &kind_id.0.to_le_bytes());
            }
            IdentityBasisScope::RelationKind(kind_id) => {
                mix_bytes(&mut hash, b"identity_scope_relation_kind");
                mix_bytes(&mut hash, &kind_id.0.to_le_bytes());
            }
            IdentityBasisScope::AspectKey(aspect_key) => {
                mix_bytes(&mut hash, b"identity_scope_aspect_key");
                mix_interned_string(&mut hash, &aspect_key.0);
            }
        }
        match &declaration.basis {
            IdentityBasisKind::StorageIdentity => mix_bytes(&mut hash, b"identity_basis_storage"),
            IdentityBasisKind::LineageIdentity => mix_bytes(&mut hash, b"identity_basis_lineage"),
            IdentityBasisKind::StructuralFingerprint => {
                mix_bytes(&mut hash, b"identity_basis_structural")
            }
            IdentityBasisKind::DeclaredKeySet(keys) => {
                mix_bytes(&mut hash, b"identity_basis_declared_key_set");
                for key in keys.iter() {
                    mix_interned_string(&mut hash, &key.0);
                }
            }
            IdentityBasisKind::Custom(custom) => {
                mix_bytes(&mut hash, b"identity_basis_custom");
                mix_bytes(&mut hash, custom.name.as_bytes());
                mix_bytes(&mut hash, &custom.semantic_version.to_le_bytes());
            }
        }
    }
    for declaration in merge_policy_declarations {
        mix_bytes(&mut hash, b"merge_policy_declaration");
        mix_interned_string(&mut hash, &declaration.aspect_key.0);
        match &declaration.policy {
            AspectMergePolicyKind::FailOnConflict => mix_bytes(&mut hash, b"merge_policy_fail"),
            AspectMergePolicyKind::LastWriterWins => mix_bytes(&mut hash, b"merge_policy_lww"),
            AspectMergePolicyKind::MonotonicCounter => {
                mix_bytes(&mut hash, b"merge_policy_monotonic_counter")
            }
            AspectMergePolicyKind::AdditiveSet => {
                mix_bytes(&mut hash, b"merge_policy_additive_set")
            }
            AspectMergePolicyKind::PreferRicher => {
                mix_bytes(&mut hash, b"merge_policy_prefer_richer")
            }
            AspectMergePolicyKind::Custom(custom) => {
                mix_bytes(&mut hash, b"merge_policy_custom");
                mix_bytes(&mut hash, custom.name.as_bytes());
                mix_bytes(&mut hash, &custom.semantic_version.to_le_bytes());
            }
        }
    }
    AspectPlanRevision(hash)
}
