use super::NativeCapabilityDescriptor;

/// Stable key for native capability platform-support semantics.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NativeCapabilityKey {
    support_basis: String,
}

impl NativeCapabilityKey {
    pub(crate) fn from_descriptor(descriptor: &NativeCapabilityDescriptor) -> Self {
        Self {
            support_basis: support_basis(descriptor),
        }
    }

    pub fn support_basis(&self) -> &str {
        &self.support_basis
    }
}

fn support_basis(descriptor: &NativeCapabilityDescriptor) -> String {
    format!(
        "{}|{}|{}|{}|{}",
        length_prefixed(descriptor.id().as_str()),
        descriptor
            .family()
            .map(|family| length_prefixed(&family.digest_basis()))
            .unwrap_or_else(|| "none".to_owned()),
        descriptor
            .platform_posture()
            .map(|posture| posture.digest_basis())
            .unwrap_or("none"),
        shell_authority_claim_basis(descriptor),
        ambient_host_check_basis(descriptor)
    )
}

fn shell_authority_claim_basis(descriptor: &NativeCapabilityDescriptor) -> String {
    descriptor
        .shell_authority_claims()
        .iter()
        .map(|claim| length_prefixed(claim.digest_basis()))
        .collect::<Vec<_>>()
        .join("")
}

fn ambient_host_check_basis(descriptor: &NativeCapabilityDescriptor) -> String {
    descriptor
        .ambient_host_checks()
        .iter()
        .map(|check| length_prefixed(check.digest_basis()))
        .collect::<Vec<_>>()
        .join("")
}

fn length_prefixed(value: &str) -> String {
    format!("{}:{value}", value.len())
}
