#[cfg(test)]
mod certification_tests {
    use forge_relational::facade::history::BranchId;
    use worth_schema::facade::{
        RawWorthTopologyIntent,
        WorthMilestoneOnePrimitiveCase, WorthMilestoneOnePrimitiveExpectedOutcome,
        WorthMilestoneOnePrimitiveRole, WorthMutationOrigin, WorthTopologyAuthority,
        WorthTopologyMutation,
    };
    use worth_schema::facade::{WorthShellInterpretationClass, WorthWireInterpretationClass};
    use crate::certification::report::WorthReplayParityStatus;
    use crate::facade::{
        milestone_one_closeout_requirements, milestone_one_closeout_suite_definition,
        milestone_two_closeout_requirements, milestone_two_closeout_suite_definition,
        WorthCertificationCanonicalRow,
        WorthCertificationParityRow, WorthCertificationRejectionRow,
        WorthCertificationRequiredOutput, WorthCertificationSuiteDefinition,
        WorthAdmittedRangeSweepReport, WorthBridgeProofReport, WorthDeterministicDigest,
        WorthDerivedEquivalenceContractAggregateReport, WorthDerivedFallbackAggregateReport,
        WorthDerivedInvalidationAggregateReport, WorthDerivedRebuildAggregateReport,
        WorthDerivedValidatorCoverageReport, WorthFailureLocalityReport, WorthMilestoneOneCounters,
        WorthMilestoneTwoCloseoutReport, WorthMilestoneTwoCounters,
        WorthPrimitiveCorpusCoverageMatrix, WorthPrimitiveCorpusParityReport,
    };

    use crate::certification::{
        certify_milestone_one_branch_local_primitive_scenarios,
        certify_milestone_one_closeout,
        certify_milestone_one_default_primitive_corpus,
        certify_milestone_one_primitive_corpus, certify_milestone_one_read_view,
        certify_milestone_two_default_derived_corpus, certify_milestone_two_read_view,
        certify_milestone_two_verified_topology_commit, certify_milestone_two_closeout,
        certify_verified_topology_commit,
    };
    use crate::fixtures::authored_topology::milestone_one_default_corpus_scenarios;
    use crate::fixtures::branch_replay_cases::milestone_one_default_branch_local_admitted_scenarios;
    use crate::fixtures::validated_topology::{
        seeded_bootstrap, verified_primitive, verified_primitive_on_branch,
    };

    #[test]
    fn seeded_bootstrap_earns_milestone_one_certification_report() {
        let mut runtime = crate::facade::worth_milestone_one_runtime_builder()
            .expect("worth milestone one runtime builder")
            .build();

        let seeded = seeded_bootstrap(&mut runtime, "cert-harness")
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
            .any(|row| row.validator == "ownership" && row.status == "passed"));
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
    fn seeded_bootstrap_earns_direct_milestone_two_read_report() {
        let mut runtime = crate::facade::worth_milestone_one_runtime_builder()
            .expect("worth milestone one runtime builder")
            .build();

        let seeded = seeded_bootstrap(&mut runtime, "cert-m2-read").expect("seed worth topology");
        let read_view = runtime
            .read_truth()
            .read_snapshot(&seeded.snapshot)
            .expect("worth snapshot read");

        let report = certify_milestone_two_read_view(&read_view, seeded.read_basis)
            .expect("milestone two read certification should succeed");

        assert_eq!(report.materialized_topology_digest.algorithm, "fnv1a64");
        assert_eq!(report.interpreted_topology_digest.algorithm, "fnv1a64");
        assert_eq!(report.derived_validation_digest.algorithm, "fnv1a64");
        assert!(report.derived_invalidation_report.topology_touched);
        assert!(report.derived_rebuild_report.whole_view_rebuild);
        assert!(report.derived_fallback_report.whole_view_materialization);
        assert_eq!(report.milestone_2_counter_report.derived_read_count, 1);
        assert_eq!(
            report.read_artifact.interpretations,
            report.certified_interpretation.interpretations
        );
    }

