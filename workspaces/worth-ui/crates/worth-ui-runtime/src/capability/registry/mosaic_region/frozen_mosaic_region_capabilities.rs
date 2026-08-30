use crate::capability::MosaicRegionKindId;

use super::{MosaicRegionAcceptedRegistrationProof, MosaicRegionKindDescriptor};

/// Canonical frozen mosaic region kind capability index.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenMosaicRegionCapabilities {
    descriptors: Vec<MosaicRegionKindDescriptor>,
}

impl FrozenMosaicRegionCapabilities {
    #[cfg(test)]
    pub(crate) fn empty() -> Self {
        Self {
            descriptors: Vec::new(),
        }
    }

    pub(crate) fn from_accepted_descriptors(
        mut descriptors: Vec<MosaicRegionKindDescriptor>,
        accepted_regions: &MosaicRegionAcceptedRegistrationProof,
    ) -> Self {
        descriptors.retain(|descriptor| accepted_regions.admits(descriptor));
        descriptors.sort_by(|left, right| left.id().cmp(right.id()));
        Self { descriptors }
    }

    pub fn is_empty(&self) -> bool {
        self.descriptors.is_empty()
    }

    pub fn len(&self) -> usize {
        self.descriptors.len()
    }

    pub fn descriptors(&self) -> &[MosaicRegionKindDescriptor] {
        &self.descriptors
    }

    pub fn get(&self, id: &MosaicRegionKindId) -> Option<&MosaicRegionKindDescriptor> {
        self.descriptors
            .binary_search_by(|descriptor| descriptor.id().cmp(id))
            .ok()
            .map(|index| &self.descriptors[index])
    }

    pub(crate) fn runtime_service_support(&self) -> crate::capability::UiRuntimeServiceSupport {
        use crate::capability::{MosaicScrollOwnership as Ownership, UiRuntimeServiceFamily};

        let scroll_declared = self.descriptors.iter().any(|descriptor| {
            matches!(
                descriptor.scroll_ownership(),
                Some(Ownership::RegionOwned | Ownership::SurfaceOwned | Ownership::ViewportOwned)
            )
        });
        if scroll_declared {
            crate::capability::UiRuntimeServiceSupport::none_installed()
                .with_installed(UiRuntimeServiceFamily::Scroll)
        } else {
            crate::capability::UiRuntimeServiceSupport::none_installed()
        }
    }

    pub(crate) fn digest_basis(&self) -> u64 {
        self.descriptors
            .iter()
            .fold(0xd46a_193f_70c5_8ab1, fold_mosaic_region_descriptor)
    }
}

fn fold_mosaic_region_descriptor(accumulator: u64, descriptor: &MosaicRegionKindDescriptor) -> u64 {
    let with_id = fold_bytes(accumulator, descriptor.id().as_str().as_bytes());
    let with_role = fold_bytes(with_id, descriptor.role().digest_basis().as_bytes());
    let with_sizing = fold_optional_str(
        with_role,
        descriptor
            .sizing_behavior()
            .map(|sizing_behavior| sizing_behavior.digest_basis()),
    );
    let with_scroll = fold_optional_str(
        with_sizing,
        descriptor
            .scroll_ownership()
            .map(|scroll_ownership| scroll_ownership.digest_basis()),
    );
    let with_focus = fold_optional_str(
        with_scroll,
        descriptor
            .focus_scope()
            .map(|focus_scope| focus_scope.digest_basis()),
    );
    let with_child = fold_optional_str(
        with_focus,
        descriptor
            .child_rule()
            .map(|child_rule| child_rule.digest_basis()),
    );
    let with_surface_classes = descriptor.allowed_surface_classes().iter().fold(
        fold_bytes(with_child, b"allowed_surface_classes"),
        |accumulator, surface_class| fold_list_item(accumulator, &surface_class.digest_basis()),
    );
    let with_persistence = fold_optional_str(
        with_surface_classes,
        descriptor
            .persistence()
            .map(|persistence| persistence.digest_basis()),
    );
    let with_clipping = fold_optional_str(
        with_persistence,
        descriptor
            .clipping()
            .map(|clipping| clipping.digest_basis()),
    );
    let with_hit_test = fold_optional_str(
        with_clipping,
        descriptor
            .hit_test()
            .map(|hit_test| hit_test.digest_basis()),
    );
    fold_optional_str(with_hit_test, descriptor.label())
}

fn fold_list_item(accumulator: u64, value: &str) -> u64 {
    fold_bytes(fold_bytes(accumulator, b"item"), value.as_bytes())
}

fn fold_optional_str(accumulator: u64, value: Option<&str>) -> u64 {
    match value {
        Some(value) => fold_bytes(fold_bytes(accumulator, b"some"), value.as_bytes()),
        None => fold_bytes(accumulator, b"none"),
    }
}

fn fold_bytes(mut accumulator: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        accumulator ^= u64::from(*byte);
        accumulator = accumulator.wrapping_mul(0x0000_0100_0000_01b3);
    }
    accumulator
}
