#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TenantBasisEpoch {
    Synthetic(u64),
}

impl TenantBasisEpoch {
    pub fn as_u64(&self) -> u64 {
        match self {
            Self::Synthetic(epoch) => *epoch,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TenantResolutionClass {
    DirectBinding,
    CachedBinding,
    DerivedBinding,
}

impl TenantResolutionClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DirectBinding => "direct_binding",
            Self::CachedBinding => "cached_binding",
            Self::DerivedBinding => "derived_binding",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TenantTruthBasisIdentity(String);

impl TenantTruthBasisIdentity {
    pub(crate) fn new(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TenantSchemaBasisIdentity(String);

impl TenantSchemaBasisIdentity {
    pub(crate) fn new(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TenantTruthBasis {
    identity: TenantTruthBasisIdentity,
    tenant_identity: String,
    branch_identity: String,
    resolution_class: TenantResolutionClass,
    epoch: TenantBasisEpoch,
}

impl TenantTruthBasis {
    pub(crate) fn admitted(
        identity: TenantTruthBasisIdentity,
        tenant_identity: String,
        branch_identity: String,
        resolution_class: TenantResolutionClass,
        epoch: TenantBasisEpoch,
    ) -> Self {
        Self {
            identity,
            tenant_identity,
            branch_identity,
            resolution_class,
            epoch,
        }
    }

    pub fn identity(&self) -> &TenantTruthBasisIdentity {
        &self.identity
    }

    pub fn tenant_identity(&self) -> &str {
        &self.tenant_identity
    }

    pub fn branch_identity(&self) -> &str {
        &self.branch_identity
    }

    pub fn resolution_class(&self) -> TenantResolutionClass {
        self.resolution_class
    }

    pub fn epoch(&self) -> TenantBasisEpoch {
        self.epoch
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TenantSchemaBasis {
    identity: TenantSchemaBasisIdentity,
    tenant_identity: String,
    schema_identity: String,
    epoch: TenantBasisEpoch,
}

impl TenantSchemaBasis {
    pub(crate) fn admitted(
        identity: TenantSchemaBasisIdentity,
        tenant_identity: String,
        schema_identity: String,
        epoch: TenantBasisEpoch,
    ) -> Self {
        Self {
            identity,
            tenant_identity,
            schema_identity,
            epoch,
        }
    }

    pub fn identity(&self) -> &TenantSchemaBasisIdentity {
        &self.identity
    }

    pub fn tenant_identity(&self) -> &str {
        &self.tenant_identity
    }

    pub fn schema_identity(&self) -> &str {
        &self.schema_identity
    }

    pub fn epoch(&self) -> TenantBasisEpoch {
        self.epoch
    }
}
