use worth_foundational::facade::AspectBinding;

use crate::facade::RuntimeBridge;
use crate::mapping::aspects::FrozenAspectRegistration;
use crate::mapping::{
    AspectKeySelector, MappingSelector, SliceWideningPolicy, TruthDeltaSurfaceKind,
    TruthPatchTargetSelector,
};

use super::{
    BridgeCorrespondenceDenialKind, BridgeCorrespondencePrecision,
    BridgeSemanticDependencyCandidate, BridgeSignalAspectTargetDeclaration,
};

pub(super) fn mapping_for<'a>(
    runtime: &'a RuntimeBridge,
    declaration: &BridgeSignalAspectTargetDeclaration,
) -> Option<&'a FrozenAspectRegistration> {
    runtime
        .aspect_registry
        .by_id(&declaration.aspect_registration_id)
}

pub(super) fn admit_mapping(
    mapping: &FrozenAspectRegistration,
    dependency: &BridgeSemanticDependencyCandidate,
) -> Result<BridgeCorrespondencePrecision, BridgeCorrespondenceDenialKind> {
    let scope = mapping.truth_scope();
    let source_record_identity = dependency
        .source_record_identity
        .map(|identity| identity.bridge_entity_identity());
    let entity_matches = match (scope.entity_selector(), source_record_identity.as_deref()) {
        (MappingSelector::Any, _) => true,
        (MappingSelector::Exact(expected), Some(actual)) => expected.as_ref() == actual,
        (MappingSelector::Exact(_), None) => false,
    };
    let aspect_matches = match scope.aspect_selector() {
        AspectKeySelector::Any => true,
        AspectKeySelector::Exact(key) => key == dependency.contract.key(),
    };
    if !entity_matches || !aspect_matches || !target_matches(scope.target_selector(), dependency) {
        return Err(BridgeCorrespondenceDenialKind::MappingSemanticMismatch);
    }
    if mapping.snapshot_read_contract().aspect_contract() != &dependency.contract
        || Some(mapping.truth_surface_kind()) != dependency_surface(dependency)
    {
        return Err(BridgeCorrespondenceDenialKind::MappingSemanticMismatch);
    }
    use crate::mapping::BridgeMappingWideningClass as Class;
    let widening_class = widening_class_for(scope, dependency);
    let admitted = match (mapping.widening_policy(), widening_class) {
        (SliceWideningPolicy::Disallow, None) => return Ok(BridgeCorrespondencePrecision::Exact),
        (SliceWideningPolicy::RegisteredEntityCoarseWidening, Some(Class::Entity))
        | (SliceWideningPolicy::RegisteredAspectCoarseWidening, None)
        | (SliceWideningPolicy::RegisteredSurfaceCoarseWidening, Some(Class::Surface)) => true,
        (SliceWideningPolicy::RegisteredPartitionWidening, None) => matches!(
            dependency.locality,
            super::BridgeSemanticLocality::SourcePartition(_)
        ),
        _ => false,
    };
    admitted
        .then_some(BridgeCorrespondencePrecision::DeclaredWidening)
        .ok_or(BridgeCorrespondenceDenialKind::MappingSemanticMismatch)
}

pub(crate) fn unique_mapping_id_for_dependency(
    runtime: &RuntimeBridge,
    dependency: &BridgeSemanticDependencyCandidate,
) -> Result<crate::mapping::BridgeAspectRegistrationId, BridgeCorrespondenceDenialKind> {
    let mut matches = runtime
        .aspect_registry
        .registrations()
        .iter()
        .filter(|mapping| admit_mapping(mapping, dependency).is_ok());
    let first = matches
        .next()
        .ok_or(BridgeCorrespondenceDenialKind::MissingMapping)?;
    if matches.next().is_some() {
        return Err(BridgeCorrespondenceDenialKind::AmbiguousMapping);
    }
    Ok(first.registration_id().clone())
}

