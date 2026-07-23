use super::{
    PhysicalSignalAspectDeclaration, PhysicalSignalAspectRole, PhysicalSignalPolicySelection,
    PhysicalWorkCapacity, PHYSICAL_ASYNC_CAPABILITIES,
};
use sha2::{Digest, Sha256};

const PROFILE_DOMAIN: &[u8] = b"worth-store.physical-signal-profile.v2";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhysicalSignalProfileIdentity(pub(super) [u8; 32]);

impl PhysicalSignalProfileIdentity {
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

pub(super) fn profile_identity(
    aspects: &[PhysicalSignalAspectDeclaration],
    capacity: PhysicalWorkCapacity,
) -> PhysicalSignalProfileIdentity {
    let mut digest = Sha256::new();
    digest.update(PROFILE_DOMAIN);
    digest.update((PHYSICAL_ASYNC_CAPABILITIES.len() as u64).to_le_bytes());
    for capability in PHYSICAL_ASYNC_CAPABILITIES {
        digest.update([family_code(capability.family())]);
        digest.update(capability.contract_id().to_le_bytes());
        digest.update(capability.max_payload_bytes().to_le_bytes());
    }
    PhysicalSignalPolicySelection::update_profile_identity(&mut digest);
    digest.update((aspects.len() as u64).to_le_bytes());
    for aspect in aspects {
        update_aspect(&mut digest, aspect);
    }
    for value in [
        capacity.commands(),
        capacity.scope_members_per_work(),
        capacity.total_scope_members(),
        capacity.semantic_bytes_per_work(),
        capacity.total_semantic_bytes(),
    ] {
        digest.update((value as u128).to_le_bytes());
    }
    PhysicalSignalProfileIdentity(digest.finalize().into())
}

const fn family_code(family: super::PhysicalWorkSignalFamily) -> u8 {
    match family {
        super::PhysicalWorkSignalFamily::ReadFault => 1,
        super::PhysicalWorkSignalFamily::ExactWriteback => 2,
        super::PhysicalWorkSignalFamily::Publication => 3,
        super::PhysicalWorkSignalFamily::Lifecycle => 4,
    }
}

fn update_aspect(digest: &mut Sha256, aspect: &PhysicalSignalAspectDeclaration) {
    let contract = aspect.contract();
    digest.update((contract.aspect_key().as_str().len() as u64).to_le_bytes());
    digest.update(contract.aspect_key().as_str().as_bytes());
    digest.update(contract.binding_stamp().as_bytes());
    digest.update([match aspect.role() {
        PhysicalSignalAspectRole::Dependency => 1,
        PhysicalSignalAspectRole::Output => 2,
        PhysicalSignalAspectRole::DependencyAndOutput => 3,
    }]);
    digest.update([aspect.families().bits()]);
    match aspect.partition() {
        Some(partition) => {
            digest.update([1]);
            digest.update((partition.partition.0.len() as u64).to_le_bytes());
            digest.update(partition.partition.0.as_bytes());
            if let Some(detail) = partition.detail.as_deref() {
                digest.update([1]);
                digest.update((detail.len() as u64).to_le_bytes());
                digest.update(detail.as_bytes());
            } else {
                digest.update([0]);
            }
            digest.update([partition.match_mode as u8]);
        }
        None => digest.update([0]),
    }
}

#[cfg(test)]
mod tests {
    use super::PhysicalSignalProfileIdentity;
    use crate::physical_runtime::work::PhysicalWorkProfileDeclaration;

    #[test]
    fn empty_profile_identity_is_stable() {
        let left = PhysicalWorkProfileDeclaration::default().identity();
        let right = PhysicalWorkProfileDeclaration::default().identity();
        assert_eq!(left, right);
        assert_ne!(left, PhysicalSignalProfileIdentity([0; 32]));
    }
}
