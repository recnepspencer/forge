use crate::policy_basis::{BranchAccessGrant, PolicyRuleSnapshot};
use crate::tenant_basis::{SchemaVariantSnapshot, TenantBindingSnapshot};

use super::WorthQueryReadRelationshipProofs;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryReadContextKind {
    Current,
    CurrentPolicyTenant,
    CurrentPolicyTenantRelationship,
}

impl WorthQueryReadContextKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::CurrentPolicyTenant => "current_policy_tenant",
            Self::CurrentPolicyTenantRelationship => "current_policy_tenant_relationship",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryCurrentReadContext;

#[derive(Debug, Eq, PartialEq)]
pub struct WorthQueryCurrentPolicyTenantReadContext {
    pub(crate) policy: PolicyRuleSnapshot,
    pub(crate) tenant: TenantBindingSnapshot,
    pub(crate) branch: BranchAccessGrant,
    pub(crate) schema: SchemaVariantSnapshot,
}

#[derive(Debug, Eq, PartialEq)]
pub struct WorthQueryCurrentRelationshipReadContext {
    pub(crate) policy_tenant: WorthQueryCurrentPolicyTenantReadContext,
    pub(crate) relationship_proofs: WorthQueryReadRelationshipProofs,
}

#[derive(Debug, Eq, PartialEq)]
pub enum WorthQueryReadContextDeclaration {
    Current(WorthQueryCurrentReadContext),
    CurrentPolicyTenant(WorthQueryCurrentPolicyTenantReadContext),
    CurrentPolicyTenantRelationship(WorthQueryCurrentRelationshipReadContext),
}

pub fn current() -> WorthQueryCurrentReadContext {
    WorthQueryCurrentReadContext
}

impl WorthQueryCurrentReadContext {
    pub fn under_policy_tenant(
        self,
        policy: PolicyRuleSnapshot,
        tenant: TenantBindingSnapshot,
        branch: BranchAccessGrant,
        schema: SchemaVariantSnapshot,
    ) -> WorthQueryCurrentPolicyTenantReadContext {
        WorthQueryCurrentPolicyTenantReadContext {
            policy,
            tenant,
            branch,
            schema,
        }
    }
}

impl WorthQueryCurrentPolicyTenantReadContext {
    pub fn with_relationship_proofs(
        self,
        relationship_proofs: WorthQueryReadRelationshipProofs,
    ) -> WorthQueryCurrentRelationshipReadContext {
        WorthQueryCurrentRelationshipReadContext {
            policy_tenant: self,
            relationship_proofs,
        }
    }
}

impl WorthQueryReadContextDeclaration {
    pub fn kind(&self) -> WorthQueryReadContextKind {
        match self {
            Self::Current(_) => WorthQueryReadContextKind::Current,
            Self::CurrentPolicyTenant(_) => WorthQueryReadContextKind::CurrentPolicyTenant,
            Self::CurrentPolicyTenantRelationship(_) => {
                WorthQueryReadContextKind::CurrentPolicyTenantRelationship
            }
        }
    }
}

impl From<WorthQueryCurrentReadContext> for WorthQueryReadContextDeclaration {
    fn from(context: WorthQueryCurrentReadContext) -> Self {
        Self::Current(context)
    }
}

impl From<WorthQueryCurrentPolicyTenantReadContext> for WorthQueryReadContextDeclaration {
    fn from(context: WorthQueryCurrentPolicyTenantReadContext) -> Self {
        Self::CurrentPolicyTenant(context)
    }
}

impl From<WorthQueryCurrentRelationshipReadContext> for WorthQueryReadContextDeclaration {
    fn from(context: WorthQueryCurrentRelationshipReadContext) -> Self {
        Self::CurrentPolicyTenantRelationship(context)
    }
}
