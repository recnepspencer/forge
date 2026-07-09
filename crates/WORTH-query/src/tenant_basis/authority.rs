use crate::identity::hash_parts;

use super::{TenantBasisEpoch, TenantResolutionClass};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TenantBindingSnapshot {
    tenant_identity: String,
    truth_branch_identity: Option<String>,
    schema_basis_identity: Option<String>,
    resolution_class: TenantResolutionClass,
    epoch: TenantBasisEpoch,
    ambiguous: bool,
    hidden_filter: bool,
    digest: String,
}

impl TenantBindingSnapshot {
    pub fn synthetic_direct(
        tenant_identity: impl Into<String>,
        truth_branch_identity: impl Into<String>,
        schema_basis_identity: impl Into<String>,
        epoch: TenantBasisEpoch,
    ) -> Self {
        Self::synthetic(
            tenant_identity,
            Some(truth_branch_identity.into()),
            Some(schema_basis_identity.into()),
            TenantResolutionClass::DirectBinding,
            epoch,
            false,
            false,
        )
    }

    pub fn synthetic_cached(
        tenant_identity: impl Into<String>,
        truth_branch_identity: impl Into<String>,
        schema_basis_identity: impl Into<String>,
        epoch: TenantBasisEpoch,
    ) -> Self {
        Self::synthetic(
            tenant_identity,
            Some(truth_branch_identity.into()),
            Some(schema_basis_identity.into()),
            TenantResolutionClass::CachedBinding,
            epoch,
            false,
            false,
        )
    }

    pub fn synthetic_derived(tenant_identity: impl Into<String>, epoch: TenantBasisEpoch) -> Self {
        Self::synthetic(
            tenant_identity,
            None,
            None,
            TenantResolutionClass::DerivedBinding,
            epoch,
            false,
            false,
        )
    }

    pub fn synthetic_ambiguous(
        tenant_identity: impl Into<String>,
        epoch: TenantBasisEpoch,
    ) -> Self {
        Self::synthetic(
            tenant_identity,
            None,
            None,
            TenantResolutionClass::DirectBinding,
            epoch,
            true,
            false,
        )
    }

    pub fn synthetic_hidden_filter(
        tenant_identity: impl Into<String>,
        truth_branch_identity: impl Into<String>,
        schema_basis_identity: impl Into<String>,
        epoch: TenantBasisEpoch,
    ) -> Self {
        Self::synthetic(
            tenant_identity,
            Some(truth_branch_identity.into()),
            Some(schema_basis_identity.into()),
            TenantResolutionClass::DirectBinding,
            epoch,
            false,
            true,
        )
    }

    fn synthetic(
        tenant_identity: impl Into<String>,
        truth_branch_identity: Option<String>,
        schema_basis_identity: Option<String>,
        resolution_class: TenantResolutionClass,
        epoch: TenantBasisEpoch,
        ambiguous: bool,
        hidden_filter: bool,
    ) -> Self {
        let tenant_identity = tenant_identity.into();
        let digest = hash_parts(&[
            format!("tenant:{tenant_identity}"),
            format!(
                "truth:{}",
                truth_branch_identity.as_deref().unwrap_or("none")
            ),
            format!(
                "schema:{}",
                schema_basis_identity.as_deref().unwrap_or("none")
            ),
            format!("resolution:{}", resolution_class.as_str()),
            format!("epoch:{}", epoch.as_u64()),
            format!("ambiguous:{ambiguous}"),
            format!("hidden_filter:{hidden_filter}"),
        ]);
        Self {
            tenant_identity,
            truth_branch_identity,
            schema_basis_identity,
            resolution_class,
            epoch,
            ambiguous,
            hidden_filter,
            digest,
        }
    }

    pub fn tenant_identity(&self) -> &str {
        &self.tenant_identity
    }

    pub fn truth_branch_identity(&self) -> Option<&str> {
        self.truth_branch_identity.as_deref()
    }

    pub fn schema_basis_identity(&self) -> Option<&str> {
        self.schema_basis_identity.as_deref()
    }

    pub fn resolution_class(&self) -> TenantResolutionClass {
        self.resolution_class
    }

    pub fn epoch(&self) -> TenantBasisEpoch {
        self.epoch
    }

    pub fn ambiguous(&self) -> bool {
        self.ambiguous
    }

    pub fn hidden_filter(&self) -> bool {
        self.hidden_filter
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SchemaVariantSnapshot {
    tenant_identity: String,
    schema_basis_identity: String,
    compatibility_class: String,
    global_fallback: bool,
    digest: String,
}

impl SchemaVariantSnapshot {
    pub fn synthetic_authority(
        tenant_identity: impl Into<String>,
        schema_basis_identity: impl Into<String>,
        compatibility_class: impl Into<String>,
    ) -> Self {
        Self::synthetic(
            tenant_identity,
            schema_basis_identity,
            compatibility_class,
            false,
        )
    }

    pub fn synthetic_global_fallback(
        tenant_identity: impl Into<String>,
        schema_basis_identity: impl Into<String>,
    ) -> Self {
        Self::synthetic(
            tenant_identity,
            schema_basis_identity,
            "global_fallback_attempt",
            true,
        )
    }

    fn synthetic(
        tenant_identity: impl Into<String>,
        schema_basis_identity: impl Into<String>,
        compatibility_class: impl Into<String>,
        global_fallback: bool,
    ) -> Self {
        let tenant_identity = tenant_identity.into();
        let schema_basis_identity = schema_basis_identity.into();
        let compatibility_class = compatibility_class.into();
        let digest = hash_parts(&[
            format!("tenant:{tenant_identity}"),
            format!("schema:{schema_basis_identity}"),
            format!("compatibility:{compatibility_class}"),
            format!("global_fallback:{global_fallback}"),
        ]);
        Self {
            tenant_identity,
            schema_basis_identity,
            compatibility_class,
            global_fallback,
            digest,
        }
    }

    pub fn tenant_identity(&self) -> &str {
        &self.tenant_identity
    }

    pub fn schema_basis_identity(&self) -> &str {
        &self.schema_basis_identity
    }

    pub fn compatibility_class(&self) -> &str {
        &self.compatibility_class
    }

    pub fn global_fallback(&self) -> bool {
        self.global_fallback
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}
