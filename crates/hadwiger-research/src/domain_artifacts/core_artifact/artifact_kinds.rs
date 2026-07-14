#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HadwigerArtifactKind {
    GraphIdentity,
    GraphVersion,
    VertexIdentity,
    EdgeIdentity,
    EmbeddingCandidate,
    UnitDistanceVerification,
    ColorabilityEncoding,
    SolverRun,
    ColorabilityVerification,
    UnsatCoreArtifact,
    GadgetDefinition,
    GadgetContract,
    GraphComposition,
    ReductionTrace,
    ProofClaim,
    AIAdvisoryArtifact,
    AgentAdvisoryArtifact,
    AgentExplorationBatch,
    AgentAdvisoryContributionRecord,
    AgentExperimentProposalScreening,
    WholePlaneColoringConstruction,
    WholePlaneColoringVerification,
    LowerBoundWitnessArtifact,
    RetainedBackgroundTheorem,
    RejectionExplanation,
    PartialAdmissionExplanation,
    QueryRecoveryExplanation,
    RepairObligation,
    ReusableNegativeEvidence,
    ConservativeEscalationExplanation,
    ResearchEvidenceCorpus,
    GraphResidentFailure,
    FailureBasisFingerprint,
    PatternSignature,
    MotifObservation,
    InvariantHypothesis,
    InvariantCandidate,
    CounterexampleObligation,
    DeadEndSignature,
    ExperimentSuppressionProof,
    ExperimentPlan,
    ExperimentBatch,
    ExperimentResult,
    DiscoveryFrontier,
    DerivedFrontierState,
    RetiredHypothesisRecord,
    ReactivationCondition,
    HadwigerResearchInvariantCatalog,
    ResearchGraphInvariantRule,
    ResearchGraphInvariantViolation,
    ResearchGraphInvariantDenial,
    ResearchGraphInvariantRegistrationPlan,
    ResearchGraphLegalityReport,
    ResearchGraphInvariantRuntimeProjection,
    ResearchGraphInvariantRegistrationChecked,
    ResearchCockpitSession,
    ResearchCockpitActionPacket,
    ResearchCockpitEquivalenceClass,
    ResearchCockpitReport,
    HadwigerCertificationBundle,
    TileEquivalenceWitness,
    CandidateScreeningInvariantCatalog,
    CandidateScreeningInvariantNode,
    CandidateScreeningEvaluation,
    CandidateScreeningEvaluationReport,
    CandidateScreeningAdvisoryArtifact,
    CandidateScreeningAdvisoryContributionRecord,
    MotifArtifact,
    TerminalForcingRelation,
    TilingCell,
    TilingGeometryCertification,
    TilingBoundaryOwnershipReport,
    TilingContactReplayReport,
    PeriodicQuotientCell,
    GeneratedPatternReplaySuite,
    GeneratedPatternReplayReport,
    TilingConflictGraph,
    ConflictCoreExtractionReport,
    TilingCandidateEquivalenceProof,
    TilingCandidateSuppressionProof,
    TilingReactivationChecked,
    TilingIterationPacket,
    TilingIterationReplayReport,
    FrontierGraphSeedArtifact,
    FrontierMotifMiningReport,
    FrontierExplorationRunReport,
    G27PressureEscapeLeadReport,
    G27MoserAnchorScanReport,
    MwisUpperBoundCertificateReplayReport,
    G27MwisFrontierClosureCampaignScoutReport,
    G27MwisFrontierClosureExactReplayReport,
    G27OutsideMoserAnchorReplayReport,
    G27QuadraticAnchorSearchReport,
    G27MutationEligibilityReport,
    G27UnitAttachmentObligationReport,
    G27RoundDecisionReport,
    G27DualUnitAnchorTestReport,
    G27TightAtomHittingSetReport,
    G27OneAnchorTransversalReport,
    G27PressureSkeletonSpindleReport,
    G27SpindleRotationSearchReport,
    G27CrossRingFusionSearchReport,
    G27ExactMoserBasisAuditReport,
    G27RotationPinClosureSearchReport,
    G27ExactRotationPinEquationReport,
    G27ExactRotationPinClosureReplayReport,
    G27RotationPinBatchExactReplayReport,
    G27RotationPinPressureScoreReport,
    G27CrossRingFusionPreflightReport,
    G27CrossRingColumnGenerationReplayReport,
    G27FiniteFractionalCoreAuditReport,
    G27DualSlackInversionReport,
    G27AlgebraicFieldFrictionReport,
    G27WCirclesExactGeometryAuditReport,
    G27WCirclesPublicArtifactInventoryReport,
    G27WCirclesSymmetryPreflightReport,
    G27WCirclesWeightedRankCutReplayReport,
    G27WCirclesRootDualCoverReplayReport,
    G27WCirclesGamma0LeafDualReplayReport,
    G27WCirclesGamma1LeafDualReplayReport,
    G27WCirclesProjectedParentLiftReplayReport,
    G27WCirclesBranchSlackLiftReplayReport,
    G27WCirclesFullTerminalExportReplayReport,
    G27WCirclesV304IncludeDualCoverReplayReport,
    G27WCirclesV304ExcludeDualCoverReplayReport,
    G27WCirclesWeightedCertificatePreflightReport,
    G27WCirclesSemanticPartitionReplayReport,
    G27WCirclesRowFamilySemanticsReplayReport,
    G27WCirclesParentLiftReadinessReplayReport,
    G27WCirclesCertificateAdmissionGapReplayReport,
    G27SameFieldPressureInterfaceSearchReport,
    FrontierResearchProjectionGraph,
}

