use crate::capability::AppearanceTokenId;

use super::{
    WorthUiAppearanceAcceptedRegistrationProof, WorthUiAppearanceTokenDescriptor,
    WorthUiAppearanceTokenKey,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenAppearanceCapabilities {
    descriptors: Vec<WorthUiAppearanceTokenDescriptor>,
}

impl FrozenAppearanceCapabilities {
    pub(crate) fn empty() -> Self {
        Self {
            descriptors: Vec::new(),
        }
    }

    pub(crate) fn from_accepted_descriptors(
        mut descriptors: Vec<WorthUiAppearanceTokenDescriptor>,
        accepted: &WorthUiAppearanceAcceptedRegistrationProof,
    ) -> Self {
        descriptors.retain(|descriptor| accepted.admits(descriptor));
        descriptors.sort_by(|left, right| left.id().cmp(right.id()));
        Self { descriptors }
    }

    pub fn len(&self) -> usize {
        self.descriptors.len()
    }

    pub fn is_empty(&self) -> bool {
        self.descriptors.is_empty()
    }

    pub fn entries(&self) -> &[WorthUiAppearanceTokenDescriptor] {
        &self.descriptors
    }

    pub fn get(&self, id: &AppearanceTokenId) -> Option<&WorthUiAppearanceTokenDescriptor> {
        self.descriptors
            .binary_search_by(|descriptor| descriptor.id().cmp(id))
            .ok()
            .map(|index| &self.descriptors[index])
    }

    pub(crate) fn digest_basis(&self) -> u64 {
        self.descriptors
            .iter()
            .fold(0x8422_5d1d_b77a_11e3, |basis, descriptor| {
                fold_bytes(
                    basis,
                    WorthUiAppearanceTokenKey::from_descriptor(descriptor)
                        .projection_basis()
                        .as_bytes(),
                )
            })
    }
}

fn fold_bytes(mut accumulator: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        accumulator ^= u64::from(*byte);
        accumulator = accumulator.wrapping_mul(0x0000_0100_0000_01b3);
    }
    accumulator
}
