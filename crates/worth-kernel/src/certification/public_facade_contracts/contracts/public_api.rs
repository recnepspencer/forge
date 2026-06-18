mod public_api_construction_branch_preview;
mod public_api_planar_boolean_7_0_closeout;
mod public_api_planar_boolean_canonical_segments;
mod public_api_planar_boolean_collinear_relations;
mod public_api_planar_boolean_common_plane_local_frame_selection;
mod public_api_planar_boolean_common_plane_operand_a_projection_consumption;
mod public_api_planar_boolean_common_plane_operand_b_projection_consumption;
mod public_api_planar_boolean_common_plane_precision_agreement;
mod public_api_planar_boolean_common_plane_reduced_operand_pair;
mod public_api_planar_boolean_common_plane_reduction;
mod public_api_planar_boolean_common_plane_shared_plane_identity;
mod public_api_planar_boolean_edge_split_request;
mod public_api_planar_boolean_edge_split_scope_admission;
mod public_api_planar_boolean_edge_splitting;
mod public_api_planar_boolean_edge_splitting_metaboss_closeout;
mod public_api_planar_boolean_edge_splitting_public_contract;
mod public_api_planar_boolean_entry;
mod public_api_planar_boolean_entry_basis;
mod public_api_planar_boolean_event_extraction_denials;
mod public_api_planar_boolean_event_extraction_metaboss;
mod public_api_planar_boolean_event_extraction_request;
mod public_api_planar_boolean_event_ledger;
mod public_api_planar_boolean_event_predicate_binding;
mod public_api_planar_boolean_interval_events;
mod public_api_planar_boolean_local_frame_evidence;
mod public_api_planar_boolean_operand_a_projection_evidence;
mod public_api_planar_boolean_operand_b_projection_evidence;
mod public_api_planar_boolean_point_events;
mod public_api_planar_boolean_precision_evidence;
mod public_api_planar_boolean_segment_carriers;
mod public_api_planar_boolean_segment_pair_enumeration;
mod public_api_planar_boolean_shared_plane_evidence;
mod public_api_workload_catalog;

#[cfg(test)]
mod workload_vocabulary {
    use topology::facade::TopologyWorkload;
    use worth_kernel::workload_composition::{
        OperatorReadyWorkload, OperatorSupportPosture, OperatorWorkloadError,
        UnsupportedOperatorFamily, WorkloadCatalog, WorkloadCompositionError, WorkloadOperator,
        WorkloadOperatorFamily, WorkloadStageRequirement, WorthWorkload, WorthWorkloadParts,
    };
    use worth_spatial::facade::workload_vocabulary::{
        DiagnosticWorkload, GeometryBindingWorkload, ProjectionWorkload, ResponseWorkload,
        RetainedReplayWorkload, SurfaceSupportWorkload, TransformWorkload, WorkloadEvidenceLedger,
        WorkloadEvidenceLedgerError, WorkloadEvidenceRow, WorkloadEvidenceStage,
    };

    #[test]
    fn workload_operator_rejects_simple_receipt_ledger_before_execution() {
        let workload = certified_workload();
        assert_operator_ready(&workload);

        let error = WorkloadOperator::for_family(WorkloadOperatorFamily::CoplanarOverlap)
            .requiring(WorkloadStageRequirement::RetainedReplay)
            .declared_by_query("kernel simple receipt ledger probe")
            .admit_for(&workload)
            .expect_err("operator harness must reject simple receipt counters");

        assert!(matches!(error, OperatorWorkloadError::EvidenceGuard(_)));
        assert!(error.human_reason().contains("real topology"));
    }

    #[test]
    fn operator_harness_denies_unsupported_family_without_stub_execution() {
        let workload = certified_workload();
        let error = WorkloadOperator::for_family(WorkloadOperatorFamily::Unsupported(
            UnsupportedOperatorFamily::BooleanDifference,
        ))
        .requiring(WorkloadStageRequirement::RetainedReplay)
        .declared_by_query("kernel unsupported boolean difference probe")
        .admit_for(&workload)
        .expect_err("unsupported family must deny before execution");

        match error {
            OperatorWorkloadError::UnsupportedOperatorFamily { family, support } => {
                assert_eq!(family, UnsupportedOperatorFamily::BooleanDifference);
                assert_eq!(support.posture(), OperatorSupportPosture::Unsupported);
                assert!(support.human_reason().contains("not supported"));
                assert!(!support.query_support_digest().is_empty());
                assert!(!support.query_support_digest().contains("operator-support"));
                assert!(!support
                    .query_support_digest()
                    .contains("unsupported.boolean_difference"));
            }
            other => panic!("expected unsupported family denial, got {other:?}"),
        }
    }

