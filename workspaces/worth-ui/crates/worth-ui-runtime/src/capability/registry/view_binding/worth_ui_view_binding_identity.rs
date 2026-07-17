use super::{QueryDenialPresentation, ViewBindingDescriptor, ViewBindingFamily};

/// Canonical identity of one UI view-binding descriptor.
///
/// Query contributes its binding-owned definition identity, while UI-owned
/// presentation and visible-state semantics remain explicit UI inputs.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthUiViewBindingIdentity(u64);

impl WorthUiViewBindingIdentity {
    pub(crate) fn from_descriptor(descriptor: &ViewBindingDescriptor) -> Self {
        let mut identity = fold_bytes(0x776f_7274_6875_6976, descriptor.id().as_str().as_bytes());
        identity = fold_family(identity, descriptor.family());
        identity = fold(identity, descriptor.definition().digest().as_u64());
        for binding in descriptor.visible_state_bindings() {
            identity = fold_bytes(identity, binding.name().as_bytes());
        }
        identity = fold(
            identity,
            match descriptor.denial_presentation() {
                QueryDenialPresentation::Hidden => 1,
                QueryDenialPresentation::AdvisoryText => 2,
                QueryDenialPresentation::StructuredStatus => 3,
            },
        );
        Self(identity)
    }

    pub const fn as_u64(self) -> u64 {
        self.0
    }
}

fn fold_family(mut identity: u64, family: &ViewBindingFamily) -> u64 {
    let tag = match family {
        ViewBindingFamily::Collection => 1,
        ViewBindingFamily::Detail => 2,
        ViewBindingFamily::Grouped => 3,
        ViewBindingFamily::Relationship => 4,
        ViewBindingFamily::OrderedEvent => 5,
        ViewBindingFamily::Spatial => 6,
        ViewBindingFamily::CustomAdmitted(name) => {
            identity = fold_bytes(identity, name.as_bytes());
            7
        }
    };
    fold(identity, tag)
}

fn fold(identity: u64, value: u64) -> u64 {
    identity.rotate_left(13).wrapping_mul(0x100_0000_01b3) ^ value
}

fn fold_bytes(mut identity: u64, bytes: &[u8]) -> u64 {
    identity = fold(identity, bytes.len() as u64);
    for byte in bytes {
        identity ^= u64::from(*byte);
        identity = identity.wrapping_mul(0x100_0000_01b3);
    }
    identity
}
