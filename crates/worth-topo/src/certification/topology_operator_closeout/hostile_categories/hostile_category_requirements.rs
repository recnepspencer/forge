use super::hostile_category_types::MilestoneThreeHostileCertificationCategory;

pub(super) fn required_hostile_certification_categories(
) -> &'static [MilestoneThreeHostileCertificationCategory] {
    &[
        MilestoneThreeHostileCertificationCategory::MutationPipelineIntegrity,
        MilestoneThreeHostileCertificationCategory::PrimitiveTopologyFamilyClosure,
        MilestoneThreeHostileCertificationCategory::OperatorBrutality,
        MilestoneThreeHostileCertificationCategory::QueryTraversalBrutality,
        MilestoneThreeHostileCertificationCategory::NonManifoldRadialBrutality,
        MilestoneThreeHostileCertificationCategory::DegeneracyCorruptionLocalization,
        MilestoneThreeHostileCertificationCategory::DeterminismOrderAssault,
        MilestoneThreeHostileCertificationCategory::DiagnosticsFailureTaxonomy,
        MilestoneThreeHostileCertificationCategory::ScaleDepthSustainedPressure,
    ]
}

pub(super) fn partial_status_allowed(category: MilestoneThreeHostileCertificationCategory) -> bool {
    matches!(
        category,
        MilestoneThreeHostileCertificationCategory::PrimitiveTopologyFamilyClosure
    )
}

pub(in crate::certification::topology_operator_closeout) fn milestone_three_expected_primitive_family_labels(
) -> &'static [&'static str] {
    &[
        "WireOpen(n)",
        "WireClosed(n)",
        "SheetDisk(n)",
        "SheetPatch(f)",
        "SolidShell(f)",
        "NmtEdgeFan(k)",
    ]
}
