#[cfg(test)]
mod certification_tests {
    use forge_relational::facade::history::BranchId;
    use worth_schema::facade::{
        milestone_one_default_primitive_corpus, seed_minimal_topology, RawWorthTopologyIntent,
        WorthMilestoneOnePrimitiveCase, WorthMilestoneOnePrimitiveExpectedOutcome,
        WorthMilestoneOnePrimitiveRole, WorthMutationOrigin, WorthTopologyAuthority,
        WorthTopologyMutation,
    };
    use worth_schema::facade::{WorthShellInterpretationClass, WorthWireInterpretationClass};
    use crate::certification::report::WorthReplayParityStatus;

    use crate::certification::{
        certify_milestone_one_branch_local_primitive_scenarios,
        certify_milestone_one_closeout,
        certify_milestone_one_default_primitive_corpus,
        certify_milestone_one_primitive_corpus, certify_milestone_one_read_view,
        certify_verified_topology_commit,
    };

    #[test]
    fn seeded_bootstrap_earns_milestone_one_certification_report() {
        let mut runtime = crate::facade::worth_milestone_one_runtime_builder()
            .expect("worth milestone one runtime builder")
            .build();

        let seeded = seed_minimal_topology(&mut runtime, "cert-harness")
            .expect("seed worth topology");
        let read_view = runtime
            .read_truth()
            .read_snapshot(&seeded.snapshot)
            .expect("worth snapshot read");

        let report =
            certify_milestone_one_read_view(&read_view, seeded.read_basis.clone())
                .expect("milestone one certification should succeed");

        assert!(report.named_truth_validated);
        assert!(report.topology_validated);
        assert_eq!(report.topology_truth_digest.algorithm, "fnv1a64");
        assert!(report.topology_truth_digest.row_count > 0);
        assert_eq!(report.naming_truth_digest.algorithm, "fnv1a64");
        assert_eq!(report.topology_validation_digest.algorithm, "fnv1a64");
        assert_eq!(report.topology_validation_report.rows.len(), 5);
        assert!(report
            .topology_validation_report
            .rows
            .iter()
            .any(|row| row.validator == "reference_integrity" && row.status == "passed"));
        assert!(report.naming_attachment_report.fully_named);
        assert_eq!(
            report.branch_local_topology_report.mutation_origin,
            worth_schema::facade::WorthMutationOrigin::Seed
        );
        assert!(!report.branch_local_topology_report.branch_local);
        assert_eq!(report.branch_local_topology_report.branch_id.0, "main");
        assert_eq!(
            report.milestone_1_replay_parity_report.parity_status,
            WorthReplayParityStatus::NotChecked
        );
        assert_eq!(report.milestone_1_replay_parity_report.branch_id.0, "main");
        assert!(!report
            .milestone_1_replay_parity_report
            .relational_replay_checked);
        assert!(!report
            .milestone_1_replay_parity_report
            .relational_replay_verified);
        assert!(report
            .milestone_1_replay_parity_report
            .replayed_commit_id
            .is_none());
        assert_eq!(report.milestone_1_replay_parity_report.mismatch_count, 0);
        assert!(report
            .milestone_1_replay_parity_report
            .replay_failure
            .is_none());
        assert!(report
            .milestone_1_replay_parity_report
            .interpretation_digest_match);
        assert!(report.milestone_1_replay_parity_report.truth_digest_match);
        assert!(report
            .milestone_1_replay_parity_report
            .validation_digest_match);
        assert_eq!(report.counters.topology_entity_upsert_count, 0);
        assert_eq!(report.counters.topology_relation_upsert_count, 0);
        assert_eq!(report.counters.commit_boundary_validator_count, 6);
        assert_eq!(report.counters.naming_target_lookup_count, 11);
        assert_eq!(report.read_artifact.snapshot, seeded.snapshot);
        assert_eq!(report.read_artifact.interpretations.wires.len(), 1);
        assert_eq!(report.read_artifact.interpretations.shells.len(), 1);
        assert_eq!(
            report.read_artifact.interpretations.wires[0].class,
            WorthWireInterpretationClass::OpenChain
        );
        assert_eq!(
            report.read_artifact.interpretations.shells[0].class,
            WorthShellInterpretationClass::OpenSheet
        );
        assert_eq!(
            report.certified_interpretation.interpretations,
            report.read_artifact.interpretations
        );
        assert!(
            report
                .primitive_family_coverage_matrix
                .entries
                .iter()
                .any(|entry| entry.family == "WireOpen(n)" && entry.observed)
        );
        assert!(
            report
                .primitive_family_coverage_matrix
                .entries
                .iter()
                .any(|entry| entry.family == "SheetDisk(n)" && entry.observed)
        );
    }

