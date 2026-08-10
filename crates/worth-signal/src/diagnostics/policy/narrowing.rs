use super::definition::{
    FrontierCyclePolicy, FrontierPropagationPolicy, FrontierTracingPolicy, ParallelAdmissionPolicy,
    ReconstructionBudget, ReplayDetailPolicy, RetentionBudget, SemanticRetentionPolicy,
    SignalRuntimePolicy, SnapshotRestoreLineageMode,
};
use super::materialization::ArtifactRetentionPolicy;
use crate::data::node::{ArtifactPolicyClass, AuthorityPolicy, PathClass};
use crate::data::performance::ResolvedPerformancePolicy;
use crate::diagnostics::profile::DiagnosticsTier;

impl RetentionBudget {
    pub fn operational() -> Self {
        Self {
            history_limit: super::definition::HistoryLimit::new(4),
            detail_limit: super::definition::DetailLimit::new(16),
            retain_history_details: false,
            retain_flow_explanation: false,
            retain_latest_failure_context: false,
            retain_stage_details: false,
            capture_forensic_failure_context: false,
            explanation_retention: ArtifactRetentionPolicy::Reconstruct,
            provenance_retention: ArtifactRetentionPolicy::Reconstruct,
            replay_detail: ReplayDetailPolicy::Minimal,
            semantic_detail: SemanticRetentionPolicy::Minimal,
        }
    }

    pub fn development() -> Self {
        Self {
            history_limit: super::definition::HistoryLimit::new(16),
            detail_limit: super::definition::DetailLimit::new(64),
            retain_history_details: true,
            retain_flow_explanation: true,
            retain_latest_failure_context: true,
            retain_stage_details: true,
            capture_forensic_failure_context: false,
            explanation_retention: ArtifactRetentionPolicy::Retain,
            provenance_retention: ArtifactRetentionPolicy::Retain,
            replay_detail: ReplayDetailPolicy::Standard,
            semantic_detail: SemanticRetentionPolicy::Development,
        }
    }

    pub fn forensic() -> Self {
        Self {
            history_limit: super::definition::HistoryLimit::new(64),
            detail_limit: super::definition::DetailLimit::new(256),
            retain_history_details: true,
            retain_flow_explanation: true,
            retain_latest_failure_context: true,
            retain_stage_details: true,
            capture_forensic_failure_context: true,
            explanation_retention: ArtifactRetentionPolicy::Retain,
            provenance_retention: ArtifactRetentionPolicy::Retain,
            replay_detail: ReplayDetailPolicy::Forensic,
            semantic_detail: SemanticRetentionPolicy::Forensic,
        }
    }

    pub fn for_tier(tier: DiagnosticsTier) -> Self {
        match tier {
            DiagnosticsTier::Operational => Self::operational(),
            DiagnosticsTier::Development => Self::development(),
            DiagnosticsTier::Forensic => Self::forensic(),
        }
    }
}

impl ReconstructionBudget {
    pub fn operational() -> Self {
        Self {
            allow_explanation_reconstruction: true,
            allow_provenance_reconstruction: true,
            allow_replay_reconstruction: false,
            allow_certification_materialization: false,
        }
    }

    pub fn development() -> Self {
        Self {
            allow_explanation_reconstruction: true,
            allow_provenance_reconstruction: true,
            allow_replay_reconstruction: false,
            allow_certification_materialization: true,
        }
    }

    pub fn forensic() -> Self {
        Self {
            allow_explanation_reconstruction: true,
            allow_provenance_reconstruction: true,
            allow_replay_reconstruction: true,
            allow_certification_materialization: true,
        }
    }

    pub fn for_tier(tier: DiagnosticsTier) -> Self {
        match tier {
            DiagnosticsTier::Operational => Self::operational(),
            DiagnosticsTier::Development => Self::development(),
            DiagnosticsTier::Forensic => Self::forensic(),
        }
    }
}

impl Default for ParallelAdmissionPolicy {
    fn default() -> Self {
        Self {
            operational_min_parallel_tasks: 2,
            development_min_parallel_tasks: 4,
            forensic_min_parallel_tasks: 8,
            full_parallel_min_tasks: 8,
        }
    }
}

