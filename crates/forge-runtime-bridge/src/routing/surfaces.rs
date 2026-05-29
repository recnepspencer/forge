use std::sync::Arc;

use forge_foundational::facade::AspectKey;

use crate::error::{
    BridgeErrorContext, BridgePatchCoordinate, BridgeRouteError, BridgeRouteErrorKind,
};
use crate::identity::{BridgeIdentity, TruthDeltaSurfaceIdentityTag};
use crate::input::envelope::{BridgeCommittedPatchEnvelope, BridgeCommittedPatchItem};
use crate::mapping::{FrozenAspectMappingRegistry, TruthDeltaSurfaceKind};

pub(crate) type TruthDeltaSurfaceIdentity = BridgeIdentity<TruthDeltaSurfaceIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TruthDeltaSurface {
    surface_identity: TruthDeltaSurfaceIdentity,
    entity_identity: Arc<str>,
    aspect_key: AspectKey,
    surface_label: Arc<str>,
    surface_kind: TruthDeltaSurfaceKind,
}

impl TruthDeltaSurface {
    pub(crate) fn new(
        entity_identity: impl Into<Arc<str>>,
        aspect_key: AspectKey,
        surface_label: impl Into<Arc<str>>,
        surface_kind: TruthDeltaSurfaceKind,
    ) -> Self {
        let entity_identity = entity_identity.into();
        let surface_label = surface_label.into();
        let surface_identity = TruthDeltaSurfaceIdentity::new(format!(
            "{}:{}:{}:{}",
            entity_identity.as_ref(),
            aspect_key.as_str(),
            surface_label.as_ref(),
            canonical_truth_surface_kind_label(surface_kind)
        ));

        Self {
            surface_identity,
            entity_identity,
            aspect_key,
            surface_label,
            surface_kind,
        }
    }

    pub(crate) fn entity_identity(&self) -> &str {
        self.entity_identity.as_ref()
    }

    pub(crate) fn aspect_label(&self) -> &str {
        self.aspect_key.as_str()
    }

    pub(crate) fn surface_label(&self) -> &str {
        self.surface_label.as_ref()
    }

    pub(crate) fn surface_kind(&self) -> TruthDeltaSurfaceKind {
        self.surface_kind
    }

