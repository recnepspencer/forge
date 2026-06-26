use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DerivedInvalidationProductCategory {
    MaterializedGraph,
    TraversalViews,
    LoopCycles,
    RadialRings,
    ShellViews,
    VertexDisks,
    WireViews,
    ProjectionReadStage,
    OperatorCloseout,
    CertificationBootstrap,
}

impl DerivedInvalidationProductCategory {
    pub const COVERED_ORDINARY: [Self; 8] = [
        Self::MaterializedGraph,
        Self::TraversalViews,
        Self::LoopCycles,
        Self::RadialRings,
        Self::ShellViews,
        Self::VertexDisks,
        Self::WireViews,
        Self::ProjectionReadStage,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MaterializedGraph => "materialized_graph",
            Self::TraversalViews => "traversal_views",
            Self::LoopCycles => "loop_cycles",
            Self::RadialRings => "radial_rings",
            Self::ShellViews => "shell_views",
            Self::VertexDisks => "vertex_disks",
            Self::WireViews => "wire_views",
            Self::ProjectionReadStage => "projection_read_stage",
            Self::OperatorCloseout => "operator_closeout",
            Self::CertificationBootstrap => "certification_bootstrap",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DerivedInvalidationOldAuthorityKind {
    WholeViewMaterialization,
    QueryInputMaterialization,
    TraversalInterpretation,
    ProjectionReadStage,
    OperatorDerivedBreadthCloseout,
    FallbackPolicyDenial,
    DerivedValidationDiagnostic,
    TestOnlyWholeViewFixture,
    CertificationBootstrapMaterialization,
}

impl DerivedInvalidationOldAuthorityKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WholeViewMaterialization => "whole_view_materialization",
            Self::QueryInputMaterialization => "query_input_materialization",
            Self::TraversalInterpretation => "traversal_interpretation",
            Self::ProjectionReadStage => "projection_read_stage",
            Self::OperatorDerivedBreadthCloseout => "operator_derived_breadth_closeout",
            Self::FallbackPolicyDenial => "fallback_policy_denial",
            Self::DerivedValidationDiagnostic => "derived_validation_diagnostic",
            Self::TestOnlyWholeViewFixture => "test_only_whole_view_fixture",
            Self::CertificationBootstrapMaterialization => {
                "certification_bootstrap_materialization"
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DerivedInvalidationReplacementPhase {
    PhaseTwoCatalogDeclaration,
    PhaseThreePlanSelection,
    PhaseFourExecutionReceipt,
    PhaseSixProductMigrationSweep,
    PhaseEightDeletionFirewall,
    TrueQueryCapabilityGap,
    CertificationBootstrapResidue,
}

impl DerivedInvalidationReplacementPhase {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PhaseTwoCatalogDeclaration => "phase_two_catalog_declaration",
            Self::PhaseThreePlanSelection => "phase_three_plan_selection",
            Self::PhaseFourExecutionReceipt => "phase_four_execution_receipt",
            Self::PhaseSixProductMigrationSweep => "phase_six_product_migration_sweep",
            Self::PhaseEightDeletionFirewall => "phase_eight_deletion_firewall",
            Self::TrueQueryCapabilityGap => "true_query_capability_gap",
            Self::CertificationBootstrapResidue => "certification_bootstrap_residue",
        }
    }
}