    #[test]
    fn verified_commit_earns_direct_milestone_two_read_report() {
        let mut runtime = crate::facade::worth_milestone_one_runtime_builder()
            .expect("worth milestone one runtime builder")
            .build();

        let verified = verified_primitive(
            &mut runtime,
            "cert-m2-verified",
            &WorthMilestoneOnePrimitiveCase::WireBranch { branch_count: 4 },
        )
        .expect("verified primitive");

        let report = certify_milestone_two_verified_topology_commit(&mut runtime, &verified)
            .expect("milestone two verified certification should succeed");

        assert!(report.materialized_topology_digest.row_count > 0);
        assert!(report.interpreted_topology_digest.row_count > 0);
        assert!(report.derived_validation_digest.row_count > 0);
        assert!(
            report
                .derived_replay_parity_report
                .relational_replay_checked
        );
    }

    #[test]
    fn default_primitive_corpus_earns_direct_milestone_two_derived_corpus_report() {
        let report = certify_milestone_two_default_derived_corpus(
            || {
                crate::facade::worth_milestone_one_runtime_builder()
                    .expect("worth milestone one runtime builder")
                    .build()
            },
            "cert-m2-corpus",
        )
        .expect("milestone two derived corpus");

        assert!(report.materialized_topology_digest.row_count > 0);
        assert!(report.interpreted_topology_digest.row_count > 0);
        assert!(report.derived_validation_digest.row_count > 0);
        assert!(report
            .derived_family_coverage_matrix
            .rows
            .iter()
            .any(|row| row.family == "WireOpen(n)" && row.coverage_complete));
        assert!(report
            .derived_family_parity_matrix
            .rows
            .iter()
            .any(|row| row.family == "WireBranch(k)" && row.parity_complete));
        assert!(report.bridge_routing_digest.row_count > 0);
        assert!(report.bridge_historical_evaluation_digest.row_count > 0);
        assert!(report.milestone_2_counter_report.derived_read_count > 0);
    }

    #[test]
    fn public_facade_exports_closeout_field_types() {
        fn _accepts_surface_types(
            _digest: WorthDeterministicDigest,
            _coverage: WorthPrimitiveCorpusCoverageMatrix,
            _parity: WorthPrimitiveCorpusParityReport,
            _sweeps: WorthAdmittedRangeSweepReport,
            _failures: WorthFailureLocalityReport,
            _bridge_family_coverage: crate::facade::WorthBridgeFamilyCoverageReport,
            _bridge: WorthBridgeProofReport,
            _counters: WorthMilestoneOneCounters,
        ) {
        }

        fn _closeout_fields_are_publicly_reachable(
            report: crate::facade::WorthMilestoneOneCloseoutReport,
        ) {
            let _: WorthAdmittedRangeSweepReport = report.admitted_range_sweep_report;
            let _: WorthFailureLocalityReport = report.failure_locality_report;
            let _: crate::facade::WorthBridgeFamilyCoverageReport =
                report.bridge_family_coverage_report;
            let _: WorthBridgeProofReport = report.bridge_proof_report;
            let _: WorthMilestoneOneCounters = report.milestone_1_counter_report;
        }

        fn _milestone_two_closeout_fields_are_publicly_reachable(
            report: WorthMilestoneTwoCloseoutReport,
        ) {
            let _: WorthDerivedValidatorCoverageReport = report.derived_validator_coverage_report;
            let _: WorthDerivedInvalidationAggregateReport = report.derived_invalidation_report;
            let _: WorthDerivedRebuildAggregateReport = report.derived_rebuild_report;
            let _: WorthDerivedEquivalenceContractAggregateReport =
                report.derived_equivalence_contract_report;
            let _: WorthDerivedFallbackAggregateReport = report.derived_fallback_report;
            let _: WorthMilestoneTwoCounters = report.milestone_2_counter_report;
        }
    }

