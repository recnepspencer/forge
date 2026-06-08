use crate::application::ForgeQueryDomainOperatingRequirement;
use crate::application::{
    ForgeQueryCapabilityFamily, ForgeQueryCapabilityStatus, ForgeQuerySupportMatrix,
    ForgeQuerySupportReport, ForgeQuerySupportSectionPosture,
};
use crate::identity::hash_parts;
use crate::runtime::{
    ForgeQueryRuntimeBackendPosture, ForgeQueryRuntimeFamilySupportStatus,
    ForgeQueryRuntimePublicApiContract, ForgeQueryRuntimePublicSupportMatrix,
    ForgeQueryRuntimeSupportProfile,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDomainEntrySupportSnapshot {
    report: ForgeQuerySupportReport,
    runtime_support_matrix: ForgeQueryRuntimePublicSupportMatrix,
    snapshot_digest: String,
}

impl ForgeQueryDomainEntrySupportSnapshot {
    pub(crate) fn from_support_report(report: ForgeQuerySupportReport) -> Self {
        let runtime_support_matrix = ForgeQueryRuntimePublicSupportMatrix::from_public_api_contract(
            &ForgeQueryRuntimePublicApiContract::from_support_profile(
                &ForgeQueryRuntimeSupportProfile::scaffold_backend_profile()
                    .with_posture(ForgeQueryRuntimeBackendPosture::Primary),
            ),
        );
        let snapshot_digest = hash_parts(&[
            format!("report:{}", report.report_digest()),
            format!("runtime_support:{}", runtime_support_matrix.matrix_digest()),
        ]);
        Self {
            report,
            runtime_support_matrix,
            snapshot_digest,
        }
    }

    pub fn support_matrix(&self) -> &ForgeQuerySupportMatrix {
        self.report.support_matrix()
    }

    pub fn admitted_capability_families(&self) -> &[ForgeQueryCapabilityFamily] {
        self.report.admitted_capability_families()
    }

    pub fn deferred_capability_families(&self) -> &[ForgeQueryCapabilityFamily] {
        self.report.deferred_capability_families()
    }

    pub fn unsupported_capability_families(&self) -> &[ForgeQueryCapabilityFamily] {
        self.report.unsupported_capability_families()
    }

    pub fn section_postures(&self) -> &[ForgeQuerySupportSectionPosture] {
        self.report.section_postures()
    }

    pub fn runtime_support_matrix(&self) -> &ForgeQueryRuntimePublicSupportMatrix {
        &self.runtime_support_matrix
    }

    pub fn validated_config_digest(&self) -> &str {
        self.report.validated_config_digest()
    }

    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }

    pub fn capability_status(
        &self,
        family: ForgeQueryCapabilityFamily,
    ) -> Option<ForgeQueryCapabilityStatus> {
        self.support_matrix()
            .descriptor(family)
            .map(|descriptor| descriptor.status())
    }

    pub fn operating_requirement_status(
        &self,
        requirement: ForgeQueryDomainOperatingRequirement,
    ) -> Option<ForgeQueryRuntimeFamilySupportStatus> {
        self.runtime_support_matrix()
            .row_for_family(requirement.runtime_facade_family())
            .map(|row| row.status())
    }
}
