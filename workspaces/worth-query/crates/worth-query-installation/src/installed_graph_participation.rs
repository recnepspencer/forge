use std::any::Any;
use std::sync::Arc;

use sha2::{Digest, Sha256};
use worth_proof::{
    Admitted, AssumptionBasis, AuthorityMarker, AuthorityWitness, CapabilityMarker,
    CapabilityWitness, CurrentValidity, FreshnessScopedBasis, Recipe, Unresolved,
};

use crate::generation::WorthQueryInstallationRuntimeIdentity;

/// Opaque installation proof for one volatile graph-participation provider.
///
/// The authority is minted from the same non-cloneable installation-runtime
/// identity that owns installed package authority and retains the provider
/// registration anchor. Labels remain descriptive fields of that authority;
/// they cannot be supplied later to reconstruct it.
pub struct WorthQueryInstalledGraphParticipationAuthority {
    recipe: InstalledGraphParticipationRecipe,
    provider_anchor: Arc<dyn Any + Send + Sync>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct InstalledGraphParticipationCandidate {
    runtime_ordinal: u64,
    role: String,
    provider_identity: String,
    authority_identity: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct InstalledGraphParticipationBasis {
    runtime_ordinal: u64,
    provider_anchor_identity: usize,
}

struct GraphParticipationPairingAuthority {
    _private: (),
}
impl AuthorityMarker for GraphParticipationPairingAuthority {}

struct CallableGraphProviderCapability {
    _private: (),
}
impl CapabilityMarker for CallableGraphProviderCapability {}

struct GraphParticipationInstallationAuthority {
    _private: (),
}
impl AuthorityMarker for GraphParticipationInstallationAuthority {}

type InstalledGraphParticipationRecipe = Recipe<
    Admitted,
    InstalledGraphParticipationCandidate,
    FreshnessScopedBasis<CurrentValidity, AssumptionBasis<InstalledGraphParticipationBasis>>,
>;

impl WorthQueryInstalledGraphParticipationAuthority {
    pub fn install<P: Any + Send + Sync>(
        runtime: &WorthQueryInstallationRuntimeIdentity,
        role: impl Into<String>,
        provider_identity: impl Into<String>,
        provider_anchor: Arc<P>,
    ) -> Result<Self, &'static str> {
        let role = role.into();
        let provider_identity = provider_identity.into();
        if role.trim().is_empty() || role.trim() != role {
            return Err("invalid-installed-graph-participation-role");
        }
        if provider_identity.trim().is_empty() || provider_identity.trim() != provider_identity {
            return Err("invalid-installed-graph-provider-identity");
        }
        let pointer = Arc::as_ptr(&provider_anchor) as *const () as usize;
        let mut hash = Sha256::new();
        hash.update(b"worth-query-installed-graph-participation-v1");
        hash.update(runtime.ordinal().to_le_bytes());
        hash.update(role.len().to_le_bytes());
        hash.update(role.as_bytes());
        hash.update(provider_identity.len().to_le_bytes());
        hash.update(provider_identity.as_bytes());
        hash.update(pointer.to_le_bytes());
        let candidate = InstalledGraphParticipationCandidate {
            runtime_ordinal: runtime.ordinal(),
            role,
            provider_identity,
            authority_identity: format!("{:x}", hash.finalize()),
        };
        let resolved = Recipe::<Unresolved, _>::new(candidate).resolve_with_authority(
            InstalledGraphParticipationBasis {
                runtime_ordinal: runtime.ordinal(),
                provider_anchor_identity: pointer,
            },
            AuthorityWitness::from_authority_marker(GraphParticipationPairingAuthority {
                _private: (),
            }),
        );
        let lowered = resolved.lower_with_capability(CapabilityWitness::from_capability_marker(
            CallableGraphProviderCapability { _private: () },
        ));
        let recipe = lowered.admit_with_authority(AuthorityWitness::from_authority_marker(
            GraphParticipationInstallationAuthority { _private: () },
        ));
        Ok(Self {
            recipe,
            provider_anchor,
        })
    }

    pub fn runtime_ordinal(&self) -> u64 {
        self.recipe.payload().runtime_ordinal
    }

    pub fn role(&self) -> &str {
        &self.recipe.payload().role
    }

    pub fn provider_identity(&self) -> &str {
        &self.recipe.payload().provider_identity
    }

    pub fn authority_identity(&self) -> &str {
        &self.recipe.payload().authority_identity
    }
}

impl std::fmt::Debug for WorthQueryInstalledGraphParticipationAuthority {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryInstalledGraphParticipationAuthority")
            .field("runtime_ordinal", &self.runtime_ordinal())
            .field("role", &self.role())
            .field("provider_identity", &self.provider_identity())
            .field("authority_identity", &self.authority_identity())
            .finish_non_exhaustive()
    }
}

impl PartialEq for WorthQueryInstalledGraphParticipationAuthority {
    fn eq(&self, other: &Self) -> bool {
        self.recipe.payload() == other.recipe.payload()
            && Arc::ptr_eq(&self.provider_anchor, &other.provider_anchor)
    }
}

impl Eq for WorthQueryInstalledGraphParticipationAuthority {}