    #[test]
    fn verified_topology_commit_is_the_canonical_certification_input() {
        let mut runtime = crate::facade::worth_milestone_one_runtime_builder()
            .expect("worth milestone one runtime builder")
            .build();

        let _seeded = seeded_bootstrap(&mut runtime, "cert-verified-commit")
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

        let _seeded = seeded_bootstrap(&mut runtime, "cert-branch-local")
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

        let verified = verified_primitive(
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
    fn primitive_corpus_reports_keep_the_full_canonical_family_set_even_when_input_is_partial() {
        let corpus = certify_milestone_one_primitive_corpus(
            || {
                crate::facade::worth_milestone_one_runtime_builder()
                    .expect("worth milestone one runtime builder")
                    .build()
            },
            "cert-partial-corpus",
            &[WorthMilestoneOnePrimitiveCase::WireOpen { half_edge_count: 4 }],
        )
        .expect("partial primitive corpus certification should succeed");

        assert_eq!(corpus.coverage_matrix.entries.len(), 7);
        assert_eq!(corpus.parity_report.entries.len(), 7);
        assert!(corpus
            .coverage_matrix
            .entries
            .iter()
            .any(|entry| entry.family == "WireOpen(n)" && entry.admitted_generic_count == 1));
        assert!(corpus
            .coverage_matrix
            .entries
            .iter()
            .any(|entry| entry.family == "SolidShell(f)" && !entry.role_closure_complete));
        assert!(corpus
            .parity_report
            .entries
            .iter()
            .any(|entry| entry.family == "NmtEdgeFan(k)" && !entry.parity_closure_complete));
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
                "OutOfClass" | "IllegalAdmittedTopology" | "AuthorityBlocked" | "InvariantFailure"
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
        assert!(corpus
            .parity_report
            .entries
            .iter()
            .all(|entry| entry.branch_ids.iter().any(|branch| branch == "main")));
        assert!(corpus
            .parity_report
            .entries
            .iter()
            .all(|entry| entry.branch_ids.iter().any(|branch| branch == "feature")));
    }

    #[test]
    fn branch_local_default_primitive_corpus_preserves_branch_local_reports_for_admitted_cases() {
        let scenarios = milestone_one_default_branch_local_admitted_scenarios();

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
            let verified = verified_primitive(&mut runtime, &format!("sweep.case.{index}"), &primitive)
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
            let verified = verified_primitive_on_branch(
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
        assert_eq!(report.topology_truth_digest.algorithm, "fnv1a64");
        assert_eq!(report.naming_truth_digest.algorithm, "fnv1a64");
        assert_eq!(report.topology_validation_digest.algorithm, "fnv1a64");
        assert!(report.topology_truth_digest.row_count >= 1);
        assert!(report.naming_truth_digest.row_count >= 1);
        assert!(report.topology_validation_digest.row_count >= 1);
        assert!(report.topology_validation_report.rows.len() >= 5);
        assert!(report.topology_localization_report.topology_entities.len() >= 1);
        assert!(report.topology_localization_report.topology_relations.len() >= 1);
        assert!(report.naming_attachment_report.fully_named);
        assert!(report.naming_attachment_report.attachments.len() >= 1);
        assert!(report
            .primitive_family_coverage_matrix
            .entries
            .iter()
            .all(|entry| entry.role_closure_complete));
        assert!(report
            .primitive_corpus_parity_report
            .entries
            .iter()
            .all(|entry| entry.parity_closure_complete));
        assert!(report
            .primitive_corpus_parity_report
            .entries
            .iter()
            .all(|entry| entry.branch_ids.iter().any(|branch| branch == "main")));
        assert!(report
            .primitive_corpus_parity_report
            .entries
            .iter()
            .all(|entry| entry.branch_ids.iter().any(|branch| branch == "feature")));
        assert!(report
            .admitted_range_sweep_report
            .rows
            .iter()
            .all(|row| row.sweep_closure_complete
                && row.out_of_class_case_count >= 1
                && row.out_of_class_rejection_count >= 1));
        assert!(report
            .topology_validation_report
            .rows
            .iter()
            .any(|row| row.source == "seeded_bootstrap"
                && row.family == "SeededBootstrap"
                && row.validator == "ownership"));
        assert!(report
            .validator_coverage_report
            .rows
            .iter()
            .any(|row| row.family == "SeededBootstrap"
                && row.validator == "ownership"
                && row.passed_count >= 1));
        assert!(report
            .validator_coverage_report
            .rows
            .iter()
            .any(|row| row.family == "WireBranch(k)"
                && row.validator == "vertex_branching"
                && row.passed_count >= 1));
        assert!(report
            .validator_coverage_report
            .rows
            .iter()
            .any(|row| row.family == "SolidShell(f)"
                && row.validator == "shell_closure"
                && row.passed_count >= 1));
        assert!(report
            .validator_coverage_report
            .rows
            .iter()
            .any(|row| row.family == "NmtEdgeFan(k)"
                && row.validator == "radial"
                && row.passed_count >= 1));
        assert!(report.branch_local_topology_report.mainline_case_count >= 1);
        assert!(report.branch_local_topology_report.branch_local_case_count >= 1);
        assert!(report
            .branch_local_topology_report
            .branch_ids
            .iter()
            .any(|branch| branch == "main"));
        assert!(report
            .branch_local_topology_report
            .branch_ids
            .iter()
            .any(|branch| branch == "feature"));
        assert!(report
            .branch_local_topology_report
            .branch_local_closure_complete);
        assert!(report.milestone_1_replay_parity_report.replay_checked_case_count >= 1);
        assert!(report.milestone_1_replay_parity_report.replay_verified_case_count >= 1);
        assert!(report
            .milestone_1_replay_parity_report
            .branch_local_replay_checked_case_count
            >= 1);
        assert!(report
            .milestone_1_replay_parity_report
            .branch_local_replay_verified_case_count
            >= 1);
        assert_eq!(report.milestone_1_replay_parity_report.replay_mismatch_case_count, 0);
        assert!(report.milestone_1_replay_parity_report.replay_closure_complete);
        assert!(report
            .rejection_class_report
            .rows
            .iter()
            .any(|row| row.family == "WireClosed(n)" && row.case_count >= 1));
        assert!(report
            .rejection_class_report
            .rows
            .iter()
            .any(|row| row.family == "WireBranch(k)"
                && row.rejection_class == "IllegalAdmittedTopology"
                && row.case_count >= 1));
        assert!(report
            .failure_locality_report
            .rows
            .iter()
            .any(|row| row.family == "WireClosed(n)"
                && row.role == "OutOfClass"
                && row.rejection_class == "OutOfClass"));
        assert!(report
            .failure_locality_report
            .rows
            .iter()
            .any(|row| row.family == "NmtEdgeFan(k)"
                && row.validator_family.as_deref() == Some("radial")
                && row.rejection_class == "IllegalAdmittedTopology"));
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
        assert_eq!(report.bridge_proof_report.proof_case_count, 7);
        assert!(report
            .bridge_proof_report
            .family_coverage_report
            .rows
            .iter()
            .all(|row| row.proof_complete
                && row.routed_case_count >= 1
                && row.historical_evaluation_count >= 1));
        assert!(report
            .bridge_family_coverage_report
            .rows
            .iter()
            .all(|row| row.proof_complete
                && row.routed_case_count >= 1
                && row.historical_evaluation_count >= 1));
        assert_eq!(
            report.bridge_family_coverage_report.rows,
            report.bridge_proof_report.family_coverage_report.rows
        );
        assert!(report.bridge_proof_report.proved_families.iter().any(|family| family == "WireOpen(n)"));
        assert!(report.bridge_proof_report.proved_families.iter().any(|family| family == "WireClosed(n)"));
        assert!(report.bridge_proof_report.proved_families.iter().any(|family| family == "WireBranch(k)"));
        assert!(report.bridge_proof_report.proved_families.iter().any(|family| family == "SheetDisk(n)"));
        assert!(report.bridge_proof_report.proved_families.iter().any(|family| family == "SheetPatch(f)"));
        assert!(report.bridge_proof_report.proved_families.iter().any(|family| family == "SolidShell(f)"));
        assert!(report.bridge_proof_report.proved_families.iter().any(|family| family == "NmtEdgeFan(k)"));
        assert!(report.bridge_proof_report.route_record_count >= 1);
        assert!(report.bridge_proof_report.historical_evaluation_record_count >= 1);
        assert!(report.bridge_proof_report.bridge_routing_digest.row_count >= 1);
        assert!(report.bridge_proof_report.bridge_historical_evaluation_digest.row_count >= 1);

        assert!(!report.primitive_corpus.cases.is_empty());
        assert!(!report.primitive_corpus.rejected_cases.is_empty());
        assert_eq!(report.illegal_topology_rejection_report.case_count, 7);
        assert_eq!(report.illegal_topology_rejection_report.cases.len(), 7);
        assert_eq!(report.milestone_1_counter_report.commit_boundary_rejection_count, 7);
        assert!(report.milestone_1_counter_report.topology_entity_upsert_count >= 11);
        assert!(report.milestone_1_counter_report.topology_relation_upsert_count >= 14);
        assert!(report.milestone_1_counter_report.commit_boundary_validator_count >= 6);
        assert!(report
            .illegal_topology_rejection_report
            .cases
            .iter()
            .all(|case| case.rejection.rejection_class == "IllegalAdmittedTopology"
                || case.rejection.rejection_class == "InvariantFailure"));
        assert!(report
            .illegal_topology_rejection_report
            .cases
            .iter()
            .any(|case| case.name == "non_manifold_closed_shell"));
        assert!(report
            .illegal_topology_rejection_report
            .cases
            .iter()
            .any(|case| case.name == "illegal_wire_branch"));
        assert!(report
            .illegal_topology_rejection_report
            .cases
            .iter()
            .any(|case| case.name == "broken_loop_wiring"));
        assert!(report
            .illegal_topology_rejection_report
            .cases
            .iter()
            .any(|case| case.name == "broken_radial_ring"));
        assert!(report
            .illegal_topology_rejection_report
            .cases
            .iter()
            .any(|case| case.name == "open_boundary_solid_shell"));
        assert!(report
            .illegal_topology_rejection_report
            .rejection_digest
            .row_count
            >= 7);
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

        let requirements = milestone_one_closeout_requirements();
        for family in &requirements.required_family_rows {
            assert!(report
                .primitive_family_coverage_matrix
                .entries
                .iter()
                .any(|entry| &entry.family == family));
            assert!(report
                .primitive_corpus_parity_report
                .entries
                .iter()
                .any(|entry| &entry.family == family));
            assert!(report
                .admitted_range_sweep_report
                .rows
                .iter()
                .any(|row| &row.family == family));
        }
    }

    #[test]
    fn milestone_two_closeout_emits_direct_derived_proof_surfaces() {
        let report = certify_milestone_two_closeout(
            || {
                crate::facade::worth_milestone_one_runtime_builder()
                    .expect("worth milestone one runtime builder")
                    .build()
            },
            "milestone-two-closeout",
        )
        .expect("milestone two closeout should succeed");

        assert!(report.materialized_topology_digest.row_count > 0);
        assert!(report.interpreted_topology_digest.row_count > 0);
        assert!(report.derived_validation_digest.row_count > 0);
        assert!(report.derived_truth_basis_digest.row_count > 0);
        assert!(report.bridge_routing_digest.row_count > 0);
        assert!(report.bridge_historical_evaluation_digest.row_count > 0);
        assert!(!report.derived_family_coverage_matrix.rows.is_empty());
        assert!(!report.derived_family_parity_matrix.rows.is_empty());
        assert!(!report.derived_validator_coverage_report.rows.is_empty());
        assert!(report
            .derived_validator_coverage_report
            .rows
            .iter()
            .any(|row| row.family == "WireBranch(k)" && row.validator == "vertex_branching"));
        assert!(report
            .derived_validator_coverage_report
            .rows
            .iter()
            .any(|row| row.family == "SolidShell(f)" && row.validator == "shell_closure"));
        assert!(!report.derived_invalidation_report.rows.is_empty());
        assert!(!report.derived_rebuild_report.rows.is_empty());
        assert!(!report.derived_equivalence_contract_report.rows.is_empty());
        assert!(!report.derived_fallback_report.rows.is_empty());
        assert!(!report.derived_failure_locality_report.rows.is_empty());
        assert!(!report
            .derived_branch_local_parity_report
            .branch_ids
            .is_empty());
        assert!(report.derived_replay_parity_report.replay_checked_case_count > 0);
        assert!(!report
            .derived_bridge_family_coverage_report
            .rows
            .is_empty());
        assert!(report.milestone_2_counter_report.derived_read_count > 0);
    }

    #[test]
    fn milestone_one_closeout_requirements_registry_matches_canonical_closeout_shape() {
        let requirements = milestone_one_closeout_requirements();
        let suite = milestone_one_closeout_suite_definition();

        assert_eq!(requirements.suite_name, "worth.milestone_1.closeout");
        assert_eq!(requirements.required_family_rows.len(), 7);
        assert_eq!(requirements.required_rejection_rows.len(), 7);
        assert_eq!(requirements.required_parity_rows.len(), 7);
        assert_eq!(requirements.required_bridge_rows.len(), 7);
        assert_eq!(suite.suite_name, requirements.suite_name);
        assert_eq!(suite.canonical_rows.len(), 21);
        assert_eq!(suite.rejection_rows.len(), 7);
        assert_eq!(suite.parity_rows.len(), 14);
        assert!(requirements
            .required_outputs
            .contains(&WorthCertificationRequiredOutput::BridgeFamilyCoverageReport));
        assert!(requirements
            .validator_expectations
            .iter()
            .any(|expectation| expectation.family == "WireBranch(k)"
                && expectation.validators.iter().any(|validator| validator == "vertex_branching")));
        assert!(requirements
            .validator_expectations
            .iter()
            .any(|expectation| expectation.family == "SolidShell(f)"
                && expectation.validators.iter().any(|validator| validator == "shell_closure")));
    }

    #[test]
    fn milestone_two_closeout_requirements_registry_matches_direct_derived_outputs() {
        let requirements = milestone_two_closeout_requirements();
        let suite = milestone_two_closeout_suite_definition();

        assert_eq!(requirements.suite_name, "worth.milestone_2.closeout");
        assert_eq!(requirements.required_family_rows.len(), 7);
        assert_eq!(requirements.required_rejection_rows.len(), 7);
        assert_eq!(requirements.required_parity_rows.len(), 7);
        assert_eq!(requirements.required_bridge_rows.len(), 7);
        assert_eq!(suite.suite_name, requirements.suite_name);
        assert_eq!(suite.canonical_rows.len(), 21);
        assert_eq!(suite.rejection_rows.len(), 7);
        assert_eq!(suite.parity_rows.len(), 14);
        assert!(requirements
            .required_outputs
            .contains(&WorthCertificationRequiredOutput::MaterializedTopologyDigest));
        assert!(requirements
            .required_outputs
            .contains(&WorthCertificationRequiredOutput::DerivedEquivalenceContractReport));
        assert!(requirements
            .required_outputs
            .contains(&WorthCertificationRequiredOutput::DerivedTruthBasisDigest));
        assert!(requirements
            .required_outputs
            .contains(&WorthCertificationRequiredOutput::DerivedValidatorCoverageReport));
        assert!(requirements
            .required_outputs
            .contains(&WorthCertificationRequiredOutput::DerivedBridgeFamilyCoverageReport));
        assert!(requirements
            .required_outputs
            .contains(&WorthCertificationRequiredOutput::MilestoneTwoCounterReport));
        assert!(requirements
            .validator_expectations
            .iter()
            .any(|expectation| expectation.family == "WireBranch(k)"
                && expectation.validators.iter().any(|validator| validator == "vertex_branching")));
        assert!(requirements
            .validator_expectations
            .iter()
            .any(|expectation| expectation.family == "SolidShell(f)"
                && expectation.validators.iter().any(|validator| validator == "shell_closure")));
    }

    #[test]
    fn local_certification_core_expresses_suite_rows_without_worth_specific_branching() {
        let suite = WorthCertificationSuiteDefinition {
            suite_name: "worth.test.shape".to_string(),
            canonical_rows: vec![WorthCertificationCanonicalRow {
                family: "WireOpen(n)".to_string(),
                role: "Generic".to_string(),
            }],
            rejection_rows: vec![WorthCertificationRejectionRow {
                family: "WireClosed(n)".to_string(),
                role: "OutOfClass".to_string(),
                rejection_class: "OutOfClass".to_string(),
            }],
            parity_rows: vec![WorthCertificationParityRow {
                family: "WireBranch(k)".to_string(),
                parity_kind: "branch".to_string(),
            }],
            required_outputs: vec![
                WorthCertificationRequiredOutput::TopologyTruthDigest,
                WorthCertificationRequiredOutput::FailureLocalityReport,
            ],
        };

        assert_eq!(suite.canonical_rows.len(), 1);
        assert_eq!(suite.rejection_rows.len(), 1);
        assert_eq!(suite.parity_rows.len(), 1);
        assert_eq!(suite.required_outputs.len(), 2);
    }

    #[test]
    fn worth_fixtures_provide_named_phase_inputs_for_milestone_one_closeout() {
        let authored = milestone_one_default_corpus_scenarios();
        let branch_local = milestone_one_default_branch_local_admitted_scenarios();

        assert!(!authored.is_empty());
        assert!(authored.iter().any(|scenario| {
            scenario.expected_outcome == WorthMilestoneOnePrimitiveExpectedOutcome::Reject
        }));
        assert!(!branch_local.is_empty());
        assert!(branch_local.iter().all(|scenario| {
            scenario.expected_outcome == WorthMilestoneOnePrimitiveExpectedOutcome::Admit
        }));
    }
}
