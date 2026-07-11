use forge_store_contracts::{DurableArtifactFamilyId, DurableArtifactRebuildPosture};

use crate::{
    admit_layout_access_security_boundary, reject_non_store_security_scope_source,
    AdmittedRepairBlastRadiusLayoutRule, SecurityCustodyLookupAccessShape,
    StoreLayoutAccessSecurityBoundaryWitness, StoreRepairPhysicalRegionWitness,
    StoreSecurityAuthoritySource, StoreSecurityScopeAdmissionCounterSnapshot,
    StoreSecurityScopeDenial,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepairBlastRadiusAuthorityPosture {
    ReadinessOnly,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepairBlastRadiusLayoutReport {
    rule: AdmittedRepairBlastRadiusLayoutRule,
    rebuild_posture: DurableArtifactRebuildPosture,
    authority_posture: RepairBlastRadiusAuthorityPosture,
    security_boundary: StoreLayoutAccessSecurityBoundaryWitness,
    exact_counters: StoreSecurityScopeAdmissionCounterSnapshot,
    region_id: String,
}

impl RepairBlastRadiusLayoutReport {
    pub const fn rule(&self) -> AdmittedRepairBlastRadiusLayoutRule {
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

    pub const fn authority_posture(&self) -> RepairBlastRadiusAuthorityPosture {
        self.authority_posture
    }

    pub const fn security_boundary(&self) -> StoreLayoutAccessSecurityBoundaryWitness {
        self.security_boundary
    }

    pub const fn exact_counters(&self) -> StoreSecurityScopeAdmissionCounterSnapshot {
        self.exact_counters
    }

    pub fn region_id(&self) -> &str {
        &self.region_id
    }

    pub const fn deny_authority_source(
        &self,
        source: StoreSecurityAuthoritySource,
    ) -> StoreSecurityScopeDenial {
        let _ = self;
        reject_non_store_security_scope_source(source)
    }
}

impl StoreRepairPhysicalRegionWitness {
    pub fn admit_repair_blast_radius_layout(
        &self,
        rule: &AdmittedRepairBlastRadiusLayoutRule,
    ) -> RepairBlastRadiusLayoutReport {
        RepairBlastRadiusLayoutReport {
            rule: *rule,
            rebuild_posture: DurableArtifactRebuildPosture::QuarantineOnly,
            authority_posture: RepairBlastRadiusAuthorityPosture::ReadinessOnly,
            security_boundary: admit_layout_access_security_boundary(
                self.admitted_scope().witnesses(),
            ),
            exact_counters: self.admitted_scope().receipt().counters(),
            region_id: self.region_id().to_owned(),
        }
    }
}