impl ParallelAdmissionPolicy {
    pub fn min_parallel_tasks_for(self, tier: DiagnosticsTier) -> usize {
        match tier {
            DiagnosticsTier::Operational => self.operational_min_parallel_tasks,
            DiagnosticsTier::Development => self.development_min_parallel_tasks,
            DiagnosticsTier::Forensic => self.forensic_min_parallel_tasks,
        }
    }
}

impl Default for SignalRuntimePolicy {
    fn default() -> Self {
        Self::development()
    }
}

impl SignalRuntimePolicy {
    pub fn default_path_class(self) -> crate::data::node::PathClass {
        match self.tier {
            DiagnosticsTier::Operational => PathClass::Operational,
            DiagnosticsTier::Development | DiagnosticsTier::Forensic => PathClass::Rich,
        }
    }

    pub fn default_artifact_policy_class(self) -> crate::data::node::ArtifactPolicyClass {
        match (
            self.tier,
            self.retention_budget.explanation_retention,
            self.retention_budget.provenance_retention,
        ) {
            (
                DiagnosticsTier::Development,
                ArtifactRetentionPolicy::Retain,
                ArtifactRetentionPolicy::Retain,
            ) => ArtifactPolicyClass::DevelopmentRetained,
            (DiagnosticsTier::Forensic, _, _) => ArtifactPolicyClass::ForensicReconstructable,
            _ => ArtifactPolicyClass::OperationalMinimal,
        }
    }

    pub fn default_execution_strategy(self) -> crate::logic::planner::ResolvedExecutionStrategy {
        match self.tier {
            DiagnosticsTier::Operational => {
                crate::logic::planner::ResolvedExecutionStrategy::SparseIncremental
            }
            DiagnosticsTier::Development | DiagnosticsTier::Forensic => {
                crate::logic::planner::ResolvedExecutionStrategy::DenseStageBatched
            }
        }
    }

    pub fn default_maintenance_strategy(
        self,
    ) -> crate::logic::planner::ResolvedMaintenanceStrategy {
        match self.tier {
            DiagnosticsTier::Operational => {
                crate::logic::planner::ResolvedMaintenanceStrategy::DensityAdaptive
            }
            DiagnosticsTier::Development => {
                crate::logic::planner::ResolvedMaintenanceStrategy::Incremental
            }
            DiagnosticsTier::Forensic => {
                crate::logic::planner::ResolvedMaintenanceStrategy::Rebuild
            }
        }
    }

    pub fn default_authority_policy(self) -> crate::data::node::AuthorityPolicy {
        AuthorityPolicy::SpeculativeThenReconcile
    }

    pub fn resolve_performance_policy(self) -> ResolvedPerformancePolicy {
        ResolvedPerformancePolicy {
            path_class: self.default_path_class(),
            artifact_policy: self.default_artifact_policy_class(),
            execution_strategy: self.default_execution_strategy(),
            maintenance_strategy: self.default_maintenance_strategy(),
            authority_policy: self.default_authority_policy(),
        }
    }

    pub fn for_tier(tier: DiagnosticsTier) -> Self {
        let snapshot_restore_lineage_mode = match tier {
            DiagnosticsTier::Forensic => SnapshotRestoreLineageMode::PerNode,
            DiagnosticsTier::Operational | DiagnosticsTier::Development => {
                SnapshotRestoreLineageMode::CompactGlobal
            }
        };
        let frontier_tracing_policy = match tier {
            DiagnosticsTier::Operational => FrontierTracingPolicy::SummaryOnly,
            DiagnosticsTier::Development => FrontierTracingPolicy::RetainWaveRecords,
            DiagnosticsTier::Forensic => FrontierTracingPolicy::FullForensic,
        };

        Self {
            tier,
            retention_budget: RetentionBudget::for_tier(tier),
            reconstruction_budget: ReconstructionBudget::for_tier(tier),
            snapshot_restore_lineage_mode,
            frontier_tracing_policy,
            frontier_propagation_policy: FrontierPropagationPolicy::CanonicalFrontier,
            frontier_cycle_policy: FrontierCyclePolicy::ReachableCycleCheck,
            parallel_admission: ParallelAdmissionPolicy::default(),
        }
    }

    pub fn operational() -> Self {
        Self::for_tier(DiagnosticsTier::Operational)
    }

