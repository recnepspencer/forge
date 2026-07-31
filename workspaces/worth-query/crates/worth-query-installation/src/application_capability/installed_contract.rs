use std::{marker::PhantomData, sync::Arc};
use worth_foundational::facade::CanonicalDigestId;

use worth_query_declaration::facade::{
    application_capability::{ApplicationCapabilityRef, ErasedApplicationCapabilityContract},
    application_schema::{
        ApplicationOperationRef, ApplicationSchema, ApplicationSchemaBindingIdentity,
    },
};

use crate::{
    application_schema::WorthQueryInstalledApplicationSchema,
    installed_index::WorthQueryInstalledPackageAuthority,
};

use super::{
    authority_seal::verify_capability_authority_seal,
    registry::{ApplicationCapabilityRegistryKey, CompiledApplicationCapability},
    WorthQueryApplicationCapabilityInstallationDenial,
    WorthQueryApplicationCapabilityInstallationDenialKind, WorthQueryCapabilityCanonicalArtifact,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorthQueryInstalledApplicationCapabilityIdentity(CanonicalDigestId);

impl WorthQueryInstalledApplicationCapabilityIdentity {
    pub(super) const fn from_canonical(digest: CanonicalDigestId) -> Self {
        Self(digest)
    }

    pub const fn digest(&self) -> &CanonicalDigestId {
        &self.0
    }

    pub const fn bytes(&self) -> &[u8; 32] {
        self.0.bytes()
    }

    pub fn render_support_hex(&self) -> String {
        self.0.render_hex()
    }
}

/// Opaque installed authority for one exact application capability contract.
///
/// A descriptive capability reference cannot substitute for installed
/// authority:
///
/// ```compile_fail
/// use worth_query_installation::facade::{
///     ApplicationCapabilityRef, WorthQueryInstalledApplicationCapability,
/// };
/// struct Schema;
/// struct Capability;
/// struct Operation;
///
/// fn requires_installed(
///     _: WorthQueryInstalledApplicationCapability<Schema, Capability, Operation, ()>,
/// ) {}
///
/// requires_installed(
///     ApplicationCapabilityRef::<Schema, Capability>::from_schema_identifier("Capability"),
/// );
/// ```
pub struct WorthQueryInstalledApplicationCapability<Schema, Capability, Operation, Input> {
    binding_identity: ApplicationSchemaBindingIdentity,
    compiled: Arc<CompiledApplicationCapability>,
    lookup_evidence: WorthQueryCapabilityLookupEvidence,
    _marker: PhantomData<fn(Input) -> (Schema, Capability, Operation)>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryCapabilityLookupEvidence {
    registry_probes: usize,
    basis_preparations: usize,
    digest_derivations: usize,
    digest_text_materializations: usize,
}

impl WorthQueryCapabilityLookupEvidence {
    pub const fn registry_probes(self) -> usize {
        self.registry_probes
    }

    pub const fn basis_preparations(self) -> usize {
        self.basis_preparations
    }

    pub const fn digest_derivations(self) -> usize {
        self.digest_derivations
    }

    pub const fn digest_text_materializations(self) -> usize {
        self.digest_text_materializations
    }
}

pub(crate) type ApplicationCapabilityRegistry = std::collections::BTreeMap<
    ApplicationCapabilityRegistryKey,
    Arc<CompiledApplicationCapability>,
>;

impl<Schema, Capability, Operation, Input>
    WorthQueryInstalledApplicationCapability<Schema, Capability, Operation, Input>
{
    pub(crate) fn from_installed_schema(
        schema: &WorthQueryInstalledApplicationSchema<Schema>,
        capability: ApplicationCapabilityRef<Schema, Capability>,
        operation: ApplicationOperationRef<Schema, Operation, Input>,
    ) -> Result<Self, WorthQueryApplicationCapabilityInstallationDenial>
    where
        Schema: ApplicationSchema,
    {
        let key = ApplicationCapabilityRegistryKey::from_references(&capability, &operation);
        let compiled = schema
            .capability_registry
            .get(&key)
            .cloned()
            .ok_or_else(|| {
                let name_exists = schema
                    .capability_registry
                    .keys()
                    .any(|candidate| candidate.name() == capability.name());
                let kind = if name_exists {
                    WorthQueryApplicationCapabilityInstallationDenialKind::CapabilityMeaningChanged
                } else {
                    WorthQueryApplicationCapabilityInstallationDenialKind::CapabilityNotInstalled
                };
                denial(kind, capability.name())
            })?;
        Ok(Self {
            binding_identity: schema.binding_identity(),
            compiled,
            lookup_evidence: WorthQueryCapabilityLookupEvidence {
                registry_probes: 1,
                basis_preparations: 0,
                digest_derivations: 0,
                digest_text_materializations: 0,
            },
            _marker: PhantomData,
        })
    }

    pub fn binding_identity(&self) -> &ApplicationSchemaBindingIdentity {
        &self.binding_identity
    }

    pub fn canonical_basis(&self) -> &WorthQueryCapabilityCanonicalArtifact {
        self.compiled.canonical()
    }

    pub fn identity(&self) -> &WorthQueryInstalledApplicationCapabilityIdentity {
        self.compiled.identity()
    }

    pub fn authority_identity(&self) -> &str {
        self.compiled.authority_identity().as_str()
    }

    pub fn contract(&self) -> &ErasedApplicationCapabilityContract {
        self.compiled.contract()
    }

    pub const fn lookup_evidence(&self) -> WorthQueryCapabilityLookupEvidence {
        self.lookup_evidence
    }

    pub(crate) fn authority_matches(&self, package: &WorthQueryInstalledPackageAuthority) -> bool {
        verify_capability_authority_seal(
            self.compiled.authority_identity(),
            &package.authority_key,
            &self.binding_identity,
            self.compiled.identity().bytes(),
            self.compiled.contract(),
        )
    }
}

fn denial(
    kind: WorthQueryApplicationCapabilityInstallationDenialKind,
    subject: impl Into<String>,
) -> WorthQueryApplicationCapabilityInstallationDenial {
    WorthQueryApplicationCapabilityInstallationDenial::new(kind, subject)
}