    pub(crate) fn surface_identity(&self) -> &TruthDeltaSurfaceIdentity {
        &self.surface_identity
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedTruthDeltaSurfaceSet {
    surfaces: Vec<TruthDeltaSurface>,
    item_surfaces: Vec<TruthDeltaSurface>,
}

impl NormalizedTruthDeltaSurfaceSet {
    fn new(surfaces: Vec<TruthDeltaSurface>, item_surfaces: Vec<TruthDeltaSurface>) -> Self {
        Self {
            surfaces,
            item_surfaces,
        }
    }

    pub(crate) fn item_surfaces(&self) -> &[TruthDeltaSurface] {
        &self.item_surfaces
    }

    pub fn len(&self) -> usize {
        self.surfaces.len()
    }
}

pub(crate) fn derive_normalized_truth_delta_surface_set(
    envelope: &BridgeCommittedPatchEnvelope,
    aspect_registry: &FrozenAspectMappingRegistry,
) -> Result<NormalizedTruthDeltaSurfaceSet, BridgeRouteError> {
    let mut item_surfaces = Vec::with_capacity(envelope.patch_body().canonical_items().len());
    for item in envelope.patch_body().canonical_items() {
        item_surfaces.push(derive_surface(item, aspect_registry)?);
    }

    let mut surfaces = item_surfaces.clone();
    surfaces.sort();
    surfaces.dedup();
    Ok(NormalizedTruthDeltaSurfaceSet::new(surfaces, item_surfaces))
}

pub(crate) fn truth_delta_surface_count(envelope: &BridgeCommittedPatchEnvelope) -> usize {
    envelope.patch_body().canonical_items().len()
}

fn derive_surface(
    item: &BridgeCommittedPatchItem,
    aspect_registry: &FrozenAspectMappingRegistry,
) -> Result<TruthDeltaSurface, BridgeRouteError> {
    let (surface_kind, normalized_surface_label) = classify_surface_label(item.surface_label())
        .map_err(|error| {
            error.with_context(BridgeErrorContext::routing(BridgePatchCoordinate::new(
                item.entity_identity(),
                item.aspect_label(),
                item.surface_label(),
            )))
        })?;

    if let Some(registration) = aspect_registry.lookup(
        item.entity_identity(),
        item.aspect_label(),
        &normalized_surface_label,
        surface_kind,
    ) {
        return Ok(TruthDeltaSurface::new(
            item.entity_identity(),
            item.aspect_key().clone(),
            normalized_surface_label,
            registration.truth_surface_kind(),
        ));
    }

    Ok(TruthDeltaSurface::new(
        item.entity_identity(),
        item.aspect_key().clone(),
        normalized_surface_label,
        surface_kind,
    ))
}

fn classify_surface_label(
    surface_label: &str,
) -> Result<(TruthDeltaSurfaceKind, Arc<str>), BridgeRouteError> {
    let Some((prefix, suffix)) = surface_label.split_once(':') else {
        return Ok((TruthDeltaSurfaceKind::EntityField, Arc::from(surface_label)));
    };

    if suffix.trim().is_empty() {
        return Err(BridgeRouteError::new(
            BridgeRouteErrorKind::UnsupportedTruthDeltaSurface,
            format!(
                "Committed patch surface label `{surface_label}` used a fine-grained prefix without a concrete surface name."
            ),
        ));
    }

    let surface_kind = match prefix {
        "field" => TruthDeltaSurfaceKind::EntityField,
        "relation-endpoint" => TruthDeltaSurfaceKind::EntityRelationEndpoint,
        "region" => TruthDeltaSurfaceKind::EntityRegion,
        "partition" => TruthDeltaSurfaceKind::EntityPartition,
        "facet" => TruthDeltaSurfaceKind::EntityFacet,
        _ => {
            return Err(BridgeRouteError::new(
                BridgeRouteErrorKind::UnsupportedTruthDeltaSurface,
                format!(
                    "Committed patch surface label `{surface_label}` used unsupported fine-grained surface prefix `{prefix}`."
                ),
            ));
        }
    };

    Ok((surface_kind, Arc::from(suffix)))
}

pub(crate) fn canonical_truth_surface_kind_label(
    surface_kind: TruthDeltaSurfaceKind,
) -> &'static str {
    match surface_kind {
        TruthDeltaSurfaceKind::EntityField => "entity-field",
        TruthDeltaSurfaceKind::EntityRelationEndpoint => "entity-relation-endpoint",
        TruthDeltaSurfaceKind::EntityRegion => "entity-region",
        TruthDeltaSurfaceKind::EntityPartition => "entity-partition",
        TruthDeltaSurfaceKind::EntityFacet => "entity-facet",
    }
}

impl FrozenAspectMappingRegistry {
    pub(crate) fn lookup(
        &self,
        entity_identity: &str,
        aspect_label: &str,
        surface_label: &str,
        surface_kind: TruthDeltaSurfaceKind,
    ) -> Option<&crate::mapping::aspects::FrozenAspectRegistration> {
        self.registrations().iter().find(|registration| {
            registration.truth_surface_kind() == surface_kind
                && registration
                    .truth_scope()
                    .entity_selector()
                    .matches(entity_identity)
                && registration
                    .truth_scope()
                    .aspect_selector()
                    .matches(aspect_label)
                && registration
                    .truth_scope()
                    .surface_selector()
                    .matches(surface_label)
        })
    }
}

#[cfg(test)]
mod tests {
    use forge_foundational::facade::AspectKey;

    use crate::error::BridgeRouteErrorKind;
    use crate::input::envelope::{
        BridgeCommittedPatchBody, BridgeCommittedPatchDigest, BridgeCommittedPatchEnvelope,
        BridgeCommittedPatchItem, BridgeCommittedPatchSummary, BridgeProducerMetadata,
        NormalizedBridgePatchEnvelope, TruthBranchIdentity, TruthCommitIdentity,
        TruthPatchIdentity,
    };
    use crate::mapping::{
        BridgeAspectRegistration, BridgeAspectRegistrationId, FrozenAspectMappingRegistry,
        MappingSelector, SliceFallbackPolicy, SubscriptionSliceKind, TruthDeltaSurfaceKind,
        TruthPatchScope,
    };
    use crate::snapshot::TruthSnapshotIdentity;

    use super::{derive_normalized_truth_delta_surface_set, truth_delta_surface_count};

