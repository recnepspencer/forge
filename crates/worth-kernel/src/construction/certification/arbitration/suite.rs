use forge_query::facade::ForgeQueryWorkspace;

use crate::construction::digest::digest_owned_parts;

use super::{
    prepare_primitive_construction_intent_arbitration_representative_evidence,
    required_arbitration_representative_cases, PrimitiveConstructionIntentArbitrationBundleCase,
    PrimitiveConstructionIntentArbitrationRepresentativeEvidence,
    PrimitiveConstructionIntentArbitrationRepresentativeEvidenceError,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionIntentArbitrationHostilitySuiteReport {
    evidence_rows: Vec<PrimitiveConstructionIntentArbitrationRepresentativeEvidence>,
    suite_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionIntentArbitrationHostilitySuiteReport {
    fn new(
        evidence_rows: Vec<PrimitiveConstructionIntentArbitrationRepresentativeEvidence>,
    ) -> Self {
        let expected_cases = required_arbitration_representative_cases();
        let suite_verified = evidence_rows.len() == expected_cases.len()
            && expected_cases.iter().all(|case| {
                evidence_rows
                    .iter()
                    .any(|evidence| evidence.case() == *case)
            })
            && {
                let unique_digests = evidence_rows
                    .iter()
                    .map(|evidence| evidence.report_digest())
                    .collect::<std::collections::BTreeSet<_>>();
                unique_digests.len() == evidence_rows.len()
            }
            && evidence_rows
                .iter()
                .all(|evidence| evidence.parity_verified());
        let report_digest = digest_owned_parts(
            &evidence_rows
                .iter()
                .map(|evidence| format!("{:?}:{}", evidence.case(), evidence.report_digest()))
                .collect::<Vec<_>>(),
        );
        Self {
            evidence_rows,
            suite_verified,
            report_digest,
        }
    }

    pub fn evidence_rows(&self) -> &[PrimitiveConstructionIntentArbitrationRepresentativeEvidence] {
        &self.evidence_rows
    }

    pub fn evidence(
        &self,
        case: PrimitiveConstructionIntentArbitrationBundleCase,
    ) -> Option<&PrimitiveConstructionIntentArbitrationRepresentativeEvidence> {
        self.evidence_rows
            .iter()
            .find(|evidence| evidence.case() == case)
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
    PrimitiveConstructionIntentArbitrationRepresentativeEvidenceError,
> {
    let evidence_rows = required_arbitration_representative_cases()
        .iter()
        .copied()
        .map(|case| {
            prepare_primitive_construction_intent_arbitration_representative_evidence(
                workspace, case,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(PrimitiveConstructionIntentArbitrationHostilitySuiteReport::new(evidence_rows))
}

#[cfg(test)]
mod tests {
    use super::prepare_primitive_construction_intent_arbitration_hostility_suite_report;
    use crate::construction::PrimitiveConstructionIntentArbitrationBundleCase;
    use topology::facade::{
        milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters,
    };
    use worth_spatial::facade::arbitration::{SpatialBlockedCapability, SpatialIntentEscalation};

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
            .evidence(PrimitiveConstructionIntentArbitrationBundleCase::GrazingSnapExplicitChoice)
            .is_some());
        assert_eq!(
            report
                .evidence(
                    PrimitiveConstructionIntentArbitrationBundleCase::HostPenetrationBlockedCut
                )
                .expect("host")
                .policy_row()
                .escalation(),
            SpatialIntentEscalation::BlockedByMissingCapability(
                SpatialBlockedCapability::CutOpening
            )
        );
    }
}