    pub fn development() -> Self {
        Self::for_tier(DiagnosticsTier::Development)
    }

    pub fn forensic() -> Self {
        Self::for_tier(DiagnosticsTier::Forensic)
    }

    pub fn web_development() -> Self {
        Self::operational().with_parallel_admission(ParallelAdmissionPolicy {
            operational_min_parallel_tasks: 4,
            development_min_parallel_tasks: 8,
            forensic_min_parallel_tasks: 12,
            full_parallel_min_tasks: 16,
        })
    }

    pub fn kernel() -> Self {
        Self::forensic().with_parallel_admission(ParallelAdmissionPolicy {
            operational_min_parallel_tasks: 4,
            development_min_parallel_tasks: 8,
            forensic_min_parallel_tasks: 16,
            full_parallel_min_tasks: 16,
        })
    }

    pub fn fintech() -> Self {
        let mut policy = Self::development();
        policy.retention_budget.replay_detail = ReplayDetailPolicy::Forensic;
        policy.with_parallel_admission(ParallelAdmissionPolicy {
            operational_min_parallel_tasks: 4,
            development_min_parallel_tasks: 8,
            forensic_min_parallel_tasks: 12,
            full_parallel_min_tasks: 12,
        })
    }

    pub fn game_engine() -> Self {
        Self::operational().with_parallel_admission(ParallelAdmissionPolicy {
            operational_min_parallel_tasks: 2,
            development_min_parallel_tasks: 4,
            forensic_min_parallel_tasks: 8,
            full_parallel_min_tasks: 8,
        })
    }

    pub fn with_explanation_retention(mut self, retention: ArtifactRetentionPolicy) -> Self {
        self.retention_budget.explanation_retention = retention;
        self
    }

    pub fn with_provenance_retention(mut self, retention: ArtifactRetentionPolicy) -> Self {
        self.retention_budget.provenance_retention = retention;
        self
    }

    pub fn with_replay_detail(mut self, replay_detail: ReplayDetailPolicy) -> Self {
        self.retention_budget.replay_detail = replay_detail;
        self
    }

    pub fn with_semantic_retention(mut self, semantic_retention: SemanticRetentionPolicy) -> Self {
        self.retention_budget.semantic_detail = semantic_retention;
        self
    }

    pub fn with_snapshot_restore_lineage_mode(mut self, mode: SnapshotRestoreLineageMode) -> Self {
        self.snapshot_restore_lineage_mode = mode;
        self
    }

    pub fn with_parallel_admission(mut self, parallel_admission: ParallelAdmissionPolicy) -> Self {
        self.parallel_admission = parallel_admission;
        self
    }

    pub fn with_history_limit(mut self, history_limit: usize) -> Self {
        self.retention_budget.history_limit = super::definition::HistoryLimit::new(history_limit);
        self
    }

    pub fn with_detail_limit(mut self, detail_limit: usize) -> Self {
        self.retention_budget.detail_limit = super::definition::DetailLimit::new(detail_limit);
        self
    }

    pub fn with_history_details(mut self, retain_history_details: bool) -> Self {
        self.retention_budget.retain_history_details = retain_history_details;
        self
    }

    pub fn retains_explanation_facts(self) -> bool {
        matches!(
            self.retention_budget.explanation_retention,
            ArtifactRetentionPolicy::Retain
        )
    }

    pub fn retains_provenance_facts(self) -> bool {
        matches!(
            self.retention_budget.provenance_retention,
            ArtifactRetentionPolicy::Retain
        )
    }

    pub fn can_reconstruct_explanation(self) -> bool {
        !matches!(
            self.retention_budget.explanation_retention,
            ArtifactRetentionPolicy::Omit
        ) && self.reconstruction_budget.allow_explanation_reconstruction
    }

    pub fn can_reconstruct_provenance(self) -> bool {
        !matches!(
            self.retention_budget.provenance_retention,
            ArtifactRetentionPolicy::Omit
        ) && self.reconstruction_budget.allow_provenance_reconstruction
    }

    pub fn explanation_behavior_summary(self) -> &'static str {
        self.retention_budget.explanation_retention.description()
    }

    pub fn provenance_behavior_summary(self) -> &'static str {
        self.retention_budget.provenance_retention.description()
    }
}
