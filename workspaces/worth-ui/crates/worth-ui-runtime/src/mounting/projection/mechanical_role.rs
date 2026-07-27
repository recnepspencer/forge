pub(super) fn mechanical_role(
    operator: crate::declaration::UiDeclarationPlanningOperatorKind,
) -> worth_ui_host_contract::UiMountedMechanicalRole {
    use crate::declaration::UiDeclarationPlanningOperatorKind as Operator;
    use worth_ui_host_contract::UiMountedMechanicalRole as Role;

    match operator {
        Operator::PageRoot | Operator::PageSet => Role::Surface,
        Operator::Control => Role::Control,
        Operator::DiagnosticSurface => Role::Diagnostic,
        Operator::PortalAnchor => Role::Portal,
        Operator::Region
        | Operator::Mosaic
        | Operator::LocalComposition
        | Operator::Stack
        | Operator::Row
        | Operator::Grid
        | Operator::Split
        | Operator::Overlay
        | Operator::Scroll => Role::Container,
    }
}