    #[test]
    fn verified_topology_commit_is_the_canonical_certification_input() {
        let mut runtime = crate::facade::worth_milestone_one_runtime_builder()
            .expect("worth milestone one runtime builder")
            .build();

        let _seeded = seed_minimal_topology(&mut runtime, "cert-verified-commit")
            .expect("seed worth topology");
        let verified = WorthTopologyAuthority::new(&mut runtime)
            .apply_topology_intent(RawWorthTopologyIntent::new(
                Vec::<WorthTopologyMutation>::new(),
                WorthMutationOrigin::LocalEdit,
            ))
            .expect("verified topology commit");

        let report = certify_verified_topology_commit(&mut runtime, &verified)
            .expect("verified commit certification should succeed");

        assert!(report.named_truth_validated);
        assert!(report.topology_validated);
        assert_eq!(report.read_artifact.snapshot, verified.persisted_truth.snapshot);
        assert_eq!(
            report.branch_local_topology_report.mutation_origin,
            WorthMutationOrigin::LocalEdit
        );
        assert_eq!(report.branch_local_topology_report.branch_id.0, "main");
        assert!(!report
            .milestone_1_replay_parity_report
            .relational_replay_checked);
        assert!(!report
            .milestone_1_replay_parity_report
            .relational_replay_verified);
        assert_eq!(
            report.milestone_1_replay_parity_report.parity_status,
            WorthReplayParityStatus::NotChecked
        );
        assert!(verified.commits.is_empty());
    }

    #[test]
    fn branch_local_verified_commit_certifies_against_the_feature_branch_truth_basis() {
        let mut runtime = crate::facade::worth_milestone_one_runtime_builder()
            .expect("worth milestone one runtime builder")
            .build();

        let _seeded = seed_minimal_topology(&mut runtime, "cert-branch-local")
            .expect("seed worth topology");
        runtime
            .history_authority()
            .create_branch(BranchId("feature".to_string()), &BranchId("main".to_string()))
            .expect("feature branch");

        let verified = WorthTopologyAuthority::new(&mut runtime)
            .apply_topology_intent_on_branch(
                RawWorthTopologyIntent::new(Vec::<WorthTopologyMutation>::new(), WorthMutationOrigin::BranchLocalApplication),
                BranchId("feature".to_string()),
            )
            .expect("branch-local verified topology commit");

        let report = certify_verified_topology_commit(&mut runtime, &verified)
            .expect("branch-local certification should succeed");

        assert!(report.named_truth_validated);
        assert!(report.topology_validated);
        assert!(report.branch_local_topology_report.branch_local);
        assert_eq!(report.branch_local_topology_report.branch_id.0, "feature");
        assert_eq!(report.milestone_1_replay_parity_report.branch_id.0, "feature");
        assert!(!report
            .milestone_1_replay_parity_report
            .relational_replay_checked);
        assert!(!report
            .milestone_1_replay_parity_report
            .relational_replay_verified);
        assert_eq!(
            report.milestone_1_replay_parity_report.parity_status,
            WorthReplayParityStatus::NotChecked
        );
        assert!(verified.commits.is_empty());
    }

