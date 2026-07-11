use forge_store_contracts::{DurableArtifactFamilyId, DurableArtifactRebuildPosture};

use crate::{
    admit_layout_access_security_boundary, reject_non_store_security_scope_source,
    AdmittedCustodyLayoutRule, SecurityCustodyLookupAccessShape, StoreAdmittedSecurityScope,
    StoreCustodyPosture, StoreLayoutAccessSecurityBoundaryWitness, StoreSecurityAuthoritySource,
    StoreSecurityScopeAdmissionCounterSnapshot, StoreSecurityScopeDenial,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CustodyLayoutReport {
    rule: AdmittedCustodyLayoutRule,
    rebuild_posture: DurableArtifactRebuildPosture,
    security_boundary: StoreLayoutAccessSecurityBoundaryWitness,
    exact_counters: StoreSecurityScopeAdmissionCounterSnapshot,
}

impl CustodyLayoutReport {
    pub const fn rule(&self) -> AdmittedCustodyLayoutRule {
        self.rule
    }

    pub const fn family_id(&self) -> DurableArtifactFamilyId {
        self.rule.family_id()
    }

    pub const fn declared_access_shape(&self) -> SecurityCustodyLookupAccessShape {
        self.rule.declared_access_shape()
    }

    pub const fn rebuild_posture(&self) -> DurableArtifactRebuildPosture {
        self.rebuild_posture
    }

    pub const fn security_boundary(&self) -> StoreLayoutAccessSecurityBoundaryWitness {
        self.security_boundary
    }

    pub const fn custody_posture(&self) -> StoreCustodyPosture {
        self.security_boundary.custody_posture()
    }

    pub const fn exact_counters(&self) -> StoreSecurityScopeAdmissionCounterSnapshot {
        self.exact_counters
    }

    pub const fn deny_authority_source(
        &self,
        source: StoreSecurityAuthoritySource,
    ) -> StoreSecurityScopeDenial {
        let _ = self;
        reject_non_store_security_scope_source(source)
    }
}

impl StoreAdmittedSecurityScope {
    pub fn admit_custody_layout(&self, rule: &AdmittedCustodyLayoutRule) -> CustodyLayoutReport {
        CustodyLayoutReport {
            rule: *rule,
            rebuild_posture: DurableArtifactRebuildPosture::QuarantineOnly,
            security_boundary: admit_layout_access_security_boundary(self.witnesses()),
            exact_counters: self.receipt().counters(),
        }
    }
}
