use crate::{
    layout_access::{
        authenticity_family::AuthenticityLayoutReport, custody_family::CustodyLayoutReport,
        key_scope_family::KeyScopeLayoutReport,
        repair_blast_radius_family::RepairBlastRadiusLayoutReport,
        tenant_scope_family::TenantScopeLayoutReport,
    },
    StoreSecurityScopeDenial, StoreSecurityScopeDenialKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SecurityLayoutCloseout;

impl SecurityLayoutCloseout {
    pub const fn phase27_ready(
        _tenant: &TenantScopeLayoutReport,
        _key: &KeyScopeLayoutReport,
        _authenticity: &AuthenticityLayoutReport,
        _custody: &CustodyLayoutReport,
        _repair: &RepairBlastRadiusLayoutReport,
    ) -> Self {
        Self
    }

    pub const fn denied_source_kind(
        denial: StoreSecurityScopeDenial,
    ) -> StoreSecurityScopeDenialKind {
        denial.kind()
    }
}
