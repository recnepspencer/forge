use crate::capability::DensityTokenId;

use super::{
    WorthUiDensityAcceptedRegistrationProof, WorthUiDensityTokenDescriptor, WorthUiDensityTokenKey,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenDensityCapabilities {
    descriptors: Vec<WorthUiDensityTokenDescriptor>,
}

impl FrozenDensityCapabilities {
    pub(crate) fn empty() -> Self {
        Self {
            descriptors: Vec::new(),
        }
    }

    pub(crate) fn from_accepted_descriptors(
        mut descriptors: Vec<WorthUiDensityTokenDescriptor>,
        accepted: &WorthUiDensityAcceptedRegistrationProof,
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

    pub fn entries(&self) -> &[WorthUiDensityTokenDescriptor] {
        &self.descriptors
    }

    pub fn get(&self, id: &DensityTokenId) -> Option<&WorthUiDensityTokenDescriptor> {
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
                    WorthUiDensityTokenKey::from_descriptor(descriptor)
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