    #[test]
    fn operator_harness_rejects_boolean_evidence_requirements_as_non_operator_stages() {
        run_with_large_stack(|| {
            let workload = operator_ready_catalog_workload();
            let error = WorkloadOperator::for_family(WorkloadOperatorFamily::CoplanarOverlap)
                .requiring(WorkloadStageRequirement::BooleanDeclarationEntry)
                .declared_by_query("kernel boolean stage misuse probe")
                .admit_for(&workload)
                .expect_err("operator harness must reject boolean-only workload requirements");

            assert_eq!(
                error,
                OperatorWorkloadError::UnsupportedRequirement(
                    WorkloadStageRequirement::BooleanDeclarationEntry
                )
            );
            assert!(error
                .human_reason()
                .contains("not a valid operator workload requirement"));
        });
    }

    #[test]
    fn workload_vocabulary_rejects_mismatched_evidence_ledger() {
        let parts = certified_workload_parts_with_mismatched_projection_evidence();
        let error = WorthWorkload::compose(parts)
            .expect_err("workload composition must reject mismatched projection evidence");

        assert_eq!(
            error,
            WorkloadCompositionError::MismatchedEvidenceStage(WorkloadEvidenceStage::Projection)
        );
        assert!(error.human_reason().contains("projection evidence"));
    }

    #[test]
    fn workload_vocabulary_rejects_missing_evidence_stage() {
        let error = incomplete_ledger_without_projection_evidence()
            .expect_err("complete ledger certification must reject missing projection evidence");

        assert_eq!(
            error,
            WorkloadEvidenceLedgerError::MissingAuthorityStage(WorkloadEvidenceStage::Projection)
        );
        assert!(error.human_reason().contains("missing projection evidence"));
    }

    fn assert_operator_ready(workload: &impl OperatorReadyWorkload) {
        assert_eq!(workload.evidence_rows(), 8);
    }

    fn certified_workload() -> WorthWorkload {
        WorthWorkload::compose(certified_workload_parts()).expect("worth workload should certify")
    }

    fn operator_ready_catalog_workload() -> WorthWorkload {
        WorkloadCatalog::cube()
            .with_retained_replay_artifacts()
            .build()
            .expect("catalog cube workload should build")
            .into_workload()
    }

    fn run_with_large_stack(body: impl FnOnce() + Send + 'static) {
        std::thread::Builder::new()
            .name("workload-vocabulary-contract".to_string())
            .stack_size(16 * 1024 * 1024)
            .spawn(body)
            .expect("workload vocabulary contract thread should spawn")
            .join()
            .expect("workload vocabulary contract thread should finish");
    }

    fn certified_workload_parts_with_mismatched_projection_evidence() -> WorthWorkloadParts {
        certified_workload_parts_with_projection_mode(ProjectionEvidenceMode::OtherReceipt)
    }

    fn incomplete_ledger_without_projection_evidence() -> Result<(), WorkloadEvidenceLedgerError> {
        let parts = workload_receipts();
        let ledger_rows = receipt_backed_rows(&parts, ProjectionEvidenceMode::Missing);
        WorkloadEvidenceLedger::from_rows(ledger_rows)
            .expect("incomplete ledger rows should stay inspectable")
            .certify_complete()
            .map(|_| ())
    }

    fn certified_workload_parts() -> WorthWorkloadParts {
        certified_workload_parts_with_projection_mode(ProjectionEvidenceMode::MatchingReceipt)
    }

