use forge_query::facade::ForgeQueryWorkspace;

use crate::construction::digest::digest_owned_parts;

use super::{
    prepare_primitive_construction_intent_arbitration_report_bundle,
    PrimitiveConstructionIntentArbitrationBundleCase,
    PrimitiveConstructionIntentArbitrationReportBundle,
    PrimitiveConstructionIntentArbitrationReportBundleError,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionIntentArbitrationHostilitySuiteReport {
    bundles: Vec<PrimitiveConstructionIntentArbitrationReportBundle>,
    suite_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionIntentArbitrationHostilitySuiteReport {
    fn new(bundles: Vec<PrimitiveConstructionIntentArbitrationReportBundle>) -> Self {
        let suite_verified = bundles.iter().all(|bundle| bundle.bundle_verified());
        let report_digest = digest_owned_parts(
            &bundles
                .iter()
                .map(|bundle| {
                    format!(
                        "{:?}:{}:{}",
                        bundle.case(),
                        bundle.bundle_digest(),
                        bundle.bundle_verified()
                    )
                })
                .collect::<Vec<_>>(),
        );
        Self {
            bundles,
            suite_verified,
            report_digest,
        }
    }

    pub fn bundles(&self) -> &[PrimitiveConstructionIntentArbitrationReportBundle] {
        &self.bundles
    }

    pub fn bundle(
        &self,
        case: PrimitiveConstructionIntentArbitrationBundleCase,
    ) -> Option<&PrimitiveConstructionIntentArbitrationReportBundle> {
        self.bundles.iter().find(|bundle| bundle.case() == case)
    }

    pub fn suite_verified(&self) -> bool {
        self.suite_verified
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub fn prepare_primitive_construction_intent_arbitration_hostility_suite_report(
    workspace: &mut ForgeQueryWorkspace,
) -> Result<
    PrimitiveConstructionIntentArbitrationHostilitySuiteReport,
    PrimitiveConstructionIntentArbitrationReportBundleError,
> {
    let bundles = [
        PrimitiveConstructionIntentArbitrationBundleCase::DirectMoveOnlyPolicy,
        PrimitiveConstructionIntentArbitrationBundleCase::GrazingSnapExplicitChoice,
        PrimitiveConstructionIntentArbitrationBundleCase::OverlapMoveOnlyWithBlockedMerge,
        PrimitiveConstructionIntentArbitrationBundleCase::HostPenetrationBlockedCut,
        PrimitiveConstructionIntentArbitrationBundleCase::FrameAlignedIntent,
        PrimitiveConstructionIntentArbitrationBundleCase::OverlapAdvancedCapabilities,
    ]
    .into_iter()
    .map(|case| prepare_primitive_construction_intent_arbitration_report_bundle(workspace, case))
    .collect::<Result<Vec<_>, _>>()?;
    Ok(PrimitiveConstructionIntentArbitrationHostilitySuiteReport::new(bundles))
}

#[cfg(test)]
mod tests {
    use super::prepare_primitive_construction_intent_arbitration_hostility_suite_report;
    use crate::construction::PrimitiveConstructionIntentArbitrationBundleCase;
    use topology::facade::{
        milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters,
    };
    use worth_spatial::facade::{SpatialBlockedCapability, SpatialIntentEscalation};

    #[test]
    fn arbitration_hostility_suite_covers_contact_containment_and_blocked_future_candidates() {
        let runtime = milestone_one_runtime_builder()
            .expect("runtime builder")
            .build();
        let mut workspace = topology_runtime(
            TopologyRuntimeAdapters::current_head(runtime),
            "worth-kernel.arbitration-hostility-suite".to_string(),
        )
        .expect("workspace");
        let report = prepare_primitive_construction_intent_arbitration_hostility_suite_report(
            &mut workspace,
        )
        .expect("report");

        assert!(report.suite_verified());
        assert!(report
            .bundle(PrimitiveConstructionIntentArbitrationBundleCase::GrazingSnapExplicitChoice)
            .is_some());
        assert_eq!(
            report
                .bundle(PrimitiveConstructionIntentArbitrationBundleCase::HostPenetrationBlockedCut)
                .expect("host")
                .policy_row()
                .escalation(),
            SpatialIntentEscalation::BlockedByMissingCapability(
                SpatialBlockedCapability::CutOpening
            )
        );
    }
}