    #[test]
    fn verified_commit_certification_runs_relational_replay_when_commit_exists() {
        let mut runtime = crate::facade::worth_milestone_one_runtime_builder()
            .expect("worth milestone one runtime builder")
            .build();

        let verified = worth_schema::facade::seed_milestone_one_primitive(
            &mut runtime,
            "replay-backed-certification",
            &WorthMilestoneOnePrimitiveCase::WireOpen { half_edge_count: 4 },
        )
        .expect("verified admitted primitive commit");

        let report = certify_verified_topology_commit(&mut runtime, &verified)
            .expect("verified commit certification should succeed");

        assert!(report
            .milestone_1_replay_parity_report
            .relational_replay_checked);
        assert!(report
            .milestone_1_replay_parity_report
            .relational_replay_verified);
        assert_eq!(
            report.milestone_1_replay_parity_report.parity_status,
            WorthReplayParityStatus::Match
        );
        assert!(report
            .milestone_1_replay_parity_report
            .replayed_commit_id
            .is_some());
        assert_eq!(report.milestone_1_replay_parity_report.mismatch_count, 0);
        assert!(report
            .milestone_1_replay_parity_report
            .replay_failure
            .is_none());
    }

    #[test]
    fn primitive_corpus_certification_runs_cases_through_authority_and_reports_family_coverage() {
        let corpus = certify_milestone_one_primitive_corpus(
            || {
                crate::facade::worth_milestone_one_runtime_builder()
                    .expect("worth milestone one runtime builder")
                    .build()
            },
            "cert-corpus",
            &[
                WorthMilestoneOnePrimitiveCase::WireOpen { half_edge_count: 4 },
                WorthMilestoneOnePrimitiveCase::WireClosed { half_edge_count: 4 },
                WorthMilestoneOnePrimitiveCase::WireBranch { branch_count: 4 },
                WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 5 },
                WorthMilestoneOnePrimitiveCase::SheetPatch { face_count: 3 },
                WorthMilestoneOnePrimitiveCase::SolidShell { face_count: 4 },
                WorthMilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
            ],
        )
        .expect("primitive corpus certification should succeed");