fn widening_class_for(
    scope: &crate::mapping::TruthPatchScope,
    dependency: &BridgeSemanticDependencyCandidate,
) -> Option<crate::mapping::BridgeMappingWideningClass> {
    use crate::mapping::BridgeMappingWideningClass as Class;

    // An entity wildcard broadens only a record-scoped dependency. Partition-
    // and graph-scoped truth has no entity identity to discard.
    let entity = matches!(scope.entity_selector(), MappingSelector::Any)
        && matches!(
            dependency.locality,
            super::BridgeSemanticLocality::SourceRecord
                | super::BridgeSemanticLocality::ManagedSourceRecord
        );
    let aspect = matches!(scope.aspect_selector(), AspectKeySelector::Any);
    let surface = matches!(scope.target_selector(), TruthPatchTargetSelector::Any);
    match (entity, aspect, surface) {
        (false, false, false) => None,
        (true, false, false) => Some(Class::Entity),
        (false, true, false) => Some(Class::Aspect),
        (false, false, true) => Some(Class::Surface),
        (true, true, false) => Some(Class::EntityAspect),
        (true, false, true) => Some(Class::EntitySurface),
        (false, true, true) => Some(Class::AspectSurface),
        (true, true, true) => Some(Class::EntityAspectSurface),
    }
}

fn target_matches(
    selector: &TruthPatchTargetSelector,
    dependency: &BridgeSemanticDependencyCandidate,
) -> bool {
    match selector {
        TruthPatchTargetSelector::Any => true,
        TruthPatchTargetSelector::AuthoritativeAspect => {
            dependency.projection_mask.is_whole_aspect()
        }
        TruthPatchTargetSelector::EntityField(path) => {
            (dependency.projection_mask.paths() == std::slice::from_ref(path)
                || scalar_field_binding_matches(path, dependency))
                && matches!(
                    dependency.binding,
                    AspectBinding::EntityField { .. } | AspectBinding::RelationField { .. }
                )
        }
        TruthPatchTargetSelector::EntityRelationEndpoint => matches!(
            dependency.binding,
            AspectBinding::RelationSourceEndpoint | AspectBinding::RelationTargetEndpoint
        ),
        TruthPatchTargetSelector::EntityRegion => {
            matches!(dependency.binding, AspectBinding::StructuralRegion)
        }
        TruthPatchTargetSelector::EntityPartition => {
            matches!(dependency.binding, AspectBinding::StructuralPartition)
        }
        TruthPatchTargetSelector::EntityFacet => {
            matches!(dependency.binding, AspectBinding::StructuralFacet)
        }
        TruthPatchTargetSelector::LifecycleTransition => {
            matches!(dependency.binding, AspectBinding::LifecycleTransition)
        }
    }
}

fn scalar_field_binding_matches(
    path: &worth_foundational::facade::CanonicalFieldPath,
    dependency: &BridgeSemanticDependencyCandidate,
) -> bool {
    dependency.projection_mask.is_whole_aspect()
        && matches!(
            dependency.contract.shape(),
            worth_foundational::facade::AspectShape::Scalar(_)
        )
        && matches!(
            &dependency.binding,
            AspectBinding::EntityField { field } | AspectBinding::RelationField { field }
                if path.fields() == std::slice::from_ref(field)
        )
}

fn dependency_surface(
    dependency: &BridgeSemanticDependencyCandidate,
) -> Option<TruthDeltaSurfaceKind> {
    match dependency.binding {
        AspectBinding::EntityField { .. } | AspectBinding::RelationField { .. } => Some(
            if dependency.projection_mask.is_whole_aspect()
                && (!matches!(
                    dependency.contract.shape(),
                    worth_foundational::facade::AspectShape::Scalar(_)
                ) || !matches!(
                    dependency.locality,
                    super::BridgeSemanticLocality::ManagedSourceRecord
                ))
            {
                TruthDeltaSurfaceKind::AuthoritativeAspect
            } else {
                TruthDeltaSurfaceKind::EntityField
            },
        ),
        AspectBinding::RelationSourceEndpoint | AspectBinding::RelationTargetEndpoint => {
            Some(TruthDeltaSurfaceKind::EntityRelationEndpoint)
        }
        AspectBinding::StructuralRegion => Some(TruthDeltaSurfaceKind::EntityRegion),
        AspectBinding::StructuralPartition => Some(TruthDeltaSurfaceKind::EntityPartition),
        AspectBinding::StructuralFacet => Some(TruthDeltaSurfaceKind::EntityFacet),
        AspectBinding::LifecycleTransition => Some(TruthDeltaSurfaceKind::LifecycleTransition),
        _ => None,
    }
}
