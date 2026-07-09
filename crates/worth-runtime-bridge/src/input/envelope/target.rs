use super::*;
use crate::mapping::TruthDeltaSurfaceKind;

mod native_basis;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeCommittedPatchTarget {
    aspect_locator: AspectLocator,
    field_locator: Option<AspectFieldLocator>,
    mutation_mask: AspectMask<MutationMask>,
    projection_mask: AspectMask<ProjectionMask>,
    surface_kind: TruthDeltaSurfaceKind,
}

impl BridgeCommittedPatchTarget {
    pub fn entity_field_path(
        aspect_locator: AspectLocator,
        field_path: CanonicalFieldPath,
    ) -> Self {
        Self::entity_field(AspectFieldLocator::from_aspect(aspect_locator, field_path))
    }

    pub fn entity_field(field_locator: AspectFieldLocator) -> Self {
        let field_path = field_locator.field_path().clone();
        Self {
            aspect_locator: field_locator.aspect().clone(),
            field_locator: Some(field_locator),
            mutation_mask: AspectMask::new([field_path.clone()]),
            projection_mask: AspectMask::new([field_path.clone()]),
            surface_kind: TruthDeltaSurfaceKind::EntityField,
        }
    }

    pub fn entity_relation_endpoint(aspect_locator: AspectLocator) -> Self {
        Self::whole_aspect_target(
            aspect_locator,
            TruthDeltaSurfaceKind::EntityRelationEndpoint,
        )
    }

    pub fn entity_region(aspect_locator: AspectLocator) -> Self {
        Self::whole_aspect_target(aspect_locator, TruthDeltaSurfaceKind::EntityRegion)
    }

    pub fn entity_partition(aspect_locator: AspectLocator) -> Self {
        Self::whole_aspect_target(aspect_locator, TruthDeltaSurfaceKind::EntityPartition)
    }

    pub fn entity_facet(aspect_locator: AspectLocator) -> Self {
        Self::whole_aspect_target(aspect_locator, TruthDeltaSurfaceKind::EntityFacet)
    }

    fn whole_aspect_target(
        aspect_locator: AspectLocator,
        surface_kind: TruthDeltaSurfaceKind,
    ) -> Self {
        debug_assert!(
            surface_kind != TruthDeltaSurfaceKind::EntityField,
            "field targets require an AspectFieldLocator"
        );
        Self {
            aspect_locator,
            field_locator: None,
            mutation_mask: AspectMask::whole_aspect(),
            projection_mask: AspectMask::whole_aspect(),
            surface_kind,
        }
    }

    pub(crate) fn from_admitted_target_shape(
        aspect_locator: AspectLocator,
        field_locator: Option<AspectFieldLocator>,
        projection_mask: &AspectMask<ProjectionMask>,
        surface_kind: TruthDeltaSurfaceKind,
    ) -> Self {
        let target = match (field_locator, surface_kind) {
            (Some(field_locator), TruthDeltaSurfaceKind::EntityField) => {
                Self::entity_field(field_locator)
            }
            (None, TruthDeltaSurfaceKind::EntityRelationEndpoint) => {
                Self::entity_relation_endpoint(aspect_locator)
            }
            (None, TruthDeltaSurfaceKind::EntityRegion) => Self::entity_region(aspect_locator),
            (None, TruthDeltaSurfaceKind::EntityPartition) => {
                Self::entity_partition(aspect_locator)
            }
            (None, TruthDeltaSurfaceKind::EntityFacet) => Self::entity_facet(aspect_locator),
            (Some(_), _) => {
                panic!("non-field committed patch targets cannot carry a field locator")
            }
            (None, TruthDeltaSurfaceKind::EntityField) => {
                panic!("field committed patch targets require a field locator")
            }
        };
        assert_eq!(
            target.projection_mask(),
            projection_mask,
            "admitted target shape projection mask must match foundational target law"
        );
        target
    }

    pub fn aspect_locator(&self) -> &AspectLocator {
        &self.aspect_locator
    }

    pub fn field_locator(&self) -> Option<&AspectFieldLocator> {
        self.field_locator.as_ref()
    }

    pub fn mutation_mask(&self) -> &AspectMask<MutationMask> {
        &self.mutation_mask
    }

    pub fn projection_mask(&self) -> &AspectMask<ProjectionMask> {
        &self.projection_mask
    }

    pub fn aspect_key(&self) -> &AspectKey {
        self.aspect_locator.aspect_key()
    }

    pub fn surface_kind(&self) -> TruthDeltaSurfaceKind {
        self.surface_kind
    }

    pub(crate) fn canonical_basis(&self) -> String {
        native_basis::committed_patch_target_canonical_basis(
            &self.aspect_locator,
            self.field_locator.as_ref(),
            &self.mutation_mask,
            &self.projection_mask,
            self.surface_kind,
        )
    }
}

