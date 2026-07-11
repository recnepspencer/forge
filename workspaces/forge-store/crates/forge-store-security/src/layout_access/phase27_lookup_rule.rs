use forge_store_contracts::DurableArtifactFamilyId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityCustodyLookupAccessShape {
    PointLookup,
}

macro_rules! phase27_rule {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name {
            _private: (),
        }

        impl $name {
            pub(crate) const fn internal_phase27() -> Self {
                Self { _private: () }
            }

            #[cfg(feature = "phase27-layout-rule-construction")]
            #[doc(hidden)]
            pub const fn phase27() -> Self {
                Self::internal_phase27()
            }

            pub const fn family_id(&self) -> DurableArtifactFamilyId {
                DurableArtifactFamilyId::SecurityCustodyLookup
            }

            pub const fn declared_access_shape(&self) -> SecurityCustodyLookupAccessShape {
                SecurityCustodyLookupAccessShape::PointLookup
            }
        }
    };
}

phase27_rule!(AdmittedTenantScopeLayoutRule);
phase27_rule!(AdmittedKeyScopeLayoutRule);
phase27_rule!(AdmittedAuthenticityLayoutRule);
phase27_rule!(AdmittedCustodyLayoutRule);
phase27_rule!(AdmittedRepairBlastRadiusLayoutRule);
