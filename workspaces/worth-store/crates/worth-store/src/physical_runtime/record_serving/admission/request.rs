use super::super::{
    AdmittedPhysicalRecordFormat, AdmittedPhysicalRecordResidencyPolicy,
    AdmittedRecordAccessPolicy, AdmittedRecordPlacementPolicy,
};
use super::residency_policy::canonical_residency_policy;
use crate::physical_runtime::PhysicalWorkProfileDeclaration;

pub struct PhysicalRecordInitialization {
    pub(in crate::physical_runtime::record_serving) format: AdmittedPhysicalRecordFormat,
    pub(in crate::physical_runtime::record_serving) placement: AdmittedRecordPlacementPolicy,
    pub(in crate::physical_runtime::record_serving) access: AdmittedRecordAccessPolicy,
    pub(in crate::physical_runtime::record_serving) residency:
        AdmittedPhysicalRecordResidencyPolicy,
    pub(in crate::physical_runtime::record_serving) work_profile: PhysicalWorkProfileDeclaration,
}

impl PhysicalRecordInitialization {
    pub fn new(
        format: AdmittedPhysicalRecordFormat,
        placement: AdmittedRecordPlacementPolicy,
        access: AdmittedRecordAccessPolicy,
    ) -> Self {
        Self {
            format,
            placement,
            access,
            residency: canonical_residency_policy(format),
            work_profile: PhysicalWorkProfileDeclaration::default(),
        }
    }

    pub const fn with_residency_policy(
        mut self,
        policy: AdmittedPhysicalRecordResidencyPolicy,
    ) -> Self {
        self.residency = policy;
        self
    }

    pub fn with_physical_work_profile(mut self, profile: PhysicalWorkProfileDeclaration) -> Self {
        self.work_profile = profile;
        self
    }
}

pub struct PhysicalRecordOpen {
    pub(in crate::physical_runtime::record_serving) format: AdmittedPhysicalRecordFormat,
    pub(in crate::physical_runtime::record_serving) access: AdmittedRecordAccessPolicy,
    pub(in crate::physical_runtime::record_serving) residency:
        AdmittedPhysicalRecordResidencyPolicy,
    pub(in crate::physical_runtime::record_serving) work_profile: PhysicalWorkProfileDeclaration,
}

impl PhysicalRecordOpen {
    pub fn new(format: AdmittedPhysicalRecordFormat, access: AdmittedRecordAccessPolicy) -> Self {
        Self {
            format,
            access,
            residency: canonical_residency_policy(format),
            work_profile: PhysicalWorkProfileDeclaration::default(),
        }
    }

    pub const fn with_residency_policy(
        mut self,
        policy: AdmittedPhysicalRecordResidencyPolicy,
    ) -> Self {
        self.residency = policy;
        self
    }

    pub fn with_physical_work_profile(mut self, profile: PhysicalWorkProfileDeclaration) -> Self {
        self.work_profile = profile;
        self
    }
}