pub(crate) fn committed_patch_surface_kind_label(
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

#[cfg(test)]
mod tests {
    use worth_foundational::facade::{
        AspectFieldLocator, AspectKey, AspectLocator, CanonicalFieldPath, FieldKey,
        LocatorAuthority,
    };

    use super::BridgeCommittedPatchTarget;
    use crate::mapping::TruthDeltaSurfaceKind;

    #[test]
    fn entity_field_target_preserves_foundational_field_locator_and_masks() {
        let field_locator = field_locator("profile", "name");
        let target = BridgeCommittedPatchTarget::entity_field(field_locator.clone());

        assert_eq!(target.field_locator(), Some(&field_locator));
        assert_eq!(
            target.canonical_basis(),
            "committed-patch-target|locator=version=bridge.committed-patch-target.v1;domain=locator;entries=[locus=named:aspect_field.aspect_key,kind=locator,value=exact-text:profile;locus=named:aspect_field.authority,kind=locator,value=exact-text:authoritative;locus=named:aspect_field.field_path,kind=locator,value=exact-text:name;locus=named:aspect_field.kind,kind=locator,value=exact-text:aspect]|mutation-mask=version=bridge.committed-patch-target.v1;domain=aspect-mask;entries=[locus=named:profile.mutation.field.name,kind=mask,value=exact-text:name]|projection-mask=version=bridge.committed-patch-target.v1;domain=aspect-mask;entries=[locus=named:profile.projection.field.name,kind=mask,value=exact-text:name]|kind=entity-field",
        );
        assert!(!target.mutation_mask().is_whole_aspect());
        assert!(!target.projection_mask().is_whole_aspect());
    }

    #[test]
    fn entity_region_target_uses_whole_aspect_masks() {
        let target = BridgeCommittedPatchTarget::entity_region(aspect_locator("profile"));

        assert_eq!(target.field_locator(), None);
        assert_eq!(
            target.canonical_basis(),
            "committed-patch-target|locator=version=bridge.committed-patch-target.v1;domain=locator;entries=[locus=named:aspect.aspect_key,kind=locator,value=exact-text:profile;locus=named:aspect.authority,kind=locator,value=exact-text:authoritative;locus=named:aspect.kind,kind=locator,value=exact-text:aspect]|mutation-mask=version=bridge.committed-patch-target.v1;domain=aspect-mask;entries=[locus=named:profile.mutation.whole,kind=mask,value=exact-text:whole]|projection-mask=version=bridge.committed-patch-target.v1;domain=aspect-mask;entries=[locus=named:profile.projection.whole,kind=mask,value=exact-text:whole]|kind=entity-region",
        );
        assert!(target.mutation_mask().is_whole_aspect());
        assert!(target.projection_mask().is_whole_aspect());
    }

    #[test]
    fn named_whole_aspect_targets_cover_full_native_matrix() {
        let cases = [
            (
                BridgeCommittedPatchTarget::entity_relation_endpoint(aspect_locator("profile")),
                TruthDeltaSurfaceKind::EntityRelationEndpoint,
                "entity-relation-endpoint",
            ),
            (
                BridgeCommittedPatchTarget::entity_region(aspect_locator("profile")),
                TruthDeltaSurfaceKind::EntityRegion,
                "entity-region",
            ),
            (
                BridgeCommittedPatchTarget::entity_partition(aspect_locator("profile")),
                TruthDeltaSurfaceKind::EntityPartition,
                "entity-partition",
            ),
            (
                BridgeCommittedPatchTarget::entity_facet(aspect_locator("profile")),
                TruthDeltaSurfaceKind::EntityFacet,
                "entity-facet",
            ),
        ];

        for (target, expected_kind, expected_label) in cases {
            assert_eq!(target.field_locator(), None);
            assert_eq!(target.surface_kind(), expected_kind);
            assert!(target.mutation_mask().is_whole_aspect());
            assert!(target.projection_mask().is_whole_aspect());
            assert!(target
                .canonical_basis()
                .contains(&format!("|kind={expected_label}")));
            assert!(target
                .canonical_basis()
                .contains("profile.mutation.whole,kind=mask,value=exact-text:whole"));
            assert!(target
                .canonical_basis()
                .contains("profile.projection.whole,kind=mask,value=exact-text:whole"));
        }
    }

    fn aspect_key(value: &str) -> AspectKey {
        AspectKey::new(value).expect("valid patch target aspect key")
    }

    fn aspect_locator(value: &str) -> AspectLocator {
        AspectLocator::new(LocatorAuthority::Authoritative, aspect_key(value))
    }

    fn field_key(value: &str) -> FieldKey {
        FieldKey::new(value.to_owned()).expect("valid foundational field key")
    }

    fn field_locator(aspect: &str, field: &str) -> AspectFieldLocator {
        AspectFieldLocator::from_aspect(
            aspect_locator(aspect),
            CanonicalFieldPath::single(field_key(field)),
        )
    }
}
