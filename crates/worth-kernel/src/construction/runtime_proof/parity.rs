use forge_query::facade::ForgeQueryWorkspace;

use crate::construction::digest::digest_owned_parts;
use crate::construction::outcome::{
    prepare_primitive_construction_outcome, PrimitiveConstructionPreparedOutcome,
};
use crate::construction::runtime_basis::{
    prepare_primitive_construction_branch_preview_runtime_report,
    PrimitiveConstructionBranchPreviewRuntimeReport, PrimitiveConstructionRuntimeBasisError,
};
use crate::construction::{PrimitiveConstructionFamily, PrimitiveConstructionIntent};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionReplayParityReport {
    family: PrimitiveConstructionFamily,
    direct_outcome: PrimitiveConstructionPreparedOutcome,
    replay_outcome: PrimitiveConstructionPreparedOutcome,
    parity_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionReplayParityReport {
    fn new(
        family: PrimitiveConstructionFamily,
        direct_outcome: PrimitiveConstructionPreparedOutcome,
        replay_outcome: PrimitiveConstructionPreparedOutcome,
    ) -> Self {
        let parity_verified = direct_outcome == replay_outcome;
        let report_digest = digest_owned_parts(&[
            family.as_str().to_string(),
            direct_outcome.outcome_digest().to_string(),
            replay_outcome.outcome_digest().to_string(),
            parity_verified.to_string(),
        ]);
        Self {
            family,
            direct_outcome,
            replay_outcome,
            parity_verified,
            report_digest,
        }
    }

    pub fn family(&self) -> PrimitiveConstructionFamily {
        self.family
    }

    pub fn direct_outcome(&self) -> &PrimitiveConstructionPreparedOutcome {
        &self.direct_outcome
    }

    pub fn replay_outcome(&self) -> &PrimitiveConstructionPreparedOutcome {
        &self.replay_outcome
    }

    pub fn parity_verified(&self) -> bool {
        self.parity_verified
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionBranchLocalParityReport {
    family: PrimitiveConstructionFamily,
    direct_outcome: PrimitiveConstructionPreparedOutcome,
    branch_preview_runtime_report: PrimitiveConstructionBranchPreviewRuntimeReport,
    parity_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionBranchLocalParityReport {
    fn new(
        family: PrimitiveConstructionFamily,
        direct_outcome: PrimitiveConstructionPreparedOutcome,
        branch_preview_runtime_report: PrimitiveConstructionBranchPreviewRuntimeReport,
    ) -> Self {
        let parity_verified = direct_outcome == *branch_preview_runtime_report.outcome();
        let report_digest = digest_owned_parts(&[
            family.as_str().to_string(),
            direct_outcome.outcome_digest().to_string(),
            branch_preview_runtime_report
                .outcome()
                .outcome_digest()
                .to_string(),
            branch_preview_runtime_report.report_digest().to_string(),
            parity_verified.to_string(),
        ]);
        Self {
            family,
            direct_outcome,
            branch_preview_runtime_report,
            parity_verified,
            report_digest,
        }
    }

    pub fn family(&self) -> PrimitiveConstructionFamily {
        self.family
    }

    pub fn direct_outcome(&self) -> &PrimitiveConstructionPreparedOutcome {
        &self.direct_outcome
    }

    pub fn branch_preview_runtime_report(
        &self,
    ) -> &PrimitiveConstructionBranchPreviewRuntimeReport {
        &self.branch_preview_runtime_report
    }

    pub fn parity_verified(&self) -> bool {
        self.parity_verified
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub fn prepare_primitive_construction_replay_parity_report(
    intent: impl Into<PrimitiveConstructionIntent>,
) -> PrimitiveConstructionReplayParityReport {
    let intent = intent.into();
    let family = intent.family();
    let request = intent.request().clone();
    let direct_outcome = prepare_primitive_construction_outcome(request.clone());
    let replay_outcome = prepare_primitive_construction_outcome(request);
    PrimitiveConstructionReplayParityReport::new(family, direct_outcome, replay_outcome)
}

pub fn prepare_primitive_construction_branch_local_parity_report(
    workspace: &mut ForgeQueryWorkspace,
    intent: impl Into<PrimitiveConstructionIntent>,
) -> Result<PrimitiveConstructionBranchLocalParityReport, PrimitiveConstructionRuntimeBasisError> {
    let intent = intent.into();
    let family = intent.family();
    let request = intent.request().clone();
    let direct_outcome = prepare_primitive_construction_outcome(request.clone());
    let branch_preview_runtime_report =
        prepare_primitive_construction_branch_preview_runtime_report(workspace, intent)?;
    Ok(PrimitiveConstructionBranchLocalParityReport::new(
        family,
        direct_outcome,
        branch_preview_runtime_report,
    ))
}

#[cfg(test)]
mod tests {
    use super::{
        prepare_primitive_construction_branch_local_parity_report,
        prepare_primitive_construction_replay_parity_report,
    };
    use crate::construction::outcome::PrimitiveConstructionPreparedOutcome;
    use crate::construction::{
        OrthotopeSpec, PrimitiveConstructionFamily, PrimitiveConstructionIntent, RegularPrismSpec,
        RegularPyramidSpec, ShellWithHoleSpec, WireBodySpec,
    };
    use topology::facade::{
        milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters,
    };
    use worth_geom::facade::{
        PrimitiveNormalizationDisposition, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
        PrimitiveSupportNormalClass,
    };

    #[test]
    fn replay_parity_report_verifies_accepted_requests() {
        let report = prepare_primitive_construction_replay_parity_report(
            PrimitiveConstructionIntent::regular_prism(RegularPrismSpec {
                sides: 6,
                radius: 1.0,
                height: 2.0,
            }),
        );

        assert_eq!(report.family(), PrimitiveConstructionFamily::RegularPrism);
        assert!(report.parity_verified());
        match report.direct_outcome() {
            PrimitiveConstructionPreparedOutcome::Accepted(_) => {}
            PrimitiveConstructionPreparedOutcome::Rejected(_) => {
                panic!("prism should be accepted")
            }
        }
        assert!(!report.report_digest().is_empty());
    }

    #[test]
    fn replay_parity_report_verifies_rejected_requests() {
        let report = prepare_primitive_construction_replay_parity_report(
            PrimitiveConstructionIntent::shell_with_hole(ShellWithHoleSpec {
                outer_loop_edge_count: 2,
                hole_loop_edge_counts: vec![3],
            }),
        );

        assert_eq!(report.family(), PrimitiveConstructionFamily::ShellWithHole);
        assert!(report.parity_verified());
        match report.direct_outcome() {
            PrimitiveConstructionPreparedOutcome::Accepted(_) => {
                panic!("invalid shell should be rejected")
            }
            PrimitiveConstructionPreparedOutcome::Rejected(_) => {}
        }
        assert_eq!(report.direct_outcome(), report.replay_outcome());
    }

    #[test]
    fn replay_parity_report_preserves_escalated_realization_truth() {
        let report = prepare_primitive_construction_replay_parity_report(
            PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
                sides: 3,
                radius: 1.0e-200,
                height: 1.0e-200,
            }),
        );

        assert!(report.parity_verified());
        match report.direct_outcome() {
            PrimitiveConstructionPreparedOutcome::Accepted(outcome) => {
                assert_eq!(
                    outcome.realization_strategy(),
                    PrimitiveRealizationStrategy::ExactSupport
                );
                assert_eq!(
                    outcome.stability_class(),
                    PrimitiveStabilityClass::StableAfterEscalation
                );
                assert_eq!(
                    outcome.support_normal_class(),
                    PrimitiveSupportNormalClass::Degenerate
                );
                assert_eq!(
                    outcome.normalization_disposition(),
                    PrimitiveNormalizationDisposition::LocalTransformationApplied
                );
            }
            PrimitiveConstructionPreparedOutcome::Rejected(_) => {
                panic!("tiny pyramid should be accepted after escalation")
            }
        }
    }

    #[test]
    fn branch_local_parity_report_verifies_accepted_requests() {
        let runtime = milestone_one_runtime_builder()
            .expect("runtime builder")
            .build();
        let mut workspace = topology_runtime(
            TopologyRuntimeAdapters::current_head(runtime),
            "worth-kernel.branch-local.accepted".to_string(),
        )
        .expect("workspace");
        let report = prepare_primitive_construction_branch_local_parity_report(
            &mut workspace,
            PrimitiveConstructionIntent::orthotope(OrthotopeSpec {
                half_extents: [1.0, 2.0, 3.0],
            }),
        )
        .expect("branch local parity report");

        assert_eq!(report.family(), PrimitiveConstructionFamily::Orthotope);
        assert!(report.parity_verified());
        assert_eq!(
            report.direct_outcome(),
            report.branch_preview_runtime_report().outcome()
        );
        assert!(!report.report_digest().is_empty());
    }

    #[test]
    fn branch_local_parity_report_verifies_rejected_requests() {
        let runtime = milestone_one_runtime_builder()
            .expect("runtime builder")
            .build();
        let mut workspace = topology_runtime(
            TopologyRuntimeAdapters::current_head(runtime),
            "worth-kernel.branch-local.rejected".to_string(),
        )
        .expect("workspace");
        let report = prepare_primitive_construction_branch_local_parity_report(
            &mut workspace,
            PrimitiveConstructionIntent::wire_body(WireBodySpec { edge_count: 2 }),
        )
        .expect("branch local parity report");

        assert_eq!(report.family(), PrimitiveConstructionFamily::WireBody);
        assert!(report.parity_verified());
        match report.direct_outcome() {
            PrimitiveConstructionPreparedOutcome::Accepted(_) => {
                panic!("invalid wire body should be rejected")
            }
            PrimitiveConstructionPreparedOutcome::Rejected(_) => {}
        }
    }

    #[test]
    fn branch_local_parity_report_preserves_world_collapsed_salvage_truth() {
        let runtime = milestone_one_runtime_builder()
            .expect("runtime builder")
            .build();
        let mut workspace = topology_runtime(
            TopologyRuntimeAdapters::current_head(runtime),
            "worth-kernel.branch-local.world-collapsed-salvage".to_string(),
        )
        .expect("workspace");
        let report = prepare_primitive_construction_branch_local_parity_report(
            &mut workspace,
            PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
                sides: 3,
                radius: 1.0,
                height: 1.0,
            })
            .at([1.0e308, 1.0e308, 1.0e308]),
        )
        .expect("branch local parity report");

        assert!(report.parity_verified());
        match report.direct_outcome() {
            PrimitiveConstructionPreparedOutcome::Accepted(outcome) => {
                assert_eq!(
                    outcome.stability_class(),
                    PrimitiveStabilityClass::StableDirect
                );
                assert_eq!(
                    outcome.normalization_disposition(),
                    PrimitiveNormalizationDisposition::WorldSpaceSufficient
                );
                assert_eq!(
                    outcome.realization_strategy(),
                    PrimitiveRealizationStrategy::DirectWorld
                );
            }
            PrimitiveConstructionPreparedOutcome::Rejected(_) => {
                panic!("world-placed pyramid should stay admitted")
            }
        }
        assert_eq!(
            report
                .branch_preview_runtime_report()
                .normalization_disposition(),
            Some(PrimitiveNormalizationDisposition::WorldSpaceSufficient)
        );
    }
}