        assert_eq!(corpus.cases.len(), 7);
        assert!(corpus.cases.iter().all(|case| case.certification.named_truth_validated));
        assert!(corpus.cases.iter().all(|case| case.certification.topology_validated));
        assert_eq!(corpus.coverage_matrix.entries.len(), 7);
        assert!(corpus.coverage_matrix.entries.iter().all(|entry| {
            entry.admitted_generic_count == 1
                && entry.admitted_smallest_count == 0
                && entry.admitted_hostile_count == 0
                && entry.rejected_out_of_class_count == 0
                && !entry.role_closure_complete
        }));
        assert_eq!(corpus.parity_report.entries.len(), 7);
        assert!(corpus.parity_report.entries.iter().all(|entry| {
            entry.mainline_case_count == 1
                && entry.branch_local_case_count == 0
                && entry.mainline_replay_checked_case_count == 1
                && entry.mainline_replay_verified_case_count == 1
                && entry.mainline_digest_parity_case_count == 1
                && entry.branch_local_replay_checked_case_count == 0
                && entry.branch_local_replay_verified_case_count == 0
                && entry.branch_local_digest_parity_case_count == 0
                && entry.cross_branch_parity_case_count == 0
                && !entry.parity_closure_complete
        }));
        assert!(corpus.cases.iter().any(|case| {
            matches!(
                case.primitive,
                WorthMilestoneOnePrimitiveCase::WireOpen { .. }
            ) && case
                .certification
                .primitive_family_coverage_matrix
                .entries
                .iter()
                .any(|entry| entry.family == "WireOpen(n)" && entry.observed)
        }));
        assert!(corpus.cases.iter().any(|case| {
            matches!(
                case.primitive,
                WorthMilestoneOnePrimitiveCase::SolidShell { .. }
            ) && case
                .certification
                .primitive_family_coverage_matrix
                .entries
                .iter()
                .any(|entry| entry.family == "SolidShell(f)" && entry.observed)
        }));
        assert!(corpus.cases.iter().any(|case| {
            matches!(
                case.primitive,
                WorthMilestoneOnePrimitiveCase::NmtEdgeFan { .. }
            ) && case
                .certification
                .primitive_family_coverage_matrix
                .entries
                .iter()
                .any(|entry| entry.family == "NmtEdgeFan(k)" && entry.observed)
        }));
    }

    #[test]
    fn default_primitive_corpus_includes_smallest_generic_hostile_and_out_of_class_members() {
        let corpus = certify_milestone_one_default_primitive_corpus(
            || {
                crate::facade::worth_milestone_one_runtime_builder()
                    .expect("worth milestone one runtime builder")
                    .build()
            },
            "cert-default-corpus",
        )
        .expect("default primitive corpus certification should succeed");

        assert!(corpus
            .cases
            .iter()
            .any(|case| case.role == WorthMilestoneOnePrimitiveRole::Smallest));
        assert!(corpus
            .cases
            .iter()
            .any(|case| case.role == WorthMilestoneOnePrimitiveRole::Generic));
        assert!(corpus
            .cases
            .iter()
            .any(|case| case.role == WorthMilestoneOnePrimitiveRole::HostileAdmitted));
        assert!(corpus
            .rejected_cases
            .iter()
            .all(|case| case.expected_outcome == WorthMilestoneOnePrimitiveExpectedOutcome::Reject));
        assert!(corpus.rejected_cases.iter().all(|case| !case.rejection.detail.is_empty()));
        assert!(corpus.rejected_cases.iter().all(|case| {
            matches!(
                case.rejection.rejection_class.as_str(),
                "InvalidParameter" | "CommitConflict" | "AuthorityError"
            )
        }));
        assert!(corpus
            .rejected_cases
            .iter()
            .any(|case| case.role == WorthMilestoneOnePrimitiveRole::OutOfClass));
        assert!(corpus
            .rejected_cases
            .iter()
            .any(|case| case.family == "WireClosed(n)"));
        assert!(corpus.coverage_matrix.entries.iter().all(|entry| {
            entry.admitted_smallest_count >= 1
                && entry.admitted_generic_count >= 1
                && entry.admitted_hostile_count >= 1
                && entry.rejected_out_of_class_count >= 1
                && entry.role_closure_complete
        }));
        assert!(corpus.parity_report.entries.iter().all(|entry| {
            entry.mainline_case_count >= 3
                && entry.branch_local_case_count == entry.mainline_case_count
                && entry.mainline_replay_checked_case_count == entry.mainline_case_count
                && entry.mainline_replay_verified_case_count == entry.mainline_case_count
                && entry.mainline_digest_parity_case_count == entry.mainline_case_count
                && entry.branch_local_replay_checked_case_count == entry.branch_local_case_count
                && entry.branch_local_replay_verified_case_count == entry.branch_local_case_count
                && entry.branch_local_digest_parity_case_count == entry.branch_local_case_count
                && entry.cross_branch_parity_case_count == entry.mainline_case_count
                && entry.parity_closure_complete
        }));
    }

    #[test]
    fn branch_local_default_primitive_corpus_preserves_branch_local_reports_for_admitted_cases() {
        let scenarios = milestone_one_default_primitive_corpus()
            .into_iter()
            .filter(|scenario| scenario.expected_outcome == WorthMilestoneOnePrimitiveExpectedOutcome::Admit)
            .collect::<Vec<_>>();

        let corpus = certify_milestone_one_branch_local_primitive_scenarios(
            &mut || {
                crate::facade::worth_milestone_one_runtime_builder()
                    .expect("worth milestone one runtime builder")
                    .build()
            },
            "cert-branch-corpus",
            "feature",
            &scenarios,
        )
        .expect("branch-local primitive corpus certification should succeed");

        assert!(!corpus.cases.is_empty());
        assert!(corpus.rejected_cases.is_empty());
        assert!(corpus.cases.iter().all(|case| case
            .certification
            .branch_local_topology_report
            .branch_local));
        assert!(corpus.cases.iter().all(|case| {
            case.certification.branch_local_topology_report.branch_id.0 == "feature"
                && case
                    .certification
                    .milestone_1_replay_parity_report
                    .branch_id
                    .0
                    == "feature"
        }));
        assert!(corpus.coverage_matrix.entries.iter().all(|entry| {
            entry.admitted_smallest_count >= 1
                && entry.admitted_generic_count >= 1
                && entry.admitted_hostile_count >= 1
                && entry.rejected_out_of_class_count == 0
                && !entry.role_closure_complete
        }));
        assert!(corpus.parity_report.entries.iter().all(|entry| {
            entry.mainline_case_count >= 3
                && entry.branch_local_case_count == entry.mainline_case_count
                && entry.mainline_replay_checked_case_count == entry.mainline_case_count
                && entry.mainline_replay_verified_case_count == entry.mainline_case_count
                && entry.mainline_digest_parity_case_count == entry.mainline_case_count
                && entry.branch_local_replay_checked_case_count == entry.branch_local_case_count
                && entry.branch_local_replay_verified_case_count == entry.branch_local_case_count
                && entry.branch_local_digest_parity_case_count == entry.branch_local_case_count
                && entry.cross_branch_parity_case_count == 0
                && entry.parity_closure_complete
        }));
    }

    #[test]
    fn admitted_family_parameter_sweeps_certify_across_ranges() {
        let cases = [
            (WorthMilestoneOnePrimitiveCase::WireOpen { half_edge_count: 1 }, "WireOpen(n)"),
            (WorthMilestoneOnePrimitiveCase::WireOpen { half_edge_count: 8 }, "WireOpen(n)"),
            (WorthMilestoneOnePrimitiveCase::WireClosed { half_edge_count: 3 }, "WireClosed(n)"),
            (WorthMilestoneOnePrimitiveCase::WireClosed { half_edge_count: 9 }, "WireClosed(n)"),
            (WorthMilestoneOnePrimitiveCase::WireBranch { branch_count: 3 }, "WireBranch(k)"),
            (WorthMilestoneOnePrimitiveCase::WireBranch { branch_count: 9 }, "WireBranch(k)"),
            (WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 3 }, "SheetDisk(n)"),
            (WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 10 }, "SheetDisk(n)"),
            (WorthMilestoneOnePrimitiveCase::SheetPatch { face_count: 2 }, "SheetPatch(f)"),
            (WorthMilestoneOnePrimitiveCase::SheetPatch { face_count: 8 }, "SheetPatch(f)"),
            (WorthMilestoneOnePrimitiveCase::SolidShell { face_count: 4 }, "SolidShell(f)"),
            (WorthMilestoneOnePrimitiveCase::SolidShell { face_count: 10 }, "SolidShell(f)"),
            (WorthMilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 3 }, "NmtEdgeFan(k)"),
            (WorthMilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 9 }, "NmtEdgeFan(k)"),
        ];

        for (index, (primitive, family)) in cases.into_iter().enumerate() {
            let mut runtime = crate::facade::worth_milestone_one_runtime_builder()
                .expect("worth milestone one runtime builder")
                .build();
            let verified = worth_schema::facade::seed_milestone_one_primitive(
                &mut runtime,
                &format!("sweep.case.{index}"),
                &primitive,
            )
            .expect("admitted primitive commit");
            let report = certify_verified_topology_commit(&mut runtime, &verified)
                .expect("swept primitive certification should succeed");

            assert!(report.named_truth_validated, "{family} should retain naming truth");
            assert!(report.topology_validated, "{family} should pass topology validation");
            assert!(report
                .primitive_family_coverage_matrix
                .entries
                .iter()
                .any(|entry| entry.family == family && entry.observed));
            assert!(report
                .milestone_1_replay_parity_report
                .relational_replay_checked);
            assert!(report
                .milestone_1_replay_parity_report
                .relational_replay_verified);
        }
    }

    #[test]
    fn branch_local_parameter_sweeps_preserve_branch_and_replay_truth() {
        let cases = [
            (WorthMilestoneOnePrimitiveCase::WireBranch { branch_count: 8 }, "WireBranch(k)"),
            (WorthMilestoneOnePrimitiveCase::SheetPatch { face_count: 7 }, "SheetPatch(f)"),
            (WorthMilestoneOnePrimitiveCase::SolidShell { face_count: 9 }, "SolidShell(f)"),
            (WorthMilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 8 }, "NmtEdgeFan(k)"),
        ];

        for (index, (primitive, family)) in cases.into_iter().enumerate() {
            let mut runtime = crate::facade::worth_milestone_one_runtime_builder()
                .expect("worth milestone one runtime builder")
                .build();
            runtime
                .history_authority()
                .create_branch(BranchId("feature".to_string()), &BranchId("main".to_string()))
                .expect("feature branch");
            let verified = worth_schema::facade::seed_milestone_one_primitive_on_branch(
                &mut runtime,
                &format!("branch-sweep.case.{index}"),
                &primitive,
                BranchId("feature".to_string()),
                WorthMutationOrigin::BranchLocalApplication,
            )
            .expect("branch-local admitted primitive commit");
            let report = certify_verified_topology_commit(&mut runtime, &verified)
                .expect("branch-local swept primitive certification should succeed");

            assert!(report.branch_local_topology_report.branch_local, "{family} should remain branch-local");
            assert_eq!(report.branch_local_topology_report.branch_id.0, "feature");
            assert_eq!(report.milestone_1_replay_parity_report.branch_id.0, "feature");
            assert!(report
                .milestone_1_replay_parity_report
                .relational_replay_checked);
            assert!(report
                .milestone_1_replay_parity_report
                .relational_replay_verified);
        }
    }

    #[test]
    fn milestone_one_closeout_emits_bootstrap_and_corpus_proof_surfaces() {
        let report = certify_milestone_one_closeout(
            || {
                crate::facade::worth_milestone_one_runtime_builder()
                    .expect("worth milestone one runtime builder")
                    .build()
            },
            "milestone-one-closeout",
        )
        .expect("milestone one closeout should succeed");

        assert!(report.seeded_bootstrap.named_truth_validated);
        assert!(report.seeded_bootstrap.topology_validated);
        assert_eq!(report.seeded_bootstrap.topology_validation_report.rows.len(), 5);
        assert!(!report
            .seeded_bootstrap
            .branch_local_topology_report
            .branch_local);
        assert!(!report
            .seeded_bootstrap
            .milestone_1_replay_parity_report
            .relational_replay_checked);
        assert_eq!(
            report.seeded_bootstrap.milestone_1_replay_parity_report.parity_status,
            WorthReplayParityStatus::NotChecked
        );
        assert_eq!(report.seeded_bootstrap.counters.topology_entity_upsert_count, 11);
        assert_eq!(report.seeded_bootstrap.counters.topology_relation_upsert_count, 14);
        assert!(report.bridge_proof.route_record_count >= 1);
        assert!(report.bridge_proof.historical_evaluation_record_count >= 1);
        assert!(report.bridge_proof.bridge_routing_digest.row_count >= 1);
        assert!(report.bridge_proof.bridge_historical_evaluation_digest.row_count >= 1);

        assert!(!report.primitive_corpus.cases.is_empty());
        assert!(!report.primitive_corpus.rejected_cases.is_empty());
        assert!(report
            .primitive_corpus
            .coverage_matrix
            .entries
            .iter()
            .all(|entry| entry.role_closure_complete));
        assert!(report
            .primitive_corpus
            .parity_report
            .entries
            .iter()
            .all(|entry| entry.parity_closure_complete));
    }
}