    fn certified_workload_parts_with_projection_mode(
        mode: ProjectionEvidenceMode,
    ) -> WorthWorkloadParts {
        let parts = workload_receipts();
        let ledger_rows = receipt_backed_rows(&parts, mode);
        let ledger = WorkloadEvidenceLedger::from_rows(ledger_rows)
            .expect("ledger should certify")
            .certify_complete()
            .expect("complete receipt-backed ledger should certify");

        WorthWorkloadParts {
            topology: parts.topology,
            geometry_binding: parts.geometry,
            surface_support: parts.support,
            projection: parts.projection,
            transform: parts.transform,
            retained_replay: parts.replay,
            diagnostics: parts.diagnostics,
            response: parts.response,
            evidence_ledger: ledger,
        }
    }

    struct WorkloadReceipts {
        topology: topology::facade::TopologyWorkloadReceipt,
        geometry: worth_spatial::facade::workload_vocabulary::GeometryBindingWorkloadReceipt,
        support: worth_spatial::facade::workload_vocabulary::SurfaceSupportWorkloadReceipt,
        projection: worth_spatial::facade::workload_vocabulary::ProjectionWorkloadReceipt,
        transform: worth_spatial::facade::workload_vocabulary::TransformWorkloadReceipt,
        replay: worth_spatial::facade::workload_vocabulary::RetainedReplayWorkloadReceipt,
        diagnostics: worth_spatial::facade::workload_vocabulary::DiagnosticWorkloadReceipt,
        response: worth_spatial::facade::workload_vocabulary::ResponseWorkloadReceipt,
    }

    #[derive(Clone, Copy)]
    enum ProjectionEvidenceMode {
        MatchingReceipt,
        OtherReceipt,
        Missing,
    }

    fn workload_receipts() -> WorkloadReceipts {
        workload_receipts_named("topology seed", ".topology.seed")
    }

    fn workload_receipts_named(topology_name: &str, query_declaration: &str) -> WorkloadReceipts {
        let topology = TopologyWorkload::declared(topology_name)
            .from_query_declaration(query_declaration)
            .expect("topology workload should certify");
        let geometry = GeometryBindingWorkload::for_topology_receipt(&topology)
            .admit()
            .expect("geometry binding should certify");
        let support = SurfaceSupportWorkload::for_geometry_binding(&geometry)
            .admit()
            .expect("surface support should certify");
        let projection = ProjectionWorkload::for_surface_support(&support)
            .admit()
            .expect("projection should certify");
        let transform = TransformWorkload::for_projection(&projection)
            .admit()
            .expect("transform should certify");
        let replay = RetainedReplayWorkload::for_transform(&transform)
            .admit()
            .expect("retained replay should certify");
        let diagnostics = DiagnosticWorkload::for_retained_replay(&replay)
            .admit()
            .expect("diagnostics should certify");
        let response = ResponseWorkload::for_diagnostics(&diagnostics)
            .admit()
            .expect("response should certify");

        WorkloadReceipts {
            topology,
            geometry,
            support,
            projection,
            transform,
            replay,
            diagnostics,
            response,
        }
    }

    fn receipt_backed_rows(
        parts: &WorkloadReceipts,
        mode: ProjectionEvidenceMode,
    ) -> Vec<WorkloadEvidenceRow> {
        let mut rows = vec![
            WorkloadEvidenceRow::from_topology_receipt(&parts.topology),
            WorkloadEvidenceRow::from_geometry_binding_receipt(&parts.geometry),
            WorkloadEvidenceRow::from_surface_support_receipt(&parts.support),
            WorkloadEvidenceRow::from_transform_receipt(&parts.transform),
            WorkloadEvidenceRow::from_retained_replay_receipt(&parts.replay),
            WorkloadEvidenceRow::from_diagnostic_receipt(&parts.diagnostics),
            WorkloadEvidenceRow::from_response_receipt(&parts.response),
        ];
        match mode {
            ProjectionEvidenceMode::MatchingReceipt => rows.insert(
                3,
                WorkloadEvidenceRow::from_projection_receipt(&parts.projection),
            ),
            ProjectionEvidenceMode::OtherReceipt => {
                let other_parts = workload_receipts_named("other topology seed", ".topology.other");
                rows.insert(
                    3,
                    WorkloadEvidenceRow::from_projection_receipt(&other_parts.projection),
                );
            }
            ProjectionEvidenceMode::Missing => {}
        }
        rows
    }
}
