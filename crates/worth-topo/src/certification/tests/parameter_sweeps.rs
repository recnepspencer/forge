use super::*;

#[test]
fn admitted_family_parameter_sweeps_certify_across_ranges() {
    let cases = [
        (
            MilestoneOnePrimitiveCase::WireOpen { half_edge_count: 1 },
            "WireOpen(n)",
        ),
        (
            MilestoneOnePrimitiveCase::WireOpen { half_edge_count: 8 },
            "WireOpen(n)",
        ),
        (
            MilestoneOnePrimitiveCase::WireClosed { half_edge_count: 3 },
            "WireClosed(n)",
        ),
        (
            MilestoneOnePrimitiveCase::WireClosed { half_edge_count: 9 },
            "WireClosed(n)",
        ),
        (
            MilestoneOnePrimitiveCase::WireBranch { branch_count: 3 },
            "WireBranch(k)",
        ),
        (
            MilestoneOnePrimitiveCase::WireBranch { branch_count: 9 },
            "WireBranch(k)",
        ),
        (
            MilestoneOnePrimitiveCase::SheetDisk { edge_count: 3 },
            "SheetDisk(n)",
        ),
        (
            MilestoneOnePrimitiveCase::SheetDisk { edge_count: 10 },
            "SheetDisk(n)",
        ),
        (
            MilestoneOnePrimitiveCase::SheetPatch { face_count: 2 },
            "SheetPatch(f)",
        ),
        (
            MilestoneOnePrimitiveCase::SheetPatch { face_count: 8 },
            "SheetPatch(f)",
        ),
        (
            MilestoneOnePrimitiveCase::SolidShell { face_count: 4 },
            "SolidShell(f)",
        ),
        (
            MilestoneOnePrimitiveCase::SolidShell { face_count: 10 },
            "SolidShell(f)",
        ),
        (
            MilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 3 },
            "NmtEdgeFan(k)",
        ),
        (
            MilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 9 },
            "NmtEdgeFan(k)",
        ),
    ];

    for (index, (primitive, family)) in cases.into_iter().enumerate() {
        let mut runtime = crate::facade::milestone_one_runtime_builder()
            .expect(" milestone one runtime builder")
            .build();
        let verified = verified_primitive(&mut runtime, &format!("sweep.case.{index}"), &primitive)
            .expect("admitted primitive commit");
        let report = certify_verified_topology_commit_traced(&mut runtime, &verified)
            .expect("swept primitive certification should succeed")
            .into_primary_result();

        assert!(
            report.named_truth_validated,
            "{family} should retain naming truth"
        );
        assert!(
            report.topology_validated,
            "{family} should pass topology validation"
        );
        assert!(report
            .primitive_family_coverage_matrix
            .entries
            .iter()
            .any(|entry| entry.family == family && entry.observed));
        assert!(
            report
                .milestone_1_replay_parity_report
                .relational_replay_checked
        );
        assert!(
            report
                .milestone_1_replay_parity_report
                .relational_replay_verified
        );
    }
}

#[test]
fn branch_local_parameter_sweeps_preserve_branch_and_replay_truth() {
    let cases = [
        (
            MilestoneOnePrimitiveCase::WireBranch { branch_count: 8 },
            "WireBranch(k)",
        ),
        (
            MilestoneOnePrimitiveCase::SheetPatch { face_count: 7 },
            "SheetPatch(f)",
        ),
        (
            MilestoneOnePrimitiveCase::SolidShell { face_count: 9 },
            "SolidShell(f)",
        ),
        (
            MilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 8 },
            "NmtEdgeFan(k)",
        ),
    ];

    for (index, (primitive, family)) in cases.into_iter().enumerate() {
        let mut runtime = crate::facade::milestone_one_runtime_builder()
            .expect(" milestone one runtime builder")
            .build();
        runtime
            .history_authority()
            .create_branch(
                BranchId("feature".to_string()),
                &BranchId("main".to_string()),
            )
            .expect("feature branch");
        let verified = verified_primitive_on_branch(
            &mut runtime,
            &format!("branch-sweep.case.{index}"),
            &primitive,
            BranchId("feature".to_string()),
            MutationOrigin::BranchLocalApplication,
        )
        .expect("branch-local admitted primitive commit");
        let report = certify_verified_topology_commit_traced(&mut runtime, &verified)
            .expect("branch-local swept primitive certification should succeed")
            .into_primary_result();

        assert!(
            report.branch_local_topology_report.branch_local,
            "{family} should remain branch-local"
        );
        assert_eq!(report.branch_local_topology_report.branch_id.0, "feature");
        assert_eq!(
            report.milestone_1_replay_parity_report.branch_id.0,
            "feature"
        );
        assert!(
            report
                .milestone_1_replay_parity_report
                .relational_replay_checked
        );
        assert!(
            report
                .milestone_1_replay_parity_report
                .relational_replay_verified
        );
    }
}