impl HadwigerArtifactKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::GraphIdentity => "graph_identity",
            Self::GraphVersion => "graph_version",
            Self::VertexIdentity => "vertex_identity",
            Self::EdgeIdentity => "edge_identity",
            Self::EmbeddingCandidate => "embedding_candidate",
            Self::UnitDistanceVerification => "unit_distance_verification",
            Self::ColorabilityEncoding => "colorability_encoding",
            Self::SolverRun => "solver_run",
            Self::ColorabilityVerification => "colorability_verification",
            Self::UnsatCoreArtifact => "unsat_core_artifact",
            Self::GadgetDefinition => "gadget_definition",
            Self::GadgetContract => "gadget_contract",
            Self::GraphComposition => "graph_composition",
            Self::ReductionTrace => "reduction_trace",
            Self::ProofClaim => "proof_claim",
            Self::AIAdvisoryArtifact => "ai_advisory_artifact",
            Self::AgentAdvisoryArtifact => "agent_advisory_artifact",
            Self::AgentExplorationBatch => "agent_exploration_batch",
            Self::AgentAdvisoryContributionRecord => "agent_advisory_contribution_record",
            Self::AgentExperimentProposalScreening => "agent_experiment_proposal_screening",
            Self::WholePlaneColoringConstruction => "whole_plane_coloring_construction",
            Self::WholePlaneColoringVerification => "whole_plane_coloring_verification",
            Self::LowerBoundWitnessArtifact => "lower_bound_witness_artifact",
            Self::RetainedBackgroundTheorem => "retained_background_theorem",
            Self::RejectionExplanation => "rejection_explanation",
            Self::PartialAdmissionExplanation => "partial_admission_explanation",
            Self::QueryRecoveryExplanation => "query_recovery_explanation",
            Self::RepairObligation => "repair_obligation",
            Self::ReusableNegativeEvidence => "reusable_negative_evidence",
            Self::ConservativeEscalationExplanation => "conservative_escalation_explanation",
            Self::ResearchEvidenceCorpus => "research_evidence_corpus",
            Self::GraphResidentFailure => "graph_resident_failure",
            Self::FailureBasisFingerprint => "failure_basis_fingerprint",
            Self::PatternSignature => "pattern_signature",
            Self::MotifObservation => "motif_observation",
            Self::InvariantHypothesis => "invariant_hypothesis",
            Self::InvariantCandidate => "invariant_candidate",
            Self::CounterexampleObligation => "counterexample_obligation",
            Self::DeadEndSignature => "dead_end_signature",
            Self::ExperimentSuppressionProof => "experiment_suppression_proof",
            Self::ExperimentPlan => "experiment_plan",
            Self::ExperimentBatch => "experiment_batch",
            Self::ExperimentResult => "experiment_result",
            Self::DiscoveryFrontier => "discovery_frontier",
            Self::DerivedFrontierState => "derived_frontier_state",
            Self::RetiredHypothesisRecord => "retired_hypothesis_record",
            Self::ReactivationCondition => "reactivation_condition",
            Self::HadwigerResearchInvariantCatalog => "hadwiger_research_invariant_catalog",
            Self::ResearchGraphInvariantRule => "research_graph_invariant_rule",
            Self::ResearchGraphInvariantViolation => "research_graph_invariant_violation",
            Self::ResearchGraphInvariantDenial => "research_graph_invariant_denial",
            Self::ResearchGraphInvariantRegistrationPlan => {
                "research_graph_invariant_registration_plan"
            }
            Self::ResearchGraphLegalityReport => "research_graph_legality_report",
            Self::ResearchGraphInvariantRuntimeProjection => {
                "research_graph_invariant_runtime_projection"
            }
            Self::ResearchGraphInvariantRegistrationChecked => {
                "research_graph_invariant_registration_checked"
            }
            Self::ResearchCockpitSession => "research_cockpit_session",
            Self::ResearchCockpitActionPacket => "research_cockpit_action_packet",
            Self::ResearchCockpitEquivalenceClass => "research_cockpit_equivalence_class",
            Self::ResearchCockpitReport => "research_cockpit_report",
            Self::HadwigerCertificationBundle => "hadwiger_certification_bundle",
            Self::TileEquivalenceWitness => "tile_equivalence_witness",
            Self::CandidateScreeningInvariantCatalog => "candidate_screening_invariant_catalog",
            Self::CandidateScreeningInvariantNode => "candidate_screening_invariant_node",
            Self::CandidateScreeningEvaluation => "candidate_screening_evaluation",
            Self::CandidateScreeningEvaluationReport => "candidate_screening_evaluation_report",
            Self::CandidateScreeningAdvisoryArtifact => "candidate_screening_advisory_artifact",
            Self::CandidateScreeningAdvisoryContributionRecord => {
                "candidate_screening_advisory_contribution_record"
            }
            Self::MotifArtifact => "motif_artifact",
            Self::TerminalForcingRelation => "terminal_forcing_relation",
            Self::TilingCell => "tiling_cell",
            Self::TilingGeometryCertification => "tiling_geometry_certification",
            Self::TilingBoundaryOwnershipReport => "tiling_boundary_ownership_report",
            Self::TilingContactReplayReport => "tiling_contact_replay_report",
            Self::PeriodicQuotientCell => "periodic_quotient_cell",
            Self::GeneratedPatternReplaySuite => "generated_pattern_replay_suite",
            Self::GeneratedPatternReplayReport => "generated_pattern_replay_report",
            Self::TilingConflictGraph => "tiling_conflict_graph",
            Self::ConflictCoreExtractionReport => "conflict_core_extraction_report",
            Self::TilingCandidateEquivalenceProof => "tiling_candidate_equivalence_proof",
            Self::TilingCandidateSuppressionProof => "tiling_candidate_suppression_proof",
            Self::TilingReactivationChecked => "tiling_reactivation_checked",
            Self::TilingIterationPacket => "tiling_iteration_packet",
            Self::TilingIterationReplayReport => "tiling_iteration_replay_report",
            Self::FrontierGraphSeedArtifact => "frontier_graph_seed_artifact",
            Self::FrontierMotifMiningReport => "frontier_motif_mining_report",
            Self::FrontierExplorationRunReport => "frontier_exploration_run_report",
            Self::G27PressureEscapeLeadReport => "g27_pressure_escape_lead_report",
            Self::G27MoserAnchorScanReport => "g27_moser_anchor_scan_report",
            Self::MwisUpperBoundCertificateReplayReport => {
                "mwis_upper_bound_certificate_replay_report"
            }
            Self::G27MwisFrontierClosureCampaignScoutReport => {
                "g27_mwis_frontier_closure_campaign_scout_report"
            }
            Self::G27MwisFrontierClosureExactReplayReport => {
                "g27_mwis_frontier_closure_exact_replay_report"
            }
            Self::G27OutsideMoserAnchorReplayReport => "g27_outside_moser_anchor_replay_report",
            Self::G27QuadraticAnchorSearchReport => "g27_quadratic_anchor_search_report",
            Self::G27MutationEligibilityReport => "g27_mutation_eligibility_report",
            Self::G27UnitAttachmentObligationReport => "g27_unit_attachment_obligation_report",
            Self::G27RoundDecisionReport => "g27_round_decision_report",
            Self::G27DualUnitAnchorTestReport => "g27_dual_unit_anchor_test_report",
            Self::G27TightAtomHittingSetReport => "g27_tight_atom_hitting_set_report",
            Self::G27OneAnchorTransversalReport => "g27_one_anchor_transversal_report",
            Self::G27PressureSkeletonSpindleReport => "g27_pressure_skeleton_spindle_report",
            Self::G27SpindleRotationSearchReport => "g27_spindle_rotation_search_report",
            Self::G27CrossRingFusionSearchReport => "g27_cross_ring_fusion_search_report",
            Self::G27ExactMoserBasisAuditReport => "g27_exact_moser_basis_audit_report",
            Self::G27RotationPinClosureSearchReport => "g27_rotation_pin_closure_search_report",
            Self::G27ExactRotationPinEquationReport => "g27_exact_rotation_pin_equation_report",
            Self::G27ExactRotationPinClosureReplayReport => {
                "g27_exact_rotation_pin_closure_replay_report"
            }
            Self::G27RotationPinBatchExactReplayReport => {
                "g27_rotation_pin_batch_exact_replay_report"
            }
            Self::G27RotationPinPressureScoreReport => "g27_rotation_pin_pressure_score_report",
            Self::G27CrossRingFusionPreflightReport => "g27_cross_ring_fusion_preflight_report",
            Self::G27CrossRingColumnGenerationReplayReport => {
                "g27_cross_ring_column_generation_replay_report"
            }
            Self::G27FiniteFractionalCoreAuditReport => "g27_finite_fractional_core_audit_report",
            Self::G27DualSlackInversionReport => "g27_dual_slack_inversion_report",
            Self::G27AlgebraicFieldFrictionReport => "g27_algebraic_field_friction_report",
            Self::G27WCirclesExactGeometryAuditReport => {
                "g27_w_circles_exact_geometry_audit_report"
            }
            Self::G27WCirclesPublicArtifactInventoryReport => {
                "g27_w_circles_public_artifact_inventory_report"
            }
            Self::G27WCirclesSymmetryPreflightReport => "g27_w_circles_symmetry_preflight_report",
            Self::G27WCirclesWeightedRankCutReplayReport => {
                "g27_w_circles_weighted_rank_cut_replay_report"
            }
            Self::G27WCirclesRootDualCoverReplayReport => {
                "g27_w_circles_root_dual_cover_replay_report"
            }
            Self::G27WCirclesGamma0LeafDualReplayReport => {
                "g27_w_circles_gamma0_leaf_dual_replay_report"
            }
            Self::G27WCirclesGamma1LeafDualReplayReport => {
                "g27_w_circles_gamma1_leaf_dual_replay_report"
            }
            Self::G27WCirclesProjectedParentLiftReplayReport => {
                "g27_w_circles_projected_parent_lift_replay_report"
            }
            Self::G27WCirclesBranchSlackLiftReplayReport => {
                "g27_w_circles_branch_slack_lift_replay_report"
            }
            Self::G27WCirclesFullTerminalExportReplayReport => {
                "g27_w_circles_full_terminal_export_replay_report"
            }
            Self::G27WCirclesV304IncludeDualCoverReplayReport => {
                "g27_w_circles_v304_include_dual_cover_replay_report"
            }
            Self::G27WCirclesV304ExcludeDualCoverReplayReport => {
                "g27_w_circles_v304_exclude_dual_cover_replay_report"
            }
            Self::G27WCirclesWeightedCertificatePreflightReport => {
                "g27_w_circles_weighted_certificate_preflight_report"
            }
            Self::G27WCirclesSemanticPartitionReplayReport => {
                "g27_w_circles_semantic_partition_replay_report"
            }
            Self::G27WCirclesRowFamilySemanticsReplayReport => {
                "g27_w_circles_row_family_semantics_replay_report"
            }
            Self::G27WCirclesParentLiftReadinessReplayReport => {
                "g27_w_circles_parent_lift_readiness_replay_report"
            }
            Self::G27WCirclesCertificateAdmissionGapReplayReport => {
                "g27_w_circles_certificate_admission_gap_replay_report"
            }
            Self::G27SameFieldPressureInterfaceSearchReport => {
                "g27_same_field_pressure_interface_search_report"
            }
            Self::FrontierResearchProjectionGraph => "frontier_research_projection_graph",
        }
    }
}
