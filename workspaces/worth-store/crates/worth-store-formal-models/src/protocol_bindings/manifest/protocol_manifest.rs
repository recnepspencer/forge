use super::{OwnerBoundaryBinding, ProtocolFamily};
use crate::protocol_bindings::OwnerBoundaryGap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolBindingManifest {
    pub(super) bindings: Vec<OwnerBoundaryBinding>,
    pub(super) gaps: Vec<OwnerBoundaryGap>,
    pub(super) composed_protocols: Vec<ProtocolFamily>,
}

impl ProtocolBindingManifest {
    pub fn bindings(&self) -> impl Iterator<Item = OwnerBoundaryBinding> + '_ {
        self.bindings.iter().copied()
    }

    pub fn gaps(&self) -> impl Iterator<Item = OwnerBoundaryGap> + '_ {
        self.gaps.iter().copied()
    }

    pub fn composed_protocols(&self) -> impl Iterator<Item = ProtocolFamily> + '_ {
        self.composed_protocols.iter().copied()
    }
}
