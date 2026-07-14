use super::model::{
    WorthQueryDeclarativeCapabilityFamily as Family,
    WorthQueryDeclarativePhaseResponsibility as Phase, WorthQueryDeclarativeSurfaceClass as Class,
    WorthQueryDeclarativeSurfaceRow as Row,
};

pub(super) fn phase_eight_nine_surface_rows() -> &'static [Row] {
    ROWS
}

const ROWS: &[Row] = &[
    ordinary_fn(
        "src/ordinary/preview/mod.rs",
        "declare",
        Family::Preview,
        Phase::Declare,
    ),
    ordinary_fn(
        "src/ordinary/preview/declaration.rs",
        "declare",
        Family::Preview,
        Phase::Declare,
    ),
    ordinary_method(
        "src/ordinary/preview/declaration.rs",
        "WorthQueryReadOnlyPreviewDeclaration",
        "inspection_policy",
        Family::Preview,
        Phase::Refine,
    ),
    ordinary_method(
        "src/ordinary/preview/request.rs",
        "WorthQueryReadOnlyPreviewDeclaration",
        "using",
        Family::Preview,
        Phase::Refine,
    ),
    ordinary_method(
        "src/ordinary/preview/request.rs",
        "WorthQueryPromotionEligiblePreviewDeclaration",
        "using",
        Family::Preview,
        Phase::Refine,
    ),
    ordinary_fn(
        "src/ordinary/mutation/mod.rs",
        "declare",
        Family::Mutation,
        Phase::Declare,
    ),
    ordinary_fn(
        "src/ordinary/mutation/declaration.rs",
        "declare",
        Family::Mutation,
        Phase::Declare,
    ),
    ordinary_method(
        "src/ordinary/mutation/declaration.rs",
        "WorthQueryMutationDeclaration",
        "inspection_policy",
        Family::Mutation,
        Phase::Refine,
    ),
    ordinary_method(
        "src/ordinary/mutation/request.rs",
        "WorthQueryMutationDeclaration",
        "using",
        Family::Mutation,
        Phase::Refine,
    ),
    ordinary_method(
        "src/ordinary/mutation/execution.rs",
        "WorthQueryMutationRequest",
        "run",
        Family::Mutation,
        Phase::Execute,
    ),
    ordinary_fn(
        "src/ordinary/workflow/mod.rs",
        "declare",
        Family::Workflow,
        Phase::Declare,
    ),
    ordinary_fn(
        "src/ordinary/workflow/declaration.rs",
        "declare",
        Family::Workflow,
        Phase::Declare,
    ),
    ordinary_method(
        "src/ordinary/workflow/declaration.rs",
        "WorthQueryWorkflowDeclaration",
        "inspection_policy",
        Family::Workflow,
        Phase::Refine,
    ),
    ordinary_method(
        "src/ordinary/workflow/request.rs",
        "WorthQueryWorkflowDeclaration",
        "using",
        Family::Workflow,
        Phase::Refine,
    ),
    ordinary_method(
        "src/ordinary/workflow/execution.rs",
        "WorthQueryWorkflowRequest",
        "run",
        Family::Workflow,
        Phase::Execute,
    ),
    ordinary_fn(
        "src/ordinary/workflow/mod.rs",
        "declare_writeback",
        Family::Workflow,
        Phase::Declare,
    ),
    ordinary_fn(
        "src/ordinary/workflow/writeback/mod.rs",
        "declare_writeback",
        Family::Workflow,
        Phase::Declare,
    ),
    ordinary_fn(
        "src/ordinary/workflow/writeback/declaration.rs",
        "declare_writeback",
        Family::Workflow,
        Phase::Declare,
    ),
    ordinary_method(
        "src/ordinary/workflow/writeback/declaration.rs",
        "WorthQueryWritebackDeclaration",
        "using",
        Family::Workflow,
        Phase::Refine,
    ),
    ordinary_method(
        "src/ordinary/workflow/writeback/execution.rs",
        "WorthQueryWritebackRequest",
        "run",
        Family::Workflow,
        Phase::Execute,
    ),
    ordinary_fn(
        "src/ordinary/workflow/mod.rs",
        "declare_branch_merge",
        Family::Workflow,
        Phase::Declare,
    ),
    ordinary_fn(
        "src/ordinary/workflow/branch_merge/mod.rs",
        "declare_branch_merge",
        Family::Workflow,
        Phase::Declare,
    ),
    ordinary_fn(
        "src/ordinary/workflow/branch_merge/declaration.rs",
        "declare_branch_merge",
        Family::Workflow,
        Phase::Declare,
    ),
    ordinary_method(
        "src/ordinary/workflow/branch_merge/declaration.rs",
        "WorthQueryBranchMergeDeclaration",
        "using",
        Family::Workflow,
        Phase::Refine,
    ),
    ordinary_method(
        "src/ordinary/workflow/branch_merge/execution.rs",
        "WorthQueryBranchMergeRequest",
        "run",
        Family::Workflow,
        Phase::Execute,
    ),
    ordinary_method(
        "src/domain_installation/capabilities/mutation_workflow.rs",
        "WorthQueryInstalledDomainHandle",
        "mutation",
        Family::DomainExtension,
        Phase::Declare,
    ),
    ordinary_method(
        "src/domain_installation/capabilities/mutation_workflow.rs",
        "WorthQueryInstalledDomainWorkflowDeclaration",
        "using",
        Family::DomainExtension,
        Phase::Refine,
    ),
    ordinary_method(
        "src/domain_installation/capabilities/mutation_workflow.rs",
        "WorthQueryInstalledDomainWorkflowRequest",
        "run",
        Family::DomainExtension,
        Phase::Execute,
    ),
    ordinary_fn(
        "src/ordinary/inspection/mod.rs",
        "inspection_basis",
        Family::Inspection,
        Phase::Refine,
    ),
    ordinary_fn(
        "src/ordinary/inspection/context.rs",
        "inspection_basis",
        Family::Inspection,
        Phase::Refine,
    ),
    ordinary_fn(
        "src/ordinary/inspection/mod.rs",
        "declare",
        Family::Inspection,
        Phase::Declare,
    ),
    ordinary_fn(
        "src/ordinary/inspection/declaration.rs",
        "declare",
        Family::Inspection,
        Phase::Declare,
    ),
    ordinary_method(
        "src/ordinary/inspection/declaration.rs",
        "WorthQueryInspectionDeclaration",
        "using",
        Family::Inspection,
        Phase::Refine,
    ),
    ordinary_method(
        "src/ordinary/inspection/execution.rs",
        "WorthQueryInspectionRequest",
        "run",
        Family::Inspection,
        Phase::Inspect,
    ),
];

const fn ordinary_fn(
    path: &'static str,
    function: &'static str,
    family: Family,
    phase: Phase,
) -> Row {
    Row::new(
        path,
        function,
        family,
        phase,
        Class::OrdinaryDeclaration,
        Class::OrdinaryDeclaration,
        "ordinary capability consumer",
        "capability-owned declarative journey",
    )
}

const fn ordinary_method(
    path: &'static str,
    owner: &'static str,
    function: &'static str,
    family: Family,
    phase: Phase,
) -> Row {
    Row::method(
        path,
        owner,
        function,
        family,
        phase,
        Class::OrdinaryDeclaration,
        Class::OrdinaryDeclaration,
        "ordinary capability consumer",
        "capability-owned declarative journey",
    )
}
