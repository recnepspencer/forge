use crate::capability::NativeCapabilityId;

use super::{
    AmbientHostCheck, NativeCapabilityFamily, NativePlatformPosture, NativeShellAuthorityClaim,
};

/// Declarative native adapter seam and its explicit platform support posture.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NativeCapabilityDescriptor {
    id: NativeCapabilityId,
    family: Option<NativeCapabilityFamily>,
    platform_posture: Option<NativePlatformPosture>,
    shell_authority_claims: Vec<NativeShellAuthorityClaim>,
    ambient_host_checks: Vec<AmbientHostCheck>,
}

impl NativeCapabilityDescriptor {
    pub fn new(id: NativeCapabilityId) -> Self {
        Self {
            id,
            family: None,
            platform_posture: None,
            shell_authority_claims: Vec::new(),
            ambient_host_checks: Vec::new(),
        }
    }

    pub fn with_family(mut self, family: NativeCapabilityFamily) -> Self {
        self.family = Some(family);
        self
    }

    pub fn with_platform_posture(mut self, platform_posture: NativePlatformPosture) -> Self {
        self.platform_posture = Some(platform_posture);
        self
    }

    pub fn with_shell_authority_claim_for_diagnostics(
        mut self,
        shell_authority_claim: NativeShellAuthorityClaim,
    ) -> Self {
        self.shell_authority_claims.push(shell_authority_claim);
        self
    }

    pub fn with_ambient_host_check_for_diagnostics(
        mut self,
        ambient_host_check: AmbientHostCheck,
    ) -> Self {
        self.ambient_host_checks.push(ambient_host_check);
        self
    }

    pub fn id(&self) -> &NativeCapabilityId {
        &self.id
    }

    pub fn family(&self) -> Option<&NativeCapabilityFamily> {
        self.family.as_ref()
    }

    pub fn platform_posture(&self) -> Option<NativePlatformPosture> {
        self.platform_posture
    }

    pub(crate) fn shell_authority_claims(&self) -> &[NativeShellAuthorityClaim] {
        &self.shell_authority_claims
    }

    pub(crate) fn ambient_host_checks(&self) -> &[AmbientHostCheck] {
        &self.ambient_host_checks
    }

    pub(crate) fn canonicalized_for_freeze(mut self) -> Self {
        self.shell_authority_claims.sort();
        self.shell_authority_claims.dedup();
        self.ambient_host_checks.sort();
        self.ambient_host_checks.dedup();
        self
    }
}
