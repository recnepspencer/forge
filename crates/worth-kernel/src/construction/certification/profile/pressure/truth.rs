use crate::construction::certification::profile::pressure::delta::PrimitiveConstructionPolicyPressureDeltaReport;
use crate::construction::certification::profile::pressure::registry::{
    policy_pressure_registry, PrimitiveConstructionPolicyPressureRegistry,
};
use crate::construction::certification::profile::pressure::report::PrimitiveConstructionPolicyPressureSurfaceReport;
use crate::construction::certification::profile::pressure::{
    PrimitiveConstructionPolicyPressureCase, PrimitiveConstructionPolicyPressureDeltaCase,
};
use crate::construction::digest::{digest_owned_parts_with_scope, ConstructionDigestScope};

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionPolicyPressureCanonicalTruth {
    registry: PrimitiveConstructionPolicyPressureRegistry,
    direct_row_digests: Vec<(PrimitiveConstructionPolicyPressureCase, String)>,
    delta_row_digests: Vec<(PrimitiveConstructionPolicyPressureDeltaCase, String)>,
    truth_digest: String,
}

impl PrimitiveConstructionPolicyPressureCanonicalTruth {
    pub(crate) fn from_reports(
        direct_report: &PrimitiveConstructionPolicyPressureSurfaceReport,
        delta_report: &PrimitiveConstructionPolicyPressureDeltaReport,
    ) -> Self {
        let registry = policy_pressure_registry();
        let direct_row_digests = registry
            .direct_cases()
            .iter()
            .map(|case| {
                let row = direct_report
                    .row(*case)
                    .expect("verified policy pressure bundles require every direct registry row");
                (*case, row.row_digest().to_string())
            })
            .collect::<Vec<_>>();
        let delta_row_digests = registry
            .delta_cases()
            .iter()
            .map(|case| {
                let row = delta_report
                    .row(*case)
                    .expect("verified policy pressure bundles require every delta registry row");
                (*case, row.row_digest().to_string())
            })
            .collect::<Vec<_>>();
        let truth_digest = digest_owned_parts_with_scope(
            ConstructionDigestScope::ParityIdentity,
            &std::iter::once(registry.registry_digest().to_string())
                .chain(
                    direct_row_digests
                        .iter()
                        .map(|(case, digest)| format!("{case:?}:{digest}")),
                )
                .chain(
                    delta_row_digests
                        .iter()
                        .map(|(case, digest)| format!("{case:?}:{digest}")),
                )
                .collect::<Vec<_>>(),
        );
        Self {
            registry,
            direct_row_digests,
            delta_row_digests,
            truth_digest,
        }
    }

    pub(crate) fn direct_matches(
        &self,
        report: &PrimitiveConstructionPolicyPressureSurfaceReport,
    ) -> bool {
        report.pressure_verified()
            && report.rows().iter().map(|row| row.case()).eq(self
                .registry
                .direct_cases()
                .iter()
                .copied())
            && report
                .rows()
                .iter()
                .zip(self.direct_row_digests.iter())
                .all(|(row, (case, digest))| row.case() == *case && row.row_digest() == digest)
    }

    pub(crate) fn delta_matches(
        &self,
        report: &PrimitiveConstructionPolicyPressureDeltaReport,
    ) -> bool {
        report.delta_verified()
            && self.direct_matches(report.direct_report())
            && report.rows().iter().map(|row| row.case()).eq(self
                .registry
                .delta_cases()
                .iter()
                .copied())
            && report
                .rows()
                .iter()
                .zip(self.delta_row_digests.iter())
                .all(|(row, (case, digest))| row.case() == *case && row.row_digest() == digest)
    }

    pub fn required_direct_cases(
        &self,
    ) -> &'static [crate::construction::certification::profile::pressure::PrimitiveConstructionPolicyPressureCase]
    {
        self.registry.direct_cases()
    }

    pub fn required_delta_cases(
        &self,
    ) -> &'static [crate::construction::certification::profile::pressure::PrimitiveConstructionPolicyPressureDeltaCase]
    {
        self.registry.delta_cases()
    }

    pub fn truth_digest(&self) -> &str {
        &self.truth_digest
    }
}