    fn envelope(items: Vec<BridgeCommittedPatchItem>) -> BridgeCommittedPatchEnvelope {
        BridgeCommittedPatchEnvelope::from_normalized(NormalizedBridgePatchEnvelope::new(
            BridgeProducerMetadata::bridge_harness_fixture(),
            TruthCommitIdentity::new("commit-a"),
            TruthPatchIdentity::new("patch-a"),
            TruthSnapshotIdentity::new("snapshot-a"),
            TruthBranchIdentity::new("main"),
            BridgeCommittedPatchSummary::new(items.len(), items.len()),
            BridgeCommittedPatchBody::new(items),
            BridgeCommittedPatchDigest::new("digest-a"),
        ))
    }

    fn frozen_aspects(registrations: Vec<BridgeAspectRegistration>) -> FrozenAspectMappingRegistry {
        FrozenAspectMappingRegistry::freeze(registrations).expect("aspect registry should freeze")
    }

    #[test]
    fn derives_default_field_surface_without_prefix() {
        let normalized = derive_normalized_truth_delta_surface_set(
            &envelope(vec![BridgeCommittedPatchItem::new(
                "user",
                aspect_key("profile"),
                "name",
            )]),
            &FrozenAspectMappingRegistry::default(),
        )
        .expect("unprefixed surfaces should normalize as entity fields");

        assert_eq!(
            truth_delta_surface_count(&envelope(vec![BridgeCommittedPatchItem::new(
                "user",
                aspect_key("profile"),
                "name"
            )])),
            1
        );
        assert_eq!(normalized.len(), 1);
        let surface = &normalized.surfaces[0];
        assert_eq!(surface.surface_kind, TruthDeltaSurfaceKind::EntityField);
        assert_eq!(surface.surface_label.as_ref(), "name");
    }

    #[test]
    fn derives_prefixed_region_surface() {
        let normalized = derive_normalized_truth_delta_surface_set(
            &envelope(vec![BridgeCommittedPatchItem::new(
                "user",
                aspect_key("profile"),
                "region:viewport",
            )]),
            &FrozenAspectMappingRegistry::default(),
        )
        .expect("prefixed region surfaces should normalize");

        let surface = &normalized.surfaces[0];
        assert_eq!(surface.surface_kind, TruthDeltaSurfaceKind::EntityRegion);
        assert_eq!(surface.surface_label.as_ref(), "viewport");
    }

    #[test]
    fn rejects_unknown_surface_prefix() {
        let error = derive_normalized_truth_delta_surface_set(
            &envelope(vec![BridgeCommittedPatchItem::new(
                "user",
                aspect_key("profile"),
                "mystery:viewport",
            )]),
            &FrozenAspectMappingRegistry::default(),
        )
        .expect_err("unknown prefixes must fail explicitly");

        assert_eq!(
            error.kind(),
            BridgeRouteErrorKind::UnsupportedTruthDeltaSurface
        );
    }

    #[test]
    fn prefers_registered_surface_kind_when_scope_matches() {
        let registry = frozen_aspects(vec![BridgeAspectRegistration::new(
            BridgeAspectRegistrationId::new("profile-region"),
            TruthPatchScope::new(
                MappingSelector::exact("user"),
                MappingSelector::exact("profile"),
                MappingSelector::exact("viewport"),
            ),
            TruthDeltaSurfaceKind::EntityRegion,
            SubscriptionSliceKind::SignalRegion,
            SliceFallbackPolicy::Disallow,
        )]);

        let normalized = derive_normalized_truth_delta_surface_set(
            &envelope(vec![BridgeCommittedPatchItem::new(
                "user",
                aspect_key("profile"),
                "region:viewport",
            )]),
            &registry,
        )
        .expect("matching aspect registration should preserve the registered surface kind");

        assert_eq!(
            normalized.surfaces[0].surface_kind,
            TruthDeltaSurfaceKind::EntityRegion
        );
        assert_eq!(normalized.surfaces[0].surface_label.as_ref(), "viewport");
    }

    #[test]
    fn deduplicates_repeated_normalized_surfaces() {
        let normalized = derive_normalized_truth_delta_surface_set(
            &envelope(vec![
                BridgeCommittedPatchItem::new("user", aspect_key("profile"), "field:name"),
                BridgeCommittedPatchItem::new("user", aspect_key("profile"), "field:name"),
            ]),
            &FrozenAspectMappingRegistry::default(),
        )
        .expect("duplicate normalized surfaces should collapse");

        assert_eq!(normalized.len(), 1);
        assert!(normalized.surfaces[0]
            .surface_identity
            .as_str()
            .contains("name"));
    }

    fn aspect_key(value: &str) -> AspectKey {
        AspectKey::new(value).expect("valid bridge patch aspect key")
    }
}
